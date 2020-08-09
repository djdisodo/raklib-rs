use std::time::Instant;
use crate::protocol::{Payload, Decode, Encode};
use crate::protocol::message_identifiers::ID_CONNECTED_PING;

#[derive(Debug)]
pub struct ConnectedPing {
	pub send_ping_time: Instant
}

impl Payload for ConnectedPing {
	const ID: u8 = ID_CONNECTED_PING;
}

impl Encode for ConnectedPing {
	fn encode(&self, serializer: &mut Vec<u8>) {
		unimplemented!()
	}
}

impl Decode for ConnectedPing {
	fn decode(serializer: &mut &[u8]) -> Self {
		unimplemented!()
	}
}