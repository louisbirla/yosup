use futures::{channel::mpsc, Stream};
use libp2p::{
	core::upgrade,
	floodsub::{Floodsub, Topic},
	identity::Keypair,
	kad::{store::MemoryStore, Kademlia},
	mplex, noise,
	swarm::SwarmBuilder,
	tcp::TokioTcpConfig,
	Transport,
};

use crate::{
	app_state::AppState, behavior::ComposedBehavior, client::Client, error::YosupError,
	events::Event, floodsub::TOPIC_STRING,
};

/// Generate a random keypair, for the app to use
pub fn generate_keypair() -> Keypair {
	Keypair::generate_ed25519()
}

pub async fn new(
	keypair: Keypair,
) -> Result<(Client, impl Stream<Item = Event>, AppState), YosupError> {
	let peer_id = keypair.public().to_peer_id();

	let mut behavior = ComposedBehavior {
		kademlia: Kademlia::new(peer_id, MemoryStore::new(peer_id)),
		floodsub: Floodsub::new(peer_id),
	};
	behavior.floodsub.subscribe(Topic::new(TOPIC_STRING));

	let noise_keys = noise::Keypair::<noise::X25519Spec>::new()
		.into_authentic(&keypair)
		.expect("Signing libp2p-noise static DH keypair not to fail");

	let transport = TokioTcpConfig::new()
		.nodelay(true)
		.upgrade(upgrade::Version::V1)
		.authenticate(noise::NoiseConfig::xx(noise_keys).into_authenticated())
		.multiplex(mplex::MplexConfig::new())
		.boxed();

	// Build the Swarm, connecting the lower layer transport logic with the
	// higher layer network behaviour logic.
	let swarm = SwarmBuilder::new(transport, behavior, peer_id)
		.executor(box |fut| {
			tokio::spawn(fut);
		})
		.build();

	let (command_sender, command_receiver) = mpsc::channel(0);
	let (event_sender, event_receiver) = mpsc::channel(0);

	Ok((
		Client {
			sender: command_sender,
		},
		event_receiver,
		AppState::new(swarm, command_receiver, event_sender),
	))
}
