use crate::protocol::payload::offline_message::OfflineMessage;
use std::ops::{Deref, DerefMut};
use crate::protocol::{Payload, Encode, Decode, MessageIdentifiers};
use std::net::SocketAddr;
use bytes::{BufMut, Buf};
use crate::protocol::payload::{PutAddress, GetAddress};

#[derive(Debug)]
pub struct OpenConnectionReply2 {
	pub offline_message: OfflineMessage,
	pub server_id: u64,
	pub client_address: SocketAddr,
	pub mtu_size: u16,
	pub server_security: bool
}

impl OpenConnectionReply2 {
	pub fn create(server_id: u64, client_address: SocketAddr, mtu_size: u16, server_security: bool) -> Self {
		Self {
			offline_message: Default::default(),
			server_id,
			client_address,
			mtu_size,
			server_security
		}
	}
}

impl Deref for OpenConnectionReply2 {
	type Target = OfflineMessage;

	fn deref(&self) -> &Self::Target {
		&self.offline_message
	}
}

impl DerefMut for OpenConnectionReply2 {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.offline_message
	}
}

impl Payload for OpenConnectionReply2 {
	const ID: MessageIdentifiers = MessageIdentifiers::ID_OPEN_CONNECTION_REPLY_1;
}

impl Encode for OpenConnectionReply2 {
	fn encode(&self, serializer: &mut Vec<u8>) {
		(**self).encode(serializer);
		serializer.put_u64(self.server_id);
		serializer.put_address(&self.client_address);
		serializer.put_u16(self.mtu_size);
		serializer.put_u8(self.server_security as u8);
	}
}

impl Decode for OpenConnectionReply2 {
	fn decode(serializer: &mut &[u8]) -> Self {
		Self {
			offline_message: OfflineMessage::decode(serializer),
			server_id: serializer.get_u64(),
			client_address: serializer.get_address(),
			mtu_size: serializer.get_u16(),
			server_security: serializer.get_u8() != 0
		}
	}
}