use crate::protocol::payload::AcknowledgePacket;
use crate::protocol::{Payload, Encode, Decode, MessageIdentifiers};


#[derive(Default, Debug, Deref, DerefMut)]
pub struct NACK {
	pub acknowledge: AcknowledgePacket
}

impl Payload for NACK {
	const ID: MessageIdentifiers = MessageIdentifiers::ID_NACK;
}

impl Encode for NACK {
	fn encode(&self, serializer: &mut Vec<u8>) {
		(**self).encode(serializer);
	}
}

impl Decode for NACK {
	fn decode(serializer: &mut &[u8]) -> Self {
		Self {
			acknowledge: AcknowledgePacket::decode(serializer)
		}
	}
}