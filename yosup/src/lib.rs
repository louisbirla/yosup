#![feature(box_syntax)]
#![feature(once_cell)]
#![feature(async_stream)]

pub mod app_state;
pub mod behavior;
pub mod client;
pub mod commands;
pub mod error;
pub mod events;
pub mod floodsub;
mod new;
pub use events::Event;
use libp2p::{identity::ed25519::SecretKey, multiaddr::Protocol};
pub use libp2p::{identity::Keypair, Multiaddr, PeerId};
pub use new::{generate_keypair, new};

pub fn secret_from_keypair(keypair: Keypair) -> Vec<u8> {
	match keypair {
		Keypair::Ed25519(keypair) => keypair.secret().as_ref().to_vec(),
		_ => panic!("Keypair must be ed25519"),
	}
}

pub fn keypair_from_secret(key_data: Vec<u8>) -> Keypair {
	Keypair::Ed25519(
		SecretKey::from_bytes(key_data)
			.expect("Secret key to be correct")
			.into(),
	)
}

pub fn add_peer_to_addr(addr: Multiaddr, peer_id: PeerId) -> Multiaddr {
	addr.with(Protocol::P2p(peer_id.into()))
}
