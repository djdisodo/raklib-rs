use crate::protocol::payload::AcknowledgePacket;
use crate::protocol::{Payload, Encode, Decode};

#[derive(Default, Debug, Deref, DerefMut)]
pub struct NACK {
	pub acknowledge: AcknowledgePacket
}

impl Payload for NACK {
	const ID: u8 = 0xa0;
}

impl Encode for NACK {
	fn encode(&self, serializer: &mut Vec<u8>) {
		unimplemented!()
	}
}

impl Decode for NACK {
	fn decode(serializer: &mut &[u8]) -> Self {
		unimplemented!()
	}
}