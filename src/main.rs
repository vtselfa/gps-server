#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate chrono;
extern crate gps_lib;
extern crate lazy_static;
extern crate log;
extern crate regex;
extern crate rust_decimal;
extern crate serde_json;
extern crate strum;
extern crate superslice;
extern crate thiserror;
extern crate xml;
extern crate yaserde;
extern crate yaserde_derive;
#[macro_use]
extern crate num_derive;

mod action;
mod actions;
mod currency;
mod money;
mod types;
mod utils;
// mod action_codes;

use chrono::prelude::*;
use log::{error, info, warn};
use rocket::data::{self, FromDataSimple};
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::response;
use rocket::response::content::Content;
use rocket::{Data, Outcome::*, Request};
use rocket_contrib::json::Json;
use std::io::Read;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use action::Action;
use actions::activate::Activate;
use actions::balance_adjustment::BalanceAdjustment;
use actions::balance_enquiry::BalanceEnquiry;
use actions::balance_enquiry::BalanceEnquiryV2;
use actions::card_change_groups::CardChangeGroups;
use actions::card_statement::CardStatement;
use actions::create_card::CreateCard;
use actions::enquiry::Enquiry;
use actions::load::Load;
use actions::result::ResultV2;
use actions::status_change::StatusChange;
use actions::unload::Unload;
use types::GpsError;

struct PostStr {
    pub action: Box<dyn action::Action>,
}

impl PostStr {
    fn response(&self, state: &types::State) -> Result<String, types::GpsError> {
        let (action_code, response_sent) = match self.action.execute(state) {
            Ok(res) => (format!("000"), res),
            Err(error) => {
                warn!("Error {:?}", error);
                (
                    utils::error_to_action_code(&error),
                    self.action.report_not_successful(&error)?,
                )
            }
        };

        // Register the WSID and its result to be able to i) ensure that it's not used again,
        // and ii) be able to implement Ws_WebServiceResult_V2
        if let Some(wsid) = self.action.get_wsid() {
            let result = types::ActionResult {
                timestamp: Utc::now(),
                action_name: self.action.get_action_name().to_string(),
                action_code,
                response_sent: response_sent.clone(),
            };
            state
                .wsids
                .write()
                .expect("WSID lock poisoned")
                .insert(wsid, result);
        }
        Ok(response_sent)
    }

    fn from_data_impl(req: &Request, data: Data) -> Result<Self, types::GpsError> {
        const MAX_SIZE: usize = 4096;

        match req.headers().get("Content-Length").next() {
            None => {
                return Err(GpsError::ContentLength(
                    "Missing content-length header".to_string(),
                ))
            }
            Some(header) => match header.parse::<usize>() {
                Err(_) => {
                    return Err(GpsError::ContentLength(
                        "Malformed content-length header".to_string(),
                    ))
                }
                Ok(content_length) => {
                    if content_length > MAX_SIZE {
                        return Err(GpsError::ContentLength(
                            "Incoming data too long".to_string(),
                        ));
                    }
                }
            },
        };

        let mut buffer: [u8; MAX_SIZE] = [0; MAX_SIZE];
        let mut offset = 0;
        let mut stream = data.open();
        loop {
            let read_bytes = stream.read(&mut buffer[offset..])?;
            if read_bytes == 0 {
                break; // EOF
            }
            offset = offset + read_bytes;
        }
        let contents = std::str::from_utf8(&buffer)?.to_string();

        let action = match req.headers().get_one("soapaction") {
            None => {
                return Err(GpsError::RequestData(
                    "No soapaction SOAP header found".to_string(),
                ))
            }
            Some(a) => a,
        };

        match action::extract_name(action) {
            Ok(action_name) => match action_name {
                "Ws_CreateCard" => Ok(PostStr {
                    action: Box::new(CreateCard::new(action_name, &contents)?),
                }),
                "Ws_BalanceAdjustment" => Ok(PostStr {
                    action: Box::new(BalanceAdjustment::new(action_name, &contents)?),
                }),
                "Ws_Load" => Ok(PostStr {
                    action: Box::new(Load::new(action_name, &contents)?),
                }),
                "Ws_UnLoad" => Ok(PostStr {
                    // Yes, UnLoad, because GPS sucks
                    action: Box::new(Unload::new(action_name, &contents)?),
                }),
                "Ws_Balance_Enquiry" => Ok(PostStr {
                    action: Box::new(BalanceEnquiry::new(action_name, &contents)?),
                }),
                "Ws_Balance_Enquiry_V2" => Ok(PostStr {
                    action: Box::new(BalanceEnquiryV2::new(action_name, &contents)?),
                }),
                "Ws_Enquiry" => Ok(PostStr {
                    action: Box::new(Enquiry::new(action_name, &contents)?),
                }),
                "Ws_Card_Statement" => Ok(PostStr {
                    action: Box::new(CardStatement::new(action_name, &contents)?),
                }),
                "Ws_WebServiceResult_V2" => Ok(PostStr {
                    action: Box::new(ResultV2::new(action_name, &contents)?),
                }),
                "Ws_StatusChange" => Ok(PostStr {
                    action: Box::new(StatusChange::new(action_name, &contents)?),
                }),
                "Ws_Card_Change_Groups" => Ok(PostStr {
                    action: Box::new(CardChangeGroups::new(action_name, &contents)?),
                }),
                "Ws_Activate" => Ok(PostStr {
                    action: Box::new(Activate::new(action_name, &contents)?),
                }),
                _ => Err(GpsError::Action(format!(
                    "Action {} not implemented",
                    action
                ))),
            },
            Err(error) => Err(error),
        }
    }
}

