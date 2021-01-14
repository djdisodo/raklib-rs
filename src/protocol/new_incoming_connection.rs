use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};
use crate::protocol::{MessageIdentifierHeader, EncodeBody, DecodeBody, MessageIdentifiers, GetRaknetTime, PutRaknetTime, GetAddress, CommonPacket, PutAddress};
use bytes::{BufMut, Buf};
use crate::{SYSTEM_ADDRESS_COUNT, RaknetTime};

#[derive(Debug)]
pub struct NewIncomingConnection {
	pub address: SocketAddr,
	pub system_addresses: Vec<SocketAddr>,
	pub send_ping_time: RaknetTime,
	pub send_pong_time: RaknetTime
}

impl MessageIdentifierHeader for NewIncomingConnection {
	const ID: MessageIdentifiers = MessageIdentifiers::NewIncomingConnection;
}

impl EncodeBody for NewIncomingConnection {
	fn encode_body(&self, serializer: &mut dyn BufMut) {
		serializer.put_address(&self.address);
		for x in &self.system_addresses {
			serializer.put_address(x);
		}
		serializer.put_raknet_time(&self.send_ping_time);
		serializer.put_raknet_time(&self.send_pong_time);
	}
}

impl DecodeBody for NewIncomingConnection {
	fn decode_body(serializer: &mut dyn Buf) -> Self {
		let address = serializer.get_address();

		// TODO hack
		let mut system_addresses = Vec::new();
		let dummy = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0));
		for _i in 0..SYSTEM_ADDRESS_COUNT {
			system_addresses.push(
				if serializer.remaining() <= 16 {
					dummy.clone()
				} else {
					serializer.get_address()
				}
			)
		}
		Self {
			address,
			system_addresses,
			send_ping_time: serializer.get_raknet_time(),
			send_pong_time: serializer.get_raknet_time()
		}
	}
}

impl CommonPacket for NewIncomingConnection {}