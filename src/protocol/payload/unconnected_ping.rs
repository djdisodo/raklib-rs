use std::ops::{Deref, DerefMut};
use crate::protocol::payload::offline_message::OfflineMessage;
use crate::protocol::{Payload, Encode, Decode, MessageIdentifiers};
use std::time::SystemTime;

#[derive(Debug)]
pub struct UnconnectedPing {
	pub offline_message: OfflineMessage,
	pub send_ping_time: SystemTime,
	pub client_id: u64
}

impl Deref for UnconnectedPing {
	type Target = OfflineMessage;

	fn deref(&self) -> &Self::Target {
		&self.offline_message
	}
}

impl DerefMut for UnconnectedPing {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.offline_message
	}
}

impl Payload for UnconnectedPing {
	const ID: MessageIdentifiers = MessageIdentifiers::ID_UNCONNECTED_PING;
}

impl Encode for UnconnectedPing {
	fn encode(&self, _serializer: &mut Vec<u8>) {
		unimplemented!()
	}
}

impl Decode for UnconnectedPing {
	fn decode(_serializer: &mut &[u8]) -> Self {
		unimplemented!()
	}
}