use std::time::SystemTime;
use crate::protocol::{Payload, Encode, Decode};
use crate::protocol::MessageIdentifiers;
use bytes::{BufMut, Buf};
use crate::protocol::payload::{PutTime, GetTime};

#[derive(Debug)]
pub struct ConnectionRequest {
	pub client_id: u64,
	pub send_ping_time: SystemTime,
	pub use_security: bool
}

impl Payload for ConnectionRequest {
	const ID: MessageIdentifiers = MessageIdentifiers::ConnectionRequest;
}

impl Encode for ConnectionRequest {
	fn encode(&self, serializer: &mut Vec<u8>) {
		serializer.put_u64(self.client_id);
		serializer.put_time(&self.send_ping_time);
		serializer.put_u8(self.use_security as u8)
	}
}

impl Decode for ConnectionRequest {
	fn decode(serializer: &mut &[u8]) -> Self {
		Self {
			client_id: serializer.get_u64(),
			send_ping_time: serializer.get_time(),
			use_security: serializer.get_u8() != 0
		}
	}
}