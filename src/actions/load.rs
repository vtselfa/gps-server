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


pub struct Load {
    pub parameters: gps_lib::types::WsLoad,
}

impl_wrap_response!(Load);

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

        get_mut_card!(self.parameters.public_token, state, card, cards_map);
        utils::check_currency(&parameters.curr_code, card)?;
        let amount = utils::get_strictly_positive_amount(&parameters.load_value.to_string())?;

        // TODO: Use the tx_code to load or reload
        card.balance.amount += amount;

        // Record the transaction in the card structure
        let item_id = state.next_item_id.fetch_add(1, Ordering::SeqCst);
        let transaction = types::Transaction {
            item_id: item_id as u64, // GPS transaction ID
            txn_date: utc,
            post_date: utc,
            amt_bill: money::Money::new(amount, card.get_currency()),
            amt_txn: money::Money::new(amount, card.get_currency()), // TODO:: Currency conversion support?
            fixed_fee: None, // TODO: Loads can have fees attached
            rate_fee: None, // TODO: Same as with the fixed fee
            note: parameters.description.clone(),
        };

        let response = self.wrap_response(gps_lib::types::WsLoadResponse {
            ws_load_result: gps_lib::types::LoadCard {
                wsid: parameters.wsid,
                iss_code: parameters.iss_code.clone(),
                txn_code: parameters.txn_code.clone(),
                public_token: Some(card.public_token.clone()),
                loc_date: Some(utils::loc_date()),
                loc_time: Some(utils::loc_time()),
                item_id: transaction.item_id as i64, // GPS transaction ID, every time a different type...
                client_code: parameters.client_code.clone(),
                sys_date: Some(utils::sys_date()),
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
