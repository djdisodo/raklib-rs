use crate::server::{ProtocolAcceptor, Server, ServerEventListener};
use std::net::SocketAddr;
use crate::protocol::payload::{UnconnectedPing, UnconnectedPong, UnconnectedPingOpenConnections, OpenConnectionRequest1, IncompatibleProtocolVersion, OpenConnectionReply1, OpenConnectionRequest2, OpenConnectionReply2};
use crate::protocol::{Packet, Payload, Decode, OfflineMessageImpl};
use log::{info, debug};
use crate::server::session::Session;
use std::cmp::min;
use std::convert::TryInto;
use std::sync::RwLock;
use bytes::Buf;
use std::any::Any;

pub struct UnconnectedMessageHandler<'a> {
	pub(super) server: &'a Server<'a>,
}

impl UnconnectedMessageHandler<'_> {
	pub fn handle_raw(&mut self, mut raw_payload: &[u8], address: &SocketAddr) -> bool{
		if raw_payload.is_empty() {
			return false;
		}
		//check header
		let id = raw_payload[0].try_into();
		if id.is_err() {
			return false;
		}

		match id.unwrap() { //wtf rust
			UnconnectedPing::ID => {
				let offline_message = Packet::<UnconnectedPing>::decode(&mut raw_payload).payload;
				if !offline_message.is_valid() {
					return false;
				}
				if raw_payload.has_remaining() {
					let remains = raw_payload.len();
					debug!("Still {} bytes unread in {:?} from {}", remains, offline_message, address);
				}
				self.handle(&offline_message, address)
			},
			OpenConnectionRequest1::ID => {
				let offline_message = Packet::<OpenConnectionRequest1>::decode(&mut raw_payload).payload;
					if !offline_message.is_valid() {
					return false;
				}
				if raw_payload.has_remaining() {
					let remains = raw_payload.len();
					debug!("Still {} bytes unread in {:?} from {}", remains, offline_message, address);
				}
				self.handle(&offline_message, address)
			},
			OpenConnectionRequest2::ID => {
				let offline_message = Packet::<OpenConnectionRequest2>::decode(&mut raw_payload).payload;
					if !offline_message.is_valid() {
					return false;
				}
				if raw_payload.has_remaining() {
					let remains = raw_payload.len();
					debug!("Still {} bytes unread in {:?} from {}", remains, offline_message, address);
				}
				self.handle(&offline_message, address)
			},
			UnconnectedPingOpenConnections::ID => return true,
			_ => return false
		}
		true
	}
}

pub trait Handle<T: OfflineMessageImpl> {
	fn handle(&mut self, offline_message: &T, address: &SocketAddr);
}

impl Handle<UnconnectedPing> for UnconnectedMessageHandler<'_> {
	fn handle(&mut self, offline_message: &UnconnectedPing, address: &SocketAddr) {
		self.server.send_packet(Packet::wrap(
			UnconnectedPong {
				offline_message: Default::default(),
				send_ping_time: offline_message.send_ping_time,
				server_id: self.server.get_id(),
				server_name: self.server.get_name().to_owned()
			}
		), address)
	}
}

impl Handle<OpenConnectionRequest1> for UnconnectedMessageHandler<'_> {
	fn handle(&mut self, offline_message: &OpenConnectionRequest1, address: &SocketAddr) {
		if !self.server.protocol_acceptor.accepts(offline_message.protocol) {
			self.server.send_packet(Packet::wrap(IncompatibleProtocolVersion::create(
					self.server.protocol_acceptor.get_primary_version(),
					self.server.get_id()
				)), address);
			info!("Refused connection from {} due to incompatible RakNet protocol version (version {})", address, offline_message.protocol);
		} else {
			//IP header size (20 bytes) + UDP header size (8 bytes)
			self.server.send_packet(Packet::wrap(OpenConnectionReply1::create(
				self.server.get_id(), false, offline_message.mtu_size + 28
			)), address);
		}
	}
}

impl Handle<OpenConnectionRequest2> for UnconnectedMessageHandler<'_> {
	fn handle(&mut self, offline_message: &OpenConnectionRequest2, address: &SocketAddr) {
		if offline_message.server_address.port() == self.server.get_port() || !self.server.get_port_checking() {
			if offline_message.mtu_size < Session::MIN_MTU_SIZE {
				debug!("Not creating session for {} due to bad MTU size {}", address, offline_message.mtu_size);
				return;
			}
			let mtu_size = min(offline_message.mtu_size, self.server.get_max_mtu_size());
			self.server.send_packet(Packet::wrap(OpenConnectionReply2::create(
					self.server.get_id(),
					address.to_owned(),
					mtu_size,
					false
			)), address);
		}
	}
}

