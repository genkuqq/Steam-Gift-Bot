use std::{fs, thread, time::Duration};
use serde::Deserialize;
use toml;
use reqwest;

#[derive(Debug, Deserialize)]
struct Config {
    username: String,
    password: String,
    key: String,
    shared_secret: String,
    steam64_id: String,
    web_cookie: String,
}

const HEX: u128 = 76561197960265728;

fn main() -> Result<(),Box<dyn std::error::Error>> {
    let toml_file = fs::read_to_string("config.toml").expect("Error: Reading config file.");
    let config: Config = toml::from_str(&toml_file).expect("Error: Parsing TOML file.");
    let params: [(&str, &str); 4] = [("key", &config.key),("get_sent_offers","false"),("get_received_offers", "true"),("active_only", "true")];
    loop {
        println!("* Trade Bot pulling Trade Offers *");
        let response = reqwest::Url::parse_with_params("https://api.steampowered.com/IEconService/GetTradeOffers/v1/",params)?;
        let res = reqwest::blocking::get(response)?;
        println!("result = {:#?}", res.text().unwrap());
        thread::sleep(Duration::from_secs(1));
    }
}