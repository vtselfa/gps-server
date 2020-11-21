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
use yaserde::de::from_str;
use yaserde::ser::to_string;

mod weather;

struct PostStr {
    pub action: GetCityForecastByZIP,
}

struct GetCityForecastByZIP {
    pub parameters: weather::types::GetCityForecastByZIP,
}

impl GetCityForecastByZIP {
    pub fn new(contents: &str) -> Result<Self, String> {
        let envelope: Result<weather::bindings::GetCityForecastByZIPSoapInSoapEnvelope, std::string::String> = from_str(&contents);
        match envelope {
            Err(e) => Err(e),
            Ok(v) => Ok(GetCityForecastByZIP{parameters: v.body.body.parameters})
        }
    }

    fn wrap_response(&self, contents: weather::types::GetCityForecastByZIPResponse)
    -> weather::bindings::GetCityForecastByZIPSoapOutSoapEnvelope {
        weather::bindings::GetCityForecastByZIPSoapOutSoapEnvelope {
            tnsattr: None,
            urnattr: None,
            xsiattr: None,
            header: None,
            encoding_style: weather::SOAP_ENCODING.to_string(),
            body: weather::bindings::SoapGetCityForecastByZIPSoapOut{
                body: weather::messages::GetCityForecastByZIPSoapOut{
                    parameters: contents,
                },
                fault: None,
            }
        }
    }

    pub fn execute(&self) -> Result<String, String> {
        println!("{:?}", self.parameters.zip);

        let response = self.wrap_response(weather::types::GetCityForecastByZIPResponse {
            get_city_forecast_by_zip_result: Some(weather::types::ForecastReturn {
                city: Some("Geneva".to_string()),
                forecast_result: Some(weather::types::ArrayOfForecast {
                    forecast: vec!(weather::types::Forecast{
                        date: "today".to_string(),
                        desciption: Some("Long description".to_string()),
                        probability_of_precipiation: weather::types::Pop {
                            nighttime: Some("95%".to_string()),
                            daytime: Some("45%".to_string())
                        },
                        weather_id: 42,
                        temperatures: weather::types::Temp {
                            daytime_high: Some("12".to_string()),
                            morning_low: Some("34".to_string())},
                    })
                }),
                response_text: Some("All good".to_string()),
                state: Some("Arizona".to_string()),
                success: true,
                weather_station_city: Some("Milan".to_string())
            }),
        });
        to_string(&response)
    }
}

impl PostStr {
    fn response(&self) -> Result<String, String>{
        self.action.execute()
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

        let action = GetCityForecastByZIP::new(&contents);
        match action {
            Ok(v) => Success(PostStr{action: v}),
            Err(e) => Failure((Status::BadRequest, e.to_string())),
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

        // Success(PostStr {})
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
