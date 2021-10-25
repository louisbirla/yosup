use crate::CONTACTS_FILE_NAME;
use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
/// A way to keep track of PeerIds using nicknames (<PeerId, Nickname>)
pub struct ContactBook(HashMap<String, String>);

impl ContactBook {
	/// Tries to find the nickname for a PeerId from among the Contact Book.
	/// If it's not there, this will be None.
	pub fn get(&self, peer_id: &str) -> Option<String> {
		self.0.get(peer_id).map(String::from)
	}

	/// Saves the struct into a file
	fn save(&self) {
		let json = serde_json::to_string(&self.0).expect("To jsonify serializable");
		match std::fs::write(CONTACTS_FILE_NAME, json.as_bytes()) {
			Ok(_) => {}
			Err(e) => println!(
				"{} {}",
				"Unable to write file:".red().bold(),
				e.to_string().red().bold().italic()
			),
		}
	}

	/// Makes a ContactBook struct from a file if it exists, otherwise makes an empty one
	pub fn generate() -> Self {
		match std::fs::read(CONTACTS_FILE_NAME) {
			Ok(file) => match serde_json::from_slice::<ContactBook>(&file) {
				Ok(b) => b,
				Err(e) => {
					println!(
						"{} {}",
						"Unable to parse JSON into ContactBook:".red().bold(),
						e.to_string().red().bold().italic()
					);
					panic!();
				}
			},
			Err(_) => Default::default(),
		}
	}

	/// Can add a new contact or update an existing one, if the PeerId is the same
	pub fn save_contact(&mut self, peer_id: String, nickname: String) {
		self.0.insert(peer_id, nickname);
		self.save();
	}

	/// Removes the contact entry that matches the peer_id
	pub fn remove_contact(&mut self, peer_id: &str) -> Option<String> {
		let nickname = self.0.remove(peer_id);
		self.save();
		nickname
	}

	/// Removes the contact entry that matches the peer_id
	pub fn contacts(&self) -> HashMap<String, String> {
		self.0.clone()
	}
}
