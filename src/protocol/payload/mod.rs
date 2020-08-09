
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
mod offline_message;
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

use std::fmt::Debug;
use bytes::{Buf, BufMut};

use std::net::SocketAddr;
use crate::protocol::{Encode, Decode, MessageIdentifiers};
use bytes::buf::BufExt;
use std::time::{SystemTime, Duration};


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
		unimplemented!()
	}
}

impl<T: BufMut> PutAddress for T {
	fn put_address(&mut self, _address: &SocketAddr) {
		unimplemented!()
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