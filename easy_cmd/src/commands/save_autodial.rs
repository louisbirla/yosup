use crate::AUTO_DIAL_FILE_NAME;
use colored::*;
use yosup::error::YosupError;

pub fn save_autodial_command(line: &str) -> Result<(), YosupError> {
	let addr = line.strip_prefix(".save_autodial:").expect("1+1 to be 2");
	let content;
	if let Ok(exist) = std::fs::read_to_string(AUTO_DIAL_FILE_NAME) {
		content = exist + "\n" + addr;
	} else {
		content = addr.to_string();
	}

	match std::fs::write(AUTO_DIAL_FILE_NAME, content.as_bytes()) {
		Ok(_) => {
			println!("{}", "Successfully added to autodial!".green().bold(),);
		}
		Err(e) => println!(
			"{} {}",
			"Unable to write file:".red().bold(),
			e.to_string().red().bold().italic()
		),
	}
	println!();
	Ok(())
}
