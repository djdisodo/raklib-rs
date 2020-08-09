use crate::protocol::{Encode, Decode};
use bytes::{BufMut, Buf};

#[derive(Debug)]
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

impl Encode for SplitPacketInfo {
	fn encode(&self, serializer: &mut Vec<u8>) {
		serializer.put_u32(self.total_part_count);
		serializer.put_u16(self.id);
		serializer.put_u32(self.part_index);
	}
}

impl Decode for SplitPacketInfo {
	fn decode(serializer: &mut &[u8]) -> Self {
		Self {
			total_part_count: serializer.get_u32(),
			id: serializer.get_u16(),
			part_index: serializer.get_u32()
		}
	}
}