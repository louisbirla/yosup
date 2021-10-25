use crate::contacts::ContactBook;
use colored::*;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn list_contacts_command(contact_book: Arc<RwLock<ContactBook>>) {
	let contact_book = contact_book.read().await.clone();

	println!("{}", "Contacts you have saved:".bold());
	for (peer_id, nickname) in contact_book.contacts() {
		println!(
			"{}{} {}",
			nickname.to_string().bright_blue().bold(),
			":".dimmed(),
			peer_id.dimmed()
		);
	}
	println!();
}
