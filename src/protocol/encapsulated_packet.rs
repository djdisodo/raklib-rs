use crate::protocol::{Encode, Decode};
use bytes::{BufMut, Buf};
use crate::protocol::PacketReliability;
use bytes_addition::{PutTriad, GetTriad};
use crate::protocol::SplitPacketInfo;
use std::convert::TryInto;
use std::io::Read;

#[derive(Default, Debug)]
pub struct EncapsulatedPacket {
	pub reliability: PacketReliability,
	pub message_index: u32,
	pub sequence_index: u32,
	pub order_index: u32,
	pub order_channel: u8,
	pub split_info: Option<SplitPacketInfo>,
	pub buffer: Vec<u8>,
	pub identifier_ack: Option<u64> //TODO check type.
}

impl EncapsulatedPacket {
	const RELIABILITY_SHIFT: u8 = 5;
	const RELIABILITY_FLAGS: u8 = 0b111 << Self::RELIABILITY_SHIFT;

	const SPLIT_FLAG: u8 = 0b00010000;
}

impl Encode for EncapsulatedPacket {
	fn encode(&self, serializer: &mut Vec<u8>) {
		serializer.put_u8(
			((self.reliability as u8) << Self::RELIABILITY_SHIFT) |
				if self.split_info.is_some() {
					Self::SPLIT_FLAG
				} else {
					0
				}
		);
		serializer.put_u16((self.buffer.len() << 3) as u16);
		if self.reliability.is_reliable() {
			serializer.put_u24_le(self.message_index);
		}
		if self.reliability.is_sequenced() {
			serializer.put_u24_le(self.sequence_index);
		}
		if self.reliability.is_sequenced() || self.reliability.is_ordered() {
			serializer.put_u24_le(self.order_index);
			serializer.put_u8(self.order_channel);
		}
		match &self.split_info {
			Some(split_info) => split_info.encode(serializer),
			_ => {}
		}
		serializer.put_slice(self.buffer.as_slice());
	}
}

impl Decode for EncapsulatedPacket {
	fn decode(serializer: &mut &[u8]) -> Self {
		let mut packet = EncapsulatedPacket::default();
		let flags = serializer.get_u8();
		packet.reliability = ((flags & Self::RELIABILITY_FLAGS) >> Self::RELIABILITY_SHIFT).try_into().expect("failed to decode packet reliability");
		let has_split = (flags & Self::SPLIT_FLAG) != 0;

		let length = (serializer.get_u16() as f32 / 8f32).ceil() as u16;
		if length == 0 {
			panic!("Encapsulated payload length cannot be zero"); // todo implement Error
		}

		if packet.reliability.is_reliable() {
			packet.message_index = serializer.get_u24_le();
		}

		if packet.reliability.is_sequenced() {
			packet.sequence_index = serializer.get_u24_le();
		}

		if packet.reliability.is_sequenced() || packet.reliability.is_ordered() {
			packet.order_index = serializer.get_u24_le();
			packet.order_channel = serializer.get_u8();
		}

		if has_split {
			packet.split_info.replace(SplitPacketInfo::decode(serializer));
		}

		packet.buffer = vec![0; length as usize];
		serializer.read(&mut packet.buffer);

		packet
	}
}