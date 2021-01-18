use crate::protocol::{MessageIdentifierHeader, EncodeBody, DecodeBody, MessageIdentifiers, OfflineMessage, OfflineMessageImpl, EncodePacket, EncodeHeader, DecodePacket};

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

impl MessageIdentifierHeader for OpenConnectionRequest1 {
	const ID: MessageIdentifiers = MessageIdentifiers::OpenConnectionRequest1;
}

impl EncodeBody for OpenConnectionRequest1 {
	fn encode_body(&self, serializer: &mut dyn BufMut) {
		self.offline_message.encode_body(serializer);
		serializer.put_u8(self.protocol);
	}
}

impl DecodeBody for OpenConnectionRequest1 {
	fn decode_body(serializer: &mut dyn Buf) -> Self {
		Self {
			offline_message: OfflineMessage::decode_body(serializer),
			protocol: serializer.get_u8(),
			mtu_size: serializer.remaining() as u16
		}
	}
}

impl EncodePacket for OpenConnectionRequest1 {
	fn encode_packet(&self, serializer: &mut dyn BufMut) {
		let mut serializer = serializer.limit(self.mtu_size as usize);
		serializer.put_u8(self.encode_header());
		self.encode_packet(&mut serializer);
		serializer.put_slice(&vec![0; serializer.remaining_mut()])
	}
}

impl DecodePacket for OpenConnectionRequest1 {
	fn decode_packet(serializer: &mut dyn Buf) -> Self {
		let original = serializer.remaining();
		if Self::ID as u8 != serializer.get_u8() {
			panic!("message identifier doesn't match");
		}
		let packet = Self::decode_body(serializer);
		let read = original - serializer.remaining();
		serializer.advance(packet.mtu_size as usize - read);
		packet
	}
}