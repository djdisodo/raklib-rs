use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};
use std::time::SystemTime;
use crate::protocol::{Payload, Encode, Decode, MessageIdentifiers};
use crate::protocol::payload::{PutAddress, PutTime, GetAddress, GetTime};
use bytes::{Buf};
use crate::SYSTEM_ADDRESS_COUNT;

#[derive(Debug)]
pub struct NewIncomingConnection {
	pub address: SocketAddr,
	pub system_addresses: Vec<SocketAddr>,
	pub send_ping_time: SystemTime,
	pub send_pong_time: SystemTime
}

impl Payload for NewIncomingConnection {
	const ID: MessageIdentifiers = MessageIdentifiers::NewIncomingConnection;
}

impl Encode for NewIncomingConnection {
	fn encode(&self, serializer: &mut Vec<u8>) {
		serializer.put_address(&self.address);
		for x in &self.system_addresses {
			serializer.put_address(x);
		}
		serializer.put_time(&self.send_ping_time);
		serializer.put_time(&self.send_pong_time);
	}
}

impl Decode for NewIncomingConnection {
	fn decode(serializer: &mut &[u8]) -> Self {
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
			send_ping_time: serializer.get_time(),
			send_pong_time: serializer.get_time()
		}
	}
}