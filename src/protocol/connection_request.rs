use crate::protocol::{MessageIdentifiers, MessageIdentifierHeader, EncodeBody, DecodeBody, PutRaknetTime, GetRaknetTime, CommonPacket};
use bytes::{BufMut, Buf};
use crate::RaknetTime;

#[derive(Debug)]
pub struct ConnectionRequest {
	pub client_id: u64,
	pub send_ping_time: RaknetTime,
	pub use_security: bool
}

impl MessageIdentifierHeader for ConnectionRequest {
	const ID: MessageIdentifiers = MessageIdentifiers::ConnectionRequest;
}

impl EncodeBody for ConnectionRequest {
	fn encode_body(&self, serializer: &mut dyn BufMut) {
		serializer.put_u64(self.client_id);
		serializer.put_raknet_time(&self.send_ping_time);
		serializer.put_u8(self.use_security as u8)
	}
}

impl DecodeBody for ConnectionRequest {
	fn decode_body(serializer: &mut dyn Buf) -> Self {
		Self {
			client_id: serializer.get_u64(),
			send_ping_time: serializer.get_raknet_time(),
			use_security: serializer.get_u8() != 0
		}
	}
}

impl CommonPacket for ConnectionRequest {}