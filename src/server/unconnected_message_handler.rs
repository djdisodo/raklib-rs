use std::net::SocketAddr;
use crate::protocol::{MessageIdentifierHeader, OfflineMessageImpl, UnconnectedPing, OpenConnectionRequest1, OpenConnectionRequest2, UnconnectedPingOpenConnections, UnconnectedPong, IncompatibleProtocolVersion, OpenConnectionReply1, OpenConnectionReply2, DecodePacket, PacketImpl};
use log::{info, debug};
use crate::server::SessionInternal;
use std::cmp::min;
use std::convert::TryInto;


use crate::server::ServerInternal;
use bytes::Buf;
use std::ops::Deref;

pub(super) trait UnconnectedMessageHandler {
	fn handle_raw(&mut self, address: &SocketAddr, raw: &[u8]) -> bool;
}

fn get_packet(buffer: &mut &[u8]) -> Option<Box<dyn OfflineMessageImpl>> {
	if let Ok(id) = buffer[0].try_into() {
		Some(match id {
			UnconnectedPing::ID => Box::new(UnconnectedPing::decode_packet(buffer)),
			UnconnectedPingOpenConnections::ID => Box::new(UnconnectedPong::decode_packet(buffer)),
			OpenConnectionRequest1::ID => Box::new(OpenConnectionRequest1::decode_packet(buffer)),
			OpenConnectionRequest2::ID => Box::new(OpenConnectionRequest2::decode_packet(buffer)),
			UnconnectedPingOpenConnections::ID => Box::new(UnconnectedPingOpenConnections::decode_packet(buffer)),
			_ => return None
		})
	} else {
		None
	}
}

impl UnconnectedMessageHandler for ServerInternal<'_> {

	fn handle_raw(&mut self, address: &SocketAddr, mut raw: &[u8]) -> bool{
		if raw.is_empty() {
			return false;
		}

		let offline_message = get_packet(&mut raw);
		if offline_message.is_none() {
			return false;
		}
		let mut offline_message = offline_message.unwrap();

		if !offline_message.is_valid() {
			return false;
		}

		if raw.has_remaining() {
			let remains = raw.len();
			debug!("Still {} bytes unread in {:?} from {}", remains, offline_message, address);
		}

		if let Some(offline_message) = offline_message.as_any().downcast_ref::<UnconnectedPing>() {
			self.send_packet(&UnconnectedPong {
				offline_message: Default::default(),
				send_ping_time: offline_message.send_ping_time,
				server_id: self.id,
				server_name: self.name.to_owned()
			}, address)
		} else if let Some(offline_message) = offline_message.as_any().downcast_ref::<UnconnectedPingOpenConnections>() {
			self.send_packet(&UnconnectedPong {
				offline_message: Default::default(),
				send_ping_time: offline_message.send_ping_time,
				server_id: self.id,
				server_name: self.name.to_owned()
			}, address)
		} else if let Some(offline_message) = offline_message.as_any().downcast_ref::<OpenConnectionRequest1>() {
			if !self.protocol_acceptor.accepts(offline_message.protocol) {
				self.send_packet(&IncompatibleProtocolVersion::create(
					self.protocol_acceptor.get_primary_version(),
					self.id
				), address);
				info!("Refused connection from {} due to incompatible RakNet protocol version (version {})", address, offline_message.protocol);
			} else {
				//IP header size (20 bytes) + UDP header size (8 bytes)
				self.send_packet(&OpenConnectionReply1::create(
					self.id, false, offline_message.mtu_size + 28
				), address);
			}
		} else if let Some(offline_message) = offline_message.as_any().downcast_ref::<OpenConnectionRequest2>() {
			if offline_message.server_address.port() == self.get_port() || !self.get_port_checking() {
				if (offline_message.mtu_size as usize) < SessionInternal::MIN_MTU_SIZE {
					debug!("Not creating session for {} due to bad MTU size {}", address, offline_message.mtu_size);
					return false;
				}
				let mtu_size = min(offline_message.mtu_size, self.max_mtu_size as u16);
				self.send_packet(&OpenConnectionReply2::create(
					self.id,
					address.to_owned(),
					mtu_size,
					false
				), address);
				self.create_session(address.clone(), offline_message.client_id, mtu_size as usize);
			}
		} else {
			panic!("invalid packet");
		}
		//TODO error handle (buffer underflow)


		true
	}
}