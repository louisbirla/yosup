use crate::contacts::ContactBook;
use colored::*;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn remove_contact_command(contact_book: Arc<RwLock<ContactBook>>, line: String) {
	let mut book = contact_book.write().await;

	let peer_id = line
		.strip_prefix(".remove_contact:")
		.expect("1+1 to be be 2");
	match book.remove_contact(peer_id) {
		Some(nickname) => println!(
			"{}{}{}",
			"Contact '".dimmed(),
			nickname.dimmed().bold(),
			"' has been removed.".dimmed()
		),
		None => println!(
			"{}",
			"That contact didn't exist, so nothing was removed.".dimmed()
		),
	};
	println!();
}
