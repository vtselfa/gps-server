use log::info;
use paste::paste;
use yaserde::de::from_str;
use yaserde::ser::to_string;

use crate::action;
use crate::get_card;
use crate::impl_wrap_response;
use crate::types::GpsError;
use crate::types;
use crate::utils;


pub struct BalanceEnquiry {
    pub parameters: gps_lib::types::WsBalanceEnquiry,
}


pub struct BalanceEnquiryV2 {
    pub parameters: gps_lib::types::WsBalanceEnquiryV2,
}


impl_wrap_response!(BalanceEnquiry);
impl_wrap_response!(BalanceEnquiryV2);


impl action::Action for BalanceEnquiry {
    fn new(contents: &str) -> Result<Self, types::GpsError> {
        let envelope: Result<gps_lib::bindings::WsBalanceEnquirySoapInSoapEnvelope, std::string::String> =
            from_str(&contents);
        match envelope {
            Err(e) => Err(types::GpsError::RequestData(e)),
            Ok(v) => Ok(BalanceEnquiry {
                parameters: v.body.body.parameters,
            }),
        }
    }

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
                blk_amt: format!("0.0"), // TODO: Implement blocked balances in cards
                cur_code: Some(card.get_currency_info().iso_numeric_code),
                pin_status: 0, // TODO: WTF is this?
                limit_info: None, // TODO: WTF is this?
            },
        });

        match to_string(&response) {
            Err(e) => Err(types::GpsError::Serialization(e)),
            Ok(v) => Ok(v),
        }
    }
}


impl action::Action for BalanceEnquiryV2 {
    fn new(contents: &str) -> Result<Self, types::GpsError> {
        let envelope: Result<gps_lib::bindings::WsBalanceEnquiryV2SoapInSoapEnvelope, std::string::String> =
            from_str(&contents);
        match envelope {
            Err(e) => Err(types::GpsError::RequestData(e)),
            Ok(v) => Ok(BalanceEnquiryV2 {
                parameters: v.body.body.parameters,
            }),
        }
    }

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
                blk_amt: format!("0.0"), // TODO: Implement blocked balances in cards
                cur_code: Some(card.get_currency_info().iso_numeric_code),
                pin_status: 0, // TODO: WTF is this?
                limit_info: None, // TODO: WTF is this?
            }),
        });

        match to_string(&response) {
            Err(e) => Err(types::GpsError::Serialization(e)),
            Ok(v) => Ok(v),
        }
    }
}
