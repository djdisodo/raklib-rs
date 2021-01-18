use std::net::{UdpSocket, SocketAddr, IpAddr};
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};
use regex::bytes::Regex;
use crate::server::ipc::{UserToRaknetMessageReceiver, UserToRaknetMessage};
use crate::server::{ServerEventListener, ProtocolAcceptor, Session, ServerInterface};
use std::io::ErrorKind;
use crate::protocol::{Datagram, EncodePacket, DecodeBody, NACK, ACK, EncapsulatedPacket, MessageIdentifiers, DecodePacket, PacketImpl, UnconnectedPong};
use parking_lot::{Mutex};
use crate::server::unconnected_message_handler::UnconnectedMessageHandler;
use std::ops::{Deref, RangeFrom};
use crate::RaknetTime;
use log::{debug, info};
use std::sync::{Arc};
use crate::server::session::SessionExport;
use std::convert::TryFrom;
use blockingqueue::BlockingQueue;
use std::sync::atomic::AtomicBool;
use std::thread::sleep;
use std::fmt::Debug;


pub struct Server<'a> {
	pub internal: Mutex<ServerInternal<'a>>,
	pub export: Arc<ServerExport<'a>>
}

impl<'a> Deref for Server<'a> {
	type Target = ServerExport<'a>;

	fn deref(&self) -> &Self::Target {
		self.export.deref()
	}
}

impl<'a> Server<'a> {

	const RAKLIB_TPS: u32 = 100;
	const RAKLIB_TIME_PER_TICK: Duration = Duration::from_millis(10);

	pub fn new(
		server_id: u64,
		udp_socket: UdpSocket,
		max_mtu_size: usize,
		protocol_acceptor: impl ProtocolAcceptor + 'a,
		event_source: UserToRaknetMessageReceiver,
		event_listener: impl ServerEventListener + 'a,
	) -> Self {
		let immutable = Arc::new(ServerExport::new(
			server_id,
			udp_socket,
			max_mtu_size,
			protocol_acceptor,
			event_listener,
			event_source
		));
		Self {
			internal: Mutex::new(ServerInternal::new(
				immutable.clone()
			)),
			export: immutable
		}
	}

	pub fn run(&self) {
		self.udp_socket.set_nonblocking(false);
		let stop = Arc::new(Mutex::new(()));
		let stop_c = stop.clone();
		let thread = unsafe { std::thread::Builder::new().spawn_unchecked(move || while !stop_c.is_locked() {
			self.receive_packet();
		}).unwrap() };
		while {
			true // TODO !self.mutable.lock().shutdown && self.mutable.lock().sessions.len() > 0
		} {
			for _ in 0..Self::RAKLIB_TPS {
				self.tick_processor();
			}
		}
		let lock = stop.lock();
		self.udp_socket.set_nonblocking(true);
		thread.join().unwrap()
	}

	fn tick_processor(&self) {
		let start = Instant::now();
		{
			let mut mutable = self.internal.lock();
			while let Some(message) = self.event_source.receive() {
				if mutable.shutdown {
					break;
				}
				mutable.handle_message(message)
			}

			mutable.tick()
		}
		let elapsed = start.elapsed();
		if elapsed < Self::RAKLIB_TIME_PER_TICK {
			sleep(Self::RAKLIB_TIME_PER_TICK - elapsed);
		}
	}

	fn receive_packet(&self) -> bool {
		let mut buffer = self.buffer.lock();
		return match self.udp_socket.recv_from(&mut buffer) {
			Err(e) => {
				match e.kind() {
					ErrorKind::ConnectionReset => true,
					ErrorKind::WouldBlock | ErrorKind::TimedOut => false,
					_ => {
						debug!("{:?}", e);
						false
					}
				}
			}
			Ok((read, address)) => {
				self.internal.lock().receive_packet(address, &buffer.as_slice()[..read]);
				true
			}
		}
	}
}

pub struct ServerInternal<'a> { // TODO move out values not being wrote while running

	export: Arc<ServerExport<'a>>,

	receive_bytes: usize,

	sessions: Vec<Option<Session<'a>>>,
	session_ids_by_address: HashMap<SocketAddr, usize/* index in sessions */>,

	pub name: String,

	pub packet_per_tick_limit: usize, //default 200

	shutdown: bool,

	ticks: u32, //default 0
	
	block: HashMap<IpAddr, Instant>,
	ip_sec: HashMap<IpAddr, usize>,

	raw_packet_filters: Vec<Regex>,

	reusable_address: SocketAddr,

	reusable_session_ids: VecDeque<usize>,

	//trace_cleaner
}

