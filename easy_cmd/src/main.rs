#![feature(async_closure)]
use auto_dial::auto_dial;
use colored::*;
use contacts::ContactBook;
use events::event_listener;
use keypair::get_keypair;
use std::{error::Error, sync::Arc};
use tokio::{
	io::{self, AsyncBufReadExt},
	sync::RwLock,
};
use yosup::new;

mod auto_dial;
mod commands;
mod contacts;
mod events;
mod keypair;

use crate::commands::{
	add_peer::add_peer_command, del_secret_key::del_secret_key_command,
	get_peers::get_peers_command, help::help_command, list_contacts::list_contacts_command,
	listen_on::listen_on_command, listening::listening_command, my_id::my_id_command,
	remove_contact::remove_contact_command, save_autodial::save_autodial_command,
	save_contact::save_contact_command, save_secret_key::save_secret_key_command,
	whois::whois_command,
};

pub const SECRET_KEY_FILE_NAME: &str = ".yosup_secret_key";
pub const AUTO_DIAL_FILE_NAME: &str = ".yosup_autodial";
pub const CONTACTS_FILE_NAME: &str = ".yosup_contacts";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	println!("{}", "Yosup!".bold());
	println!("{}", "If you're having trouble, enter '.help'.".dimmed());
	println!();

	pretty_env_logger::init();

	let keypair = get_keypair();
	let (mut client, events, event_loop) = new(keypair.clone()).await?;

	// Spawn Yosup's logic loop
	tokio::spawn(event_loop.run());

	// Start Yosup wherever is available
	client
		.start_listening("/ip4/0.0.0.0/tcp/0".parse()?)
		.await?;

	let contact_book = Arc::new(RwLock::new(ContactBook::generate()));

	// Listen to the events sent by Yosup
	tokio::spawn(event_listener(events, contact_book.clone()));

	// Automatically dial specified addresses
	auto_dial(&mut client).await;

	// Read the terminal input
	let mut stdin = io::BufReader::new(io::stdin()).lines();
	loop {
		tokio::select! {
			// Whenever we get a line submitted
			line = stdin.next_line() => {
				// Get the line's content
				let line: String = line?.expect("stdin closed");
				// Check if it's a command
				if line.starts_with(".add_peer:") {
					add_peer_command(line, &mut client).await?;
				} else if line.starts_with(".save_autodial:") {
					save_autodial_command(&line)?;
				} else if line.starts_with(".save_secret_key") {
					save_secret_key_command(keypair.clone());
				} else if line.starts_with(".del_secret_key") {
					del_secret_key_command()
				} else if line.starts_with(".help") {
					help_command();
				} else if line.starts_with(".my_id") {
					my_id_command(&keypair);
				} else if line.starts_with(".listening") {
					listening_command(&mut client, keypair.public().to_peer_id()).await?;
				} else if line.starts_with(".listen_on:") {
					listen_on_command(line, &mut client).await?;
				} else if line.starts_with(".get_peers") {
					get_peers_command(&mut client, contact_book.clone()).await?;
				} else if line.starts_with(".remove_contact:") {
					remove_contact_command(contact_book.clone(), line).await;
				} else if line.starts_with(".whois:") {
					whois_command(contact_book.clone(), line).await;
				} else if line.starts_with(".save_contact:") {
					save_contact_command(contact_book.clone(), line).await;
				} else if line.starts_with(".list_contacts") {
					list_contacts_command(contact_book.clone()).await;
				} else {
					// It's not a command, so send the line as a message
					client.send_message(line).await.unwrap();
				}
			}
			// TODO: Add .save_autodial:xxx command
		}
	}
}
