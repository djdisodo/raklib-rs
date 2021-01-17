use crate::protocol::{MessageIdentifierHeader, EncodeBody, DecodeBody, MessageIdentifiers, UnconnectedPing, CommonPacket, OfflineMessageImpl, OfflineMessage};
use bytes::{BufMut, Buf};

#[derive(Debug, Deref, DerefMut)]
pub struct UnconnectedPingOpenConnections {
	unconnected_ping: UnconnectedPing
}

impl MessageIdentifierHeader for UnconnectedPingOpenConnections {
	const ID: MessageIdentifiers = MessageIdentifiers::UnconnectedPingOpenConnections;
}

impl EncodeBody for UnconnectedPingOpenConnections {
	fn encode_body(&self, serializer: &mut dyn BufMut) {
		(**self).encode_body(serializer);
	}
}

impl DecodeBody for UnconnectedPingOpenConnections {
	fn decode_body(serializer: &mut dyn Buf) -> Self {
		Self {
			unconnected_ping: UnconnectedPing::decode_body(serializer)
		}
	}
}

impl CommonPacket for UnconnectedPingOpenConnections {}

impl OfflineMessageImpl for UnconnectedPingOpenConnections {
	fn get_offline_message(&self) -> &OfflineMessage {
		self.get_offline_message()
	}
}