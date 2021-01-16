use std::net::{UdpSocket, SocketAddr, IpAddr};
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};
use regex::bytes::Regex;
use crate::server::ipc::UserToRaknetMessageReceiver;
use crate::server::{ServerEventListener, ProtocolAcceptor, ServerEvent, Session};
use std::io::ErrorKind;
use crate::protocol::{Datagram, EncodePacket, DecodeBody, EncodeBody, NACK, ACK, EncapsulatedPacket};
use parking_lot::{Mutex, MutexGuard};
use crate::server::unconnected_message_handler::UnconnectedMessageHandler;
use std::ops::{Deref, DerefMut, Range, RangeFrom};
use crate::RaknetTime;
use log::{debug, info};
use std::sync::{Arc, Weak};
use crate::server::session::SessionImmutable;
use std::cell::RefCell;

pub struct Server<'a> {
	mutable: Mutex<ServerMutable<'a>>,
	immutable: Arc<ServerImmutable<'a>>
}

impl<'a> Deref for Server<'a> {
	type Target = ServerImmutable<'a>;

	fn deref(&self) -> &Self::Target {
		self.immutable.deref()
	}
}

impl<'a> Server<'a> {
	pub fn new(
		server_id: u64,
		udp_socket: UdpSocket,
		max_mtu_size: usize,
		protocol_acceptor: impl ProtocolAcceptor + 'a,
		message_receiver: UserToRaknetMessageReceiver,
		event_listener: impl ServerEventListener + 'a,
	) -> Self {
		let immutable = Arc::new(ServerImmutable::new(
			server_id,
			udp_socket,
			max_mtu_size,
			protocol_acceptor,
			event_listener
		));
		Self {
			mutable: Mutex::new(ServerMutable::new(
				immutable.clone(),
				message_receiver,
			)),
			immutable
		}
	}

	pub fn create_session(&self, address: SocketAddr, client_id: usize, mtu_size: u16) {
		//TODO self.check_sessions
		//let next_session_id = self.next_session_id.pop_back().unwrap_or_else(|| {
		//self.sessions.len().
		//});
	}

	pub fn tick_processor(&self) {
		/*while !self.shutdown && self.sessions.len() > 0 {
			for _ in 0..100 {
				let message = self.message_receiver.receive();
				if self.shutdown || message.is_none() {
					break;
				} else {
					//self.handle_message(message.unwrap());
				}
			}
		}*/
	}
}

pub struct ServerMutable<'a> { // TODO move out values not being wrote while running

	immutable: Arc<ServerImmutable<'a>>,

	receive_bytes: usize,
	send_bytes: usize,
	buffer: Vec<u8>,

	sessions: Vec<Option<Session<'a>>>,
	session_ids_by_address: HashMap<SocketAddr, usize/* index in sessions */>,

	pub name: String,

	pub packet_per_tick_limit: usize, //default 200

	shutdown: bool,

	ticks: u8, //default 0
	
	block: HashMap<IpAddr, Instant>,
	ip_sec: HashMap<IpAddr, usize>,

	raw_packet_filters: Vec<Regex>,

	reusable_address: SocketAddr,

	next_session_id: RangeFrom<usize>,

	reusable_session_ids: VecDeque<usize>,

	message_receiver: UserToRaknetMessageReceiver, // event_source

	//trace_cleaner
}

impl<'a> Deref for ServerMutable<'a> {
	type Target = ServerImmutable<'a>;

	fn deref(&self) -> &Self::Target {
		self.immutable.deref()
	}
}

impl<'a> ServerMutable<'a> {
	fn new(
		immutable: Arc<ServerImmutable<'a>>,
		message_receiver: UserToRaknetMessageReceiver,
	) -> Self {
		let reusable_address = immutable.udp_socket.local_addr().unwrap();
		let max_mtu_size = immutable.max_mtu_size;
		Self {
			immutable,
			receive_bytes: 0,
			send_bytes: 0,
			buffer: Vec::with_capacity(max_mtu_size),
			session_ids_by_address: HashMap::new(),
			sessions: Vec::new(),
			name: "".to_string(),
			packet_per_tick_limit: 200,
			shutdown: false,
			ticks: 0,
			block: HashMap::new(),
			ip_sec: HashMap::new(),
			raw_packet_filters: Vec::new(),
			next_session_id: 0..,
			reusable_address,
			reusable_session_ids: VecDeque::new(),
			message_receiver
		}
	}

