use bytes::{BufMut, Buf};
use crate::protocol::{AcknowledgePacket, MessageIdentifiers, EncodeBody, DecodeBody, CommonPacket, MessageIdentifierHeader};


#[derive(Default, Debug, Deref, DerefMut)]
pub struct ACK {
	pub acknowledge: AcknowledgePacket
}

impl MessageIdentifierHeader for ACK {
	const ID: MessageIdentifiers = MessageIdentifiers::Ack;
}

impl EncodeBody for ACK {
	fn encode_body(&self, serializer: &mut dyn BufMut) {
		(**self).encode_body(serializer)
	}
}

impl DecodeBody for ACK {
	fn decode_body(serializer: &mut dyn Buf) -> Self {
		Self {
			acknowledge: AcknowledgePacket::decode_body(serializer)
		}
	}
}

impl CommonPacket for ACK {}