use crate::events::ComposedEvent;
use libp2p::{
	floodsub::Floodsub,
	kad::{store::MemoryStore, Kademlia},
	NetworkBehaviour,
};

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "ComposedEvent")]
/// The modules that the Network uses. The modules are often borrowed as mutable
/// to publish messages, add peers, etc.
pub struct ComposedBehavior {
	/// The module that handles publishing messages across the network
	/// and subscribing to topics that are published
	pub floodsub: Floodsub,
	/// The module that registers peers and includes decentralized key-value storing.
	pub kademlia: Kademlia<MemoryStore>,
}
