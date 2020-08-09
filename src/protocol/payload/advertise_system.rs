use crate::protocol::{Payload, Encode, Decode};
use crate::protocol::message_identifiers::ID_ADVERTISE_SYSTEM;

#[derive(Default, Debug)]
pub struct AdvertiseSystem {
	pub server_name: String
}

impl Payload for AdvertiseSystem {
	const ID: u8 = ID_ADVERTISE_SYSTEM;
}

impl Encode for AdvertiseSystem {
	fn encode(&self, serializer: &mut Vec<u8>) {
		unimplemented!()
	}
}

impl Decode for AdvertiseSystem {
	fn decode(serializer: &mut &[u8]) -> Self {
		unimplemented!()
	}
}