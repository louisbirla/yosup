use crate::SECRET_KEY_FILE_NAME;
use colored::*;

pub fn del_secret_key_command() {
	// Delete the secret key
	match std::fs::remove_file(SECRET_KEY_FILE_NAME) {
		Ok(()) => println!("{}", "Successfully removed the secret_key".green().bold()),
		Err(e) => println!(
			"{} {}",
			"Error when trying to remove secret file:".red().bold(),
			e.to_string().red().bold()
		),
	};
	println!();
}
