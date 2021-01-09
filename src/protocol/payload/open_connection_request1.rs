use crate::protocol::{Payload, Encode, Decode, MessageIdentifiers, OfflineMessage, OfflineMessageImpl};
use std::ops::{DerefMut, Deref};
use bytes::{BufMut, Buf};

#[derive(Default, Debug)]
pub struct OpenConnectionRequest1 {
	pub offline_message: OfflineMessage,
	pub protocol: u8,
	pub mtu_size: u16
}

impl OfflineMessageImpl for OpenConnectionRequest1 {
	fn get_offline_message(&self) -> &OfflineMessage {
		&self.offline_message
	}
}

impl Payload for OpenConnectionRequest1 {
	const ID: MessageIdentifiers = MessageIdentifiers::OpenConnectionRequest1;
}

impl Encode for OpenConnectionRequest1 {
	fn encode(&self, serializer: &mut Vec<u8>) {
		self.offline_message.encode(serializer);
		serializer.put_u8(self.protocol);
		for _ in 0..(self.mtu_size - serializer.len() as u16) {
			serializer.put_u8(0);
		}
	}
}

impl Decode for OpenConnectionRequest1 {
	fn decode(serializer: &mut &[u8]) -> Self {
		let ret = Self {
			offline_message: OfflineMessage::decode(serializer),
			protocol: serializer.get_u8(),
			mtu_size: serializer.len() as u16
		};
		serializer.advance(serializer.remaining()); // silence unread warnings
		ret
	}
}