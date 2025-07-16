use crate::constant::CONFIG_PATH;
use crate::constant::config::AppConfig;
use serde_json::from_str;
use std::fs::read_to_string;

pub fn parser_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    let config_content = read_to_string(&*CONFIG_PATH).expect("Failed to read config file");
    let app_config = from_str(&config_content)?;
    Ok(app_config)
}
