use log::{debug, info};
use paste::paste;
use yaserde::de::from_str;
use yaserde::ser::to_string;

use crate::action;
use crate::get_mut_card;
use crate::impl_action_boilerplate;
use crate::impl_wrap_response;
use crate::types;
use crate::types::GpsError;
use crate::utils;

pub struct Activate {
    pub parameters: gps_lib::types::WsActivate,
    pub action_name: String,
}

impl_wrap_response!(Activate);

impl action::Action for Activate {
    impl_action_boilerplate!(Activate);

    fn execute(&self, state: &types::State) -> Result<String, types::GpsError> {
        let parameters = &self.parameters;

        debug!("Parameters: {:?}", parameters);
        get_mut_card!(self.parameters.public_token, state, card, cards_map);
        info!("PublicToken: {}", card.public_token);

        card.is_live = true;

        let response = self.wrap_response(gps_lib::types::WsActivateResponse {
            ws_activate_result: gps_lib::types::Activate {
                wsid: parameters.wsid,
                iss_code: parameters.iss_code.clone(),
                txn_code: parameters.txn_code.clone(),
                client_code: parameters.client_code.clone(),
                public_token: parameters.public_token.clone(),
                loc_date: Some(utils::loc_date()),
                loc_time: Some(utils::loc_time()),
                sys_date: Some(utils::sys_date()),
                action_code: Some("000".to_string()),
                is_live: card.is_live,
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
        let response = self.wrap_response(gps_lib::types::WsActivateResponse {
            ws_activate_result: gps_lib::types::Activate {
                wsid: parameters.wsid,
                iss_code: parameters.iss_code.clone(),
                txn_code: parameters.txn_code.clone(),
                client_code: parameters.client_code.clone(),
                public_token: parameters.public_token.clone(),
                loc_date: Some(utils::loc_date()),
                loc_time: Some(utils::loc_time()),
                sys_date: Some(utils::sys_date()),
                action_code: Some(action_code),
                is_live: false,
            },
        });

        match to_string(&response) {
            Err(e) => Err(types::GpsError::Serialization(e)),
            Ok(v) => Ok(v),
        }
    }
}
