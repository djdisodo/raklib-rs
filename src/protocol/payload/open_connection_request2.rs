use std::net::SocketAddr;
use std::ops::{Deref, DerefMut};
use crate::protocol::payload::offline_message::OfflineMessage;
use crate::protocol::{Payload, Encode, Decode};
use crate::protocol::message_identifiers::ID_OPEN_CONNECTION_REQUEST_2;

#[derive(Debug)]
pub struct OpenConnectionRequest2 {
	pub offline_message: OfflineMessage,
	pub client_id: u64,
	pub server_address: SocketAddr,
	pub mtu_size: u16
}

impl Deref for OpenConnectionRequest2 {
	type Target = OfflineMessage;

	fn deref(&self) -> &Self::Target {
		&self.offline_message
	}
}

impl DerefMut for OpenConnectionRequest2 {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.offline_message
	}
}

impl Payload for OpenConnectionRequest2 {
	const ID: u8 = ID_OPEN_CONNECTION_REQUEST_2;
}

impl Encode for OpenConnectionRequest2 {
	fn encode(&self, serializer: &mut Vec<u8>) {
		unimplemented!()
	}
}

impl Decode for OpenConnectionRequest2 {
	fn decode(serializer: &mut &[u8]) -> Self {
		unimplemented!()
	}
}