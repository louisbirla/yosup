use crate::{commands::Command, error::YosupError, floodsub::Message};
use futures::{
	channel::{mpsc, oneshot},
	prelude::*,
};
use libp2p::{
	core::{Multiaddr, PeerId},
	multiaddr::Protocol,
};

#[derive(Clone)]
/// The struct that the library implementor should use to send commands to the system
pub struct Client {
	/// Shouldn't be touched outside of library, this is used to send commands
	pub sender: mpsc::Sender<Command>,
}

impl Client {
	/// Listen for incoming connections on the given address.
	pub async fn start_listening(&mut self, addr: Multiaddr) -> Result<(), YosupError> {
		let (sender, receiver) = oneshot::channel();
		// Send the StartListening command
		self.sender
			.send(Command::StartListening { addr, sender })
			.await
			.expect("Command channel to be open");
		// Wait until the command is complete
		receiver.await.expect("Sender not to be dropped.")
	}

	/// Dial the given peer at the given address.
	pub async fn dial(&mut self, peer_id: PeerId, peer_addr: Multiaddr) -> Result<(), YosupError> {
		let (sender, receiver) = oneshot::channel();
		// Send the Dial command
		self.sender
			.send(Command::Dial {
				peer_id,
				peer_addr,
				sender,
			})
			.await
			.expect("Command channel to be open");
		// Wait until the command is complete
		receiver.await.expect("Sender not to be dropped.")
	}

	/// Dial the given peer at the given address, but they are combined in a MultiAddr.
	pub async fn dial_from_string(&mut self, addr: impl ToString) -> Result<(), YosupError> {
		// Extract the PeerID and Addr from the string
		let addr: Multiaddr = match addr.to_string().parse() {
			Ok(a) => a,
			Err(_) => return Err(YosupError::ValidAddr(addr.to_string())),
		};
		let peer_id = match addr.iter().last() {
			Some(Protocol::P2p(hash)) => PeerId::from_multihash(hash).expect("Valid hash."),
			_ => panic!("Multiaddr should have peer ID"),
		};
		// With the data, run the real function that sends the command
		self.dial(peer_id, addr).await
	}

	/// Broadcasts a given message with a given nickname
	pub async fn send_message(&mut self, content: String) -> Result<(), YosupError> {
		let (sender, receiver) = oneshot::channel();
		// Send the SendMessage command
		self.sender
			.send(Command::SendMessage {
				message: Message { content },
				sender,
			})
			.await
			.expect("Command channel to be open");
		// Wait until the command is complete
		receiver.await.expect("Sender not to be dropped.")
	}

	/// Get the listening addresses
	pub async fn get_listeners(&mut self) -> Result<Vec<Multiaddr>, YosupError> {
		let (sender, receiver) = oneshot::channel();
		// Send the command
		self.sender
			.send(Command::ReturnListeners { sender })
			.await
			.expect("Command channel to be open");
		// Wait until the command is complete
		receiver.await.expect("Sender not to be dropped.")
	}

	/// Asks peers to say they are present, then returns whoever did within 1 second
	pub async fn get_peers(&mut self) -> Result<Vec<PeerId>, YosupError> {
		let (sender, receiver) = oneshot::channel();
		// Send the command
		self.sender
			.send(Command::SearchPeers { sender })
			.await
			.expect("Command channel to be open");
		// Wait until the command is complete
		receiver.await.expect("Sender not to be dropped.")
	}
	// TODO: End-to-end encrypted private messaging
}
