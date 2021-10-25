use colored::*;
use yosup::Keypair;

pub fn my_id_command(keypair: &Keypair) {
	println!(
		"{} - {}",
		"Your PeerID".dimmed(),
		keypair
			.public()
			.to_peer_id()
			.to_string()
			.bold()
			.bright_blue()
	);
	println!();
}
