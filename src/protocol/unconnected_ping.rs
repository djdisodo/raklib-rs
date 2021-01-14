use crate::protocol::{MessageIdentifierHeader, EncodeBody, DecodeBody, MessageIdentifiers, OfflineMessage, OfflineMessageImpl, CommonPacket, GetRaknetTime, PutRaknetTime};
use bytes::{BufMut, Buf};
use crate::RaknetTime;

#[derive(Debug)]
pub struct UnconnectedPing {
	pub offline_message: OfflineMessage,
	pub send_ping_time: RaknetTime,
	pub client_id: u64
}

impl OfflineMessageImpl for UnconnectedPing {
	fn get_offline_message(&self) -> &OfflineMessage {
		&self.offline_message
	}
}

impl MessageIdentifierHeader for UnconnectedPing {
	const ID: MessageIdentifiers = MessageIdentifiers::UnconnectedPing;
}

impl EncodeBody for UnconnectedPing {
	fn encode_body(&self, serializer: &mut dyn BufMut) {
		serializer.put_raknet_time(&self.send_ping_time);
		self.offline_message.encode_body(serializer);
		serializer.put_u64(self.client_id);
	}
}

impl DecodeBody for UnconnectedPing {
	fn decode_body(serializer: &mut dyn Buf) -> Self {
		Self {
			send_ping_time: serializer.get_raknet_time(),
			offline_message: OfflineMessage::decode_body(serializer),
			client_id: serializer.get_u64()
		}
	}
}

impl CommonPacket for UnconnectedPing {}