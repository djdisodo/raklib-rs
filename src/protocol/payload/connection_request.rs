use std::time::Instant;
use crate::protocol::{Payload, Encode, Decode};
use crate::protocol::message_identifiers::ID_CONNECTION_REQUEST;

#[derive(Debug)]
pub struct ConnectionRequest {
	pub client_id: u64,
	pub send_ping_time: Instant,
	pub use_security: bool
}

impl Payload for ConnectionRequest {
	const ID: u8 = ID_CONNECTION_REQUEST;
}

impl Encode for ConnectionRequest {
	fn encode(&self, serializer: &mut Vec<u8>) {
		unimplemented!()
	}
}

impl Decode for ConnectionRequest {
	fn decode(serializer: &mut &[u8]) -> Self {
		unimplemented!()
	}
}