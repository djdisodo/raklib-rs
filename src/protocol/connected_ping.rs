use crate::protocol::{MessageIdentifierHeader, DecodeBody, EncodeBody, MessageIdentifiers, PutRaknetTime, CommonPacket, GetRaknetTime};
use bytes::{BufMut, Buf};
use crate::RaknetTime;

#[derive(Debug)]
pub struct ConnectedPing {
	pub send_ping_time: RaknetTime
}

impl MessageIdentifierHeader for ConnectedPing {
	const ID: MessageIdentifiers = MessageIdentifiers::ConnectedPing;
}

impl EncodeBody for ConnectedPing {
	fn encode_body(&self, serializer: &mut dyn BufMut) {
		serializer.put_raknet_time(&self.send_ping_time)
	}
}

impl DecodeBody for ConnectedPing {
	fn decode_body(serializer: &mut dyn Buf) -> Self {
		Self {
			send_ping_time: serializer.get_raknet_time()
		}
	}
}

impl CommonPacket for ConnectedPing {}