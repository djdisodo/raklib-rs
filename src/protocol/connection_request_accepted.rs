use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};
use crate::protocol::{MessageIdentifierHeader, EncodeBody, DecodeBody, PutAddress, GetAddress, GetRaknetTime, PutRaknetTime, MessageIdentifiers, CommonPacket};
use bytes::{BufMut, Buf};
use crate::{SYSTEM_ADDRESS_COUNT, RaknetTime};

#[derive(Debug)]
pub struct ConnectionRequestAccepted {
	pub address: SocketAddr,
	pub system_addresses: Vec<SocketAddr>,
	pub send_ping_time: RaknetTime,
	pub send_pong_time: RaknetTime
}

impl ConnectionRequestAccepted {
	pub fn create(client_address: SocketAddr, system_addresses: Vec<SocketAddr>, send_ping_time: RaknetTime, send_pong_time: RaknetTime) -> Self {
		Self {
			address: client_address,
			system_addresses,
			send_ping_time,
			send_pong_time
		}
	}
}

impl MessageIdentifierHeader for ConnectionRequestAccepted {
	const ID: MessageIdentifiers = MessageIdentifiers::ConnectionRequestAccepted;
}

impl EncodeBody for ConnectionRequestAccepted {
	fn encode_body(&self, serializer: &mut dyn BufMut) {
		serializer.put_address(&self.address);
		serializer.put_u16(0);

		let dummy = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0));

		for i in 0..SYSTEM_ADDRESS_COUNT {
			serializer.put_address(self.system_addresses.get(i).unwrap_or(&dummy));
		}

		serializer.put_raknet_time(&self.send_ping_time);
		serializer.put_raknet_time(&self.send_pong_time);
	}
}

impl DecodeBody for ConnectionRequestAccepted {
	fn decode_body(serializer: &mut dyn Buf) -> Self {
		let address = serializer.get_address();
		serializer.get_u16(); // TODO check this

		let dummy = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0));

		let mut system_addresses = Vec::new();
		for _i in 0..SYSTEM_ADDRESS_COUNT {
			system_addresses.push(
				if serializer.remaining() > 16 {
					serializer.get_address()
				} else {
					dummy.clone()
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

impl CommonPacket for ConnectionRequestAccepted {}