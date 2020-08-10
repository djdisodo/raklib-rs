use crate::protocol::{Payload, Encode, Decode};
use crate::protocol::MessageIdentifiers;

#[derive(Default, Debug)]
pub struct DisconnectionNotification;

impl Payload for DisconnectionNotification {
	const ID: MessageIdentifiers = MessageIdentifiers::DisconnectionNotification;
}

impl Encode for DisconnectionNotification {
	fn encode(&self, _serializer: &mut Vec<u8>) {
		// do nothing
	}
}

impl Decode for DisconnectionNotification {
	fn decode(_serializer: &mut &[u8]) -> Self {
		Self
	}
}