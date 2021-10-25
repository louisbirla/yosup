use crate::contacts::ContactBook;
use colored::*;
use std::sync::Arc;
use tokio::sync::RwLock;
use yosup::{floodsub::Message, PeerId};

pub async fn display_message(
	contact_book: Arc<RwLock<ContactBook>>,
	message: Message,
	peer_id: PeerId,
) {
	let contact_book = contact_book.read().await.clone();
	println!(
		"{}{}{} {}",
		"[".dimmed(),
		contact_book
			.get(&peer_id.to_string())
			.unwrap_or_else(|| peer_id.to_string())
			.bright_blue()
			.bold(),
		"]:".dimmed(),
		message.content
	);
}
