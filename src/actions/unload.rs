use chrono::prelude::*;
use paste::paste;
use std::sync::atomic::Ordering;
use yaserde::de::from_str;
use yaserde::ser::to_string;

use crate::action;
use crate::get_mut_card;
use crate::impl_wrap_response;
use crate::money;
use crate::types::GpsError;
use crate::types;
use crate::utils;


pub struct Unload {
    pub parameters: gps_lib::types::WsUnLoad,
}

impl_wrap_response!(Unload, UnLoad); // Our name is different than the GPS one

impl action::Action for Unload {
    fn new(contents: &str) -> Result<Self, types::GpsError> {
        let envelope: Result<gps_lib::bindings::WsUnLoadSoapInSoapEnvelope, std::string::String> =
            from_str(&contents);
        match envelope {
            Err(e) => Err(types::GpsError::RequestData(e)),
            Ok(v) => Ok(Unload {
                parameters: v.body.body.parameters,
            }),
        }
    }

    fn execute(&self, state: &types::State) -> Result<String, types::GpsError> {
        let parameters = &self.parameters;
        let utc: DateTime<Utc> = Utc::now();

        get_mut_card!(self.parameters.public_token, state, card, cards_map);
        utils::check_currency(&parameters.curr_code, card)?;
        let amount = utils::get_strictly_positive_amount(&parameters.amt_un_load.to_string())?;

        if card.balance.amount < amount {
            return Err(GpsError::ActionCode{num: 999, msg: format!("Unload amount is greater than the current balance")});
        }
        card.balance.amount -= amount;

        // Record the transaction in the card structure
        let item_id = state.next_item_id.fetch_add(1, Ordering::SeqCst);
        let transaction = types::Transaction {
            item_id: item_id as u64, // GPS transaction ID
            txn_date: utc,
            post_date: utc,
            amt_bill: money::Money::new(amount, card.get_currency()),
            amt_txn: money::Money::new(amount, card.get_currency()), // TODO: Currency conversion support?
            fixed_fee: None, // Unloads have no fees attached
            rate_fee: None, // Same as with the fixed fee
            note: parameters.description.clone(),
        };

        let response = self.wrap_response(gps_lib::types::WsUnLoadResponse {
            ws_un_load_result: gps_lib::types::UnLoad {
                wsid: parameters.wsid,
                iss_code: parameters.iss_code.clone(),
                txn_code: parameters.txn_code.clone(),
                public_token: Some(card.public_token.clone()),
                loc_date: Some(format!("{}", utc.format("%Y-%m-%d"))),
                loc_time: Some(format!("{}", utc.format("%H%M%S"))),
                item_id: transaction.item_id as i64, // GPS transaction ID, every time a different type...
                client_code: parameters.client_code.clone(),
                sys_date: Some(format!("{}", utc.format("%Y-%m-%d"))),
                action_code: Some("000".to_string()),
                amt_un_load: parameters.amt_un_load,
                avl_bal: format!("{}", card.balance.amount),
                cur_code: Some(card.get_currency_info().iso_numeric_code),
                blk_amt: format!("0.0"), // TODO: Implement blocked balances in cards
            },
        });

        card.transactions.push(transaction);

        match to_string(&response) {
            Err(e) => Err(types::GpsError::Serialization(e)),
            Ok(v) => Ok(v),
        }
    }
}
