use std::ops::{Deref, DerefMut};
use crate::protocol::{MessageIdentifierHeader, EncodeBody, DecodeBody, MessageIdentifiers, OfflineMessage, OfflineMessageImpl, CommonPacket, GetRaknetTime, GetString, PutRaknetTime, PutStr};
use bytes::{BufMut, Buf};
use crate::RaknetTime;

#[derive(Debug)]
pub struct UnconnectedPong {
	pub offline_message: OfflineMessage,
	pub send_ping_time: RaknetTime,
	pub server_id: u64,
	pub server_name: String
}

impl UnconnectedPong {
	pub fn create(send_ping_time: RaknetTime, server_id: u64, server_name: String) -> Self {
		Self {
			offline_message: OfflineMessage::default(),
			send_ping_time,
			server_id,
			server_name
		}
	}
}

impl OfflineMessageImpl for UnconnectedPong {
	fn get_offline_message(&self) -> &OfflineMessage {
		&self.offline_message
	}
}

impl MessageIdentifierHeader for UnconnectedPong {
	const ID: MessageIdentifiers = MessageIdentifiers::UnconnectedPong;
}

impl EncodeBody for UnconnectedPong {
	fn encode_body(&self, serializer: &mut dyn BufMut) {
		serializer.put_raknet_time(&self.send_ping_time);
		serializer.put_u64(self.server_id);
		self.offline_message.encode_body(serializer);
		serializer.put_str(&self.server_name);
	}
}

impl DecodeBody for UnconnectedPong {
	fn decode_body(serializer: &mut dyn Buf) -> Self {
		Self {
			send_ping_time: serializer.get_raknet_time(),
			server_id: serializer.get_u64(),
			offline_message: OfflineMessage::decode_body(serializer),
			server_name: serializer.get_string()
		}
	}
}

impl CommonPacket for UnconnectedPong {}