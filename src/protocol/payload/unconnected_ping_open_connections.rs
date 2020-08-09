use crate::protocol::payload::unconnected_ping::UnconnectedPing;
use crate::protocol::{Payload, Encode, Decode};
use crate::protocol::message_identifiers::ID_UNCONNECTED_PING_OPEN_CONNECTIONS;

#[derive(Debug, Deref, DerefMut)]
pub struct UnconnectedPingOpenConnections {
	unconnected_ping: UnconnectedPing
}

impl Payload for UnconnectedPingOpenConnections {
	const ID: u8 = ID_UNCONNECTED_PING_OPEN_CONNECTIONS;
}

impl Encode for UnconnectedPingOpenConnections {
	fn encode(&self, serializer: &mut Vec<u8>) {
		unimplemented!()
	}
}

impl Decode for UnconnectedPingOpenConnections {
	fn decode(serializer: &mut &[u8]) -> Self {
		unimplemented!()
	}
}