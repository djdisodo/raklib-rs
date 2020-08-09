use crate::protocol::{Encode, Decode};

#[derive(Default, Debug)]
pub struct EncapsulatedPacket {
	pub reliability: u8,
	pub message_index: u32,
	pub sequence_index: u32,
	pub order_index: u32,
	pub order_channel: u8,
	pub split_info: Option<()>, //TODO SplitPacketInfo
	pub buffer: Vec<u8>, //TODO remove if can
	pub identifier_ack: Option<u64> //TODO check type.
}

impl EncapsulatedPacket {
	const RELIABILITY_SHIFT: u8 = 5;
	const RELIABILITY_FLAGS: u8 = 0b111 << Self::RELIABILITY_SHIFT;

	const SPLIT_FLAG: u8 = 0b00010000;
}

impl Encode for EncapsulatedPacket {
	fn encode(&self, serializer: &mut Vec<u8>) {
		unimplemented!()
	}
}

impl Decode for EncapsulatedPacket {
	fn decode(serializer: &mut &[u8]) -> Self {
		unimplemented!()
	}
}