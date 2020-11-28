pub trait Action {
    fn new(contents: &str) -> Result<Self, String> where Self: Sized;
    fn execute(&self) -> Result<String, String>;
}

pub fn extract_name(action: &str) -> Result<&str, String> {
    let ns = "http://www.globalprocessing.ae/HyperionWeb";
    let re = format!("^\"{}/(\\w+)\"$", ns);
    let re = regex::Regex::new(&re[..]).unwrap();
    match re.captures(action) {
        None => Err(format!("{} is not a valid action", action)),
        Some(caps) => Ok(caps.get(1).unwrap().as_str()),
    }
}