impl<'a> Deref for ServerInternal<'a> {
	type Target = ServerExport<'a>;

	fn deref(&self) -> &Self::Target {
		self.export.deref()
	}
}

impl<'a> ServerInternal<'a> {
	fn new(
		immutable: Arc<ServerExport<'a>>
	) -> Self {
		let reusable_address = immutable.udp_socket.local_addr().unwrap();
		let max_mtu_size = immutable.max_mtu_size;
		Self {
			export: immutable,
			receive_bytes: 0,
			session_ids_by_address: HashMap::new(),
			sessions: Vec::new(),
			name: "".to_string(),
			packet_per_tick_limit: 200,
			shutdown: false,
			ticks: 0,
			block: HashMap::new(),
			ip_sec: HashMap::new(),
			raw_packet_filters: Vec::new(),
			reusable_address,
			reusable_session_ids: VecDeque::new(),
		}
	}


	fn receive_packet(&mut self, address: SocketAddr, mut buffer: &[u8]) {
		if buffer.len() == 0 {
			return;
		}
		debug!("recv: {} from {}", if let Ok(id) = MessageIdentifiers::try_from(buffer[0]) {
			format!("{:?}", id)
		} else {
			format!("{:#04x}", buffer[0])
		}, address);
		match self.session_ids_by_address.get(&address).cloned() {
			Some(session_id) => {
				let session = self.sessions[session_id].as_ref().unwrap();
				let header = buffer[0];
				if (header & Datagram::FLAG_VALID) != 0 {
					if (header & Datagram::FLAG_ACK) != 0 {
						session.get_mut().handle_ack(ACK::decode_packet(&mut buffer));
					} else if (header & Datagram::FLAG_NAK) != 0 {
						session.get_mut().handle_nack(NACK::decode_packet(&mut buffer));
					} else {
						session.get_mut().handle_datagram(Datagram::decode_packet(&mut buffer));
					}
				} else {
					debug!("Ignored unconnected packet from {} due to session already opened ({})", address, if let Ok(id) = MessageIdentifiers::try_from(header) {
						format!("{:?}", id)
					} else {
						format!("{:#04x}", buffer[0])
					})
				}
			},
			None => if !self.shutdown {
				let mut handled = self.handle_raw(&address, buffer);
				if !handled {
					for x in &self.raw_packet_filters {
						if x.find(&buffer).is_some() {
							handled = true;
							self.export.event_listener.lock().on_raw_packet_receive(address, &buffer);
							break;
						}
					}
				}

				if !handled {
					debug!(
						"Ignored packet from $address due to no session opened ({})",
						if let Ok(id) = MessageIdentifiers::try_from(buffer[0]) {
							format!("{:#04x}, {:?}", buffer[0], id)
						} else {
							format!("{:#04x}", buffer[0])
						}
					);
				}
			}
		}
		//TODO catch bad packet
	}

	pub fn get_session_by_address(&self, address: &SocketAddr) -> Option<&Session<'a>> {
		self.session_ids_by_address.get(address).map(| x | self.sessions.get(*x).unwrap().as_ref().unwrap())
	}

	pub fn create_session(&mut self, address: SocketAddr, client_id: u64, mtu_size: usize) {
		self.check_sessions();
		let session_id = self.reusable_session_ids.pop_front().unwrap_or_else(|| {
			let id = self.sessions.len();
			self.sessions.push(None);
			id
		});

		self.sessions[session_id] = Some(Session::new(self.export.clone(), address.clone(), client_id, mtu_size, session_id));
		self.session_ids_by_address.insert(address, session_id);
		debug!("Created session for {} with MTU size {}", address, mtu_size);
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

	fn tick(&mut self) {
		let time = Instant::now();

		let mut to_remove = Vec::new();
		for x in &mut self.sessions {
			if let Some(session) = x {
				let mut session = session.get_mut();
				session.update(time);
				if session.is_fully_disconnected() {
					to_remove.push(session.internal_id);
				}
			}
		}
		for x in to_remove {
			self.remove_session_internal(x);
		}

		self.ip_sec.clear();

		if !self.shutdown && self.ticks == Server::RAKLIB_TPS {
			self.ticks = 0;
			{
				let mut send_bytes = self.export.send_bytes.lock();
				if *send_bytes > 0 || self.receive_bytes > 0 {
					self.export.event_listener.lock().on_bandwidth_stats_update(*send_bytes, self.receive_bytes);
					*send_bytes = 0;
					self.receive_bytes = 0;
				}
			}

			if !self.block.is_empty() {
				let now = Instant::now();
				let mut to_remove = Vec::new();
				for (address, instant) in &self.block {
					if *instant < now {
						to_remove.push(address.clone());
					}
				}
				for x in to_remove {
					self.unblock_address(&x);
				}
			}
		}
		self.ticks += 1;
	}

}

