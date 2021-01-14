use crate::protocol::{MessageIdentifierHeader, EncodeBody, DecodeBody, MessageIdentifiers, AcknowledgePacket, CommonPacket};
use bytes::{BufMut, Buf};

#[derive(Default, Debug, Deref, DerefMut)]
pub struct NACK {
	pub acknowledge: AcknowledgePacket
}

impl MessageIdentifierHeader for NACK {
	const ID: MessageIdentifiers = MessageIdentifiers::Nack;
}

impl EncodeBody for NACK {
	fn encode_body(&self, serializer: &mut dyn BufMut) {
		(**self).encode_body(serializer);
	}
}

impl DecodeBody for NACK {
	fn decode_body(serializer: &mut dyn Buf) -> Self {
		Self {
			acknowledge: AcknowledgePacket::decode_body(serializer)
		}
	}
}

impl CommonPacket for NACK {}