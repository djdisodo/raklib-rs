use crate::protocol::{Payload, Encode, Decode};
use crate::protocol::message_identifiers::ID_OPEN_CONNECTION_REQUEST_1;
use crate::protocol::payload::offline_message::OfflineMessage;
use std::ops::{DerefMut, Deref};

#[derive(Default, Debug)]
pub struct OpenConnectionRequest1 {
	pub offline_message: OfflineMessage,
	pub protocol: u8,
	pub mtu_size: u16
}

impl Deref for OpenConnectionRequest1 {
	type Target = OfflineMessage;

	fn deref(&self) -> &Self::Target {
		&self.offline_message
	}
}

impl DerefMut for OpenConnectionRequest1 {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.offline_message
	}
}

impl Payload for OpenConnectionRequest1 {
	const ID: u8 = ID_OPEN_CONNECTION_REQUEST_1;
}

impl Encode for OpenConnectionRequest1 {
	fn encode(&self, serializer: &mut Vec<u8>) {
		unimplemented!()
	}
}

impl Decode for OpenConnectionRequest1 {
	fn decode(serializer: &mut &[u8]) -> Self {
		unimplemented!()
	}
}