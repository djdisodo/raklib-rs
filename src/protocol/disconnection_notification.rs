use crate::protocol::{MessageIdentifierHeader, EncodeBody, DecodeBody, CommonPacket, MessageIdentifiers};
use bytes::{BufMut, Buf};

#[derive(Default, Debug)]
pub struct DisconnectionNotification;

impl MessageIdentifierHeader for DisconnectionNotification {
	const ID: MessageIdentifiers = MessageIdentifiers::DisconnectionNotification;
}

impl EncodeBody for DisconnectionNotification {
	fn encode_body(&self, mut _serializer: &mut dyn BufMut) {
		// do nothing
	}
}

impl DecodeBody for DisconnectionNotification {
	fn decode_body(_serializer: &mut dyn Buf) -> Self {
		Self
	}
}

impl CommonPacket for DisconnectionNotification {}