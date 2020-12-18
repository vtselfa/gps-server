use yaserde::de::from_str;
use yaserde::ser::to_string;
use rusty_money::{Money,Currency};
use std::sync::atomic::Ordering;
use chrono::prelude::*;
use rust_decimal::prelude::*;
use paste::paste;

use crate::types;
use crate::types::GpsError;
use crate::action;
use crate::impl_wrap_response;


pub struct BalanceAdjustment {
    pub parameters: gps_lib::types::WsBalanceAdjustment,
}

impl_wrap_response!(BalanceAdjustment);

impl action::Action for BalanceAdjustment {
    fn new(contents: &str) -> Result<Self, types::GpsError> {
        let envelope: Result<gps_lib::bindings::WsBalanceAdjustmentSoapInSoapEnvelope, std::string::String> =
            from_str(&contents);
        match envelope {
            Err(e) => Err(types::GpsError::RequestData(e)),
            Ok(v) => Ok(BalanceAdjustment {
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

        // We do not support a balance adjustment with a currency different than the one in the
        // card. Not sure if GPS does.
        match parameters.cur_code.as_ref().map(String::as_str) {
            None => (),
            Some(v) =>  {
                let currency = Currency::find(v)?;
                if currency != card.currency {
                    return Err(GpsError::ActionCode{num: 999, msg: format!("Currency missmatch")});
                }
            }
        };

        // Get the amount with the correct sign, and as a string because that's what Money expects
        let amount = match parameters.deb_or_cred.as_ref().map(String::as_ref) {
            Some("-1") => format!("{}", -parameters.amt_adjustment),
            Some("1") => parameters.amt_adjustment.to_string(),
            _ => return Err(GpsError::ActionCode{num: 999, msg: format!("Invalid deb_or_cred")}),
        };
        let amount = Decimal::from_str(&amount)?;
        let amount = Money::from_decimal(amount, card.currency);

        card.balance = card.balance.clone() + amount.clone();

        // Record the transaction in the card structure
        let item_id = state.next_item_id.fetch_add(1, Ordering::SeqCst);
        let transaction = types::Transaction {
            item_id: item_id as u64, // GPS transaction ID
            txn_date: utc,
            post_date: utc,
            amt_bill: amount.clone(),
            amt_txn: amount,
            fixed_fee: None,
            rate_fee: None,
            note: parameters.description.clone(),
        };

        let response = self.wrap_response(gps_lib::types::WsBalanceAdjustmentResponse {
            ws_balance_adjustment_result: gps_lib::types::BalanceAdjust {
                wsid: parameters.wsid,
                iss_code: parameters.iss_code.clone(),
                txn_code: parameters.txn_code.clone(),
                public_token: Some(public_token.clone()),
                loc_date: Some(format!("{}", utc.format("%Y-%m-%d"))),
                loc_time: Some(format!("{}", utc.format("%H%M%S"))),
                item_id: Some(transaction.item_id.to_string()), // GPS transaction ID
                client_code: parameters.client_code.clone(),
                sys_date: Some(format!("{}", utc.format("%Y-%m-%d"))),
                action_code: Some("000".to_string()),
                avl_bal: format!("{}", card.balance.amount()),
                cur_code: Some(card.currency.iso_numeric_code.to_string()),
            },
        });

        card.transactions.push(transaction);

        match to_string(&response) {
            Err(e) => Err(types::GpsError::Serialization(e)),
            Ok(v) => Ok(v),
        }
    }
}
