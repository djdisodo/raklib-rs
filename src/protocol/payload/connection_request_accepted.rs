use std::net::SocketAddr;
use std::time::SystemTime;
use crate::protocol::{Payload, Encode, Decode};
use crate::protocol::message_identifiers::ID_CONNECTION_REQUEST_ACCEPTED;

#[derive(Debug)]
pub struct ConnectionRequestAccepted {
	pub address: SocketAddr,
	pub system_addresses: Vec<SocketAddr>,
	pub send_ping_time: SystemTime,
	pub send_pong_time: SystemTime
}

impl ConnectionRequestAccepted {
	pub fn create(client_address: SocketAddr, system_addresses: Vec<SocketAddr>, send_ping_time: SystemTime, send_pong_time: SystemTime) -> Self {
		Self {
			address: client_address,
			system_addresses,
			send_ping_time,
			send_pong_time
		}
	}
}

impl Payload for ConnectionRequestAccepted {
	const ID: u8 = ID_CONNECTION_REQUEST_ACCEPTED;
}

impl Encode for ConnectionRequestAccepted {
	fn encode(&self, serializer: &mut Vec<u8>) {
		unimplemented!()
	}
}

impl Decode for ConnectionRequestAccepted {
	fn decode(serializer: &mut &[u8]) -> Self {
		unimplemented!()
	}
}