use crate::contacts::ContactBook;
use colored::*;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn save_contact_command(contact_book: Arc<RwLock<ContactBook>>, line: String) {
	let mut contact_book = contact_book.write().await;

	let args = line.strip_prefix(".save_contact:").expect("1+1 to be be 2");
	let args: Vec<&str> = args.split(':').collect();
	let peer_id = match args.get(0) {
		Some(a) => a.to_string(),
		None => {
			println!(
				"{}",
				"This command requires a PeerId. See `.help` for more information."
					.red()
					.bold()
			);
			return;
		}
	};
	let nickname = match args.get(1) {
		Some(a) => a.to_string(),
		None => {
			println!(
				"{}",
				"This command requires a name. See `.help` for more information."
					.red()
					.bold()
			);
			return;
		}
	};

	contact_book.save_contact(peer_id, nickname.clone());
	println!(
		"{} {}{}",
		"Successfully saved contact".green().bold(),
		nickname.italic().green().bold(),
		".".green().bold(),
	);
	println!();
}
