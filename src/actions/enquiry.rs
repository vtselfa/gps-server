use yaserde::de::from_str;
use yaserde::ser::to_string;
use paste::paste;

use crate::types;
use crate::utils;
use crate::get_card;
use crate::impl_wrap_response;
use crate::types::GpsError;
use crate::action;


pub struct Enquiry {
    pub parameters: gps_lib::types::WsEnquiry,
}

impl_wrap_response!(Enquiry);

impl action::Action for Enquiry {
    fn new(contents: &str) -> Result<Self, types::GpsError> {
        let envelope: Result<gps_lib::bindings::WsEnquirySoapInSoapEnvelope, std::string::String> =
            from_str(&contents);
        match envelope {
            Err(e) => Err(types::GpsError::RequestData(e)),
            Ok(v) => Ok(Enquiry {
                parameters: v.body.body.parameters,
            }),
        }
    }

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
                avl_bal: format!("{}", card.balance.amount()),
                blk_amt: format!("0.0"), // TODO: Implement blocked balances in cards
                cur_code: Some(card.currency.iso_numeric_code.to_string()),
                start_date: Some(card.get_start_date()),
                end_date: Some(card.get_end_date()),
                exp_date: Some(card.get_exp_date()),
                stat_code: Some(card.get_status_code()),
                emboss_name: Some(card.get_emboss_name()),
                is_live: card.is_live,
                scheme: None, // TODO: We use it 
                product: None, // TODO: We use it
                masked_pan: Some(card.pan),
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
}
