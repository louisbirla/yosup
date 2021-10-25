use crate::contacts::ContactBook;
use colored::*;
use std::sync::Arc;
use tokio::sync::RwLock;
use yosup::PeerId;

pub async fn peer_added(contact_book: Arc<RwLock<ContactBook>>, peer_id: PeerId, new: bool) {
	let contact_book = contact_book.read().await.clone();

	if new {
		if let Some(name) = contact_book.get(&peer_id.to_string()) {
			println!(
				"{} {} {}{}{}",
				"New peer added:".dimmed(),
				name.bright_blue().bold(),
				"(".dimmed(),
				peer_id.to_string().dimmed(),
				")".dimmed(),
			);
		} else {
			println!(
				"{} {}",
				"New peer added:".dimmed(),
				peer_id.to_string().bright_blue().bold()
			);
		}
		println!();
	}
}
