use crate::types;


pub trait Action {
    fn new(contents: &str) -> Result<Self, types::GpsError> where Self: Sized;
    fn execute(&self, state: &types::State) -> Result<String, types::GpsError>;
}

pub fn extract_name(action: &str) -> Result<&str, types::GpsError> {
    let ns = "http://www.globalprocessing.ae/HyperionWeb";
    let re = format!("^\"{}/(\\w+)\"$", ns);
    let re = regex::Regex::new(&re[..]).unwrap();
    match re.captures(action) {
        None => Err(types::GpsError::Action(format!("{} is not a valid action", action))),
        Some(caps) => Ok(caps.get(1).unwrap().as_str()),
    }
}

// Implements the wrap_response method for a given action.
// It relies on the GPS types having sane names...
// Note that this macro uses paste! to concatenate identifiers.
#[macro_export]
macro_rules! impl_wrap_response {
    ($name:ident) => {
        paste! {
            impl $name {
                fn wrap_response(
                    &self,
                    contents: gps_lib::types::[<Ws $name Response>],
                ) -> gps_lib::bindings::[<Ws $name SoapOutSoapEnvelope>] {
                    gps_lib::bindings::[<Ws $name SoapOutSoapEnvelope>] {
                        tnsattr: None,
                        urnattr: None,
                        xsiattr: None,
                        header: None,
                        encoding_style: gps_lib::SOAP_ENCODING.to_string(),
                        body: gps_lib::bindings::[<SoapWs $name SoapOut>] {
                            body: gps_lib::messages::[<Ws $name SoapOut>] {
                                parameters: contents,
                            },
                            fault: None,
                        },
                    }
                }
            }
        }
    }
}
