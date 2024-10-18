use std::fs;
use std::error::Error;
use serde::Deserialize;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub username: String,
    pub password: String,
    pub key: String,
    pub shared_secret: String,
    pub steam64_id: String,
    pub web_cookie: String,
    pub duration: u8
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let toml_file = fs::read_to_string(path).expect("Error: Reading config file.");
        let config: Config = toml::from_str(&toml_file).expect("Error: Parsing TOML file.");
        Ok(config)
    }
}