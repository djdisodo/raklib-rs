use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};
use std::time::SystemTime;
use crate::protocol::{Payload, Encode, Decode};
use crate::protocol::MessageIdentifiers;
use crate::protocol::payload::{PutAddress, PutTime, GetAddress, GetTime};
use bytes::{BufMut, Buf};
use crate::SYSTEM_ADDRESS_COUNT;

#[derive(Debug)]
pub struct ConnectionRequestAccepted {
	pub address: SocketAddr,
	pub system_addresses: Vec<SocketAddr>,
	pub send_ping_time: SystemTime,
	pub send_pong_time: SystemTime
}

impl ConnectionRequestAccepted {
	pub fn create(client_address: SocketAddr, system_addresses: Vec<SocketAddr>, send_ping_time: SystemTime, send_pong_time: SystemTime) -> Self {
		Self {
			address: client_address,
			system_addresses,
			send_ping_time,
			send_pong_time
		}
	}
}

impl Payload for ConnectionRequestAccepted {
	const ID: MessageIdentifiers = MessageIdentifiers::ID_CONNECTION_REQUEST_ACCEPTED;
}

impl Encode for ConnectionRequestAccepted {
	fn encode(&self, serializer: &mut Vec<u8>) {
		serializer.put_address(&self.address);
		serializer.put_u16(0);

		let dummy = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0));

		for i in 0..SYSTEM_ADDRESS_COUNT {
			serializer.put_address(self.system_addresses.get(i).unwrap_or(&dummy));
		}

		serializer.put_time(&self.send_ping_time);
		serializer.put_time(&self.send_pong_time);
	}
}

impl Decode for ConnectionRequestAccepted {
	fn decode(serializer: &mut &[u8]) -> Self {
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
			send_ping_time: serializer.get_time(),
			send_pong_time: serializer.get_time()
		}
	}
}