use crate::protocol::{Payload, Encode, Decode};
use crate::protocol::message_identifiers::ID_INCOMPATIBLE_PROTOCOL_VERSION;

#[derive(Default, Debug)]
pub struct IncompatibleProtocolVersion {
	pub protocol_version: u8,
	pub server_id: u64
}

impl IncompatibleProtocolVersion {
	pub fn create(protocol_version: u8, server_id: u64) -> Self {
		Self {
			protocol_version,
			server_id
		}
	}
}

impl Payload for IncompatibleProtocolVersion {
	const ID: u8 = ID_INCOMPATIBLE_PROTOCOL_VERSION;
}

impl Encode for IncompatibleProtocolVersion {
	fn encode(&self, serializer: &mut Vec<u8>) {
		unimplemented!()
	}
}

impl Decode for IncompatibleProtocolVersion {
	fn decode(serializer: &mut &[u8]) -> Self {
		unimplemented!()
	}
}