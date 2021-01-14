use crate::protocol::{Datagram, EncodePacket, PacketReliability, EncapsulatedPacket, EncodeBody, ConnectedPing, ACK, NACK, PacketImpl, MessageIdentifiers, ConnectionRequest, MessageIdentifierHeader, DecodePacket, ConnectionRequestAccepted, NewIncomingConnection, DisconnectionNotification, ConnectedPong};
use crate::server::Server;
use std::net::SocketAddr;
use std::time::{SystemTime, Duration, Instant};
use crate::generic::{ReceiveReliabilityLayer, SendReliabilityLayer};
use log::{debug, warn};
use std::convert::TryFrom;
use crate::server::session::SessionState::Disconnecting;
use crate::RaknetTime;

pub struct Session<'a> {
	//TODO
	server: &'a Server<'a>,

	address: SocketAddr,

	state: SessionState, //default SessionState::Connecting

	id: u64,

	last_update: Instant,

	is_temporal: bool, //default true

	is_active: bool, //default false

	last_ping_time: Instant, //default -1

	last_ping_measure: Duration, //default 1

	internal_id: usize,

	recv_layer: ReceiveReliabilityLayer<'a>,

	send_layer: SendReliabilityLayer<'a>
}

impl<'a> Session<'a> {

	pub const MAX_SPLIT_PART_COUNT: usize = 128;
	pub const MAX_CONCURRENT_SPLIT_COUNT: usize = 4;

	pub const MIN_MTU_SIZE: usize = 400;

	pub fn new(server: &'a Server<'a>, address: SocketAddr, client_id: u64, mtu_size: usize, internal_id: usize) -> Self {
		if mtu_size < Self::MIN_MTU_SIZE {
			panic!("MTU size must be at least {}, got {}", Self::MIN_MTU_SIZE, mtu_size);
		}
		Self {
			server,
			address,
			state: SessionState::Connecting,
			id: client_id,

			last_update: Instant::now(),

			is_temporal: false,
			is_active: false,
			last_ping_time: Instant::now() - SystemTime::UNIX_EPOCH.elapsed().unwrap(),
			last_ping_measure: Default::default(),
			internal_id,

			recv_layer: ReceiveReliabilityLayer::with_split_limit(
				| _ | {}, //TODO
				| _ | {}, //TODO
				Self::MAX_SPLIT_PART_COUNT,
				Self::MAX_CONCURRENT_SPLIT_COUNT
			),

			send_layer: SendReliabilityLayer::new(
				mtu_size,
				| _ | {}, //TODO
				| _ | {}, //TODO
			)
		}
	}

	pub fn get_internal_id(&self) -> usize {
		self.internal_id
	}

	pub fn get_address(&self) -> &SocketAddr {
		&self.address
	}

	pub fn get_id(&self) -> u64 {
		self.id
	}

	pub fn get_state(&self) -> SessionState {
		self.state
	}

	pub fn is_temporal(&self) -> bool {
		self.is_temporal
	}

	pub fn is_connected(&self) -> bool {
		match self.state {
			SessionState::Connected | SessionState::Connecting => true,
			_ => false
		}
	}

	pub fn update(&mut self, time: Instant) {
		let timeout = Duration::from_secs(10);
		if !self.is_active && time.duration_since(self.last_update)> timeout {
			self.forcibly_disconnect("timeout");

			return;
		}

		if let SessionState::Disconnecting { disconnection_time } = self.state {
			//by this point we already told the event listener that the session is closing, so we don't need to do it again
			if !self.recv_layer.needs_update() && !self.send_layer.needs_update() {
				self.state = SessionState::Disconnected {
					disconnection_time
				};
				debug!("Client cleanly disconnected, marking session for destruction");
				return;
			} else if time.duration_since(disconnection_time) > timeout {
				self.state = SessionState::Disconnected {
					disconnection_time
				};
				debug!("Timeout during graceful disconnect, forcibly closing session");
				return;
			}
		}

		self.is_active = false;

		self.recv_layer.update();
		self.send_layer.update();

		if time.duration_since(self.last_ping_time) < Duration::from_secs(5) {
			self.send_ping();
			self.last_ping_time = Instant::now();
		}
	}

	fn queue_connected_packet(
		&mut self,
		packet: &impl PacketImpl,
		reliability: PacketReliability,
		order_channel: u8,
		immediate: bool // default false
	) {
		let mut encapsulated = EncapsulatedPacket::default();
		encapsulated.reliability = reliability;
		encapsulated.order_channel = Some(order_channel);
		packet.encode_packet(&mut encapsulated.buffer);

		self.send_layer.add_encapsulated_to_queue(encapsulated, immediate);
	}

