use colored::*;
use yosup::{client::Client, error::YosupError};

pub async fn add_peer_command(line: String, client: &mut Client) -> Result<(), YosupError> {
	// Isolate the address
	let addr = line.strip_prefix(".add_peer:").expect("1+1 to be 2");
	// Dial up the address!
	match client.dial_from_string(addr).await {
		Ok(_) => {}
		Err(e) => println!("{}", e.to_string().red().bold()),
	};
	Ok(())
}
