use std::ops::{Deref, DerefMut};
use crate::protocol::payload::offline_message::OfflineMessage;
use crate::protocol::{Payload, Encode, Decode, MessageIdentifiers};
use std::time::SystemTime;

#[derive(Debug)]
pub struct UnconnectedPong {
	pub offline_message: OfflineMessage,
	pub send_ping_time: SystemTime,
	pub server_id: u64,
	pub server_name: String
}

impl Deref for UnconnectedPong {
	type Target = OfflineMessage;

	fn deref(&self) -> &Self::Target {
		&self.offline_message
	}
}

impl DerefMut for UnconnectedPong {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.offline_message
	}
}

impl Payload for UnconnectedPong {
	const ID: MessageIdentifiers = MessageIdentifiers::ID_UNCONNECTED_PONG;
}

impl Encode for UnconnectedPong {
	fn encode(&self, _serializer: &mut Vec<u8>) {
		unimplemented!()
	}
}

impl Decode for UnconnectedPong {
	fn decode(_serializer: &mut &[u8]) -> Self {
		unimplemented!()
	}
}