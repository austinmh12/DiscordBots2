use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use mongodb::{
	bson::{
		doc,
		Document,
		oid::ObjectId,
	}, 
	Collection
};
use chrono::{
	DateTime, 
	Utc, Local,
};
use futures::stream::{TryStreamExt};
use serenity::{builder::CreateEmbed, utils::Colour};

use crate::{
	commands::get_client
};

use super::{PaginateEmbed, upgrade::Upgrade, binder::Binder};

fn def_10() -> i64 {
	10
}

fn def_0() -> i64 {
	0
}

fn def_upgrade() -> Upgrade {
	Upgrade::new()
}

fn def_false() -> bool {
	false
}

fn def_binder() -> Binder {
	Binder::empty()
}

fn def_empty_vec_str() -> Vec<String> {
	vec![]
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	id: Option<ObjectId>,
	pub discord_id: i64,
	pub cash: f64,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub daily_reset: DateTime<Utc>,
	pub packs: HashMap<String, i64>,
	pub packs_opened: i64,
	pub packs_bought: i64,
	pub total_cash: f64,
	pub cards: HashMap<String, i64>,
	pub total_cards: i64,
	pub cards_sold: i64,
	pub daily_packs: i64,
	pub quiz_questions: i64,
	pub current_multiplier: i64,
	pub quiz_correct: i64,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub quiz_reset: DateTime<Utc>,
	pub savelist: Vec<String>,
	pub perm_multiplier: i64,
	#[serde(default = "def_10")]
	pub daily_slots: i64,
	#[serde(default = "def_0")]
	pub slots_rolled: i64,
	#[serde(default = "def_0")]
	pub jackpots: i64,
	#[serde(default = "def_0")]
	pub boofs: i64,
	#[serde(default = "def_0")]
	pub tokens: i64,
	#[serde(default = "def_0")]
	pub total_tokens: i64,
	#[serde(default = "def_upgrade")]
	pub upgrades: Upgrade,
	#[serde(default = "def_false")]
	pub light_mode: bool,
	#[serde(default = "def_binder")]
	pub current_binder: Binder,
	#[serde(default = "def_empty_vec_str")]
	pub completed_binders: Vec<String>
}

impl Player {
	fn new_from_discord_id(discord_id: i64) -> Self {
		Self {
			id: None,
			discord_id,
			cash: 25.0,
			daily_reset: Utc::now(),
			packs: HashMap::new(),
			packs_opened: 0,
			packs_bought: 0,
			total_cash: 25.0,
			cards: HashMap::new(),
			total_cards: 0,
			cards_sold: 0,
			daily_packs: 50,
			quiz_questions: 5,
			current_multiplier: 1,
			quiz_correct: 0,
			quiz_reset: Utc::now(),
			savelist: vec![],
			perm_multiplier: 50,
			daily_slots: 10,
			slots_rolled: 0,
			jackpots: 0,
			boofs: 0,
			tokens: 0,
			total_tokens: 0,
			upgrades: Upgrade::new(),
			light_mode: false,
			current_binder: Binder::empty(),
			completed_binders: vec![],
		}
	}
}

impl PaginateEmbed for Player {
	fn embed(&self) -> CreateEmbed {
		let quiz_reset_local: DateTime<Local> = DateTime::from(self.quiz_reset);
		let daily_reset_local: DateTime<Local> = DateTime::from(self.daily_reset);
		let mut desc = format!("**Wallet:** ${:.2} | **Total Earned:** ${:.2}\n\n", &self.cash, &self.total_cash);
		desc.push_str(&format!("**Current Packs:** {}\n", self.packs.values().map(|v| v.clone() as i32).sum::<i32>()));
		desc.push_str(&format!("**Opened Packs:** {} | **Bought Packs:** {}\n\n", &self.packs_opened, &self.packs_bought));
		desc.push_str(&format!("**Total Cards:** {} | **Cards Sold:** {}\n\n", &self.total_cards, &self.cards_sold));
		desc.push_str(&format!("**Slot Rolls:** {} | **Slots Rolled:** {}\n", &self.daily_slots, &self.slots_rolled));
		desc.push_str(&format!("**Tokens:** {} | **Total Tokens:** {}\n", &self.tokens, &self.total_tokens));
		desc.push_str(&format!("**Jackpots:** {} | **Boofs:** {}\n\n", &self.jackpots, &self.boofs));
		desc.push_str(&format!("**Quiz Questions Remaining:** {}\n", &self.quiz_questions));
		desc.push_str(&format!("**Quiz Questions Answered:** {}\n\n", &self.quiz_correct));
		desc.push_str(&format!("Quiz resets at **{}**\n", quiz_reset_local.format("%m/%d %H:%M")));
		desc.push_str(&format!("Daily reset at **{}**", daily_reset_local.format("%m/%d %H:%M")));
		let mut ret = CreateEmbed::default();
		ret
			.description(desc)
			.colour(Colour::from_rgb(255, 50, 20));

		ret
	}
}

async fn get_player_collection() -> Collection<Player> {
	let client = get_client().await.unwrap();
	let collection = client.database("poketcg").collection::<Player>("players");

	collection
}

// Database functions
pub async fn get_players() -> Vec<Player> { // Will change to Player
	let player_collection = get_player_collection().await;
	let players = player_collection
		.find(None, None)
		.await
		.unwrap()
		.try_collect::<Vec<Player>>()
		.await
		.unwrap();

	players
}

pub async fn get_player(discord_id: u64) -> Player { // Will change to Player
	let discord_id = discord_id as i64;
	let player_collection = get_player_collection().await;
	let player = player_collection
		.find_one(doc! { "discord_id": discord_id }, None)
		.await
		.unwrap();
	match player {
		Some(x) => return x,
		None => return add_player(discord_id).await
	}
}

async fn add_player(discord_id: i64) -> Player {
	let ret = Player::new_from_discord_id(discord_id);
	let player_collection = get_player_collection().await;
	player_collection
		.insert_one(&ret, None)
		.await
		.unwrap();
	
	ret
}

pub async fn update_player(player: &Player, update: Document) {
	let player_collection = get_player_collection().await;
	player_collection
		.update_one(
			doc! {"_id": &player.id.unwrap() }, 
			update, 
			None)
		.await
		.unwrap();
}