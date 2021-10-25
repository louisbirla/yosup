use colored::*;
use std::sync::Arc;
use tokio::sync::RwLock;
use yosup::{client::Client, error::YosupError};

use crate::contacts::ContactBook;

pub async fn get_peers_command(
	client: &mut Client,
	contact_book: Arc<RwLock<ContactBook>>,
) -> Result<(), YosupError> {
	let contact_book = contact_book.read().await.clone();

	println!("{}", "Addresses you are listening to".bold());
	println!("{}", "Will finish in 1 seconds...".dimmed());
	let peers = client.get_peers().await?;
	if peers.is_empty() {
		println!("{}", "No peers found.".yellow().bold());
	}
	for peer in peers {
		let name = contact_book
			.get(&peer.to_string())
			.unwrap_or_else(|| "Unknown".to_string());
		println!(
			"{}: {}",
			name.bright_blue().bold(),
			peer.to_string().dimmed().bold()
		);
	}
	println!();
	Ok(())
}
