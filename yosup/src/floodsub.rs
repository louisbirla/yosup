use serde::{Deserialize, Serialize};

/// The Floodsub protocol topic name for Yosup
pub const TOPIC_STRING: &str = "YOSUP/0";

#[derive(Debug, Serialize, Deserialize)]
pub enum FloodsubEncodedEvent {
	Message(Message),
	ImHere(ImHere),
	HiThere(HiThere),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
	/// Text content of the message
	pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
/// Broadcasts yourself to the Yosup network, so that others
/// can send you their HiThere events, so you can know who's on.
pub struct ImHere {
	pub peer_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
/// When you recieve a ImHere, send this out so that the new node knows
/// everybody
pub struct HiThere {
	pub peer_id: String,
}
