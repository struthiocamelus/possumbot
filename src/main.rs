extern crate serenity;
extern crate giphy;
extern crate reqwest;
extern crate dotenv;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use giphy::v1::sync::*;
use giphy::v1::gifs::RandomRequest;
use dotenv::dotenv;
use std::env;

struct Handler;

impl EventHandler for Handler {
	fn message(&self, _: Context, msg: Message) {
		if msg.content == "!possum" {
			let http_client = reqwest::Client::new();
			let giphy_api = SyncApi::new(env::var("POSSUM_GIPHY_TOKEN").unwrap(), http_client);

			let response = RandomRequest::new().with_tag("possum")
				.send_to(&giphy_api)
				.unwrap_or_else(|e| panic!("Error while calling search endpoint: {:?}", e));

			if let Err(why) = msg.reply(&response.data.embed_url) {
				println!("Error sending message: {:?}", why);
			}
		}
	}

	fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
	}
}

fn main() {
	dotenv().ok();

	let mut client = Client::new(&env::var("POSSUM_DISCORD_TOKEN").unwrap(), Handler).expect("Error creating client");

	if let Err(why) = client.start() {
		println!("An error occurred while running the client: {:?}", why);
	}
}