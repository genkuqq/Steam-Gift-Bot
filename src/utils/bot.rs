use super::config::Config;
use std::error::Error;
use reqwest::blocking::Client;

#[derive(Debug)]
pub struct Bot {
	pub username: String,
	pub password: String,
	pub key: String,
	pub shared_secret: String,
	pub steam64_id: String,
	pub web_cookie: String,
	pub duration: u8
}

impl Bot {
	pub fn new(config: Config) -> Self{
		Self{
			username: config.username,
			password: config.password,
			key: config.key,
			shared_secret: config.shared_secret,
			steam64_id: config.steam64_id,
			web_cookie: config.web_cookie,
			duration: config.duration
		}
	}
	pub fn pull_trades(&self, params: [(&str, &str); 4]) -> Result<(), Box<dyn Error>> {
		let client = Client::new();
		let request = reqwest::Url::parse_with_params("https://api.steampowered.com/IEconService/GetTradeOffers/v1/",params)?;
		let response = client.get(request).send()?;
		println!("result = {:#?}", response.text()?);
		Ok(())
	}
}