use std::net::SocketAddr;
use crate::protocol::{MessageIdentifierHeader, OfflineMessageImpl, UnconnectedPing, OpenConnectionRequest1, OpenConnectionRequest2, UnconnectedPingOpenConnections, UnconnectedPong, IncompatibleProtocolVersion, OpenConnectionReply1, OpenConnectionReply2, DecodePacket};
use log::{info, debug};
use crate::server::SessionMutable;
use std::cmp::min;
use std::convert::TryInto;

use bytes::Buf;


use crate::server::ServerMutable;

pub struct UnconnectedMessageHandler<'a> {
	pub(super) server: &'a mut ServerMutable<'a>,
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
				let offline_message = UnconnectedPing::decode_packet(&mut raw_payload);
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
				let offline_message = OpenConnectionRequest1::decode_packet(&mut raw_payload);
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
				let offline_message = OpenConnectionRequest2::decode_packet(&mut raw_payload);
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
		self.server.send_packet(&UnconnectedPong {
				offline_message: Default::default(),
				send_ping_time: offline_message.send_ping_time,
				server_id: self.server.id,
				server_name: self.server.name.to_owned()
			}, address)
	}
}

impl Handle<OpenConnectionRequest1> for UnconnectedMessageHandler<'_> {
	fn handle(&mut self, offline_message: &OpenConnectionRequest1, address: &SocketAddr) {
		if !self.server.protocol_acceptor.accepts(offline_message.protocol) {
			self.server.send_packet(&IncompatibleProtocolVersion::create(
					self.server.protocol_acceptor.get_primary_version(),
					self.server.id
				), address);
			info!("Refused connection from {} due to incompatible RakNet protocol version (version {})", address, offline_message.protocol);
		} else {
			//IP header size (20 bytes) + UDP header size (8 bytes)
			self.server.send_packet(&OpenConnectionReply1::create(
				self.server.id, false, offline_message.mtu_size + 28
			), address);
		}
	}
}

impl Handle<OpenConnectionRequest2> for UnconnectedMessageHandler<'_> {
	fn handle(&mut self, offline_message: &OpenConnectionRequest2, address: &SocketAddr) {
		if offline_message.server_address.port() == self.server.get_port() || !self.server.port_checking {
			if (offline_message.mtu_size as usize) < SessionMutable::MIN_MTU_SIZE {
				debug!("Not creating session for {} due to bad MTU size {}", address, offline_message.mtu_size);
				return;
			}
			let mtu_size = min(offline_message.mtu_size, self.server.max_mtu_size as u16);
			self.server.send_packet(&OpenConnectionReply2::create(
					self.server.id,
					address.to_owned(),
					mtu_size,
					false
			), address);
		}
	}
}

