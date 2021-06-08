use num_traits::FromPrimitive;
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

pub struct StatusChange {
    pub parameters: gps_lib::types::WsStatusChange,
    pub action_name: String,
}

impl_wrap_response!(StatusChange);

impl action::Action for StatusChange {
    impl_action_boilerplate!(StatusChange);

    fn execute(&self, state: &types::State) -> Result<String, types::GpsError> {
        let parameters = &self.parameters;

        get_mut_card!(self.parameters.public_token, state, card, cards_map);

        let new_status = self
            .parameters
            .new_stat_code
            .clone()
            .ok_or(GpsError::ActionCode {
                num: 999,
                msg: format!("New status missing"),
            })?
            .parse::<u32>()?;
        let new_status: Option<types::CardStatus> = FromPrimitive::from_u32(new_status);
        let new_status = new_status.ok_or(GpsError::ActionCode {
            num: 999,
            msg: format!("The new status is not valid"),
        })?;

        card.status = new_status;

        let response = self.wrap_response(gps_lib::types::WsStatusChangeResponse {
            ws_status_change_result: gps_lib::types::StatusChange {
                wsid: parameters.wsid,
                iss_code: parameters.iss_code.clone(),
                txn_code: parameters.txn_code.clone(),
                public_token: parameters.public_token.clone(),
                loc_date: Some(utils::loc_date()),
                loc_time: Some(utils::loc_time()),
                client_code: parameters.client_code.clone(),
                sys_date: Some(utils::sys_date()),
                action_code: Some("000".to_string()),
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
        let response = self.wrap_response(gps_lib::types::WsStatusChangeResponse {
            ws_status_change_result: gps_lib::types::StatusChange {
                wsid: parameters.wsid,
                iss_code: parameters.iss_code.clone(),
                txn_code: parameters.txn_code.clone(),
                public_token: parameters.public_token.clone(),
                loc_date: Some(utils::loc_date()),
                loc_time: Some(utils::loc_time()),
                client_code: parameters.client_code.clone(),
                sys_date: Some(utils::sys_date()),
                action_code: Some(action_code),
            },
        });

        match to_string(&response) {
            Err(e) => Err(types::GpsError::Serialization(e)),
            Ok(v) => Ok(v),
        }
    }
}
