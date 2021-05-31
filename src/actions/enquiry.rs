use yaserde::de::from_str;
use yaserde::ser::to_string;
use paste::paste;

use crate::types;
use crate::utils;
use crate::get_card;
use crate::impl_wrap_response;
use crate::impl_action_boilerplate;
use crate::types::GpsError;
use crate::action;


pub struct Enquiry {
    pub parameters: gps_lib::types::WsEnquiry,
    pub action_name: String,
}

impl_wrap_response!(Enquiry);

impl action::Action for Enquiry {
    impl_action_boilerplate!(Enquiry);

    fn execute(&self, state: &types::State) -> Result<String, types::GpsError> {
        let parameters = &self.parameters;

        get_card!(self.parameters.public_token, state, card, cards_map);

        let response = self.wrap_response(gps_lib::types::WsEnquiryResponse {
            ws_enquiry_result: gps_lib::types::Card2 {
                wsid: parameters.wsid,
                iss_code: parameters.iss_code.clone(),
                txn_code: parameters.txn_code.clone(),
                public_token: Some(card.public_token.clone()),
                loc_date: Some(utils::loc_date()),
                loc_time: Some(utils::loc_time()),
                client_code: parameters.client_code.clone(),
                sys_date: Some(utils::sys_date()),
                action_code: Some("000".to_string()),
                avl_bal: format!("{}", card.balance.amount),
                blk_amt:  format!("{}", card.blocked_balance),
                cur_code: Some(card.get_currency_info().iso_numeric_code),
                start_date: Some(card.get_start_date()),
                end_date: Some(card.get_end_date()),
                exp_date: Some(card.get_exp_date()),
                stat_code: Some(card.get_status_code()),
                emboss_name: Some(card.get_emboss_name()),
                is_live: card.is_live,
                scheme: None, // TODO: We use it
                product: None, // TODO: We use it
                masked_pan: Some(card.pan.clone()),
                limit_group: None, // TODO: We use it
                mcc_group: None,
                perms_group: None, // TODO: We use it
                fee_group: None,
                sched_fee_group: None,
                ws_fee_group: None,
                linkage_group: None, // TODO: We use it
                primary_token: Some(card.public_token.clone()), // TODO: When is it different?
                auth_calendar_group: None,
                fx_group: None,
                black_list: None,
                white_list: None,
                payment_token_usage_group: None, // We use this
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
        let response = self.wrap_response(gps_lib::types::WsEnquiryResponse {
            ws_enquiry_result: gps_lib::types::Card2 {
                wsid: parameters.wsid,
                iss_code: parameters.iss_code.clone(),
                txn_code: parameters.txn_code.clone(),
                public_token: parameters.public_token.clone(),
                cur_code: parameters.cur_code.clone(),
                loc_date: Some(utils::loc_date()),
                loc_time: Some(utils::loc_time()),
                client_code: parameters.client_code.clone(),
                sys_date: Some(utils::sys_date()),
                action_code: Some(action_code),
                avl_bal: format!("0.00"),
                blk_amt:  format!("0.00"),
                ..Default::default()
            },
        });

        match to_string(&response) {
            Err(e) => Err(types::GpsError::Serialization(e)),
            Ok(v) => Ok(v),
        }
    }
}
