#[macro_use]
extern crate serenity;

extern crate config;

use serenity::client::Client;
use serenity::prelude::EventHandler;
use serenity::framework::standard::StandardFramework;

struct Handler;

impl EventHandler for Handler {}

fn main() {
	let mut settings = config::Config::default();
	settings.merge(config::File::with_name("possumconfig")).unwrap();

	let mut client = Client::new(&settings.get_str("discord.token").unwrap(), Handler)
		.expect("Error creating client");
	client.with_framework(StandardFramework::new()
						  .configure(|c| c.prefix(&settings.get_str("discord.prefix").unwrap()))
						  .cmd("ping", ping));
	
	if let Err(why) = client.start() {
		println!("An error occurred while running the client: {:?}", why);
	}
}

command!(ping(_context, msg) {
	let _ = msg.reply("pong!");
});