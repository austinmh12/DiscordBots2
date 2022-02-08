use dotenv;

async fn api_call(endpoint: &str, params: Option<&str>) -> Option<serde_json::Value> {
	dotenv::dotenv().ok();
	let poketcg_key = dotenv::var("POKETCGAPIKEY").unwrap();
	let client = reqwest::Client::new();

	let mut req = client
		.get(format!("https://api.pokemontcg.io/v2/{}", endpoint))
		.header("X-Api-Key", poketcg_key);
	req = match params {
		Some(x) => req.query(&[("q", x)]),
		None => req
	};
	let data: serde_json::Value = req
		.send().await.unwrap()
		.json().await.unwrap();
	

	if data.is_null() {
		None
	} else {
		Some(data)
	}
}

pub mod card;

// use std::{
// 	time::Duration,
// 	sync::{
// 		Arc,
// 	},
// 	collections::HashMap,
// };

use serenity::framework::standard::{
	macros::{
		command,
	},
	Args,
	CommandResult
};
use serenity::model::{
	channel::{Message, ReactionType},
	//id::{ChannelId}
	//prelude::*,
};
use serenity::utils::Colour;
use serenity::prelude::*;
//use serenity::collector::MessageCollectorBuilder;
use serde_json;

#[command]
async fn search(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	println!("Calling search");
	let search_str = args.rest();
	let left_arrow = ReactionType::try_from("⬅️").unwrap();
	let right_arrow = ReactionType::try_from("➡️").unwrap();
	println!("Got emojis");
	let cards = card::get_cards_with_query(search_str).await;
	println!("Got cards: {}", cards.len());
	let cur_card = &cards[0];
	let _ = msg
		.channel_id
		.send_message(&ctx.http, |m| {
			m.embed(|e| {
				e.title(cur_card.name.clone())
					.description(format!("**ID:** {}\n**Price:** ${:.2}\n", cur_card.id.clone(), cur_card.price.clone()))
					.colour(Colour::from_rgb(255, 50, 20))
					.image(cur_card.image.clone())
			})
			.reactions([left_arrow, right_arrow])
		}).await;

	Ok(())
}

/* Command list
 * mycards
 * sell
 * 		card
 * 		under
 * 		dups
 * 		all
 * 		packs
 * search
 * sets
 * set
 * packs
 * openpack
 * store
 * stats
 * daily
 * quiz
 * savelist
 * 		add
 * 		remove
 * 		clear
 * trade
 * 		card
 * 		pack
*/

/* Tasks
 * Refresh daily packs
 * Cache store packs
 * Cache player cards
*/

/* Admin commands
 * cache
 * addcash
 * resetpacks
 * resetquiz
*/

/* Other things
 * Need to find a way to replicate the paginated embeds
 * 		I should look into how to work with generics so I can make a function like
 * 		async fn paginated_embeds(pages: Vec<T>)
 * Need to find a way to implement the store
 * Need to find a way to implement the cache
 * Need to learn how to add and wait for reactions
*/