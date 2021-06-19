use log::{debug, info};
use paste::paste;
use yaserde::de::from_str;
use yaserde::ser::to_string;

use crate::action;
use crate::impl_action_boilerplate;
use crate::impl_wrap_response;
use crate::types;
use crate::types::ActionResult;
use crate::types::GpsError;
use crate::utils;

pub struct ResultV2 {
    pub parameters: gps_lib::types::WsWebServiceResultV2,
    pub action_name: String,
}

impl_wrap_response!(ResultV2, WebServiceResultV2);

fn extract_prev_response(prev_action: &ActionResult) -> Result<&str, types::GpsError> {
    // We get the old response and parse it, because we only want the nodes that strictly
    // contain the response, i.e. the parent of the node <WSID>
    let root = roxmltree::Document::parse(&prev_action.response_sent).unwrap();
    let err_msg = format!("Could not parse the old response");
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

    // We need to know the position in the original string where the response starts and ends to
    // copy it into the serialized response that we send back.
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

    // The part of the previous response that we will insert in the new one
    Ok(&prev_action.response_sent[first.start..last.end])
}

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

        // We cannot put the serialized previous response here because it will be escaped when
        // serializing the struct. Therefore, we serialize it and manually manipulate the XML.
        let response = self.wrap_response(gps_lib::types::WsWebServiceResultV2Response {
            ws_web_service_result_v2_result: gps_lib::types::Wsresult2 {
                action_code: Some("000".to_string()),
                web_method: Some(prev_action.action_name.clone()),
                response_sent: Some(gps_lib::types::ResponseSent {
                    response: gps_lib::types::Response {
                        // This is the only way I found to force yaserde into putting the namespace
                        // in this node
                        xmlns: format!("http://www.globalprocessing.ae/HyperionWeb"),
                        ..Default::default()
                    },
                }),
            },
        });

        // Extract the part of the old response that we care about
        let prev_response = extract_prev_response(prev_action)?;

        // Here we have our serialized output, but we need to patch it to include the old response
        let serialized = to_string(&response).map_err(|e| types::GpsError::Serialization(e))?;
        let placeholder_range = {
            let root = roxmltree::Document::parse(&serialized).unwrap();
            root.descendants()
                .find(|x| x.tag_name().name() == "Placeholder")
                .ok_or(GpsError::ActionCode {
                    num: 999,
                    msg: format!("Could not find the placeholder node in '{}'", serialized),
                })?
                .range()
        };
        let mut serialized = serialized;
        serialized.replace_range(placeholder_range, prev_response);

        Ok(serialized)
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
