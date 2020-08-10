use crate::protocol::payload::UnconnectedPing;
use crate::protocol::{Payload, Encode, Decode, MessageIdentifiers};

#[derive(Debug, Deref, DerefMut)]
pub struct UnconnectedPingOpenConnections {
	unconnected_ping: UnconnectedPing
}

impl Payload for UnconnectedPingOpenConnections {
	const ID: MessageIdentifiers = MessageIdentifiers::UnconnectedPingOpenConnections;
}

impl Encode for UnconnectedPingOpenConnections {
	fn encode(&self, serializer: &mut Vec<u8>) {
		(**self).encode(serializer);
	}
}

impl Decode for UnconnectedPingOpenConnections {
	fn decode(serializer: &mut &[u8]) -> Self {
		Self {
			unconnected_ping: UnconnectedPing::decode(serializer)
		}
	}
}