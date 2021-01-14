use crate::protocol::{EncodeBody, DecodeBody};
use bytes::{BufMut, Buf};

#[derive(Debug, Clone)]
pub struct SplitPacketInfo {
	id: u16,
	part_index: u32,
	total_part_count: u32
}

impl SplitPacketInfo {
	pub fn new(
		id: u16,
		part_index: u32,
		total_part_count: u32
	) -> Self {
		Self {
			id,
			part_index,
			total_part_count
		}
	}

	pub fn get_id(&self) -> u16 {
		self.id
	}

	pub fn get_part_index(&self) -> u32 {
		self.part_index
	}

	pub fn get_total_part_count(&self) -> u32 {
		self.total_part_count
	}
}

impl EncodeBody for SplitPacketInfo {
	fn encode_body(&self, serializer: &mut dyn BufMut) {
		serializer.put_u32(self.total_part_count);
		serializer.put_u16(self.id);
		serializer.put_u32(self.part_index);
	}
}

impl DecodeBody for SplitPacketInfo {
	fn decode_body(serializer: &mut dyn Buf) -> Self {
		Self {
			total_part_count: serializer.get_u32(),
			id: serializer.get_u16(),
			part_index: serializer.get_u32()
		}
	}
}