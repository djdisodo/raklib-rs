use crate::protocol::{EncodeBody, DecodeBody, MessageIdentifiers, GetString, PutStr, CommonPacket, MessageIdentifierHeader};
use bytes::{BufMut, Buf};

#[derive(Default, Debug)]
pub struct AdvertiseSystem {
	pub server_name: String
}

impl MessageIdentifierHeader for AdvertiseSystem {
	const ID: MessageIdentifiers = MessageIdentifiers::AdvertiseSystem;
}

impl EncodeBody for AdvertiseSystem {
	fn encode_body(&self, serializer: &mut dyn BufMut) {
		serializer.put_str(&self.server_name);
	}
}

impl DecodeBody for AdvertiseSystem {
	fn decode_body(serializer: &mut dyn Buf) -> Self {
		Self {
			server_name: serializer.get_string()
		}
	}
}

impl CommonPacket for AdvertiseSystem {}