use crate::{error::YosupError, floodsub::Message};
use futures::channel::oneshot;
use libp2p::{Multiaddr, PeerId};
pub mod handle_loop;

#[derive(Debug)]
/// The command that is sent to trigger Yosup logic
pub enum Command {
	/// Get the app to listen at the address
	StartListening {
		addr: Multiaddr,
		sender: oneshot::Sender<Result<(), YosupError>>,
	},
	/// Dial (connect) to a peer at the address
	Dial {
		peer_id: PeerId,
		peer_addr: Multiaddr,
		sender: oneshot::Sender<Result<(), YosupError>>,
	},
	/// Send out a message
	SendMessage {
		message: Message,
		sender: oneshot::Sender<Result<(), YosupError>>,
	},
	/// Send out a message with the swarm listeners
	ReturnListeners {
		sender: oneshot::Sender<Result<Vec<Multiaddr>, YosupError>>,
	},
	/// Introduce yourself to the network and wait for peers to say hi
	SearchPeers {
		sender: oneshot::Sender<Result<Vec<PeerId>, YosupError>>,
	},
}
