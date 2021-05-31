use log::info;
use paste::paste;
use yaserde::de::from_str;
use yaserde::ser::to_string;

use crate::action;
use crate::get_card;
use crate::impl_action_boilerplate;
use crate::impl_wrap_response;
use crate::types;
use crate::types::GpsError;
use crate::utils;

pub struct BalanceEnquiry {
    pub parameters: gps_lib::types::WsBalanceEnquiry,
    pub action_name: String,
}

pub struct BalanceEnquiryV2 {
    pub parameters: gps_lib::types::WsBalanceEnquiryV2,
    pub action_name: String,
}

impl_wrap_response!(BalanceEnquiry);
impl_wrap_response!(BalanceEnquiryV2);

impl action::Action for BalanceEnquiry {
    impl_action_boilerplate!(BalanceEnquiry);

    fn execute(&self, state: &types::State) -> Result<String, types::GpsError> {
        let parameters = &self.parameters;

        info!("Parameters: {:?}", parameters);
        get_card!(self.parameters.public_token, state, card, cards_map);

        let response = self.wrap_response(gps_lib::types::WsBalanceEnquiryResponse {
            ws_balance_enquiry_result: gps_lib::types::BalanceEnquire {
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
                blk_amt: format!("{}", card.blocked_balance),
                cur_code: Some(card.get_currency_info().iso_numeric_code),
                pin_status: 0,    // TODO: WTF is this?
                limit_info: None, // TODO: WTF is this?
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
        let response = self.wrap_response(gps_lib::types::WsBalanceEnquiryResponse {
            ws_balance_enquiry_result: gps_lib::types::BalanceEnquire {
                wsid: parameters.wsid,
                iss_code: parameters.iss_code.clone(),
                txn_code: parameters.txn_code.clone(),
                public_token: parameters.public_token.clone(),
                loc_date: Some(utils::loc_date()),
                loc_time: Some(utils::loc_time()),
                client_code: parameters.client_code.clone(),
                sys_date: Some(utils::sys_date()),
                action_code: Some(action_code),
                avl_bal: format!("0.0"),
                blk_amt: format!("0.0"),
                ..Default::default()
            },
        });

        match to_string(&response) {
            Err(e) => Err(types::GpsError::Serialization(e)),
            Ok(v) => Ok(v),
        }
    }
}

impl action::Action for BalanceEnquiryV2 {
    impl_action_boilerplate!(BalanceEnquiryV2);

    fn execute(&self, state: &types::State) -> Result<String, types::GpsError> {
        let parameters = &self.parameters;

        info!("Parameters: {:?}", parameters);

        get_card!(self.parameters.public_token, state, card, cards_map);

        let response = self.wrap_response(gps_lib::types::WsBalanceEnquiryV2Response {
            ws_balance_enquiry_v2_result: Some(gps_lib::types::BalanceEnquire2 {
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
                blk_amt: format!("{}", card.blocked_balance),
                cur_code: Some(card.get_currency_info().iso_numeric_code),
                pin_status: 0,    // TODO: WTF is this?
                limit_info: None, // TODO: WTF is this?
            }),
        });

        match to_string(&response) {
            Err(e) => Err(types::GpsError::Serialization(e)),
            Ok(v) => Ok(v),
        }
    }

    fn report_not_successful(&self, error: &types::GpsError) -> Result<String, types::GpsError> {
        let parameters = &self.parameters;
        let action_code = utils::error_to_action_code(error);
        let response = self.wrap_response(gps_lib::types::WsBalanceEnquiryV2Response {
            ws_balance_enquiry_v2_result: Some(gps_lib::types::BalanceEnquire2 {
                wsid: parameters.wsid,
                iss_code: parameters.iss_code.clone(),
                txn_code: parameters.txn_code.clone(),
                public_token: parameters.public_token.clone(),
                loc_date: Some(utils::loc_date()),
                loc_time: Some(utils::loc_time()),
                client_code: parameters.client_code.clone(),
                sys_date: Some(utils::sys_date()),
                action_code: Some(action_code),
                avl_bal: format!("0.0"),
                blk_amt: format!("0.0"),
                ..Default::default()
            }),
        });

        match to_string(&response) {
            Err(e) => Err(types::GpsError::Serialization(e)),
            Ok(v) => Ok(v),
        }
    }
}
