use crate::contacts::ContactBook;
use colored::*;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn whois_command(contact_book: Arc<RwLock<ContactBook>>, line: String) {
	let contact_book = contact_book.read().await.clone();

	let peer_id = line.strip_prefix(".whois:").expect("1+1 to be be 2");
	match contact_book.get(peer_id) {
		Some(nickname) => println!(
			"{} {}{}",
			"That PeerId's saved nickname is".dimmed(),
			nickname.bright_blue().bold(),
			".".dimmed()
		),
		None => println!("{}", "You don't have that PeerId saved.".dimmed()),
	};
	println!();
}
