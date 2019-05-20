extern crate serenity;
extern crate giphy;
extern crate reqwest;
extern crate dotenv;
extern crate rand;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use giphy::v1::sync::*;
use giphy::v1::gifs::RandomRequest;
use dotenv::dotenv;
use std::env;
use rand::Rng;

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
		} else if msg.content == "!ping" {
			if let Err(why) = msg.reply("Pong!") {
				println!("Error sending message: {:?}", why);
			}
		} else if msg.content.contains("!8ball") {
			if let Err(why) = msg.reply(&eightball()) {
				println!("Error sending message: {:?}", why);
			}
		}
	}

	fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
	}
}

fn eightball() -> String {
	let choices = ["It is certain.", "It is decidedly so.", "Without a doubt.", "Yes - Definitely",
				   "You may rely on it.", "As I see it, yes.", "Most likely.", "Outlook is good.",
				   "Yes.", "Signs point to yes.", "Reply hazy, try again.", "Ask again later.",
				   "Better not tell you now.", "Cannot predict now.", "Concentrate and ask again", "Don't count on it.",
				   "My reply is no.", "My sources say no.", "Outlook not so good.", "Very doubtful.", "No"];
	let mut rng = rand::thread_rng();

	return rng.choose(&choices).unwrap().to_string();
}

fn main() {
	dotenv().ok();

	let mut client = Client::new(&env::var("POSSUM_DISCORD_TOKEN").unwrap(), Handler).expect("Error creating client");

	if let Err(why) = client.start() {
		println!("An error occurred while running the client: {:?}", why);
	}
}