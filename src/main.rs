use std::{thread, time::Duration};
use utils::config::Config;
mod utils;
use crate::utils::bot::Bot;

const HEX: u128 = 76561197960265728;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let config = Config::from_file("config.toml");
	let bot = Bot::new(config?);
	let params: [(&str, &str); 4] = [
		("key", &bot.key),
		("get_sent_offers", "false"),
		("get_received_offers", "true"),
		("active_only", "true"),
		("test", "test"),
	];
	loop {
		println!("* Trade Bot pulling Trade Offers *");
		let _ = bot.pull_trades(params);
		thread::sleep(Duration::from_secs(bot.duration.into()));
	}
}