impl FromDataSimple for PostStr {
    type Error = types::GpsError;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, Self::Error> {
        match PostStr::from_data_impl(req, data) {
            Err(e) => {
                error!("Could not parse request: {}", e);
                Failure((Status::BadRequest, e))
            }
            Ok(v) => Success(v),
        }
    }
}

#[post("/hello", data = "<input>")]
fn server_post(input: PostStr, state: rocket::State<types::State>) -> Content<String> {
    let _guard = state.replacing_state.read().unwrap();

    let state: &types::State = state.inner();
    let content_type = ContentType::parse_flexible("application/soap+xml; charset=utf-8").unwrap();
    match input.response(state) {
        Err(e) => {
            error!("{}", e);
            Content(content_type, e.to_string())
        }
        Ok(v) => Content(content_type.clone(), v),
    }
}

#[get("/state")]
fn get_state(
    state: rocket::State<types::State>,
) -> Result<response::content::Json<String>, response::status::Custom<String>> {
    let _guard = state.replacing_state.read().unwrap();

    let state: &types::State = state.inner();
    match serde_json::to_string(state) {
        Err(e) => {
            let msg = format!("Could not serialize program state: {}", e);
            error!("{}", msg);
            Err(response::status::Custom::<String>(
                Status::InternalServerError,
                msg,
            ))
        }
        Ok(v) => Ok(response::content::Json(v)),
    }
}

#[post("/state", format = "json", data = "<new_state>")]
fn post_state(
    new_state: Json<types::State>,
    state: rocket::State<types::State>,
) -> rocket::http::Status {
    *state.replacing_state.write().unwrap() = true;

    let state = state.inner();
    state.next_public_token.swap(
        new_state.next_public_token.load(Ordering::SeqCst),
        Ordering::SeqCst,
    );
    state.next_item_id.swap(
        new_state.next_item_id.load(Ordering::SeqCst),
        Ordering::SeqCst,
    );
    *state.wsids.write().unwrap() = new_state.wsids.read().unwrap().clone();
    *state.public_tokens.write().unwrap() = new_state.public_tokens.read().unwrap().clone();
    *state.transactions.write().unwrap() = new_state.transactions.read().unwrap().clone();

    *state.replacing_state.write().unwrap() = false;
    Status::Ok
}

#[delete("/state")]
fn delete_state(state: rocket::State<types::State>) -> rocket::http::Status {
    *state.replacing_state.write().unwrap() = true;

    let state = state.inner();
    state.next_public_token.swap(0, Ordering::SeqCst);
    state.next_item_id.store(0, Ordering::SeqCst);
    state.wsids.write().unwrap().clear();
    state.public_tokens.write().unwrap().clear();
    state.transactions.write().unwrap().clear();

    *state.replacing_state.write().unwrap() = false;
    Status::Ok
}

#[post("/state/card", format = "json", data = "<cards>")]
fn post_cards(
    cards: Json<Vec<types::Card>>,
    state: rocket::State<types::State>,
) -> rocket::http::Status {
    for card in cards.iter() {
        info!("Added card with public token: {}", card.public_token);
        state
            .public_tokens
            .write()
            .expect("Public tokens lock poisoned")
            .insert(card.public_token.clone(), card.clone());
    }
    Status::Ok
}

fn main() {
    if let Err(err) = log4rs::init_file("log4rs.yml", Default::default()) {
        warn!("Unable to find log4rs.yml logging config: {}", err);
    }

    rocket::ignite()
        .manage(types::State {
            next_item_id: AtomicUsize::new(1),
            next_public_token: AtomicUsize::new(100000000), // Public tokens are 9 digits, so better start with 1 than with 0
            ..Default::default()
        })
        .mount("/", routes![server_post])
        .mount("/", routes![get_state])
        .mount("/", routes![post_state])
        .mount("/", routes![delete_state])
        .mount("/", routes![post_cards])
        .launch();
}
