#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate regex;
extern crate log;
extern crate xml;
extern crate yaserde;
extern crate yaserde_derive;
extern crate gps_lib;


use rocket::data::{self, FromDataSimple};
use rocket::http::Status;
use rocket::{Data, Outcome::*, Request};
use std::io::Read;
use yaserde::de::from_str;
use yaserde::ser::to_string;

fn extract_action_name(action: &str) -> Result<&str, String> {
    let ns = "http://www.globalprocessing.ae/HyperionWeb";
    let re = format!("^\"{}/(\\w+)\"$", ns);
    let re = regex::Regex::new(&re[..]).unwrap();
    match re.captures(action) {
        None => Err(format!("{} is not a valid action", action)),
        Some(caps) => Ok(caps.get(1).unwrap().as_str()),
    }
}

pub trait Action {
    fn new(contents: &str) -> Result<Self, String> where Self: Sized;
    fn execute(&self) -> Result<String, String>;
}

struct PostStr {
    pub action: Box<dyn Action>,
}

struct CreateCard {
    pub parameters: gps_lib::types::WsCreateCard,
}

impl CreateCard { 
    fn wrap_response(
        &self,
        contents: gps_lib::types::WsCreateCardResponse,
    ) -> gps_lib::bindings::WsCreateCardSoapOutSoapEnvelope {
        gps_lib::bindings::WsCreateCardSoapOutSoapEnvelope {
            tnsattr: None,
            urnattr: None,
            xsiattr: None,
            header: None,
            encoding_style: gps_lib::SOAP_ENCODING.to_string(),
            body: gps_lib::bindings::SoapWsCreateCardSoapOut {
                body: gps_lib::messages::WsCreateCardSoapOut {
                    parameters: contents,
                },
                fault: None,
            },
        }
    }
}

impl Action for CreateCard {
    fn new(contents: &str) -> Result<Self, String> {
        let envelope: Result<gps_lib::bindings::WsCreateCardSoapInSoapEnvelope, std::string::String> =
            from_str(&contents);
        match envelope {
            Err(e) => Err(e),
            Ok(v) => Ok(CreateCard {
                parameters: v.body.body.parameters,
            }),
        }
    }

    fn execute(&self) -> Result<String, String> {
        println!("{:?}", self.parameters);

        let response = self.wrap_response(gps_lib::types::WsCreateCardResponse {
            ws_create_card_result: gps_lib::types::VirtualCards {
                wsid: self.parameters.wsid,
                iss_code: Some("CRCD".to_string()),
                txn_code: Some("15?".to_string()),
                public_token: Some("000000001".to_string()),
                external_ref: Some("External ref".to_string()),
                loc_date: Some("2020-11-21".to_string()),
                loc_time: Some("21:41".to_string()),
                item_id: 1,
                client_code: Some("Client code".to_string()),
                sys_date: Some("2020 11 21".to_string()),
                action_code: Some("000".to_string()),
                load_value: 0.00,
                is_live: false,
                start_date: Some("2020".to_string()),
                exp_date: Some("2028".to_string()),
                cvv: Some("123".to_string()),
                masked_pan: Some("123-456-789-456".to_string()),
                image: None,
            },
        });
        to_string(&response)
    }
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
        match extract_action_name(action) {
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
    // println!("{}", input.contents);
    input.response().expect("aaaa")
}

fn main() {
    rocket::ignite().mount("/", routes![server_post]).launch();
}
