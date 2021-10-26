use colored::Colorize;
use yosup::{add_peer_to_addr, client::Client, error::YosupError, PeerId};

pub async fn listening_command(client: &mut Client, peer_id: PeerId) -> Result<(), YosupError> {
	println!("{}", "Addresses you are listening to".bold());
	for addr in client.get_listeners().await? {
		println!("\t{}", add_peer_to_addr(addr, peer_id).to_string().yellow());
	}
	println!();
	Ok(())
}
