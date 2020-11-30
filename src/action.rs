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
