use yaserde::de::from_str;
use yaserde::ser::to_string;

use crate::action::Action;


pub struct CreateCard {
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
