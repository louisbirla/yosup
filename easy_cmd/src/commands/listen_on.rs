use colored::*;
use yosup::{client::Client, error::YosupError, Multiaddr};

pub async fn listen_on_command(line: String, client: &mut Client) -> Result<(), YosupError> {
	let addr = line.strip_prefix(".listen_on:").expect("1+1 to be 2");
	let addr: Multiaddr = match addr.parse() {
		Ok(a) => a,
		Err(e) => {
			println!(
				"{} {}",
				"Could not parse the address:".red().bold(),
				e.to_string().red().bold()
			);
			return Err(YosupError::ListeningError);
		}
	};
	if let Err(e) = client.start_listening(addr).await {
		println!(
			"{} {}",
			"Could not listen on the address:".red().bold(),
			e.to_string().red().bold()
		);
	}
	Ok(())
}
