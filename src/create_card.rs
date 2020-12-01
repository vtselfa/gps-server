use yaserde::de::from_str;
use yaserde::ser::to_string;
use rusty_money::{money, Money, Currency};
use std::sync::atomic::Ordering;
use chrono::prelude::*;
use rand::Rng;

use crate::types;
use crate::types::GpsError;
use crate::action;


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

impl action::Action for CreateCard {
    fn new(contents: &str) -> Result<Self, types::GpsError> {
        let envelope: Result<gps_lib::bindings::WsCreateCardSoapInSoapEnvelope, std::string::String> =
            from_str(&contents);
        match envelope {
            Err(e) => Err(types::GpsError::RequestData(e)),
            Ok(v) => Ok(CreateCard {
                parameters: v.body.body.parameters,
            }),
        }
    }

    fn execute(&self, state: &types::State) -> Result<String, types::GpsError> {
        println!("{:?}", self.parameters);
        let parameters = &self.parameters;
        let utc: DateTime<Utc> = Utc::now();
        let public_token = state.next_public_token.fetch_add(1, Ordering::SeqCst);
        let public_token = format!("{:>09}", public_token);
            
        // TODO: Groups
        
        // TODO: Currencies are usually taken from products, so we will not have proper currency support until
        // we implement product support. For now, if a currency is not specified, we assume EUR.
        let currency = Currency::find(parameters.cur_code.as_ref().map_or("EUR", |x|{x.as_str()}))?;
        let mut rng = rand::thread_rng();
        let card = types::Card {
            wsid: parameters.wsid,
            public_token: public_token,
            external_ref: parameters.external_ref.clone(),
            start_date: utc, 
            exp_date: utc.with_year(utc.year() + 3).unwrap(), // TODO: Use provided expiration date if available
            balance: money!(parameters.load_value, currency),
            currency: currency,
            is_live: match parameters.activate_now {
                0 => false,
                1 => true,
                _ => return Err(GpsError::ActionCode{num: 999, msg: format!("Invalid activate_now value")})  // Actually, GPS does not check this
            },
            pan: format!("{:>016}",rng.gen_range(0, 9999_9999_9999_9999 + 1 as i64)),
            cvv: format!("{:>03}", rng.gen_range(0, 999 + 1)),
            stat_code: format!("0"),
            transactions: vec![],
        };
        
        let response = self.wrap_response(gps_lib::types::WsCreateCardResponse {
            ws_create_card_result: gps_lib::types::VirtualCards {
                wsid: parameters.wsid,
                iss_code: parameters.iss_code.clone(),
                txn_code: parameters.txn_code.clone(),
                public_token: Some(card.public_token.clone()),
                external_ref: card.external_ref.clone(),
                loc_date: Some(format!("{}", utc.format("%Y-%m-%d"))),
                loc_time: Some(format!("{}", utc.format("%H%M%S"))),
                item_id: 0, // TODO: Check what this is
                client_code: parameters.client_code.clone(),
                sys_date: Some(format!("{}", utc.format("%Y-%m-%d"))),
                action_code: Some("000".to_string()),
                load_value: format!("{}", card.balance),
                is_live: card.is_live,
                start_date: Some(format!("{}", card.start_date.format("%m/%y"))),
                exp_date: Some(format!("{}", card.exp_date.format("%m/%y"))),
                cvv: Some(card.cvv.clone()), 
                masked_pan: Some(card.pan.clone()),
                image: None,
            },
        });

        state.public_tokens.write().expect("Public tokens lock poisoned").insert(card.public_token.clone(), card);

        match to_string(&response) {
            Err(e) => Err(types::GpsError::Serialization(e)),
            Ok(v) => Ok(v),
        }
    }
}
