
mod ack;
mod acknowledge_packet;
mod advertise_system;
mod connected_ping;
mod connected_pong;
mod connection_request;
mod connection_request_accepted;
mod disconnection_notification;
mod incompatible_protocol_version;
mod nack;
mod new_incoming_connection;
mod open_connection_reply1;
mod open_connection_reply2;
mod open_connection_request1;
mod open_connection_request2;
mod unconnected_ping;
mod unconnected_ping_open_connections;
mod unconnected_pong;

pub use ack::ACK;
pub use acknowledge_packet::AcknowledgePacket;
pub use advertise_system::AdvertiseSystem;
pub use connected_ping::ConnectedPing;
pub use connected_pong::ConnectedPong;
pub use connection_request::ConnectionRequest;
pub use connection_request_accepted::ConnectionRequestAccepted;
pub use disconnection_notification::DisconnectionNotification;
pub use incompatible_protocol_version::IncompatibleProtocolVersion;
pub use nack::NACK;
pub use new_incoming_connection::NewIncomingConnection;
pub use open_connection_reply1::OpenConnectionReply1;
pub use open_connection_reply2::OpenConnectionReply2;
pub use open_connection_request1::OpenConnectionRequest1;
pub use open_connection_request2::OpenConnectionRequest2;
pub use unconnected_ping::UnconnectedPing;
pub use unconnected_ping_open_connections::UnconnectedPingOpenConnections;
pub use unconnected_pong::UnconnectedPong;

use std::fmt::Debug;
use bytes::{Buf, BufMut};

use std::net::{SocketAddr, Ipv4Addr, SocketAddrV4, Ipv6Addr, SocketAddrV6};
use crate::protocol::{Encode, Decode, MessageIdentifiers};
use bytes::buf::BufExt;
use std::time::{SystemTime, Duration};
use c_types::AF_INET6;

pub trait PayloadExt {
	fn get_id(&self) -> MessageIdentifiers;
}

impl<T: Payload> PayloadExt for T {
	fn get_id(&self) -> MessageIdentifiers {
		T::ID
	}
}

pub trait Payload: Debug + Encode + Decode {
	const ID: MessageIdentifiers;
}

trait GetAddress {
	fn get_address(&mut self) -> SocketAddr;
}

trait PutAddress {
	fn put_address(&mut self, address: &SocketAddr);
}

impl<T: Buf> GetAddress for T {
	fn get_address(&mut self) -> SocketAddr {
		match self.get_u8() {
			4 => {
				let mut raw_ip_bytes: [u8; 4] = [0; 4];
				for i in 0..4 {
					raw_ip_bytes[i] = !self.get_u8();
				}
				SocketAddr::V4(
					SocketAddrV4::new(
						Ipv4Addr::from(
							raw_ip_bytes
						),
						self.get_u16()
					)
				)
			},
			6 => {
				let af = self.get_u16_le();
				assert_eq!(af, AF_INET6 as u16, "{}: not AF_INET6", af);
				let port = self.get_u16();
				let flow_info = self.get_u32();
				let mut raw_ip_bytes: [u8; 16] = [0; 16];
				raw_ip_bytes.copy_from_slice(self.take(16).bytes());
				let ip = Ipv6Addr::from(raw_ip_bytes);
				let scope_id = self.get_u32();
				SocketAddr::V6(SocketAddrV6::new(ip, port, flow_info, scope_id))
			},
			v => panic!("unknown ip version: {}", v)
		}
	}
}

impl<T: BufMut> PutAddress for T {
	fn put_address(&mut self, address: &SocketAddr) {
		match address {
			SocketAddr::V4(addr) => {
				for x in addr.ip().octets().iter() {
					self.put_u8(!*x)
				}
				self.put_u16(addr.port());
			},
			SocketAddr::V6(addr) => {
				self.put_u16_le(AF_INET6 as u16);
				self.put_u16(addr.port());
				self.put_u32(addr.flowinfo());
				self.put_slice(&addr.ip().octets()); //inet_ntop
				self.put_u32(addr.scope_id());
			}
		}
	}
}

trait GetString {
	fn get_string(&mut self) -> String;
}

trait PutStr {
	fn put_str(&mut self, v: &str);
}

impl<T: Buf> GetString for T {
	fn get_string(&mut self) -> String {
		let length = self.get_u16();
		let bytes = self.take(length as usize);
		String::from_utf8(bytes.bytes().to_vec()).expect("failed to parse as utf8")
	}
}

impl<T: BufMut> PutStr for T {
	fn put_str(&mut self, v: &str) {
		let bytes = v.as_bytes();
		self.put_u16(bytes.len() as u16);
		self.put_slice(bytes);
	}
}

trait GetTime {
	fn get_time(&mut self) -> SystemTime;
}

trait PutTime {
	fn put_time(&mut self, time: &SystemTime);
}

impl<T: Buf> GetTime for T {
	fn get_time(&mut self) -> SystemTime {
		SystemTime::UNIX_EPOCH + Duration::from_millis(self.get_u64()) // TODO is millis?
	}
}

impl<T: BufMut> PutTime for T {
	fn put_time(&mut self, time: &SystemTime) {
		self.put_u64(time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_micros() as u64);
	}
}