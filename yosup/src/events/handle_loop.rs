use crate::app_state::AppState;
use crate::error::YosupError;
use crate::events::{ComposedEvent, Event};
use crate::floodsub::{FloodsubEncodedEvent, HiThere, TOPIC_STRING};
use futures::prelude::*;
use libp2p::floodsub::Topic;
use libp2p::{
	core::either::EitherError,
	floodsub::FloodsubEvent,
	kad::KademliaEvent,
	multiaddr::Protocol,
	swarm::{ProtocolsHandlerUpgrErr, SwarmEvent},
};
use tokio::io;

impl AppState {
	/// Where events are actually handled
	pub async fn handle_event(
		&mut self,
		event: SwarmEvent<
			ComposedEvent,
			EitherError<ProtocolsHandlerUpgrErr<io::Error>, io::Error>,
		>,
	) -> Result<(), YosupError> {
		match event {
			// When a new peer (or old peer) joins the KAD
			SwarmEvent::Behaviour(ComposedEvent::Kademlia(KademliaEvent::RoutingUpdated {
				peer,
				is_new_peer,
				..
			})) => {
				if is_new_peer {
					// Add this peer to the Floodsub!
					self.swarm
						.behaviour_mut()
						.floodsub
						.add_node_to_partial_view(peer);
				}
				// Let the app know
				self.event_sender
					.send(Event::PeerAdded {
						new: is_new_peer,
						peer_id: peer,
					})
					.await
					.expect("Command channel to be open");
				Ok(())
			}
			// When you recieve a message from Floodsub
			SwarmEvent::Behaviour(ComposedEvent::Floodsub(FloodsubEvent::Message(fs_message))) => {
				let event = match serde_json::from_slice::<FloodsubEncodedEvent>(&fs_message.data) {
					Ok(e) => e,
					Err(_) => return Ok(()),
				};
				match event {
					FloodsubEncodedEvent::Message(message) => {
						// Send the event for the app to handle
						self.event_sender
							.send(Event::InboundMessage {
								message,
								peer_id: fs_message.source,
							})
							.await
							.expect("Command channel to be open");
					}
					FloodsubEncodedEvent::ImHere(_) => {
						let hi_there = HiThere {
							peer_id: self.swarm.local_peer_id().to_string(),
						};
						let json = serde_json::to_string(&FloodsubEncodedEvent::HiThere(hi_there))
							.expect("To jsonify serializable");
						// Publish the message to the Floodsub
						self.swarm
							.behaviour_mut()
							.floodsub
							.publish(Topic::new(TOPIC_STRING), json.as_bytes());
					}
					FloodsubEncodedEvent::HiThere(peer) => {
						if let Some(sender) = &mut self.pending_search {
							let peer_id = match peer.peer_id.parse() {
								Ok(id) => id,
								Err(_) => return Ok(()),
							};
							sender
								.send(peer_id)
								.await
								.expect("Command channel to be open");
						}
					}
				}
				Ok(())
			}
			// When you run the app libp2p will give you addresses that peers can access you with
			SwarmEvent::NewListenAddr { address, .. } => {
				let local_peer_id = *self.swarm.local_peer_id();
				// Let the app know
				self.event_sender
					.send(Event::ListeningOn {
						addr: address.with(Protocol::P2p(local_peer_id.into())),
					})
					.await
					.expect("Command channel to be open");
				Ok(())
			}
			SwarmEvent::ConnectionEstablished {
				peer_id, endpoint, ..
			} => {
				// If we requested this dial
				if endpoint.is_dialer() {
					// Remove the dial request from the pending list
					if let Some(sender) = self.pending_dial.remove(&peer_id) {
						sender.send(Ok(())).expect("Command channel to be open");
					}
				} else {
					// If we are supposed to connect back
					if self.auto_dials_if_asked {
						self.dial(peer_id, endpoint.get_remote_address().clone(), None)
							.await?;
					}
				}
				Ok(())
			}
			SwarmEvent::OutgoingConnectionError { peer_id, error, .. } => {
				// If we failed to connect to a peer
				if let Some(peer_id) = peer_id {
					// Remove the peer from the pending dial list
					if let Some(sender) = self.pending_dial.remove(&peer_id) {
						sender
							.send(Err(YosupError::DialError(error)))
							.expect("Command channel to be open");
					}
				}
				Ok(())
			}
			// Not concerned about the other events
			_ => Ok(()),
		}
	}
}
