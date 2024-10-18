use std::fs;
use serde::Deserialize;
use toml;

#[derive(Debug, Deserialize)]
struct Config {
    username: String,
}

fn main() {
    let toml_file = fs::read_to_string("config.toml").expect("Error: Reading config file.");
    let config: Config = toml::from_str(&toml_file).expect("Error: Parsing TOML file.");
    println!("{:?}", config.username);
}