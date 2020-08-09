use crate::protocol::payload::unconnected_ping::UnconnectedPing;
use crate::protocol::{Payload, Encode, Decode, MessageIdentifiers};

#[derive(Debug, Deref, DerefMut)]
pub struct UnconnectedPingOpenConnections {
	unconnected_ping: UnconnectedPing
}

impl Payload for UnconnectedPingOpenConnections {
	const ID: MessageIdentifiers = MessageIdentifiers::ID_UNCONNECTED_PING_OPEN_CONNECTIONS;
}

impl Encode for UnconnectedPingOpenConnections {
	fn encode(&self, _serializer: &mut Vec<u8>) {
		unimplemented!()
	}
}

impl Decode for UnconnectedPingOpenConnections {
	fn decode(_serializer: &mut &[u8]) -> Self {
		unimplemented!()
	}
}