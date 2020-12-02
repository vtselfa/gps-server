use yaserde::de::from_str;
use yaserde::ser::to_string;
use rusty_money::{Money,Currency};
use std::sync::atomic::Ordering;
use chrono::prelude::*;
use rust_decimal::prelude::*;

use crate::types;
use crate::types::GpsError;
use crate::action;


pub struct Load {
    pub parameters: gps_lib::types::WsLoad,
}

impl Load {
    fn wrap_response(
        &self,
        contents: gps_lib::types::WsLoadResponse,
    ) -> gps_lib::bindings::WsLoadSoapOutSoapEnvelope {
        gps_lib::bindings::WsLoadSoapOutSoapEnvelope {
            tnsattr: None,
            urnattr: None,
            xsiattr: None,
            header: None,
            encoding_style: gps_lib::SOAP_ENCODING.to_string(),
            body: gps_lib::bindings::SoapWsLoadSoapOut {
                body: gps_lib::messages::WsLoadSoapOut {
                    parameters: contents,
                },
                fault: None,
            },
        }
    }
}

impl action::Action for Load {
    fn new(contents: &str) -> Result<Self, types::GpsError> {
        let envelope: Result<gps_lib::bindings::WsLoadSoapInSoapEnvelope, std::string::String> =
            from_str(&contents);
        match envelope {
            Err(e) => Err(types::GpsError::RequestData(e)),
            Ok(v) => Ok(Load {
                parameters: v.body.body.parameters,
            }),
        }
    }

    fn execute(&self, state: &types::State) -> Result<String, types::GpsError> {
        let parameters = &self.parameters;
        let utc: DateTime<Utc> = Utc::now();

        // Get the public token
        let public_token = match parameters.public_token.as_ref() {
            None => return Err(GpsError::ActionCode{num: 999, msg: format!("Missing public_token")}), // TODO: support other ways to get the card
            Some(v) => v,
        };

        // Get the card, with a write mutex
        let mut public_tokens = state.public_tokens.write().expect("Poisoned read lock");
        let card = match public_tokens.get_mut(public_token) {
            None => return Err(GpsError::ActionCode{num: 999, msg: format!("Public token not found")}),
            Some(card) => card,
        };

        // We do not support a load with a currency different than the one in the
        // card. Not sure if GPS does.
        match parameters.curr_code.as_ref().map(String::as_str) {
            None => (),
            Some(v) =>  {
                let currency = Currency::find(v)?;
                if currency != card.currency {
                    return Err(GpsError::ActionCode{num: 999, msg: format!("Currency missmatch")});
                }
            }
        };

        // Get the amount as a string because that's what Money expects
        // GPS expect the amount to be >= 0, so we too
        let amount = parameters.load_value.to_string();
        let amount = Decimal::from_str(&amount)?;
        let amount = Money::from_decimal(amount, card.currency);
        if amount.amount().is_sign_negative() || amount.amount().is_zero() {
            return Err(GpsError::ActionCode{num: 999, msg: format!("Load amount has to be greater than zero")});
        }

        // TODO: Use the tx_code to load or reload
        card.balance = card.balance.clone() + amount.clone();

        // Record the transaction in the card structure
        let item_id = state.next_item_id.fetch_add(1, Ordering::SeqCst);
        let transaction = types::Transaction {
            item_id: item_id as u64, // GPS transaction ID
            txn_date: utc,
            post_date: utc,
            amt_bill: amount.clone(),
            amt_txn: amount,
            fixed_fee: None, // TODO: Loads can have fees attached
            rate_fee: None, // TODO: Same as with the fixed fee
            note: parameters.description.clone(),
        };

        let response = self.wrap_response(gps_lib::types::WsLoadResponse {
            ws_load_result: gps_lib::types::LoadCard {
                wsid: parameters.wsid,
                iss_code: parameters.iss_code.clone(),
                txn_code: parameters.txn_code.clone(),
                public_token: Some(public_token.clone()),
                loc_date: Some(format!("{}", utc.format("%Y-%m-%d"))),
                loc_time: Some(format!("{}", utc.format("%H%M%S"))),
                item_id: transaction.item_id as i64, // GPS transaction ID, every time a different type...
                client_code: parameters.client_code.clone(),
                sys_date: Some(format!("{}", utc.format("%Y-%m-%d"))),
                action_code: Some("000".to_string()),
            },
        });

        card.transactions.push(transaction);

        match to_string(&response) {
            Err(e) => Err(types::GpsError::Serialization(e)),
            Ok(v) => Ok(v),
        }
    }
}
