use crate::commands::Command;
use crate::events::Event;
use crate::{behavior::ComposedBehavior, error::YosupError};
use futures::{
	channel::{mpsc, oneshot},
	prelude::*,
};
use libp2p::{core::PeerId, Swarm};
use std::collections::HashMap;

/// Yosup's main loop that manages events, commands, and all the decentralized things
pub struct AppState {
	/// The application's networks, settings, and includes the modules
	pub swarm: Swarm<ComposedBehavior>,
	/// Recieves commands, that will be passed to the command handler
	pub command_receiver: mpsc::Receiver<Command>,
	/// A way to send events that will be recieved by the library's implementor
	pub event_sender: mpsc::Sender<Event>,
	/// If true, when peers dials you, you will try to dial back
	pub auto_dials_if_asked: bool,
	// Inner-events that need to be addressed
	/// Peers that need to be dialed
	pub pending_dial: HashMap<PeerId, oneshot::Sender<Result<(), YosupError>>>,
	/// Information about shared-state that has to do with ImHere and HiThere
	pub pending_search: Option<mpsc::Sender<PeerId>>,
}

impl AppState {
	/// Makes an AppState struct with the given fields
	pub fn new(
		swarm: Swarm<ComposedBehavior>,
		command_receiver: mpsc::Receiver<Command>,
		event_sender: mpsc::Sender<Event>,
	) -> Self {
		Self {
			swarm,
			command_receiver,
			event_sender,
			pending_dial: Default::default(),
			pending_search: Default::default(),
			auto_dials_if_asked: true,
		}
	}

	/// This should be spawned in tokio, since it needs to be running for the system to work.
	/// Moves swarm events and commands to the handler functions.
	pub async fn run(mut self) {
		loop {
			futures::select! {
				event = self.swarm.next() => {
					// Move the event to the handler function
					if let Err(error) = self.handle_event(event.expect("Swarm stream to be infinite.")).await {
						// If the event errors, send it to the application
						self.event_sender.send(Event::YosupError { error }).await.expect("Command channel to be open");
					}
				}
				command = self.command_receiver.next() => match command {
					Some(c) => {
						// Move the command to handler function
						if let Err(error) = self.handle_command(c).await {
							// If the command errors, send it to the application
							self.event_sender.send(Event::YosupError { error }).await.expect("Command channel to be open");
						}
					}
					// Command channel closed, thus shutting down the network event loop.
					None =>  return,
				},
			}
		}
	}
}
