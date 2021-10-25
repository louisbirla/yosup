use futures::{Stream, StreamExt};
use std::sync::Arc;
use tokio::sync::RwLock;
use yosup::Event;

use crate::contacts::ContactBook;

use self::{display_message::display_message, listening_on::listening_on, peer_added::peer_added};

mod display_message;
mod listening_on;
mod peer_added;

pub async fn event_listener(
	mut events: impl Stream<Item = Event> + std::marker::Unpin,
	contact_book: Arc<RwLock<ContactBook>>,
) {
	loop {
		// Wait for events
		match events.next().await {
			// When we recieve a message
			Some(Event::InboundMessage { message, peer_id }) => {
				display_message(contact_book.clone(), message, peer_id).await;
			}
			Some(Event::PeerAdded { peer_id, new }) => {
				peer_added(contact_book.clone(), peer_id, new).await;
			}
			Some(Event::ListeningOn { addr }) => {
				listening_on(&addr);
			}
			_ => {}
		}
	}
}
