use crate::protocol::payload::offline_message::OfflineMessage;
use std::ops::{Deref, DerefMut};
use crate::protocol::{Payload, Encode, Decode};
use crate::protocol::message_identifiers::ID_OPEN_CONNECTION_REPLY_1;
use std::net::SocketAddr;

#[derive(Debug)]
pub struct OpenConnectionReply2 {
	pub offline_message: OfflineMessage,
	pub client_address: SocketAddr,
	pub mtu_size: u16,
	pub server_security: bool
}

impl OpenConnectionReply2 {
	pub fn create(server_id: u64, client_address: SocketAddr, mtu_size: u16, server_security: bool) -> Self {
		Self {
			offline_message: Default::default(),
			client_address,
			mtu_size,
			server_security
		}
	}
}

impl Deref for OpenConnectionReply2 {
	type Target = OfflineMessage;

	fn deref(&self) -> &Self::Target {
		&self.offline_message
	}
}

impl DerefMut for OpenConnectionReply2 {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.offline_message
	}
}

impl Payload for OpenConnectionReply2 {
	const ID: u8 = ID_OPEN_CONNECTION_REPLY_1;
}

impl Encode for OpenConnectionReply2 {
	fn encode(&self, serializer: &mut Vec<u8>) {
		unimplemented!()
	}
}

impl Decode for OpenConnectionReply2 {
	fn decode(serializer: &mut &[u8]) -> Self {
		unimplemented!()
	}
}