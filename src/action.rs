use crate::types;

pub trait Action {
    fn new(action_name: &str, contents: &str) -> Result<Self, types::GpsError>
    where
        Self: Sized;

    fn execute(&self, state: &types::State) -> Result<String, types::GpsError>;

    fn report_not_successful(&self, error: &types::GpsError) -> Result<String, types::GpsError>;

    fn get_wsid(&self) -> Option<u64>;

    fn get_action_name(&self) -> &str;
}

pub fn extract_name(action: &str) -> Result<&str, types::GpsError> {
    let ns = "http://www.globalprocessing.ae/HyperionWeb";
    let re = format!("^\"{}/(\\w+)\"$", ns);
    let re = regex::Regex::new(&re[..]).unwrap();
    match re.captures(action) {
        None => Err(types::GpsError::Action(format!(
            "{} is not a valid action",
            action
        ))),
        Some(caps) => Ok(caps.get(1).unwrap().as_str()),
    }
}

#[macro_export]
macro_rules! impl_action_boilerplate {
    ($name:ident) => {
        crate::impl_action_boilerplate_internal!($name, $name);
    };

    // In case we don't want to use the GPS name for our struct
    ($name:ident, $gps_name:ident) => {
        crate::impl_action_boilerplate_internal!($name, $gps_name);
    };
}
#[macro_export]
macro_rules! impl_action_boilerplate_internal {
    ($struct_name:ident, $gps_name:ident) => {
        paste! {
            fn new(action_name: &str, contents: &str) -> Result<Self, types::GpsError>
                where Self: Sized
            {
                let envelope: Result<gps_lib::bindings::[<Ws $gps_name SoapInSoapEnvelope>], std::string::String> =
                    from_str(&contents);
                match envelope {
                    Err(e) => Err(types::GpsError::RequestData(e)),
                    Ok(v) => Ok($struct_name {
                        parameters: v.body.body.parameters,
                        action_name : action_name.to_string(),
                    }),
                }
            }

            fn get_wsid(&self) -> Option<u64> {
                Some(self.parameters.wsid as u64)
            }

            fn get_action_name(&self) -> &str {
                &self.action_name
            }
        }
    }
}

// Implements the wrap_response method for a given action.
// It relies on the GPS types having sane names...
// Note that this macro uses paste! to concatenate identifiers.
#[macro_export]
macro_rules! impl_wrap_response {
    ($name:ident) => {
        crate::impl_wrap_response_internal!($name, $name);
    };

    // In case we don't want to use the GPS name for our struct
    ($name:ident, $gps_name:ident) => {
        crate::impl_wrap_response_internal!($name, $gps_name);
    };
}
#[macro_export]
macro_rules! impl_wrap_response_internal {
    ($struct_name:ident, $gps_name:ident) => {
        paste! {
            impl $struct_name {
                fn wrap_response(
                    &self,
                    contents: gps_lib::types::[<Ws $gps_name Response>],
                ) -> gps_lib::bindings::[<Ws $gps_name SoapOutSoapEnvelope>] {
                    gps_lib::bindings::[<Ws $gps_name SoapOutSoapEnvelope>] {
                        tnsattr: None,
                        urnattr: None,
                        xsiattr: None,
                        header: None,
                        body: gps_lib::bindings::[<SoapWs $gps_name SoapOut>] {
                            body: gps_lib::messages::[<Ws $gps_name SoapOut>] {
                                parameters: contents,
                            },
                            fault: None,
                        },
                    }
                }
            }
        }
    };
}
