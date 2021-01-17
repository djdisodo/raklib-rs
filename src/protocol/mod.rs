mod ack;
mod acknowledge_packet;
mod advertise_system;
mod connected_ping;
mod connected_pong;
mod connection_request;
mod connection_request_accepted;
mod datagram;
mod disconnection_notification;
mod encapsulated_packet;
mod incompatible_protocol_version;
mod message_identifiers;
mod nack;
mod new_incoming_connection;
mod offline_message;
mod open_connection_reply1;
mod open_connection_reply2;
mod open_connection_request1;
mod open_connection_request2;
mod packet_reliability;
mod split_packet_info;
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
pub use datagram::Datagram;
pub use disconnection_notification::DisconnectionNotification;
pub use encapsulated_packet::EncapsulatedPacket;
pub use incompatible_protocol_version::IncompatibleProtocolVersion;
pub use message_identifiers::MessageIdentifiers;
pub use nack::NACK;
pub use new_incoming_connection::NewIncomingConnection;
pub use offline_message::*;
pub use open_connection_reply1::OpenConnectionReply1;
pub use open_connection_reply2::OpenConnectionReply2;
pub use open_connection_request1::OpenConnectionRequest1;
pub use open_connection_request2::OpenConnectionRequest2;
pub use packet_reliability::PacketReliability;
pub use split_packet_info::SplitPacketInfo;
pub use unconnected_ping::UnconnectedPing;
pub use unconnected_ping_open_connections::UnconnectedPingOpenConnections;
pub use unconnected_pong::UnconnectedPong;

use bytes::Buf;
use bytes::buf::BufMut;
use std::fmt::Debug;
use crate::RaknetTime;
use std::net::{SocketAddr, SocketAddrV6, SocketAddrV4, Ipv4Addr, Ipv6Addr};
use c_types::AF_INET6;
use downcast_rs::{impl_downcast, Downcast};

pub trait PacketImpl: Debug + EncodePacket + Send + Sync + 'static + Downcast {
	fn into_dyn(self) -> Packet where Self: Sized + 'static {
		Box::new(self)
	}
}
impl_downcast!(PacketImpl);

pub type Packet = Box<dyn PacketImpl>;

impl EncodePacket for Packet {
	fn encode_packet(&self, serializer: &mut dyn BufMut) {
		(**self).encode_packet(serializer);
	}
}

impl<T: Debug + EncodePacket + Send + Sync + 'static> PacketImpl for T {}

pub trait CommonPacket: EncodeHeader + EncodeBody + DecodeBody {}

impl<T: CommonPacket> CommonEncodePacket for T {}
impl<T: CommonPacket + MessageIdentifierHeader> CommonDecodePacket for T {}

pub trait EncodePacket {
	fn encode_packet(&self, serializer: &mut dyn BufMut);
}

impl<T: CommonEncodePacket> EncodePacket for T {
	fn encode_packet(&self, serializer: &mut dyn BufMut) {
		serializer.put_u8(self.encode_header());
		self.encode_body(serializer);
	}
}

pub trait CommonEncodePacket: EncodeHeader + EncodeBody {} //flag

pub trait DecodePacket {
	fn decode_packet(serializer: &mut dyn Buf) -> Self;
}

impl<T: CommonDecodePacket> DecodePacket for T {
	fn decode_packet(serializer: &mut dyn Buf) -> Self {
		if T::ID as u8 != serializer.get_u8() {
			panic!("message identifier doesn't match");
		}
		Self::decode_body(serializer)
	}
}

pub trait CommonDecodePacket: MessageIdentifierHeader + DecodeBody {} //flag

pub trait EncodeHeader {
	fn encode_header(&self) -> u8;
}

pub trait EncodeBody {
	fn encode_body(&self, serializer: &mut dyn BufMut);
}

pub trait DecodeBody {
	fn decode_body(serializer: &mut dyn Buf) -> Self;
}

pub trait MessageIdentifierHeader {
	const ID: MessageIdentifiers;
} //packet with message identifier header

impl<T: MessageIdentifierHeader> EncodeHeader for T {
	fn encode_header(&self) -> u8 {
		T::ID as u8
	}
}


trait GetAddress {
	fn get_address(&mut self) -> SocketAddr;
}

trait PutAddress {
	fn put_address(&mut self, address: &SocketAddr);
}

impl<T: Buf + ?Sized> GetAddress for T {
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
				self.copy_to_slice(&mut raw_ip_bytes);
				let ip = Ipv6Addr::from(raw_ip_bytes);
				let scope_id = self.get_u32();
				SocketAddr::V6(SocketAddrV6::new(ip, port, flow_info, scope_id))
			},
			v => panic!("unknown ip version: {}", v)
		}
	}
}

impl<T: BufMut + ?Sized> PutAddress for T {
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

impl<T: Buf + ?Sized> GetString for T {
	fn get_string(&mut self) -> String {
		let length = self.get_u16() as usize;
		let bytes = self.copy_to_bytes(length);
		String::from_utf8(bytes.to_vec()).expect("failed to parse as utf8")
	}
}

impl<T: BufMut + ?Sized> PutStr for T {
	fn put_str(&mut self, v: &str) {
		let bytes = v.as_bytes();
		self.put_u16(bytes.len() as u16);
		self.put_slice(bytes);
	}
}

trait GetRaknetTime {
	fn get_raknet_time(&mut self) -> RaknetTime;
}


trait PutRaknetTime {
	fn put_raknet_time(&mut self, time: &RaknetTime);
}

impl<T: Buf + ?Sized> GetRaknetTime for T {
	fn get_raknet_time(&mut self) -> RaknetTime {
		RaknetTime::from_millis(self.get_u64())
	}
}

impl<T: BufMut + ?Sized> PutRaknetTime for T {
	fn put_raknet_time(&mut self, time: &RaknetTime) {
		self.put_u64(time.as_millis() as u64);
	}
}

