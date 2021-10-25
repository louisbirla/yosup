use colored::*;
use yosup::Multiaddr;

pub fn listening_on(addr: &Multiaddr) {
	println!(
		"{} - {}",
		"New address".yellow().bold(),
		"Other peers use this to connect to you (directly)"
			.dimmed()
			.bold()
	);
	println!("\t{}", addr);
	println!();
}
