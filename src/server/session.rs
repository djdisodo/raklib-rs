use crate::protocol::{Datagram, PacketReliability, EncapsulatedPacket, ConnectedPing, ACK, NACK, PacketImpl, MessageIdentifiers, ConnectionRequest, MessageIdentifierHeader, DecodePacket, ConnectionRequestAccepted, NewIncomingConnection, DisconnectionNotification, ConnectedPong, EncodePacket};

use std::net::SocketAddr;
use std::time::{SystemTime, Duration, Instant};
use crate::generic::{ReceiveReliabilityLayer, SendReliabilityLayer};
use log::{debug};
use std::convert::TryFrom;
use crate::server::session::SessionState::Disconnecting;
use crate::RaknetTime;
use parking_lot::{Mutex, MutexGuard};
use std::ops::Deref;

use std::sync::Arc;
use crate::server::server::ServerExport;

pub struct Session<'a> {
	internal: Mutex<SessionInternal<'a>>,
	export: Arc<SessionExport<'a>>
}

impl<'a> Deref for Session<'a> {
	type Target = Arc<SessionExport<'a>>;

	fn deref(&self) -> &Self::Target {
		&self.export
	}
}

impl<'a> Session<'a> {
	pub fn new(server: Arc<ServerExport<'a>>, address: SocketAddr, client_id: u64, mtu_size: usize, internal_id: usize) -> Self {
		let immutable = Arc::new(SessionExport::new(server, address, client_id, mtu_size, internal_id));
		Self {
			internal: Mutex::new(SessionInternal::new(mtu_size, immutable.clone())),
			export: immutable
		}
	}

	pub fn get_mut(&self) -> MutexGuard<'_, SessionInternal<'a>> {
		self.internal.lock()
	}
}

pub struct SessionInternal<'a> {

	export: Arc<SessionExport<'a>>,

	last_update: Instant,

	is_active: bool, //default false

	last_ping_time: Instant, //default -1


	recv_layer: ReceiveReliabilityLayer<'a>,
}

impl<'a> Deref for SessionInternal<'a> {
	type Target = SessionExport<'a>;

	fn deref(&self) -> &Self::Target {
		self.export.deref()
	}
}

impl<'a> SessionInternal<'a> {

	pub const MAX_SPLIT_PART_COUNT: usize = 128;
	pub const MAX_CONCURRENT_SPLIT_COUNT: usize = 4;

	pub const MIN_MTU_SIZE: usize = 400;

	fn new(mtu_size: usize, export: Arc<SessionExport<'a>>) -> Self {
		if mtu_size < Self::MIN_MTU_SIZE {
			panic!("MTU size must be at least {}, got {}", Self::MIN_MTU_SIZE, mtu_size);
		}
		let export_clone = export.clone();
		Self {
			export: export.clone(),

			last_update: Instant::now(),
			is_active: false,
			last_ping_time: Instant::now() - Duration::from_secs(6), // *never

			recv_layer: ReceiveReliabilityLayer::with_split_limit(
				move | pk | {
					export.handle_encapsulated_packet_route(pk);
				}, //TODO
				move | pk | {
					export_clone.send_packet(&pk);
				}, //TODO
				Self::MAX_SPLIT_PART_COUNT,
				Self::MAX_CONCURRENT_SPLIT_COUNT
			),
		}
	}

	pub fn update(&mut self, time: Instant) {
		let timeout = Duration::from_secs(10);
		if !self.is_active && time.duration_since(self.last_update) > timeout {
			self.forcibly_disconnect("timeout");

			return;
		}

		let mut send_layer = self.export.send_layer.lock();

		let mut state = self.export.state.lock();

		if let SessionState::Disconnecting { disconnection_time } = *state {
			//by this point we already told the event listener that the session is closing, so we don't need to do it again
			if !self.recv_layer.needs_update() && !send_layer.needs_update() {
				*state = SessionState::Disconnected {
					disconnection_time
				};
				debug!("Client cleanly disconnected, marking session for destruction");
				return;
			} else if time.duration_since(disconnection_time) > timeout {
				*state = SessionState::Disconnected {
					disconnection_time
				};
				debug!("Timeout during graceful disconnect, forcibly closing session");
				return;
			}
		}

		self.is_active = false;

		self.recv_layer.update();
		send_layer.update();

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

		self.send_layer.lock().add_encapsulated_to_queue(encapsulated, immediate);
	}

	pub fn add_encapsulated_to_queue(&mut self, encapsulated: EncapsulatedPacket, immediate: bool) {
		self.send_layer.lock().add_encapsulated_to_queue(encapsulated, immediate);
	}

	pub fn handle_datagram(&mut self, mut datagram: Datagram) {
		self.is_active = true;
		self.last_update = Instant::now();
		self.recv_layer.on_datagram(&mut datagram);
	}

	pub fn handle_ack(&mut self, ack: ACK) {
		self.is_active = true;
		self.last_update = Instant::now();
		self.send_layer.lock().on_ack(&ack);
	}