impl ServerInterface for ServerInternal<'_> {
	fn send_encapsulated(&mut self, session_id: usize, packet: EncapsulatedPacket, immediate: bool) {
		if let Some(Some(session)) = self.sessions.get_mut(session_id) {
			session.get_mut().add_encapsulated_to_queue(packet, immediate);
		}
	}

	fn send_raw(&mut self, address: &SocketAddr, payload: &[u8]) {
		if let Err(e) = self.udp_socket.send_to(payload, address) {
			debug!("{:?}", e);
		}
	}

	fn close_session(&mut self, session_id: usize) {
		if let Some(Some(session)) = &mut self.sessions.get_mut(session_id) {
			session.get_mut().initiate_disconnect("server disconnect");
		}
	}

	fn set_name(&mut self, name: String) {
		self.name = name;
	}

	fn set_port_check(&mut self, value: bool) {
		self.set_port_checking(value);
	}

	fn set_packet_per_tick_limit(&mut self, limit: usize) {
		self.packet_per_tick_limit = limit;
	}

	fn block_address(&mut self, address: IpAddr, timeout: Duration) {
		let fin = Instant::now() + timeout;
		if self.block.get_mut(&address).is_none() {
			info!("Blocked {} for {:?}", address, timeout);
		}
		self.block.insert(address, fin);
	}

	fn unblock_address(&mut self, address: &IpAddr) {
		self.block.remove(address);
		debug!("Unblocked {}", address);
	}

	fn add_raw_packet_filter(&mut self, regex: Regex) {
		self.raw_packet_filters.push(regex);
	}
}

pub struct ServerExport<'a> {

	pub max_mtu_size: usize,

	pub id: u64,

	pub udp_socket: UdpSocket,

	pub send_bytes: Mutex<usize>,

	buffer: Mutex<Vec<u8>>,

	send_buffer: Mutex<Vec<u8>>,

	pub start_time: Instant,

	port_checking: Mutex<bool>, //default false

	pub(super) protocol_acceptor: Box<dyn ProtocolAcceptor + 'a>,

	pub event_listener: Mutex<Box<dyn ServerEventListener + 'a>>,

	event_source: UserToRaknetMessageReceiver,

}

impl<'a> ServerExport<'a> {
	pub fn new(
		server_id: u64,
		udp_socket: UdpSocket,
		max_mtu_size: usize,
		protocol_acceptor: impl ProtocolAcceptor + 'a,
		event_listener: impl ServerEventListener + 'a,
		event_source: UserToRaknetMessageReceiver
	) -> Self {
		Self {
			max_mtu_size,
			id: server_id,
			udp_socket,
			send_bytes: Mutex::new(0),
			buffer: Mutex::new(vec![0; max_mtu_size]),
			send_buffer: Mutex::new(vec![]),
			port_checking: Mutex::new(false),
			start_time: Instant::now(),
			protocol_acceptor: Box::new(protocol_acceptor) as Box<_>,
			event_listener: Mutex::new(Box::new(event_listener)),
			event_source
		}
	}

	pub fn get_raknet_time(&self) -> RaknetTime {
		self.start_time.elapsed()
	}

	pub fn open_session(&self, session: &SessionExport) {
		self.event_listener.lock().on_client_connect(
			session.internal_id,
			session.address.clone(),
			session.client_id
		);
	}

	pub fn send_packet(&self, packet: &impl PacketImpl, address: &SocketAddr) {
		let mut buffer = self.send_buffer.lock();
		buffer.clear();
		packet.encode_packet(&mut *buffer);
		debug!("send: {} to {}", if let Ok(id) = MessageIdentifiers::try_from(buffer[0]) {
			format!("{:?}", id)
		} else {
			format!("{:#04x}", buffer[0])
		}, address);
		match self.udp_socket.send_to(&*buffer, address) {
			Ok(send) => *self.send_bytes.lock() += send,
			Err(e) => debug!("{}", e)
		}
	}

	pub fn get_port(&self) -> u16 {
		self.udp_socket.local_addr().unwrap().port()
	}

	pub fn get_port_checking(&self) -> bool {
		*self.port_checking.lock()
	}

	pub fn set_port_checking(&self, port_checking: bool) {
		*self.port_checking.lock() = port_checking;
	}
}