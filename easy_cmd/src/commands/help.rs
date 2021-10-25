use colored::*;

pub fn help_command() {
	match std::fs::write("YOSUP_HELP.md", HELP_TEXT) {
		Ok(_) => {}
		Err(e) => println!(
			"{} {}",
			"Unable to write file:".red().bold(),
			e.to_string().red().bold().italic()
		),
	}
	println!(
		"{}",
		"Created file './YOSUP_HELP.md' that contains this app's usage documentation."
			.dimmed()
			.bold()
	);
	println!();
}

const HELP_TEXT: &str = r#"# Yosup - A small P2P chatroom protocol

_Version 0.1.0_

Yosup is an app made for the purpose of learning libp2p technology, asynchronous rust, ways to incorporate a single protocol over multiple interfaces, and cryptography.

To learn more about libp2p, visit [their hompage](https://libp2p.io/). Learn a bit about P2P networks, Peers and PeerIds, and whatnot.

To get started, connect to a peer with `.add_peer`, and start messaging by typing and pressing enter! (You may want to get them to add you, if they are unable to automatically)

## Commands

Commands begin with `.` and use `:` to distinguish parameters. Here's a list of all supported commands:

- `.help` - Will make this file for you.

### Peers

- `.add_peer:xxx` - Connects to a peer, expanding the network. You'll get the messages they send, but they need to add you to get your messages and share them with their peers.

  - `xxx` - An address that you can connect to them with, usually returned from their `.listeners` command, or found when they start their session.

- `get_peers` - This will list all the peers that are in your network, as well as the name you saved them as.

### Your information

- `.save_secret_key` - Saves your current secret key, so that you'll have a consistent PeerId even when starting new sessions.

- `.del_secret_key` - Deletes the file that stores your secret key. If lost, you won't be able to realistically get the same PeerId.

- `my_id` - Displays your PeerId

- `.listeners` - Lists the addresses that your peers can use to connect to you using the `.add_peer:xxx` command.

### Contacts

- `.save_contact:xxx:yyy` - Saves the given PeerId and nickname so that you don't have to remember who's PeerId corresponds to who. This will make new contacts, or update the contact that you already saved with the PeerId.

  - `xxx` - The contact's PeerId
  - `yyy` - The name you want to save them as

- `.remove_contact:xxx` - Removes your record of a given peer from your contact book.

  - `xxx` - The contact's PeerId

- `.list_contacts` - Displays all the contacts you have saved: Their PeerId and name.

- `.whois:xxx` - Displays the name you saved a contact as.
  - `xxx` - The contact's PeerId

## Tips

### Auto dialing

You can save addresses to a file called `.yosup_autodial` to automatically dial them when you start your session.
"#;
