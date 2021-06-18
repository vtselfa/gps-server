use log::{debug, info};
use paste::paste;
use yaserde::de::from_str;
use yaserde::ser::to_string;

use crate::action;
use crate::impl_action_boilerplate;
use crate::impl_wrap_response;
use crate::types;
use crate::types::GpsError;
use crate::utils;

pub struct ResultV2 {
    pub parameters: gps_lib::types::WsWebServiceResultV2,
    pub action_name: String,
}

impl_wrap_response!(ResultV2, WebServiceResultV2);

impl action::Action for ResultV2 {
    impl_action_boilerplate!(ResultV2, WebServiceResultV2);

    fn execute(&self, state: &types::State) -> Result<String, types::GpsError> {
        let parameters = &self.parameters;

        debug!("Parameters: {:?}", parameters);
        info!("Requested WSID: {}", parameters.wsid);

        let wsid_map = state.wsids.read().expect("Poisoned write lock");

        let prev_action = wsid_map
            .get(&(parameters.wsid as u64))
            .ok_or(GpsError::ActionCode {
                num: 613,
                msg: format!("Invalid WSID"),
            })?;

        // TODO: Do this better...
        // We are puting a magic string in the response that we will later replace in the
        // serialized XML. We cannot put the serialized data here because it will be escaped when
        // serializing the struct.
        let placeholder = format!("P.L.A.C.E.H.O.L.D.E.R");
        let response = self.wrap_response(gps_lib::types::WsWebServiceResultV2Response {
            ws_web_service_result_v2_result: gps_lib::types::Wsresult2 {
                action_code: Some("000".to_string()),
                web_method: Some(prev_action.action_name.clone()),
                response_sent: Some(gps_lib::types::ResponseSent {
                    response: placeholder.clone(),
                }),
            },
        });

        let root = roxmltree::Document::parse(&prev_action.response_sent).unwrap();
        let err_msg = format!(
            "Could not parse the old response associated to WSID {}",
            parameters.wsid
        );
        let prev_response = root
            .descendants()
            .find(|x| x.tag_name().name().to_lowercase() == "wsid")
            .ok_or(GpsError::ActionCode {
                num: 999,
                msg: err_msg.clone(),
            })?
            .parent_element()
            .ok_or(GpsError::ActionCode {
                num: 999,
                msg: err_msg.clone(),
            })?;
        let first = prev_response
            .first_child()
            .ok_or(GpsError::ActionCode {
                num: 999,
                msg: err_msg.clone(),
            })?
            .range();
        let last = prev_response
            .last_child()
            .ok_or(GpsError::ActionCode {
                num: 999,
                msg: err_msg.clone(),
            })?
            .range();

        match to_string(&response) {
            Err(e) => Err(types::GpsError::Serialization(e)),
            Ok(v) => {
                let r = v.replace(
                    &placeholder,
                    &prev_action.response_sent[first.start..last.end],
                );
                Ok(r)
            }
        }
    }

    fn report_not_successful(&self, error: &types::GpsError) -> Result<String, types::GpsError> {
        let action_code = utils::error_to_action_code(error);
        let response = self.wrap_response(gps_lib::types::WsWebServiceResultV2Response {
            ws_web_service_result_v2_result: gps_lib::types::Wsresult2 {
                action_code: Some(action_code),
                ..Default::default()
            },
        });

        match to_string(&response) {
            Err(e) => Err(types::GpsError::Serialization(e)),
            Ok(v) => Ok(v),
        }
    }
}
