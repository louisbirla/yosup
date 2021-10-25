use crate::{error::YosupError, floodsub::Message};
use libp2p::{floodsub::FloodsubEvent, kad::KademliaEvent, Multiaddr, PeerId};
pub mod handle_loop;

/// Something that happened and you need to resolve it. This is passed
/// to the library user to react to events.
pub enum Event {
	/// When you recieve a message from the Floodsub
	InboundMessage {
		/// The message struct
		message: Message,
		/// The PeerID of the sender
		peer_id: PeerId,
	},
	/// When a peer has been added
	PeerAdded {
		/// Is the peer old or new to the session DHT
		new: bool,
		/// The PeerID of the new peer
		peer_id: PeerId,
	},
	/// When a listening address has been added
	ListeningOn {
		/// The address (with P2P) that this peer is listening on
		addr: Multiaddr,
	},
	/// When we get an error to send back
	YosupError { error: YosupError },
}

#[derive(Debug)]
/// A combination of all the module's events. This is used in the
/// event loop to convert these into application logic.
pub enum ComposedEvent {
	Floodsub(FloodsubEvent),
	Kademlia(KademliaEvent),
}

impl From<FloodsubEvent> for ComposedEvent {
	fn from(event: FloodsubEvent) -> Self {
		ComposedEvent::Floodsub(event)
	}
}

impl From<KademliaEvent> for ComposedEvent {
	fn from(event: KademliaEvent) -> Self {
		ComposedEvent::Kademlia(event)
	}
}
