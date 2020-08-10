use std::ops::{Deref, DerefMut};
use crate::protocol::payload::OfflineMessage;
use crate::protocol::{Payload, Encode, Decode, MessageIdentifiers};
use std::time::SystemTime;
use crate::protocol::payload::{PutTime, GetTime};
use bytes::{BufMut, Buf};

#[derive(Debug)]
pub struct UnconnectedPing {
	pub offline_message: OfflineMessage,
	pub send_ping_time: SystemTime,
	pub client_id: u64
}

impl Deref for UnconnectedPing {
	type Target = OfflineMessage;

	fn deref(&self) -> &Self::Target {
		&self.offline_message
	}
}

impl DerefMut for UnconnectedPing {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.offline_message
	}
}

impl Payload for UnconnectedPing {
	const ID: MessageIdentifiers = MessageIdentifiers::UnconnectedPing;
}

impl Encode for UnconnectedPing {
	fn encode(&self, serializer: &mut Vec<u8>) {
		serializer.put_time(&self.send_ping_time);
		(**self).encode(serializer);
		serializer.put_u64(self.client_id);
	}
}

impl Decode for UnconnectedPing {
	fn decode(serializer: &mut &[u8]) -> Self {
		Self {
			send_ping_time: serializer.get_time(),
			offline_message: OfflineMessage::decode(serializer),
			client_id: serializer.get_u64()
		}
	}
}