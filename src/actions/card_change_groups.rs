use log::warn;
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

pub struct CardChangeGroups {
    pub parameters: gps_lib::types::WsCardChangeGroups,
    pub action_name: String,
}

impl_wrap_response!(CardChangeGroups);

impl action::Action for CardChangeGroups {
    impl_action_boilerplate!(CardChangeGroups);

    fn execute(&self, state: &types::State) -> Result<String, types::GpsError> {
        let parameters = &self.parameters;

        debug!("Parameters: {:?}", parameters);
        get_mut_card!(self.parameters.public_token, state, card, cards_map);
        info!("PublicToken: {}", card.public_token);

        warn!("The Ws_Card_Change_Groups method is not fully implemented");

        if let Some(limits_group) = self.parameters.limits_group.clone() {
            card.limits_group = limits_group;
        }
        if let Some(mcc_group) = self.parameters.mcc_group.clone() {
            card.mcc_group = mcc_group;
        }
        if let Some(perms_group) = self.parameters.perms_group.clone() {
            card.perms_group = perms_group;
        }
        if let Some(ws_fee_group) = self.parameters.ws_fee_group.clone() {
            card.ws_fee_group = ws_fee_group;
        }
        if let Some(linkage_group) = self.parameters.linkage_group.clone() {
            card.linkage_group = linkage_group;
        }
        if let Some(auth_calendar_group) = self.parameters.auth_calendar_group.clone() {
            card.auth_calendar_group = auth_calendar_group.clone();
        }
        if let Some(fx_group) = self.parameters.fx_group.clone() {
            card.fx_group = fx_group;
        }

        let response = self.wrap_response(gps_lib::types::WsCardChangeGroupsResponse {
            ws_card_change_groups_result: gps_lib::types::ChangeGroup {
                wsid: parameters.wsid,
                iss_code: parameters.iss_code.clone(),
                public_token: parameters.public_token.clone(),
                loc_date: Some(utils::loc_date()),
                loc_time: Some(utils::loc_time()),
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
        let response = self.wrap_response(gps_lib::types::WsCardChangeGroupsResponse {
            ws_card_change_groups_result: gps_lib::types::ChangeGroup {
                wsid: parameters.wsid,
                iss_code: parameters.iss_code.clone(),
                public_token: parameters.public_token.clone(),
                loc_date: Some(utils::loc_date()),
                loc_time: Some(utils::loc_time()),
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
