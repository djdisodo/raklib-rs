use crate::protocol::{Payload, Encode, Decode};
use crate::protocol::MessageIdentifiers;
use bytes::{BufMut, Buf};
use crate::protocol::payload::offline_message::OfflineMessage;
use std::ops::{Deref, DerefMut};

#[derive(Default, Debug)]
pub struct IncompatibleProtocolVersion {
	pub offline_message: OfflineMessage,
	pub protocol_version: u8,
	pub server_id: u64
}

impl Deref for IncompatibleProtocolVersion {
	type Target = OfflineMessage;

	fn deref(&self) -> &Self::Target {
		&self.offline_message
	}
}

impl DerefMut for IncompatibleProtocolVersion {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.offline_message
	}
}

impl IncompatibleProtocolVersion {
	pub fn create(protocol_version: u8, server_id: u64) -> Self {
		Self {
			offline_message: OfflineMessage::default(),
			protocol_version,
			server_id
		}
	}
}

impl Payload for IncompatibleProtocolVersion {
	const ID: MessageIdentifiers = MessageIdentifiers::IncompatibleProtocolVersion;
}

impl Encode for IncompatibleProtocolVersion {
	fn encode(&self, serializer: &mut Vec<u8>) {
		serializer.put_u8(self.protocol_version);
		(**self).encode(serializer);
		serializer.put_u64(self.server_id);
	}
}

impl Decode for IncompatibleProtocolVersion {
	fn decode(serializer: &mut &[u8]) -> Self {
		Self {
			protocol_version: serializer.get_u8(),
			offline_message: OfflineMessage::decode(serializer),
			server_id: serializer.get_u64()
		}
	}
}