	pub fn add_encapsulated_to_queue(&mut self, encapsulated: EncapsulatedPacket, immediate: bool) {
		self.send_layer.add_encapsulated_to_queue(encapsulated, immediate);
	}

	fn send_packet(&mut self, packet: &impl EncodePacket) {
		self.server.send_packet(packet, &self.address);
	}

	fn send_ping(&mut self) {
		self.send_ping_with_reliability(PacketReliability::Unreliable)
	}

	fn handle_encapsulated_packet_route(&mut self, packet: &mut EncapsulatedPacket) {

		let id = packet.buffer[0];
		let mut buffer: &[u8] = &packet.buffer;
		if id < MessageIdentifiers::UserPacketEnum as u8{ //internal data packet
			let id = MessageIdentifiers::try_from(id).unwrap();
			if self.state == SessionState::Connecting {
				match id {
					ConnectionRequest::ID => {
						let data_packet = ConnectionRequest::decode_packet(&mut buffer);
						self.queue_connected_packet(&ConnectionRequestAccepted::create(
							self.address.clone(),
							vec![],
							data_packet.send_ping_time,
							self.server.get_raknet_time()
						), PacketReliability::Unreliable, 0, true);
					},
					NewIncomingConnection::ID => {
						let data_packet = NewIncomingConnection::decode_packet(&mut buffer);
						if data_packet.address.port() == self.server.get_port() || self.server.get_port_checking() {
							self.state = SessionState::Connected; //FINALLY!
							self.is_temporal = false;
							self.server.open_session(self);

							//self.handle.pong(data_packet.send_ping_time, data_packet_send_pont_time); //can't use this due to system-address count issues in MCPE >.<
							self.send_ping();
						}
					},
					_ => {}
				};
			} else {
				match id {
					DisconnectionNotification::ID => {
						self.initiate_disconnect("client disconnect");
					},
					ConnectedPing::ID => {
						let data_packet = ConnectedPing::decode_packet(&mut buffer);
						self.queue_connected_packet(&ConnectedPong {
							send_ping_time: data_packet.send_ping_time,
							send_pong_time: self.server.get_raknet_time()
						}, PacketReliability::Unreliable, 0, false);
					},
					ConnectedPong::ID => {
						let data_packet = ConnectedPong::decode_packet(&mut buffer);
						self.handle_pong(data_packet.send_ping_time, data_packet.send_pong_time);
					},
					_ => {}
				}
			}
		} else if self.state == SessionState::Connected {
			self.server.get_event_listener().write().on_packet_receive(self.internal_id, &packet.buffer)
		} else {
			//warn!("Received packet before connection: {:#04x}", packet.buffer);
		}
	}

	//TODO: clock differential stuff
	fn handle_pong(&mut self, send_ping_time: RaknetTime, send_pong_time: RaknetTime) {
		self.last_ping_measure = self.server.get_raknet_time() - send_ping_time;
		self.server.get_event_listener().write().on_ping_measure(self.internal_id, self.last_ping_measure);
	}

	fn send_ping_with_reliability(&mut self, reliability: PacketReliability) {
		self.queue_connected_packet(&ConnectedPing {
			send_ping_time: self.server.get_raknet_time()
		}, reliability, 0, true);
	}

	pub fn handle_datagram(&mut self, mut datagram: Datagram) {
		self.is_active = true;
		self.last_update = Instant::now();
		self.recv_layer.on_datagram(&mut datagram);
	}

	pub fn handle_ack(&mut self, mut ack: ACK) {
		self.is_active = true;
		self.last_update = Instant::now();
		self.send_layer.on_ack(&ack);
	}

	pub fn handle_nack(&mut self, nack: NACK) {
		self.is_active = true;
		self.last_update = Instant::now();
		self.send_layer.on_nack(&nack);
	}

	pub fn initiate_disconnect(&mut self, reason: &str) {
		if self.is_connected() {
			self.state = Disconnecting {
				disconnection_time: Instant::now()
			};
			self.queue_connected_packet(&DisconnectionNotification::default(), PacketReliability::ReliableOrdered, 0, true);
			self.server.get_event_listener().write().on_client_disconnect(self.internal_id, reason);
			debug!("Requesting graceful disconnect because \"{}\"", reason)
		}
	}

	pub fn forcibly_disconnect(&mut self, reason: &str) {
		unimplemented!()
	}
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum SessionState {
	Connecting,
	Connected,
	Disconnecting {
		disconnection_time: Instant
	},
	Disconnected {
		disconnection_time: Instant
	}
}