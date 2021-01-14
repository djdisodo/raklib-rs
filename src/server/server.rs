use std::net::{UdpSocket, SocketAddr, IpAddr};
use std::collections::{HashMap, VecDeque};
use crate::server::session::Session;
use std::time::{Duration, Instant};
use regex::bytes::Regex;
use crate::server::ipc::UserToRaknetMessageReceiver;
use crate::server::{ServerEventListener, ProtocolAcceptor, ServerEvent};
use std::io::ErrorKind;
use log::debug;
use crate::protocol::{Datagram, EncodePacket, DecodeBody, EncodeBody, NACK, ACK};
use parking_lot::RwLock;
use std::rc::Rc;
use crate::server::unconnected_message_handler::UnconnectedMessageHandler;
use std::ops::{Deref, DerefMut};
use crate::RaknetTime;

struct _Server<'a> { // TODO move out values not being wrote while running

	udp_socket: UdpSocket,
	receive_bytes: usize,
	send_bytes: usize,
	buffer: Vec<u8>,
	max_mtu_size: usize,

	server_id: u64,

	sessions: Vec<Option<Session<'a>>>,
	sessions_by_address: HashMap<SocketAddr, Session<'a>/* index in sessions */>,

	name: String,

	packet_limit: usize, //default 200

	shutdown: bool,

	ticks: u8, //default 0
	
	block: HashMap<IpAddr, Duration>, // duration = unblock time; TODO it might be SystemTime
	ip_sec: HashMap<IpAddr, usize>,

	raw_packet_filters: Vec<Regex>,

	start_time: Option<Instant>,

	pub port_checking: bool, //default false

	reusable_address: SocketAddr,

	next_session_id: VecDeque<usize>,

	message_receiver: UserToRaknetMessageReceiver, // event_source

	//trace_cleaner
}

impl _Server<'_> {
	fn new(
		server_id: u64,
		udp_socket: UdpSocket,
		max_mtu_size: usize,
		message_receiver: UserToRaknetMessageReceiver,
	) -> Self {
		let reusable_address = udp_socket.local_addr().unwrap();
		Self {
			udp_socket,
			receive_bytes: 0,
			send_bytes: 0,
			buffer: Vec::with_capacity(max_mtu_size as usize),
			max_mtu_size,
			server_id,
			sessions_by_address: HashMap::new(),
			sessions: Vec::new(),
			name: "".to_string(),
			packet_limit: 200,
			shutdown: false,
			ticks: 0,
			block: HashMap::new(),
			ip_sec: HashMap::new(),
			raw_packet_filters: Vec::new(),
			port_checking: false,
			start_time: None,
			reusable_address,
			next_session_id: VecDeque::new(),
			message_receiver
		}
	}

	pub fn get_raknet_time(&self) -> RaknetTime {
		self.start_time.unwrap().elapsed()
	}

	pub fn get_port(&self) -> u16 {
		self.udp_socket.local_addr().unwrap().port()
	}

	pub fn get_max_mtu_size(&self) -> usize {
		self.max_mtu_size
	}

	pub fn get_id(&self) -> u64 {
		self.server_id
	}

	pub fn get_name(&self) -> &String {
		&self.name
	}


	fn receive_packet(&mut self) -> bool {

		if self.buffer.capacity() != self.max_mtu_size as usize {
			self.buffer = Vec::with_capacity(self.max_mtu_size as usize);
		}


		let address = match self.udp_socket.recv_from(&mut self.buffer) {
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

		match self.sessions_by_address.get_mut(&address) {
			Some(session) => {
				let header = self.buffer[0];
				if (header & Datagram::FLAG_VALID) != 0 {
					if (header & Datagram::FLAG_ACK) != 0 {
						session.handle_ack(ACK::decode_body(&mut self.buffer.as_slice()));
					} else if (header & Datagram::FLAG_NAK) != 0 {
						session.handle_nack(NACK::decode_body(&mut self.buffer.as_slice()));
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

}

//declare struct :: `type` declaration doesn't works well in idea
pub struct Server<'a> {
	inner: RwLock<_Server<'a>>,

	pub(super) protocol_acceptor: Box<dyn ProtocolAcceptor>,
	event_listener: RwLock<Box<dyn ServerEventListener>>
}

impl Server<'_> {
	pub fn new(
		server_id: u64,
		udp_socket: UdpSocket,
		max_mtu_size: usize,
		protocol_acceptor: impl ProtocolAcceptor,
		message_receiver: UserToRaknetMessageReceiver,
		event_listener: impl ServerEventListener,
	) -> Self {
		Self {
			inner: RwLock::new(_Server::new(
				server_id,
				udp_socket,
				max_mtu_size,
				message_receiver,
			)),
			protocol_acceptor: Box::new(protocol_acceptor) as Box<_>,
			event_listener: RwLock::new(Box::new(event_listener))
		}
	}

	pub fn get_raknet_time(&self) -> RaknetTime {
		self.inner.read().get_raknet_time()
	}

	pub fn get_port(&self) -> u16 {
		self.inner.read().get_port()
	}

	pub fn get_max_mtu_size(&self) -> usize {
		self.inner.read().get_max_mtu_size()
	}

	pub fn get_id(&self) -> u64 {
		self.inner.read().get_id()
	}

	pub fn get_name(&self) -> String {
		self.inner.read().get_name().to_owned()
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

	pub fn send_packet(&self, packet: &impl EncodePacket, address: &SocketAddr) {
		self.inner.write().send_packet(packet, address);
	}

	pub fn get_event_listener(&self) -> &RwLock<Box<dyn ServerEventListener>> {
		&self.event_listener
	}

	pub fn create_session(&mut self, address: SocketAddr, client_id: u64, mtu_size: u16) {
		//TODO self.check_sessions
		//let next_session_id = self.next_session_id.pop_back().unwrap_or_else(|| {
		//self.sessions.len().
		//});
	}


	pub fn open_session(&self, session: &Session) {
		let address = session.get_address();
		self.event_listener.write().on_client_connect(
			session.get_internal_id(),
			address.clone(),
			session.get_id()
		);
	}

	pub fn get_port_checking(&self) -> bool {
		self.inner.read().port_checking
	}

	pub fn set_port_checking(&self, port_checking: bool) {
		self.inner.write().port_checking = port_checking;
	}

}