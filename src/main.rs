#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate log;
extern crate xml;
extern crate yaserde;

#[macro_use]
extern crate yaserde_derive;

use rocket::data::{self, FromDataSimple};
use rocket::http::Status;
use rocket::{Data, Outcome::*, Request};
use std::io::Read;
use crate::hello::bindings::SayHelloSoapEnvelope;
use yaserde::de::from_str;
use yaserde::ser::to_string;

mod hello;

struct PostStr{ }

struct Action1<'a> {
    pub contents: &'a str,
}

impl<'a> Action1<'a> {
    pub fn new(contents: &str) -> Result<hello::types::HelloRequest, String> {
        let envelope: Result<SayHelloSoapEnvelope, std::string::String> = from_str(&contents);
        match envelope {
            Err(e) => Err(e),
            Ok(v) => Ok(v.body.body.parameters.hello_request)
        }
    }
}

impl PostStr {
    fn response(self) -> String{
        let response = hello::bindings::SayHelloResponseSoapEnvelope {
            tnsattr: None,
            urnattr: None,
            xsiattr: None,
            header: None,
            encoding_style: "utf-8".to_string(),
            body: hello::bindings::SoapSayHelloResponse{
                body: hello::messages::SayHelloResponse{
                    parameters: hello::types::SayHelloResponse{
                        hello_response: hello::types::HelloResponse{
                            message: "Finallyyyyyy!".to_string()
                        }
                    }
                },
                fault: None,
            }
        };
        to_string(&response).expect("aaa")
    }
}

impl FromDataSimple for PostStr {
    type Error = String;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {
        println!("{:?}", req.headers());

        let mut buffer: [u8; 1024] = [0; 1024];
        if let Err(e) = data.open().read(&mut buffer) {
            return Failure((Status::BadRequest, e.to_string()));
        }
        let contents = match std::str::from_utf8(&buffer) {
            Ok(data) => data.to_string(),
            Err(e) => return Failure((Status::BadRequest, e.to_string())),
        };

        let action1 = Action1::new(&contents);
        match action1 {
            Ok(v) => println!("{}", v.name),
            Err(e) => {},
        }

        // let action = req.headers().get("soapaction").collect::<String>();
        // let servant = match &action[..] {
        //     "say_hello" => {
        //         let post = Action1::new(&contents[..]);
        //     }
        //     _ => println!("Nice"),
        // }


        // let envelope: Result<SayHelloSoapEnvelope, std::string::String> = from_str(&contents);
        // match envelope {
        //     Err(e) => return Failure((Status::InternalServerError, format!("{:?}", e))),
        //     Ok(v) => println!("Nice"),
        // }
        // if let Err(e) = data.open().read(buffer);
            //.read(buffer).read_to_string(&mut contents) {
            // return Failure((Status::InternalServerError, format!("{:?}", e)));
        // }

        Success(PostStr {})
    }
}

#[post("/hello", format = "text/xml", data = "<input>")]
fn server_post(input: PostStr) -> String {
    // println!("{}", input.contents);
    input.response()
}

fn main() {
    rocket::ignite().mount("/", routes![server_post]).launch();
}
