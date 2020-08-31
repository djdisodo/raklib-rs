use crate::server::{ProtocolAcceptor, Server, ServerEventListener};
use std::net::SocketAddr;
use crate::protocol::payload::{OfflineMessageImpl, UnconnectedPing, UnconnectedPong, UnconnectedPingOpenConnections, OpenConnectionRequest1, IncompatibleProtocolVersion, OpenConnectionReply1, OpenConnectionRequest2, OpenConnectionReply2};
use crate::protocol::{Packet, Payload, Decode};
use log::{info, debug};
use crate::server::session::Session;
use std::cmp::min;
use std::convert::TryInto;
use crate::server::server::ServerSocket;
use std::sync::RwLock;

pub struct UnconnectedMessageHandler<'a, T: ProtocolAcceptor> {
	pub(super) protocol_acceptor: T,
	pub(super) server_socket: &'a RwLock<ServerSocket>
}

impl<T: ProtocolAcceptor> UnconnectedMessageHandler<'_, T> {
	pub fn handle_raw(&mut self, mut payload: &[u8], address: SocketAddr) -> bool{
		if payload.is_empty() {
			return false;
		}
		//check header
		let id = payload[0].try_into();
		if id.is_err() {
			return false;
		}
		let packet: Packet<OM> = match id.unwrap() {
			UnconnectedPing::ID => Packet::<UnconnectedPing>::decode(&mut payload),
			OpenConnectionRequest1::ID => Packet::<OpenConnectionRequest1>::decode(&mut payload),
			OpenConnectionRequest2::ID => Packet::<OpenConnectionRequest2>::decode(&mut payload),
			UnconnectedPingOpenConnections::ID => return true,
			_ => return false
		};
		if !packet.is_valid() {
			return false;
		}
		if payload.has_remaining() {
			let remains = payload.len();
			debug!("Still {} bytes unread in {:?} from {}", remains, packet, address);
		}
		self.handle(packet, address);
		return true;
	}
}

pub trait Handle<T: OfflineMessageImpl> {
	fn handle<EL: ServerEventListener>(&mut self, packet: T, address: &SocketAddr);
}

impl<T: ProtocolAcceptor> Handle<UnconnectedPing> for UnconnectedMessageHandler<T> {
	fn handle<EL: ServerEventListener>(&mut self, packet: UnconnectedPing, address: &SocketAddr) {
		server.send_packet(Packet::wrap(
			UnconnectedPong {
				offline_message: Default::default(),
				send_ping_time: packet.send_ping_time,
				server_id: server.get_id(),
				server_name: server.get_name().to_owned()
			}
		), address)
	}
}

impl<T: ProtocolAcceptor> Handle<OpenConnectionRequest1> for UnconnectedMessageHandler<T> {
	fn handle<EL: ServerEventListener>(&mut self, packet: OpenConnectionRequest1, address: &SocketAddr) {
		if !self.protocol_acceptor.accepts(packet.protocol) {
			server.send_packet(Packet::wrap(IncompatibleProtocolVersion::create(
					self.protocol_acceptor.get_primary_version(),
					self.server_socket.read().
				)), address);
			info!("Refused connection from {} due to incompatible RakNet protocol version (version {})", address, packet.protocol);
		} else {
			//IP header size (20 bytes) + UDP header size (8 bytes)
			server.send_packet(Packet::wrap(OpenConnectionReply1::create(
				server.get_id(), false, packet.mtu_size + 28
			)), address);
		}
	}
}

impl<T: ProtocolAcceptor> Handle<OpenConnectionRequest2> for UnconnectedMessageHandler<T> {
	fn handle<EL: ServerEventListener>(&mut self, packet: OpenConnectionRequest2, address: &SocketAddr) {
		if packet.server_address.port() == server.get_port() || !server.port_checking {
			if packet.mtu_size < Session::MIN_MTU_SIZE {
				debug!("Not creating session for {} due to bad MTU size {}", address, packet.mtu_size);
				return;
			}
			let mtu_size = min(packet.mtu_size, server.get_max_mtu_size());
			server.send_packet(Packet::wrap(OpenConnectionReply2::create(
					server.get_id(),
					address.to_owned(),
					mtu_size,
					false
			)), address);
		}
	}
}

