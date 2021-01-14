use std::ops::{Deref, DerefMut};
use crate::protocol::{MessageIdentifierHeader, EncodeBody, DecodeBody, MessageIdentifiers, OfflineMessage, OfflineMessageImpl, CommonPacket};
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

impl OfflineMessageImpl for OpenConnectionReply1 {
	fn get_offline_message(&self) -> &OfflineMessage {
		&self.offline_message
	}
}

impl MessageIdentifierHeader for OpenConnectionReply1 {
	const ID: MessageIdentifiers = MessageIdentifiers::OpenConnectionReply1;
}

impl EncodeBody for OpenConnectionReply1 {
	fn encode_body(&self, serializer: &mut dyn BufMut) {
		self.offline_message.encode_body(serializer);
		serializer.put_u64(self.server_id);
		serializer.put_u8(self.server_security as u8);
		serializer.put_u16(self.mtu_size);
	}
}

impl DecodeBody for OpenConnectionReply1 {
	fn decode_body(serializer: &mut dyn Buf) -> Self {
		Self {
			offline_message: OfflineMessage::decode_body(serializer),
			server_id: serializer.get_u64(),
			server_security: serializer.get_u8() != 0,
			mtu_size: serializer.get_u16()
		}
	}
}

impl CommonPacket for OpenConnectionReply1 {}