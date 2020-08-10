use std::ops::{Deref, DerefMut};
use crate::protocol::payload::{OfflineMessage, PutTime, PutStr, GetTime, GetString};
use crate::protocol::{Payload, Encode, Decode, MessageIdentifiers};
use std::time::SystemTime;
use bytes::{BufMut, Buf};

#[derive(Debug)]
pub struct UnconnectedPong {
	pub offline_message: OfflineMessage,
	pub send_ping_time: SystemTime,
	pub server_id: u64,
	pub server_name: String
}

impl UnconnectedPong {
	pub fn create(send_ping_time: SystemTime, server_id: u64, server_name: String) -> Self {
		Self {
			offline_message: OfflineMessage::default(),
			send_ping_time,
			server_id,
			server_name
		}
	}
}

impl Deref for UnconnectedPong {
	type Target = OfflineMessage;

	fn deref(&self) -> &Self::Target {
		&self.offline_message
	}
}

impl DerefMut for UnconnectedPong {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.offline_message
	}
}

impl Payload for UnconnectedPong {
	const ID: MessageIdentifiers = MessageIdentifiers::UnconnectedPong;
}

impl Encode for UnconnectedPong {
	fn encode(&self, serializer: &mut Vec<u8>) {
		serializer.put_time(&self.send_ping_time);
		serializer.put_u64(self.server_id);
		(**self).encode(serializer);
		serializer.put_str(&self.server_name);
	}
}

impl Decode for UnconnectedPong {
	fn decode(serializer: &mut &[u8]) -> Self {
		Self {
			send_ping_time: serializer.get_time(),
			server_id: serializer.get_u64(),
			offline_message: OfflineMessage::decode(serializer),
			server_name: serializer.get_string()
		}
	}
}