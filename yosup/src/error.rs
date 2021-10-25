use libp2p::swarm::DialError;
use thiserror::Error;

#[derive(Debug, Error)]
/// The base error that all Yosup issues should go down to
pub enum YosupError {
	#[error("An unknown error occurred.")]
	UnknownError,
	#[error("An error occurred when trying to listen for peers.")]
	ListeningError,
	#[error("An error occurred when dialing a peer: {0}")]
	DialError(DialError),
	#[error("The given address '{0}' was not valid.")]
	ValidAddr(String),
}
