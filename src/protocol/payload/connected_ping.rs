use std::time::{SystemTime, Duration};
use crate::protocol::{Payload, Decode, Encode};
use crate::protocol::message_identifiers::ID_CONNECTED_PING;
use bytes::{BufMut, Buf};
use crate::protocol::payload::{PutTime, GetTime};

#[derive(Debug)]
pub struct ConnectedPing {
	pub send_ping_time: SystemTime
}

impl Payload for ConnectedPing {
	const ID: u8 = ID_CONNECTED_PING;
}

impl Encode for ConnectedPing {
	fn encode(&self, serializer: &mut Vec<u8>) {
		serializer.put_time(self.send_ping_time)
	}
}

impl Decode for ConnectedPing {
	fn decode(serializer: &mut &[u8]) -> Self {
		Self {
			send_ping_time: serializer.get_time()
		}
	}
}