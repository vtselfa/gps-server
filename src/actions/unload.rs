use chrono::prelude::*;
use log::{debug, info};
use paste::paste;
use std::sync::atomic::Ordering;
use yaserde::de::from_str;
use yaserde::ser::to_string;

use crate::action;
use crate::get_mut_card;
use crate::impl_action_boilerplate;
use crate::impl_wrap_response;
use crate::money;
use crate::types;
use crate::types::GpsError;
use crate::utils;

pub struct Unload {
    pub parameters: gps_lib::types::WsUnLoad,
    pub action_name: String,
}

impl_wrap_response!(Unload, UnLoad); // Our name is different than the GPS one

impl action::Action for Unload {
    impl_action_boilerplate!(Unload, UnLoad);

    fn execute(&self, state: &types::State) -> Result<String, types::GpsError> {
        let parameters = &self.parameters;
        let utc: DateTime<Utc> = Utc::now();

        debug!("Parameters: {:?}", parameters);
        get_mut_card!(self.parameters.public_token, state, card, cards_map);
        utils::check_currency(&parameters.curr_code, card)?;
        let amount = utils::get_strictly_positive_amount(&parameters.amt_un_load.to_string())?;

        if card.balance.amount < amount {
            return Err(GpsError::ActionCode {
                num: 999,
                msg: format!("Unload amount is greater than the current balance"),
            });
        }
        card.balance.amount -= amount;
        info!(
            "PublicToken: {} unloaded {}, balance is {}",
            card.public_token, amount, card.balance.amount
        );

        // Record the transaction in the card structure
        let item_id = state.next_item_id.fetch_add(1, Ordering::SeqCst);
        let transaction = types::Transaction {
            item_id: item_id as u64,            // GPS transaction ID
            wsid: Some(parameters.wsid as u64), // GPS transaction ID

            txn_date: utc,
            post_date: utc,

            amt_bill: amount,
            currency: card.get_currency(),
            amt_txn: money::Money::new(amount, card.get_currency()), // TODO:: Currency conversion support?

            // Unloads have no fees
            fee_fixed: None,
            fee_rate: None,
            dom_fee_fixed: None,
            dom_fee_rate: None,
            non_dom_fee_fixed: None,
            non_dom_fee_rate: None,
            fx_fee_fixed: None,
            fx_fee_rate: None,
            other_fee_fixed: None,

            note: None,
            description: parameters.description.clone(),

            balance: card.balance.amount,
            blocked_balance: card.blocked_balance,

            mcc: None,
            proc_code: None,

            txn_type: types::TransationTypeStatus::Unload,
        };

        let response = self.wrap_response(gps_lib::types::WsUnLoadResponse {
            ws_un_load_result: gps_lib::types::UnLoad {
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
                amt_un_load: parameters.amt_un_load,
                avl_bal: format!("{}", card.balance.amount),
                cur_code: Some(card.get_currency_info().iso_numeric_code),
                blk_amt: format!("{}", card.blocked_balance),
            },
        });

        card.transactions.push(transaction);

        match to_string(&response) {
            Err(e) => Err(types::GpsError::Serialization(e)),
            Ok(v) => Ok(v),
        }
    }

    fn report_not_successful(&self, error: &types::GpsError) -> Result<String, types::GpsError> {
        let parameters = &self.parameters;
        let action_code = utils::error_to_action_code(error);
        let response = self.wrap_response(gps_lib::types::WsUnLoadResponse {
            ws_un_load_result: gps_lib::types::UnLoad {
                wsid: parameters.wsid,
                iss_code: parameters.iss_code.clone(),
                txn_code: parameters.txn_code.clone(),
                public_token: parameters.public_token.clone(),
                loc_date: Some(utils::loc_date()),
                loc_time: Some(utils::loc_time()),
                item_id: 0,
                client_code: parameters.client_code.clone(),
                sys_date: Some(utils::sys_date()),
                action_code: Some(action_code),
                amt_un_load: parameters.amt_un_load,
                avl_bal: format!("0.00"),
                cur_code: parameters.curr_code.clone(),
                blk_amt: format!("0.00"),
            },
        });

        match to_string(&response) {
            Err(e) => Err(types::GpsError::Serialization(e)),
            Ok(v) => Ok(v),
        }
    }
}
