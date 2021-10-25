use crate::AUTO_DIAL_FILE_NAME;
use colored::*;
use std::{
	fs::File,
	io::{prelude::*, BufReader},
};
use yosup::client::Client;

pub async fn auto_dial(client: &mut Client) {
	if let Ok(file) = File::open(AUTO_DIAL_FILE_NAME) {
		let reader = BufReader::new(file);

		for line in reader.lines() {
			match client
				.dial_from_string(line.expect("IO error not to occur"))
				.await
			{
				Ok(()) => {
					println!(
						"{}",
						"(automatically dialed peer from .yosup_autodial)".dimmed()
					);
					println!();
				}
				Err(e) => println!("Error auto-dialing: {}", e),
			}
		}
	}
}
