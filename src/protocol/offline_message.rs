use crate::protocol::{EncodeBody, DecodeBody, MessageIdentifierHeader};
use bytes::{BufMut, Buf};

#[derive(Default, Debug)]
pub struct OfflineMessage {
	pub magic: [u8; 16]
}

impl OfflineMessage {
	const MAGIC: [u8; 16] = [0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34, 0x56, 0x78];

	pub fn is_valid(&self) -> bool {
		self.magic == Self::MAGIC
	}
}

impl EncodeBody for OfflineMessage {
	fn encode_body(&self, serializer: &mut dyn BufMut) {
		serializer.put_slice(&Self::MAGIC);
	}
}

impl DecodeBody for OfflineMessage {
	fn decode_body(serializer: &mut dyn Buf) -> Self {
		let mut magic = [0; 16];
		serializer.copy_to_slice(&mut magic);
		Self {
			magic
		}
	}
}

pub trait OfflineMessageImpl: MessageIdentifierHeader {
	fn get_offline_message(&self) -> &OfflineMessage;
	fn is_valid(&self) -> bool {
		self.get_offline_message().is_valid()
	}
}