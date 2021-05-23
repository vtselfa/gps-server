#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate regex;
extern crate lazy_static;
extern crate log;
extern crate xml;
extern crate yaserde;
extern crate yaserde_derive;
extern crate gps_lib;
extern crate chrono;
extern crate thiserror;
extern crate rust_decimal;
extern crate serde_json;
extern crate superslice;
extern crate strum;

mod action;
mod types;
mod utils;
mod actions;
mod currency;
mod money;

use rocket::data::{self, FromDataSimple};
use rocket::http::Status;
use rocket::{Data, Outcome::*, Request};
use rocket_contrib::json::Json;
use rocket::response;
use std::io::Read;
use log::{error, warn};
use std::sync::atomic::Ordering;

use action::Action;
use actions::create_card::CreateCard;
use actions::balance_adjustment::BalanceAdjustment;
use actions::load::Load;
use actions::unload::Unload;
use actions::balance_enquiry::BalanceEnquiry;
use actions::balance_enquiry::BalanceEnquiryV2;
use actions::enquiry::Enquiry;
use actions::card_statement::CardStatement;
use types::GpsError;


struct PostStr {
    pub action: Box<dyn action::Action>,
}

impl PostStr {
    fn response(&self, state: &types::State) -> Result<String, types::GpsError> {
        self.action.execute(state)
    }

    fn from_data_impl(req: &Request, data: Data) -> Result<Self, types::GpsError> {
        println!("{:?}", req.headers());

        const MAX_SIZE: usize = 4096;

        match req.headers().get("Content-Length").next() {
            None => return Err(GpsError::ContentLength("Missing content-length header".to_string())),
            Some(header) => match header.parse::<usize>() {
                Err(_) => return Err(GpsError::ContentLength("Malformed content-length header".to_string())),
                Ok(content_length) => {
                    if content_length > MAX_SIZE {
                        return Err(GpsError::ContentLength("Incoming data too long".to_string()));
                    }
                }
            }
        };

        let mut buffer: [u8; MAX_SIZE] = [0; MAX_SIZE];
        let mut offset = 0;
        let mut stream = data.open();
        loop {
            let read_bytes = stream.read(&mut buffer[offset ..])?;
            if read_bytes == 0 {
                break; // EOF
            }
            offset = offset + read_bytes;
        }
        let contents = std::str::from_utf8(&buffer)?.to_string();

        let action = match req.headers().get_one("soapaction") {
            None =>  return Err(GpsError::RequestData("No soapaction SOAP header found".to_string())),
            Some(a) => a,
        };

        println!("Action: {}", action);

        match action::extract_name(action)? {
            "Ws_CreateCard"        => Ok(PostStr{action: Box::new(CreateCard::new(&contents)?)}),
            "Ws_BalanceAdjustment" => Ok(PostStr{action: Box::new(BalanceAdjustment::new(&contents)?)}),
            "Ws_Load"              => Ok(PostStr{action: Box::new(Load::new(&contents)?)}),
            "Ws_Unload"            => Ok(PostStr{action: Box::new(Unload::new(&contents)?)}),
            "Ws_Balance_Enquiry"   => Ok(PostStr{action: Box::new(BalanceEnquiry::new(&contents)?)}),
            "Ws_Balance_Enquiry_V2"   => Ok(PostStr{action: Box::new(BalanceEnquiryV2::new(&contents)?)}),
            "Ws_Enquiry"              => Ok(PostStr{action: Box::new(Enquiry::new(&contents)?)}),
            "Ws_Card_Statement"       => Ok(PostStr{action: Box::new(CardStatement::new(&contents)?)}),
            _ => Err(GpsError::Action(format!("Action {} not implemented", action))),
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
            },
            Ok(v) => Success(v),
        }
    }
}

#[post("/hello", data = "<input>")]
fn server_post(input: PostStr, state: rocket::State<types::State>) -> String {
    let _guard = state.replacing_state.read().unwrap();

    let state: &types::State = state.inner();
    match input.response(state) {
        Err(e) => {
            error!("{}", e);
            e.to_string()
        }
        Ok(v) => v,
    }
}

#[get("/state")]
fn get_state(state: rocket::State<types::State>)
-> Result<response::content::Json<String>, response::status::Custom<String>> {
    let _guard = state.replacing_state.read().unwrap();

    let state: &types::State = state.inner();
    match serde_json::to_string(state) {
        Err(e) => {
            let msg = format!("Could not serialize program state: {}", e);
            error!("{}", msg);
            Err(response::status::Custom::<String>(Status::InternalServerError, msg))
        }
        Ok(v) => Ok(response::content::Json(v)),
    }
}

#[post("/state", format = "json", data = "<new_state>")]
fn post_state(new_state: Json<types::State>, state: rocket::State<types::State>)
-> rocket::http::Status {
    *state.replacing_state.write().unwrap() = true;

    let state = state.inner();
    state.next_public_token.swap(new_state.next_public_token.load(Ordering::SeqCst), Ordering::SeqCst);
    state.next_item_id.swap(new_state.next_item_id.load(Ordering::SeqCst), Ordering::SeqCst);
    *state.wsids.write().unwrap() = new_state.wsids.read().unwrap().clone();
    *state.public_tokens.write().unwrap() = new_state.public_tokens.read().unwrap().clone();
    *state.transactions.write().unwrap() = new_state.transactions.read().unwrap().clone();

    *state.replacing_state.write().unwrap() = false;
    Status::Ok
}

#[delete("/state")]
fn delete_state(state: rocket::State<types::State>)
-> rocket::http::Status {
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

fn main() {
    if let Err(err) = log4rs::init_file("log4rs.yml", Default::default()) {
        warn!("Unable to find log4rs.yml logging config: {}", err);
    }

    rocket::ignite()
        .manage(types::State{..Default::default()})
        .mount("/", routes![server_post])
        .mount("/", routes![get_state])
        .mount("/", routes![post_state])
        .mount("/", routes![delete_state])
        .launch();
}