	pub fn handle_nack(&mut self, nack: NACK) {
		self.is_active = true;
		self.last_update = Instant::now();
		self.send_layer.lock().on_nack(&nack);
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

pub struct SessionExport<'a> {

	server: Arc<ServerExport<'a>>,

	pub client_id: u64,

	pub address: SocketAddr,

	pub internal_id: usize,

	state: Mutex<SessionState>, //default SessionState::Connecting

	last_ping_measure: Mutex<Duration>, //default 1

	is_temporal: Mutex<bool>, //default true

	send_layer: Mutex<SendReliabilityLayer<'a>>
}

impl<'a> SessionExport<'a> {
	pub fn new(server: Arc<ServerExport<'a>>, address: SocketAddr, client_id: u64, mtu_size: usize, internal_id: usize) -> Self {
		let server_clone = server.clone();
		Self {
			server: server.clone(),
			client_id,
			address: address.clone(),
			internal_id,
			is_temporal: Mutex::new(true),
			state: Mutex::new(SessionState::Connecting),
			last_ping_measure: Mutex::new(Default::default()),
			send_layer: Mutex::new(SendReliabilityLayer::new(
				mtu_size,
				move | datagram | {
					server.send_packet(datagram, &address)
				},
				move | identifier_ack | {
					server_clone.event_listener.lock().on_packet_ack(internal_id, identifier_ack)
				},
			)),
		}
	}

	fn send_ping_with_reliability(&self, reliability: PacketReliability) {
		self.send_layer.lock().queue_connected_packet(&ConnectedPing {
			send_ping_time: self.server.get_raknet_time()
		}, reliability, 0, true);
	}

	fn send_ping(&self) {
		self.send_ping_with_reliability(PacketReliability::Unreliable)
	}

	pub fn send_packet(&self, packet: &impl PacketImpl) {
		self.server.send_packet(packet, &self.address);
	}

	fn handle_encapsulated_packet_route(&self, packet: &mut EncapsulatedPacket) {
		let id = packet.buffer[0];
		let mut buffer: &[u8] = &packet.buffer;
		let state = self.state.lock().deref().clone();
		if id < MessageIdentifiers::UserPacketEnum as u8{ //internal data packet
			let id = MessageIdentifiers::try_from(id).unwrap();
			if state == SessionState::Connecting {
				match id {
					ConnectionRequest::ID => {
						let data_packet = ConnectionRequest::decode_packet(&mut buffer);
						self.send_layer.lock().queue_connected_packet(&ConnectionRequestAccepted::create(
							self.address.clone(),
							vec![],
							data_packet.send_ping_time,
							self.server.get_raknet_time()
						), PacketReliability::Unreliable, 0, true);
					},
					NewIncomingConnection::ID => {
						let data_packet = NewIncomingConnection::decode_packet(&mut buffer);
						if data_packet.address.port() == self.server.get_port() || self.server.get_port_checking() {
							*self.state.lock() = SessionState::Connected; //FINALLY!
							*self.is_temporal.lock() = false;
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
						self.send_layer.lock().queue_connected_packet(&ConnectedPong {
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
		} else if state == SessionState::Connected {
			self.server.event_listener.lock().on_packet_receive(self.internal_id, &packet.buffer)
		} else {
			//warn!("Received packet before connection: {:#04x}", packet.buffer);
		}
	}

	//TODO: clock differential stuff
	fn handle_pong(&self, send_ping_time: RaknetTime, _send_pong_time: RaknetTime) {
		let mut last_ping_measure = self.last_ping_measure.lock();
		*last_ping_measure = self.server.get_raknet_time() - send_ping_time;
		self.server.event_listener.lock().on_ping_measure(self.internal_id, *last_ping_measure);
	}

	pub fn is_temporal(&self) -> bool {
		*self.is_temporal.lock()
	}

	pub fn is_connected(&self) -> bool {
		match *self.state.lock() {
			SessionState::Connected | SessionState::Connecting => true,
			_ => false
		}
	}

	/**
	* Initiates a graceful asynchronous disconnect which ensures both parties got all packets.
	*/
	pub fn initiate_disconnect(&self, reason: &str) {
		if self.is_connected() {
			*self.state.lock() = Disconnecting {
				disconnection_time: Instant::now()
			};
			self.send_layer.lock().queue_connected_packet(&DisconnectionNotification::default(), PacketReliability::ReliableOrdered, 0, true);
			self.server.event_listener.lock().on_client_disconnect(self.internal_id, reason);
			debug!("Requesting graceful disconnect because \"{}\"", reason)
		}
	}

	/**
	 * Disconnects the session with immediate effect, regardless of current session state. Usually used in timeout cases.
	 */
	pub fn forcibly_disconnect(&self, reason: &str) {
		*self.state.lock() = SessionState::Disconnected {
			disconnection_time: Instant::now()
		};
		self.server.event_listener.lock().on_client_disconnect(self.internal_id, reason);
		debug!("Forcibly disconnecting session due to \"{}\"", reason);
	}


	/**
	 * Returns whether the session is ready to be destroyed (either properly cleaned up or forcibly terminated)
	 */
	pub fn is_fully_disconnected(&self) -> bool {
		if let SessionState::Disconnected { .. } = *self.state.lock() {
			true
		} else {
			false
		}
	}
	
}