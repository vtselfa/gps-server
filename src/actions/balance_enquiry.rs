use yaserde::de::from_str;
use yaserde::ser::to_string;

use crate::types;
use crate::utils;
use crate::get_card;
use crate::impl_wrap_response;
use crate::types::GpsError;
use crate::action;

use paste::paste;

pub struct BalanceEnquiry {
    pub parameters: gps_lib::types::WsBalanceEnquiry,
}

impl_wrap_response!(BalanceEnquiry);

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
                avl_bal: format!("{}", card.balance.amount()),
                blk_amt: format!("0.0"), // TODO: Implement blocked balances in cards
                cur_code: Some(card.currency.iso_numeric_code.to_string()),
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
