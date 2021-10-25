use crate::SECRET_KEY_FILE_NAME;
use colored::*;
use yosup::{secret_from_keypair, Keypair};

pub fn save_secret_key_command(keypair: Keypair) {
	match std::fs::write(SECRET_KEY_FILE_NAME, secret_from_keypair(keypair)) {
		Ok(_) => {}
		Err(e) => println!(
			"{} {}",
			"Unable to write file:".red().bold(),
			e.to_string().red().bold().italic()
		),
	}
	println!(
		"{} ./{}",
		"Successfully wrote the secret_key to".green().bold(),
		SECRET_KEY_FILE_NAME
	);
	println!();
}
