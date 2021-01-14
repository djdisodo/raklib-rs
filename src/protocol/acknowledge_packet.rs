use bytes_addition::{PutTriad, GetTriad};
use crate::protocol::{EncodeBody, DecodeBody};
use bytes::{Buf, BufMut};

#[derive(Default, Debug)]
pub struct AcknowledgePacket {
	pub packets: Vec<u32>
}

const RECORD_TYPE_RANGE: u8 = 0;
const RECORD_TYPE_SINGLE: u8 = 1;

impl EncodeBody for AcknowledgePacket {
	fn encode_body(&self, serializer: &mut dyn BufMut) {
		serializer.put_u8(10);
		let mut payload: Vec<u8> = Vec::new();
		let count = self.packets.len();
		let mut records = 0;
		if count > 0 {
			let mut packets = self.packets.clone();
			packets.sort_unstable();

			let mut pointer = 1;
			let mut start = packets[0];
			let mut last = packets[0];

			while pointer < count {
				let current = packets[pointer];
				pointer += 1;
				let diff = current - last;
				if diff == 1 {
					last = current;
				} else if diff > 1 {
					if start == last {
						payload.put_u8(RECORD_TYPE_SINGLE);
						payload.put_u24_le(start);
						start = current;
						last = current;
					} else {
						payload.put_u8(RECORD_TYPE_RANGE);
						payload.put_u24_le(start);
						payload.put_u24_le(last);
						start = current;
						last = current;
					}
					records += 1;
				}
			}
			if start == last {
				payload.put_u8(RECORD_TYPE_SINGLE);
				payload.put_u24_le(start);
			} else {
				payload.put_u8(RECORD_TYPE_RANGE);
				payload.put_u24_le(start);
				payload.put_u24_le(last);
			}
			records += 1;
		}
		serializer.put_u8(records);
		serializer.put_slice(payload.as_slice());
	}
}

impl DecodeBody for AcknowledgePacket {
	fn decode_body(serializer: &mut dyn Buf) -> Self {
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