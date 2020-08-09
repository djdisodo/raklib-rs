use crate::protocol::payload::AcknowledgePacket;
use crate::protocol::{Payload, Encode, Decode, MessageIdentifiers};


#[derive(Default, Debug, Deref, DerefMut)]
pub struct ACK {
	pub acknowledge: AcknowledgePacket
}

impl Payload for ACK {
	const ID: MessageIdentifiers = MessageIdentifiers::ID_ACK;
}

impl Encode for ACK {
	fn encode(&self, serializer: &mut Vec<u8>) {
		(**self).encode(serializer)
	}
}

impl Decode for ACK {
	fn decode(serializer: &mut &[u8]) -> Self {
		Self {
			acknowledge: AcknowledgePacket::decode(serializer)
		}
	}
}