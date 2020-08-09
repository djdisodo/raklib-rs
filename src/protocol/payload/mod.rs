
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


use std::fmt::Debug;
use bytes::{BytesMut, Bytes, Buf, BufMut};
use std::io::Error;
use std::net::SocketAddr;
use crate::protocol::{Encode, Decode};

pub trait Payload: Debug + Encode + Decode {
	const ID: u8;
}

trait GetAddress {
	fn get_address(&mut self) -> SocketAddr;
}

trait PutAddress {
	fn put_address(&mut self, address: SocketAddr);
}

impl<T: Buf> GetAddress for T {
	fn get_address(&mut self) -> SocketAddr {
		unimplemented!()
	}
}

impl<T: BufMut> PutAddress for T {
	fn put_address(&mut self, address: SocketAddr) {
		unimplemented!()
	}
}