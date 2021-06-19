use chrono::prelude::*;
use log::{debug, info};
use paste::paste;
use superslice::*;
use yaserde::de::from_str;
use yaserde::ser::to_string;

use crate::action;
use crate::get_card;
use crate::impl_action_boilerplate;
use crate::impl_wrap_response;
use crate::types;
use crate::types::GpsError;
use crate::utils;

pub struct CardStatement {
    pub parameters: gps_lib::types::WsCardStatement,
    pub action_name: String,
}

impl_wrap_response!(CardStatement);

impl action::Action for CardStatement {
    impl_action_boilerplate!(CardStatement);

    fn execute(&self, state: &types::State) -> Result<String, types::GpsError> {
        let parameters = &self.parameters;

        debug!("Parameters: {:?}", parameters);
        get_card!(self.parameters.public_token, state, card, cards_map);
        info!("PublicToken: {}", card.public_token);

        let start_date = parameters
            .start_date
            .as_ref()
            .map(|x| &x[..])
            .unwrap_or("1970-01-01");
        let start_date = DateTime::<Utc>::from_utc(
            NaiveDate::parse_from_str(&start_date, "%Y-%m-%d")?.and_hms(0, 0, 0),
            Utc,
        );

        let end_date = parameters
            .end_date
            .as_ref()
            .map(|x| &x[..])
            .unwrap_or("2050-01-01");
        let end_date = DateTime::<Utc>::from_utc(
            NaiveDate::parse_from_str(&end_date, "%Y-%m-%d")?.and_hms(0, 0, 0),
            Utc,
        );

        let mut lb = card
            .transactions
            .lower_bound_by(|x| x.txn_date.cmp(&start_date));
        let ub = card
            .transactions
            .upper_bound_by(|x| x.txn_date.cmp(&end_date));

        let start_bal = if card.transactions.is_empty() {
            format!("0.0")
        } else if lb < card.transactions.len() {
            format!(
                "{}",
                card.transactions[lb].balance - card.transactions[lb].amt_bill
            )
        } else {
            format!("{}", card.transactions[lb - 1].balance)
        };

        let end_bal = if card.transactions.is_empty() {
            format!("0.0")
        } else if ub < card.transactions.len() {
            format!(
                "{}",
                card.transactions[ub].balance - card.transactions[lb].amt_bill
            )
        } else {
            format!("{}", card.transactions[ub - 1].balance)
        };

        let mut txs = gps_lib::types::ArrayOfTransaction2 {
            transaction_2: vec![],
        };

        while lb < ub {
            let tx = &card.transactions[lb];
            txs.transaction_2.push(gps_lib::types::Transaction2 {
                txn_date: Some(tx.txn_date.format("%Y-%m-%d").to_string()),
                post_date: Some(tx.post_date.format("%Y-%m-%d").to_string()),
                amt_bill: format!("{}", tx.amt_bill),
                amt_txn: format!("{}", tx.amt_txn.amount),
                bill_conv_rate: format!("{}", tx.amt_txn.amount / tx.amt_bill),
                deb_or_cred: if tx.amt_bill.is_sign_negative() {
                    -1
                } else {
                    1
                },
                terminal_id: None,
                description: tx.description.clone(),
                rrn: None,
                cur_txn: Some(tx.get_tx_currency_info().iso_alpha_code),
                item_id: tx.item_id as i64, // GPS inconsistencies
                avl_bal: format!("{}", tx.balance),
                blk_amt: format!("{}", tx.blocked_balance),
                transaction_type: Some(tx.get_type().to_string()),
                status_code: Some(tx.get_status().to_string()),
                status_desc: None,
                txn_time: Some(tx.txn_date.format("%H%M%S").to_string()),
                loc_date: None,
                fee_id: 0,
                wsid: tx.wsid.unwrap_or(0) as i64,
                fixed_fee: tx
                    .fee_fixed
                    .map(|x| format!("{}", x))
                    .unwrap_or(format!("0.00")),
                rate_fee: tx
                    .fee_rate
                    .map(|x| format!("{}", x))
                    .unwrap_or(format!("0.00")),
                fx_pdg: format!("0.00"),
                mcc_pdg: format!("0.00"),
                link_id: None,
                mcc: tx.mcc.clone(),
                orig_stan: None,
                proc_code: tx.proc_code.clone(),
                mcc_description: None,
                dom_fee_fixed: tx.dom_fee_fixed.map(|x| format!("{}", x)),
                dom_fee_rate: tx.dom_fee_rate.map(|x| format!("{}", x)),
                non_dom_fee_fixed: tx.non_dom_fee_fixed.map(|x| format!("{}", x)),
                non_dom_fee_rate: tx.non_dom_fee_rate.map(|x| format!("{}", x)),
                fx_fee_fixed: tx.fx_fee_fixed.map(|x| format!("{}", x)),
                fx_fee_rate: tx.fx_fee_rate.map(|x| format!("{}", x)),
                other_fee_desc: None,
                other_fee_amt: tx.other_fee_fixed.map(|x| format!("{}", x)),
                txn_code_type: None,
                note: tx.note.clone(),
            });
            lb += 1;
        }

        let response = self.wrap_response(gps_lib::types::WsCardStatementResponse {
            ws_card_statement_result: gps_lib::types::CardStatement2 {
                wsid: parameters.wsid,
                iss_code: parameters.iss_code.clone(),
                txn_code: parameters.txn_code.clone(),
                client_code: parameters.client_code.clone(),
                public_token: Some(card.public_token.clone()),
                loc_date: Some(utils::loc_date()),
                loc_time: Some(utils::loc_time()),
                start_bal,
                end_bal,
                txn_filter: parameters.txn_filter.clone(),
                start_date: Some(start_date.format("%Y-%m-%d").to_string()),
                end_date: Some(end_date.format("%Y-%m-%d").to_string()),
                num_txn: txs.transaction_2.len() as i32, // We report the number of transactions that we are returning
                item_src: parameters.item_src,
                cur_bill: Some(card.get_currency_info().iso_alpha_code),
                avl_bal: format!("{}", card.balance.amount),
                blk_amt: format!("{}", card.blocked_balance),
                sys_date: Some(utils::sys_date()),
                action_code: Some("000".to_string()),
                transactions: if txs.transaction_2.is_empty() {
                    None
                } else {
                    Some(txs)
                },
            },
        });

        match to_string(&response) {
            Err(e) => Err(types::GpsError::Serialization(e)),
            Ok(v) => Ok(v),
        }
    }

