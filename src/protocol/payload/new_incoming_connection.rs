use std::net::SocketAddr;
use std::time::SystemTime;
use crate::protocol::{Payload, Encode, Decode};
use crate::protocol::message_identifiers::ID_NEW_INCOMING_CONNECTION;

#[derive(Debug)]
pub struct NewIncomingConnection {
	pub address: SocketAddr,
	pub system_addresses: Vec<SocketAddr>,
	pub send_ping_time: SystemTime,
	pub send_pong_time: SystemTime
}

impl Payload for NewIncomingConnection {
	const ID: u8 = ID_NEW_INCOMING_CONNECTION;
}

impl Encode for NewIncomingConnection {
	fn encode(&self, serializer: &mut Vec<u8>) {
		unimplemented!()
	}
}

impl Decode for NewIncomingConnection {
	fn decode(serializer: &mut &[u8]) -> Self {
		unimplemented!()
	}
}