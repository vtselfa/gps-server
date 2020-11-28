#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate regex;
extern crate log;
extern crate xml;
extern crate yaserde;
extern crate yaserde_derive;
extern crate gps_lib;

mod action;
mod create_card;

use rocket::data::{self, FromDataSimple};
use rocket::http::Status;
use rocket::{Data, Outcome::*, Request};
use std::io::Read;

use action::Action;
use create_card::CreateCard;


struct PostStr {
    pub action: Box<dyn action::Action>,
}


impl PostStr {
    fn response(&self) -> Result<String, String> {
        self.action.execute()
    }
}

impl FromDataSimple for PostStr {
    type Error = String;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {
        println!("{:?}", req.headers());
        
        const MAX_SIZE: usize = 4096;

        match req.headers().get("Content-Length").next() {
            None => return Failure((Status::BadRequest, "Missing content-length header".to_string())),
            Some(header) => {
                match header.parse::<usize>() {
                    Err(_) => return Failure((Status::BadRequest, "Malformed content-length header".to_string())),
                    Ok(content_length) => {
                        if content_length > MAX_SIZE {
                            return Failure((Status::BadRequest, "Incoming data too long".to_string()));
                        }
                    }
                };
            }
        };

        let mut buffer: [u8; MAX_SIZE] = [0; MAX_SIZE];
        let mut offset = 0;
        let mut stream = data.open();
        loop {
            match stream.read(&mut buffer[offset ..]) {
                Err(e) => return Failure((Status::BadRequest, e.to_string())),
                Ok(read_bytes) => {
                    if read_bytes == 0 {
                        break; // EOF
                    }
                    offset = offset + read_bytes;
                }
            }
        }

        let contents = match std::str::from_utf8(&buffer) {
            Ok(data) => data.to_string(),
            Err(e) => return Failure((Status::BadRequest, e.to_string())),
        };

        let action = match req.headers().get_one("soapaction") {
            None =>  return Failure((Status::BadRequest, "No soapaction header found".to_string())),
            Some(a) => a,
        };

        println!("Action: {}", action);
        match action::extract_name(action) {
            Ok("Ws_CreateCard") => {
                match CreateCard::new(&contents) {
                    Ok(v) => Success(PostStr { action: Box::new(v) }),
                    Err(e) => Failure((Status::BadRequest, e.to_string())),
                }
            },
            Ok(u) => {println!("Unknown action {}", u); panic!()}
            _ => Failure((Status::BadRequest, format!("Action {} not implemented", action))),
        }
    }
}

#[post("/hello", format = "text/xml", data = "<input>")]
fn server_post(input: PostStr) -> String {
    input.response().expect("aaaa")
}

fn main() {
    rocket::ignite().mount("/", routes![server_post]).launch();
}
