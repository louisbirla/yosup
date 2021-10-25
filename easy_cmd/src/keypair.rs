use colored::*;
use yosup::{generate_keypair, keypair_from_secret, Keypair};

use crate::SECRET_KEY_FILE_NAME;

// If the secret key is already saved, use it to make a keypair. If not, generate one.
pub fn get_keypair() -> Keypair {
	match std::fs::read(SECRET_KEY_FILE_NAME) {
		Ok(data) => {
			let keypair = keypair_from_secret(data);
			println!(
				"{} - {}",
				"Keeping PeerID".dimmed(),
				keypair
					.public()
					.to_peer_id()
					.to_string()
					.bright_blue()
					.bold()
			);
			keypair
		}
		Err(_) => {
			let keypair = generate_keypair();
			println!(
				"{} - {}",
				"New PeerID".dimmed(),
				keypair
					.public()
					.to_peer_id()
					.to_string()
					.bright_blue()
					.bold()
			);
			keypair
		}
	}
}
