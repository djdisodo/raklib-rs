use std::net::SocketAddr;
use crate::protocol::{Payload, Encode, Decode, MessageIdentifiers, OfflineMessage, OfflineMessageImpl};
use bytes::{BufMut, Buf};
use crate::protocol::payload::{PutAddress, GetAddress};

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

impl Payload for OpenConnectionRequest2 {
	const ID: MessageIdentifiers = MessageIdentifiers::OpenConnectionRequest2;
}

impl Encode for OpenConnectionRequest2 {
	fn encode(&self, serializer: &mut Vec<u8>) {
		self.offline_message.encode(serializer);
		serializer.put_address(&self.server_address);
		serializer.put_u16(self.mtu_size);
		serializer.put_u64(self.client_id)
	}
}

impl Decode for OpenConnectionRequest2 {
	fn decode(serializer: &mut &[u8]) -> Self {
		Self {
			offline_message: OfflineMessage::decode(serializer),
			server_address: serializer.get_address(),
			mtu_size: serializer.get_u16(),
			client_id: serializer.get_u64()
		}
	}
}