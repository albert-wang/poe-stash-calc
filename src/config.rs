use serde::{Deserialize};
use serde_json;

#[derive(Deserialize)]
pub struct Configuration {
    pub account_name: String,
    pub cookie: String,
}

pub fn load_configuration(file: &str) -> Result<Configuration, String> {
    let res = std::fs::read_to_string(file);
    match res {
        Ok(contents) => {
            let json_value = serde_json::from_str(&contents);
            match json_value {
                Ok(v) => Ok(v),
                Err(e) => Err(
                    format!("Couldn't parse json value: {}", e).to_string()
                ),
            }
        }
        Err(e) => Err(format!("Couldn't read configuration file: {}", e).to_string())
    }
}