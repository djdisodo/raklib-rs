use crate::protocol::payload::AcknowledgePacket;
use crate::protocol::{Payload, Encode, Decode};

#[derive(Default, Debug, Deref, DerefMut)]
pub struct ACK {
	pub acknowledge: AcknowledgePacket
}

impl Payload for ACK {
	const ID: u8 = 0xc0;
}

impl Encode for ACK {
	fn encode(&self, serializer: &mut Vec<u8>) {
		unimplemented!()
	}
}

impl Decode for ACK {
	fn decode(serializer: &mut &[u8]) -> Self {
		unimplemented!()
	}
}