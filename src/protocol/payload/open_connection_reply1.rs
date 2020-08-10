use crate::protocol::payload::OfflineMessage;
use std::ops::{Deref, DerefMut};
use crate::protocol::{Payload, Encode, Decode, MessageIdentifiers};
use bytes::{BufMut, Buf};

#[derive(Default, Debug)]
pub struct OpenConnectionReply1 {
	pub offline_message: OfflineMessage,
	pub server_id: u64,
	pub server_security: bool,
	pub mtu_size: u16
}

impl OpenConnectionReply1 {
	pub fn create(server_id: u64, server_security: bool, mtu_size: u16) -> Self {
		Self {
			offline_message: Default::default(),
			server_id,
			server_security,
			mtu_size
		}
	}
}

impl Deref for OpenConnectionReply1 {
	type Target = OfflineMessage;

	fn deref(&self) -> &Self::Target {
		&self.offline_message
	}
}

impl DerefMut for OpenConnectionReply1 {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.offline_message
	}
}

impl Payload for OpenConnectionReply1 {
	const ID: MessageIdentifiers = MessageIdentifiers::OpenConnectionReply1;
}

impl Encode for OpenConnectionReply1 {
	fn encode(&self, serializer: &mut Vec<u8>) {
		(**self).encode(serializer);
		serializer.put_u64(self.server_id);
		serializer.put_u8(self.server_security as u8);
		serializer.put_u16(self.mtu_size);
	}
}

impl Decode for OpenConnectionReply1 {
	fn decode(serializer: &mut &[u8]) -> Self {
		Self {
			offline_message: OfflineMessage::decode(serializer),
			server_id: serializer.get_u64(),
			server_security: serializer.get_u8() != 0,
			mtu_size: serializer.get_u16()
		}
	}
}