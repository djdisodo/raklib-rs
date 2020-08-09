use std::time::SystemTime;
use crate::protocol::{Payload, Encode, Decode};
use crate::protocol::message_identifiers::ID_CONNECTED_PONG;

#[derive(Debug)]
pub struct ConnectedPong {
	pub send_ping_time: SystemTime,
	pub send_pong_time: SystemTime
}

impl Payload for ConnectedPong {
	const ID: u8 = ID_CONNECTED_PONG;
}

impl Encode for ConnectedPong {
	fn encode(&self, serializer: &mut Vec<u8>) {
		unimplemented!()
	}
}

impl Decode for ConnectedPong {
	fn decode(serializer: &mut &[u8]) -> Self {
		unimplemented!()
	}
}