    fn report_not_successful(&self, error: &types::GpsError) -> Result<String, types::GpsError> {
        let parameters = &self.parameters;
        let action_code = utils::error_to_action_code(error);
        let response = self.wrap_response(gps_lib::types::WsCardStatementResponse {
            ws_card_statement_result: gps_lib::types::CardStatement2 {
                wsid: parameters.wsid,
                iss_code: parameters.iss_code.clone(),
                txn_code: parameters.txn_code.clone(),
                client_code: parameters.client_code.clone(),
                public_token: parameters.public_token.clone(),
                loc_date: Some(utils::loc_date()),
                loc_time: Some(utils::loc_time()),
                start_bal: format!("0.00"),
                end_bal: format!("0.00"),
                txn_filter: parameters.txn_filter.clone(),
                start_date: parameters.start_date.clone(),
                end_date: parameters.end_date.clone(),
                num_txn: parameters.num_txn,
                item_src: parameters.item_src,
                cur_bill: None,
                avl_bal: format!("0.00"),
                blk_amt: format!("0.00"),
                sys_date: Some(utils::sys_date()),
                action_code: Some(action_code),
                transactions: None,
            },
        });

        match to_string(&response) {
            Err(e) => Err(types::GpsError::Serialization(e)),
            Ok(v) => Ok(v),
        }
    }
}
/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iso_enum_name_matches_currency_iso_alpha_code() {
        for iso in ISO_CURRENCIES {
            let iso_alpha_code = format!("{}", *iso);
            let currency = get_currency_info(iso);
            assert_eq!(iso_alpha_code, currency.iso_alpha_code);
        }
    }

    #[test]
    fn no_duplicate_currency_numeric_iso_codes() {
        let mut numeric_codes: Vec<_> = ISO_CURRENCIES
            .iter()
            .map(|iso| get_currency_info(*iso).iso_numeric_code)
            .collect();

        numeric_codes.sort();
        numeric_codes.dedup();
        assert_eq!(ISO_CURRENCIES.len(), numeric_codes.len());
    }
}*/
