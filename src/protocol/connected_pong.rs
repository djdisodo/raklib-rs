use crate::protocol::{MessageIdentifierHeader, EncodeBody, DecodeBody, MessageIdentifiers, CommonPacket, GetRaknetTime, PutRaknetTime};
use bytes::{BufMut, Buf};
use crate::RaknetTime;

#[derive(Debug)]
pub struct ConnectedPong {
	pub send_ping_time: RaknetTime,
	pub send_pong_time: RaknetTime
}

impl MessageIdentifierHeader for ConnectedPong {
	const ID: MessageIdentifiers = MessageIdentifiers::ConnectedPong;
}

impl EncodeBody for ConnectedPong {
	fn encode_body(&self, serializer: &mut dyn BufMut) {
		serializer.put_raknet_time(&self.send_ping_time);
		serializer.put_raknet_time(&self.send_pong_time);
	}
}

impl DecodeBody for ConnectedPong {
	fn decode_body(serializer: &mut dyn Buf) -> Self {
		Self {
			send_ping_time: serializer.get_raknet_time(),
			send_pong_time: serializer.get_raknet_time()
		}
	}
}

impl CommonPacket for ConnectedPong {}