use std::time::{SystemTime};
use crate::protocol::{Payload, Decode, Encode, MessageIdentifiers};

use crate::protocol::payload::{PutTime, GetTime};

#[derive(Debug)]
pub struct ConnectedPing {
	pub send_ping_time: SystemTime
}

impl Payload for ConnectedPing {
	const ID: MessageIdentifiers = MessageIdentifiers::ConnectedPing;
}

impl Encode for ConnectedPing {
	fn encode(&self, serializer: &mut Vec<u8>) {
		serializer.put_time(&self.send_ping_time)
	}
}

impl Decode for ConnectedPing {
	fn decode(serializer: &mut &[u8]) -> Self {
		Self {
			send_ping_time: serializer.get_time()
		}
	}
}