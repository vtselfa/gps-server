#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate regex;
extern crate log;
extern crate xml;
extern crate yaserde;
extern crate yaserde_derive;
extern crate gps_lib;
extern crate chrono;
extern crate rusty_money;
extern crate thiserror;
extern crate rust_decimal;

mod action;
mod create_card;
mod balance_adjustment;
mod load;
mod types;

use rocket::data::{self, FromDataSimple};
use rocket::http::Status;
use rocket::{Data, Outcome::*, Request};
use std::io::Read;

use action::Action;
use create_card::CreateCard;
use balance_adjustment::BalanceAdjustment;
use load::Load;
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
            "Ws_CreateCard" => Ok(PostStr{action: Box::new(CreateCard::new(&contents)?)}),
            "Ws_BalanceAdjustment" => Ok(PostStr{action: Box::new(BalanceAdjustment::new(&contents)?)}),
            "Ws_Load" => Ok(PostStr{action: Box::new(Load::new(&contents)?)}),
            _ => Err(GpsError::Action(format!("Action {} not implemented", action))),
        }
    }
}

impl FromDataSimple for PostStr {
    type Error = types::GpsError;


    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, Self::Error> {
        match PostStr::from_data_impl(req, data) {
            Err(e) => Failure((Status::BadRequest, e)),
            Ok(v) => Success(v),
        }
    }
}

#[post("/hello", format = "text/xml", data = "<input>")]
fn server_post(input: PostStr, state: rocket::State<types::State>) -> String {
    let state: &types::State = state.inner();
    match input.response(state) {
        Err(e) => {
            println!("{}", e);
            e.to_string()
        }
        Ok(v) => v,
    }
}

fn main() {
    rocket::ignite()
        .manage(types::State{..Default::default()})
        .mount("/", routes![server_post])
        .launch();
}