	pub fn get_unconnected_message_handler(server: &'a mut ServerMutable<'a>) -> UnconnectedMessageHandler<'a> {
		UnconnectedMessageHandler {
			server
		}
	}


	fn receive_packet(&mut self) -> bool {

		if self.buffer.capacity() != self.max_mtu_size as usize {
			self.buffer = Vec::with_capacity(self.max_mtu_size as usize);
		}

		let address = match self.immutable.udp_socket.recv_from(&mut self.buffer) {
			Ok(t) => {
				self.receive_bytes += t.0;
				t.1
			},
			Err(e) => return match e.kind() {
				ErrorKind::ConnectionReset => true,
				ErrorKind::WouldBlock => false,
				_ => {
					debug!("{}", e);
					false
				}
			}
		};

		match self.session_ids_by_address.get(&address).cloned() {
			Some(session_id) => {
				let session = self.sessions[session_id].as_ref().unwrap();
				let header = self.buffer[0];
				if (header & Datagram::FLAG_VALID) != 0 {
					if (header & Datagram::FLAG_ACK) != 0 {
						session.get_mut().handle_ack(ACK::decode_body(&mut self.buffer.as_slice()));
					} else if (header & Datagram::FLAG_NAK) != 0 {
						session.get_mut().handle_nack(NACK::decode_body(&mut self.buffer.as_slice()));
					} else {
						//TODO session.handle_datagram(Datagram::decode_body(&mut self.buffer.as_slice()));
					}
				} else {
					debug!("Ignored unconnected packet from $address due to session already opened (0x{})", format!("{:X}", header))
				}
			},
			None => if !self.shutdown {
				//let mut handled = self.unconnected_message_handler.h
			}
		}
		true
	}

	pub fn send_packet(&mut self, packet: &impl EncodePacket, address: &SocketAddr) {
		self.buffer.clear();
		packet.encode_packet(&mut self.buffer);
		match self.udp_socket.send_to(&self.buffer, address) {
			Ok(send) => self.send_bytes += send,
			Err(e) => debug!("{}", e)
		}
	}

	pub fn send_packet_to_session(&mut self, packet: &impl EncodePacket, session_id: usize) {
		if let Some(Some(session)) = self.sessions.get(session_id) {
			let address = session.address.clone();
			self.send_packet(packet, &address);
		}
	}

	pub fn send_encapsulated(&mut self, session_id: usize, packet: EncapsulatedPacket, immediate: bool) {
		if let Some(Some(session)) = self.sessions.get_mut(session_id) {
			session.get_mut().add_encapsulated_to_queue(packet, immediate);
		}
	}

	pub fn send_raw(&mut self, address: &SocketAddr, payload: &[u8]) {
		if let Err(e) = self.udp_socket.send_to(payload, address) {
			debug!("{:?}", e);
		}
	}

	pub fn close_session(&mut self, session_id: usize) {
		if let Some(Some(session)) = &mut self.sessions.get_mut(session_id) {
			session.get_mut().initiate_disconnect("server disconnect");
		}
	}

	pub fn block_address(&mut self, address: IpAddr, timeout: Duration) {
		let fin = Instant::now() + timeout;
		if self.block.get_mut(&address).is_none() {
			info!("Blocked {} for {:?}", address, timeout);
		}
		self.block.insert(address, fin);
	}

	pub fn unblock_address(&mut self, address: &IpAddr) {
		self.block.remove(address);
		debug!("Unblocked {}", address);
	}

	pub fn add_raw_packet_filter(&mut self, regex: Regex) {
		self.raw_packet_filters.push(regex);
	}

	pub fn get_session_by_address(&self, address: &SocketAddr) -> Option<&Session<'a>> {
		self.session_ids_by_address.get(address).map(| x | self.sessions.get(*x).unwrap().as_ref().unwrap())
	}

	fn remove_session_internal(&mut self, session_id: usize) {
		let session = self.sessions.remove(session_id).unwrap();
		self.session_ids_by_address.remove(&session.address);
		self.reusable_session_ids.push_back(session_id);
	}

	//gc or something
	fn check_sessions(&mut self) {
		let mut to_remove = Vec::new();

		if self.sessions.len() - self.reusable_session_ids.len() > 4096 {
			for x in &self.sessions {
				if let Some(session) = x {
					if session.get_mut().is_temporal() {
						to_remove.push(session.internal_id);
					}
				}
			}
		}

		for x in to_remove {
			self.remove_session_internal(x);
		}
	}

}

pub struct ServerImmutable<'a> {

	pub max_mtu_size: usize,

	pub id: u64,

	pub udp_socket: UdpSocket,

	pub start_time: Instant,

	pub port_checking: bool, //default false

	pub(super) protocol_acceptor: Box<dyn ProtocolAcceptor + 'a>,

	pub event_listener: Mutex<Box<dyn ServerEventListener + 'a>>

}

impl<'a> ServerImmutable<'a> {
	pub fn new(
		server_id: u64,
		udp_socket: UdpSocket,
		max_mtu_size: usize,
		protocol_acceptor: impl ProtocolAcceptor + 'a,
		event_listener: impl ServerEventListener + 'a,
	) -> Self {
		Self {
			max_mtu_size,
			id: server_id,
			udp_socket,
			port_checking: false,
			start_time: Instant::now(),
			protocol_acceptor: Box::new(protocol_acceptor) as Box<_>,
			event_listener: Mutex::new(Box::new(event_listener))
		}
	}

	pub fn get_raknet_time(&self) -> RaknetTime {
		self.start_time.elapsed()
	}

	pub fn open_session(&self, session: &SessionImmutable) {
		self.event_listener.lock().on_client_connect(
			session.internal_id,
			session.address.clone(),
			session.client_id
		);
	}

	pub fn get_port(&self) -> u16 {
		self.udp_socket.local_addr().unwrap().port()
	}
}