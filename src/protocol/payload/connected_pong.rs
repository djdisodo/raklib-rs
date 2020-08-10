use std::time::SystemTime;
use crate::protocol::{Payload, Encode, Decode, MessageIdentifiers};
use crate::protocol::payload::{PutTime, GetTime};

#[derive(Debug)]
pub struct ConnectedPong {
	pub send_ping_time: SystemTime,
	pub send_pong_time: SystemTime
}

impl Payload for ConnectedPong {
	const ID: MessageIdentifiers = MessageIdentifiers::ConnectedPong;
}

impl Encode for ConnectedPong {
	fn encode(&self, serializer: &mut Vec<u8>) {
		serializer.put_time(&self.send_ping_time);
		serializer.put_time(&self.send_pong_time);
	}
}

impl Decode for ConnectedPong {
	fn decode(serializer: &mut &[u8]) -> Self {
		Self {
			send_ping_time: serializer.get_time(),
			send_pong_time: serializer.get_time()
		}
	}
}