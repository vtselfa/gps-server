use yaserde::de::from_str;
use yaserde::ser::to_string;
use std::sync::atomic::Ordering;
use chrono::prelude::*;
use paste::paste;

use crate::types;
use crate::types::GpsError;
use crate::action;
use crate::impl_wrap_response;
use crate::money;
use crate::utils;
use crate::get_mut_card;


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

        get_mut_card!(self.parameters.public_token, state, card, cards_map);
        utils::check_currency(&parameters.cur_code, card)?;

        // Get the amount with the correct sign
        let amount = match parameters.deb_or_cred.as_ref().map(String::as_ref) {
            Some("-1") => -utils::get_strictly_positive_amount(&parameters.amt_adjustment.to_string())?,
            Some("1") => utils::get_strictly_positive_amount(&parameters.amt_adjustment.to_string())?,
            _ => return Err(GpsError::ActionCode{num: 999, msg: format!("Invalid deb_or_cred")}),
        };

        card.balance.amount += amount;

        // Record the transaction in the card structure
        let item_id = state.next_item_id.fetch_add(1, Ordering::SeqCst);
        let transaction = types::Transaction {
            item_id: item_id as u64, // GPS transaction ID
            wsid: Some(parameters.wsid as u64), // GPS transaction ID

            txn_date: utc,
            post_date: utc,

            amt_bill: amount,
            currency: card.get_currency(),
            amt_txn: money::Money::new(amount, card.get_currency()), // TODO:: Currency conversion support?

            // Balance adjustments have no fees
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

            txn_type: types::TransationTypeStatus::BalanceAdjustment,
        };

        let response = self.wrap_response(gps_lib::types::WsBalanceAdjustmentResponse {
            ws_balance_adjustment_result: gps_lib::types::BalanceAdjust {
                wsid: parameters.wsid,
                iss_code: parameters.iss_code.clone(),
                txn_code: parameters.txn_code.clone(),
                public_token: parameters.public_token.clone(),
                loc_date: Some(format!("{}", utc.format("%Y-%m-%d"))),
                loc_time: Some(format!("{}", utc.format("%H%M%S"))),
                item_id: Some(transaction.item_id.to_string()), // GPS transaction ID
                client_code: parameters.client_code.clone(),
                sys_date: Some(format!("{}", utc.format("%Y-%m-%d"))),
                action_code: Some("000".to_string()),
                avl_bal: format!("{}", card.balance.amount),
                cur_code: Some(card.get_currency_info().iso_numeric_code),
            },
        });

        card.transactions.push(transaction);

        match to_string(&response) {
            Err(e) => Err(types::GpsError::Serialization(e)),
            Ok(v) => Ok(v),
        }
    }
}
