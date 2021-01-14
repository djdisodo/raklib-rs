use std::net::SocketAddr;
use crate::protocol::{MessageIdentifierHeader, EncodeBody, DecodeBody, MessageIdentifiers, OfflineMessage, OfflineMessageImpl, PutAddress, GetAddress, CommonPacket};
use bytes::{BufMut, Buf};

#[derive(Debug)]
pub struct OpenConnectionRequest2 {
	pub offline_message: OfflineMessage,
	pub client_id: u64,
	pub server_address: SocketAddr,
	pub mtu_size: u16
}

impl OfflineMessageImpl for OpenConnectionRequest2 {
	fn get_offline_message(&self) -> &OfflineMessage {
		&self.offline_message
	}
}

impl MessageIdentifierHeader for OpenConnectionRequest2 {
	const ID: MessageIdentifiers = MessageIdentifiers::OpenConnectionRequest2;
}

impl EncodeBody for OpenConnectionRequest2 {
	fn encode_body(&self, serializer: &mut dyn BufMut) {
		self.offline_message.encode_body(serializer);
		serializer.put_address(&self.server_address);
		serializer.put_u16(self.mtu_size);
		serializer.put_u64(self.client_id)
	}
}

impl DecodeBody for OpenConnectionRequest2 {
	fn decode_body(serializer: &mut dyn Buf) -> Self {
		Self {
			offline_message: OfflineMessage::decode_body(serializer),
			server_address: serializer.get_address(),
			mtu_size: serializer.get_u16(),
			client_id: serializer.get_u64()
		}
	}
}

impl CommonPacket for OpenConnectionRequest2 {}