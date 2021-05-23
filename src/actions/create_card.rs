use chrono::prelude::*;
use paste::paste;
use rand::Rng;
use std::sync::atomic::Ordering;
use yaserde::de::from_str;
use yaserde::ser::to_string;

use crate::action;
use crate::currency;
use crate::impl_wrap_response;
use crate::money::Money;
use crate::types::GpsError;
use crate::types;
use crate::utils;


pub struct CreateCard {
    pub parameters: gps_lib::types::WsCreateCard,
}

impl_wrap_response!(CreateCard);

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
        let parameters = &self.parameters;
        let utc: DateTime<Utc> = Utc::now();
        let public_token = state.next_public_token.fetch_add(1, Ordering::SeqCst);
        let public_token = format!("{:>09}", public_token);

        // TODO: Groups

        // TODO: Currencies are usually taken from products, so we will not have proper currency support until
        // we implement product support. For now, if a currency is not specified, we assume EUR.
        let currency = currency::find_by_alpha_code(parameters.cur_code.as_ref().map_or("EUR", |x|{x.as_str()}))?;

        // Create a transaction in the card if a load amount is provided
        let amount = utils::get_strictly_positive_amount(&parameters.load_value.to_string())?;
        let transactions = if !amount.is_sign_positive() {
            vec!()
        } else {
            let item_id = state.next_item_id.fetch_add(1, Ordering::SeqCst);
            vec!(types::Transaction {
                item_id: item_id as u64, // GPS transaction ID
                wsid: Some(parameters.wsid as u64), // GPS transaction ID

                txn_date: utc,
                post_date: utc,

                amt_bill: amount,
                currency,
                amt_txn: Money::new(amount, currency),

                fee_fixed: None, // TODO: Loads can have fees attached
                fee_rate: None, // TODO: Same as with the fixed fee
                dom_fee_fixed: None,
                dom_fee_rate: None,
                non_dom_fee_fixed: None,
                non_dom_fee_rate: None,
                fx_fee_fixed: None,
                fx_fee_rate: None,
                other_fee_fixed: None,

                note: None,
                description: Some(format!("Initial load")),

                balance: amount,
                blocked_balance: Money::zero(currency).amount,

                mcc: None,
                proc_code: None,

                txn_type: types::TransationTypeStatus::Load,
            })
        };

        let mut rng = rand::thread_rng();
        let card = types::Card {
            wsid: parameters.wsid,
            public_token,
            external_ref: parameters.external_ref.clone(),
            start_date: utc,
            exp_date: utc.with_year(utc.year() + 3).unwrap().naive_utc().date(), // TODO: Use provided expiration date if available
            balance: Money::new(amount, currency),
            blocked_balance: Money::zero(currency).amount,
            is_live: match parameters.activate_now {
                0 => false,
                1 => true,
                _ => return Err(GpsError::ActionCode{num: 999, msg: format!("Invalid activate_now value")})  // Actually, GPS does not check this
            },
            status: types::CardStatus::AllGood, // GPS returns this no matter if activated or not...
            pan: format!("{:>016}",rng.gen_range(0, 9999_9999_9999_9999 + 1 as i64)),
            cvv: format!("{:>03}", rng.gen_range(0, 999 + 1)),
            transactions,
            owner: types::Consumer {
                title: parameters.title.clone().unwrap_or(format!("")),
                first_name: parameters.first_name.clone().unwrap_or(format!("")),
                last_name: parameters.last_name.clone().unwrap_or(format!("")),
            }
        };

        let response = self.wrap_response(gps_lib::types::WsCreateCardResponse {
            ws_create_card_result: gps_lib::types::VirtualCards {
                wsid: parameters.wsid,
                iss_code: parameters.iss_code.clone(),
                txn_code: parameters.txn_code.clone(),
                public_token: Some(card.public_token.clone()),
                external_ref: card.external_ref.clone(),
                loc_date: Some(utils::loc_date()),
                loc_time: Some(utils::loc_time()),
                item_id: if card.transactions.is_empty() {0} else {card.transactions[0].item_id as i64},
                client_code: parameters.client_code.clone(),
                sys_date: Some(utils::sys_date()),
                action_code: Some("000".to_string()),
                load_value: format!("{}", card.balance.amount),
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
