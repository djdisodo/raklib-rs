use bytes_addition::GetTriad;
use crate::protocol::{Encode, Decode};
use bytes::Buf;

#[derive(Default, Debug)]
pub struct AcknowledgePacket {
	pub packets: Vec<u32>
}

const RECORD_TYPE_RANGE: u8 = 0;
const RECORD_TYPE_SINGLE: u8 = 1;

impl Encode for AcknowledgePacket {
	fn encode(&self, serializer: &mut Vec<u8>) {
		unimplemented!()
	}
}

impl Decode for AcknowledgePacket {
	fn decode(serializer: &mut &[u8]) -> Self {
		let count = serializer.get_u16();
		let mut packets = Vec::new();
		{
			let mut i = 0;
			while i < count && serializer.has_remaining() {
				if serializer.get_u8() == RECORD_TYPE_RANGE {
					let start = serializer.get_u24_le();
					let end = {
						let _end = serializer.get_u24_le();
						if _end - start > 512 {
							start + 512
						} else {
							_end
						}
					};
					for c in start..=end {
						packets.push(c);
					}
				} else {
					packets.push(serializer.get_u24_le());
				}
				i += 1;
			}
		}
		Self {
			packets
		}
	}
}