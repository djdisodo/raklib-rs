use crate::protocol::{MessageIdentifierHeader, EncodeBody, DecodeBody, OfflineMessage, OfflineMessageImpl, MessageIdentifiers, CommonPacket};
use bytes::{BufMut, Buf};

#[derive(Default, Debug)]
pub struct IncompatibleProtocolVersion {
	pub offline_message: OfflineMessage,
	pub protocol_version: u8,
	pub server_id: u64
}

impl OfflineMessageImpl for IncompatibleProtocolVersion {
	fn get_offline_message(&self) -> &OfflineMessage {
		&self.offline_message
	}
}

impl IncompatibleProtocolVersion {
	pub fn create(protocol_version: u8, server_id: u64) -> Self {
		Self {
			offline_message: OfflineMessage::default(),
			protocol_version,
			server_id
		}
	}
}

impl MessageIdentifierHeader for IncompatibleProtocolVersion {
	const ID: MessageIdentifiers = MessageIdentifiers::IncompatibleProtocolVersion;
}

impl EncodeBody for IncompatibleProtocolVersion {
	fn encode_body(&self, serializer: &mut dyn BufMut) {
		serializer.put_u8(self.protocol_version);
		self.offline_message.encode_body(serializer);
		serializer.put_u64(self.server_id);
	}
}

impl DecodeBody for IncompatibleProtocolVersion {
	fn decode_body(serializer: &mut dyn Buf) -> Self {
		Self {
			protocol_version: serializer.get_u8(),
			offline_message: OfflineMessage::decode_body(serializer),
			server_id: serializer.get_u64()
		}
	}
}

impl CommonPacket for IncompatibleProtocolVersion {}