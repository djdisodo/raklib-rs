use crate::protocol::{EncodeBody, DecodeBody};
use bytes::{BufMut, Buf};
use crate::protocol::PacketReliability;
use bytes_addition::{PutTriad, GetTriad};
use crate::protocol::SplitPacketInfo;
use std::convert::TryInto;
use std::io::Read;

#[derive(Clone, Default, Debug)]
pub struct EncapsulatedPacket {
	pub reliability: PacketReliability,
	pub message_index: Option<u32>,
	pub sequence_index: Option<u32>,
	pub order_index: Option<u32>,
	pub order_channel: Option<u8>,
	pub split_info: Option<SplitPacketInfo>,
	pub buffer: Vec<u8>,
	pub identifier_ack: Option<u64> //TODO check type.
}

impl EncapsulatedPacket {
	const RELIABILITY_SHIFT: u8 = 5;
	const RELIABILITY_FLAGS: u8 = 0b111 << Self::RELIABILITY_SHIFT;

	const SPLIT_FLAG: u8 = 0b00010000;

	pub fn get_total_length(&self) -> usize {
		1 + //reliability
		2 + //length
		if self.reliability.is_reliable() { 3 } else { 0 } + //message index
		if self.reliability.is_sequenced() { 3 } else { 0 } + //sequence index
		if self.reliability.is_sequenced() || self.reliability.is_ordered() { 3 + 1 } else { 0 } + //order index (3) + order channel (1)
		if self.split_info.is_some() { 4 + 2 + 4 } else { 0 } + //split count (4) + split ID (2) + split index (4)
		self.buffer.len()
	}
}

impl EncodeBody for EncapsulatedPacket {
	fn encode_body(&self, serializer: &mut dyn BufMut) {
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
			serializer.put_u24_le(self.message_index.unwrap());
		}
		if self.reliability.is_sequenced() {
			serializer.put_u24_le(self.sequence_index.unwrap());
		}
		if self.reliability.is_sequenced() || self.reliability.is_ordered() {
			serializer.put_u24_le(self.order_index.unwrap());
			serializer.put_u8(self.order_channel.unwrap());
		}
		match &self.split_info {
			Some(split_info) => split_info.encode_body(serializer),
			_ => {}
		}
		serializer.put_slice(self.buffer.as_slice());
	}
}

impl DecodeBody for EncapsulatedPacket {
	fn decode_body(serializer: &mut dyn Buf) -> Self {
		let mut packet = EncapsulatedPacket::default();
		let flags = serializer.get_u8();
		packet.reliability = ((flags & Self::RELIABILITY_FLAGS) >> Self::RELIABILITY_SHIFT).try_into().expect("failed to decode packet reliability");
		let has_split = (flags & Self::SPLIT_FLAG) != 0;

		let length = (serializer.get_u16() as f32 / 8f32).ceil() as u16;
		if length == 0 {
			panic!("Encapsulated packet length cannot be zero"); // todo implement Error
		}

		if packet.reliability.is_reliable() {
			packet.message_index = Some(serializer.get_u24_le());
		}

		if packet.reliability.is_sequenced() {
			packet.sequence_index = Some(serializer.get_u24_le());
		}

		if packet.reliability.is_sequenced() || packet.reliability.is_ordered() {
			packet.order_index = Some(serializer.get_u24_le());
			packet.order_channel = Some(serializer.get_u8());
		}

		if has_split {
			packet.split_info.replace(SplitPacketInfo::decode_body(serializer));
		}

		packet.buffer = vec![0; length as usize];
		serializer.copy_to_slice(&mut packet.buffer);

		packet
	}
}