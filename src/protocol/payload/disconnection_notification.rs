use crate::protocol::{Payload, Encode, Decode};
use crate::protocol::message_identifiers::ID_DISCONNECTION_NOTIFICATION;

#[derive(Default, Debug)]
pub struct DisconnectionNotification;

impl Payload for DisconnectionNotification {
	const ID: u8 = ID_DISCONNECTION_NOTIFICATION;
}

impl Encode for DisconnectionNotification {
	fn encode(&self, serializer: &mut Vec<u8>) {
		// do nothing
	}
}

impl Decode for DisconnectionNotification {
	fn decode(serializer: &mut &[u8]) -> Self {
		Self
	}
}