use crate::{
	app_state::AppState,
	commands::Command,
	error::YosupError,
	floodsub::{FloodsubEncodedEvent, ImHere, TOPIC_STRING},
};
use futures::{
	channel::{mpsc, oneshot},
	StreamExt,
};
use libp2p::{floodsub::Topic, multiaddr::Protocol, Multiaddr, PeerId};
use tokio::time::{sleep, Duration};

impl AppState {
	/// Where commands are actually done
	pub async fn handle_command(&mut self, command: Command) -> Result<(), YosupError> {
		match command {
			Command::StartListening { addr, sender } => {
				match self.swarm.listen_on(addr) {
					Ok(_) => sender.send(Ok(())).expect("Command channel to be open"),
					Err(_) => sender
						.send(Err(YosupError::ListeningError))
						.expect("Command channel to be open"),
				}
				Ok(())
			}
			Command::Dial {
				peer_id,
				peer_addr,
				sender,
			} => self.dial(peer_id, peer_addr, Some(sender)).await,
			Command::SendMessage { message, sender } => {
				// Turn the message object into a JSON string
				let json = serde_json::to_string(&FloodsubEncodedEvent::Message(message))
					.expect("To jsonify serializable");
				// Publish the message to the Floodsub
				self.swarm
					.behaviour_mut()
					.floodsub
					.publish(Topic::new(TOPIC_STRING), json.as_bytes());
				// Message sent, send back OK
				sender.send(Ok(())).expect("Command channel to be open");
				Ok(())
			}
			Command::ReturnListeners { sender } => {
				let mut listeners = vec![];
				for addr in self.swarm.listeners() {
					listeners.push(addr.to_owned());
				}
				sender
					.send(Ok(listeners))
					.expect("Command channel to be open");
				Ok(())
			}
			Command::SearchPeers {
				sender: command_sender,
			} => {
				let (sender, mut receiver) = mpsc::channel(0);
				self.pending_search = Some(sender);
				let im_here = ImHere {
					peer_id: self.swarm.local_peer_id().to_string(),
				};
				// Turn the object into a JSON string
				let json = serde_json::to_string(&FloodsubEncodedEvent::ImHere(im_here))
					.expect("To jsonify serializable");
				// Publish the message to the Floodsub
				self.swarm
					.behaviour_mut()
					.floodsub
					.publish(Topic::new(TOPIC_STRING), json.as_bytes());
				// The peers that will be returned
				let mut peers = vec![];

				tokio::spawn(async move {
					// Wait for one second, fitting in all the peers you can during the time
					let sleep = sleep(Duration::from_secs(1));
					tokio::pin!(sleep);

					loop {
						tokio::select! {
							// Moves commands to handler function
							peer_id = receiver.next() => match peer_id {
								Some(peer_id) => {
									peers.push(peer_id)
								},
								// Command channel closed, thus shutting down the network event loop.
								None => break,
							},
							() = &mut sleep => break,
						}
					}
					command_sender
						.send(Ok(peers))
						.expect("Command channel to be open");
				});
				Ok(())
			}
		}
	}

	pub async fn dial(
		&mut self,
		peer_id: PeerId,
		peer_addr: Multiaddr,
		sender: Option<oneshot::Sender<Result<(), YosupError>>>,
	) -> Result<(), YosupError> {
		if &peer_id == self.swarm.local_peer_id() {
			if let Some(sender) = sender {
				sender.send(Ok(())).expect("Command channel to be open");
			}
			return Ok(());
		}
		if let std::collections::hash_map::Entry::Vacant(_) = self.pending_dial.entry(peer_id) {
			// Add the address to the KAD
			self.swarm
				.behaviour_mut()
				.kademlia
				.add_address(&peer_id, peer_addr.clone());
			// Add the peer to the multiaddr
			let addr = peer_addr.with(Protocol::P2p(peer_id.into()));
			// Dial the peer
			match self.swarm.dial_addr(addr) {
				Ok(()) => {
					if let Some(sender) = sender {
						// Add the peer to the pending dials list to access
						self.pending_dial.insert(peer_id, sender);
					}
				}
				Err(e) => return Err(YosupError::DialError(e)),
			}
		}
		Ok(())
	}
}
