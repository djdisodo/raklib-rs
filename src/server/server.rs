use std::net::{UdpSocket, SocketAddr, IpAddr};
use log4rs::Logger;
use std::collections::{HashMap, VecDeque};
use crate::server::session::Session;
use std::time::{Duration, Instant};
use regex::bytes::Regex;
use crate::server::ipc::UserToRaknetMessageReceiver;
use crate::server::{ServerEventListener, ProtocolAcceptor};
use std::io::ErrorKind;
use log::debug;
use crate::protocol::{Datagram, Packet, Decode, Encode};
use crate::protocol::payload::{ACK, NACK};
use std::sync::RwLock;
use std::rc::Rc;
use crate::server::unconnected_message_handler::UnconnectedMessageHandler;
use std::ops::{Deref, DerefMut};

pub struct Server {

	pub(super) server_socket: RwLock<ServerSocket>,

	server_id: u64,

	sessions: Vec<Session>,
	sessions_by_address: HashMap<SocketAddr, Session/* index in sessions */>,

	pub(super) protocol_acceptor: Box<dyn ProtocolAcceptor>,

	name: String,

	packet_limit: usize, //default 200

	shutdown: bool,

	ticks: u8, //default 0
	
	block: HashMap<IpAddr, Duration>, // duration = unblock time; TODO it might be SystemTime
	ip_sec: HashMap<IpAddr, usize>,

	raw_packet_filters: Vec<Regex>,

	pub port_checking: bool, //default false

	start_time: Option<Instant>,

	reusable_address: SocketAddr,

	next_session_id: VecDeque<usize>,

	message_receiver: UserToRaknetMessageReceiver, // event_source
	event_listener: RwLock<Box<dyn ServerEventListener>>,

	//trace_cleaner
}

pub struct ServerSocket {
	udp_socket: UdpSocket,
	receive_bytes: usize,
	send_bytes: usize,
	buffer: Vec<u8>,
	max_mtu_size: u16
}

impl ServerSocket {
	fn recv_raw_packet(&mut self) -> std::io::Result<(usize, SocketAddr)> {
		if self.buffer.capacity() != self.max_mtu_size as usize {
			self.buffer = Vec::with_capacity(self.max_mtu_size as usize);
		}
		self.udp_socket.recv_from(&mut self.buffer)
	}
}

impl Deref for ServerSocket {
	type Target = UdpSocket;

	fn deref(&self) -> &Self::Target {
		&self.udp_socket
	}
}

impl DerefMut for ServerSocket {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.udp_socket
	}
}

impl Server {
	pub fn new<EL: ServerEventListener, PA: ProtocolAcceptor>(
		server_id: u64,
		udp_socket: UdpSocket,
		max_mtu_size: u16,
		protocol_acceptor: PA,
		message_receiver: UserToRaknetMessageReceiver,
		event_listener: EL,
	) -> Self {
		let reusable_address = udp_socket.local_addr().unwrap();
		let server_socket = RwLock::new(ServerSocket {
			udp_socket,
			receive_bytes: 0,
			send_bytes: 0,
			buffer: Vec::with_capacity(max_mtu_size as usize),
			max_mtu_size
		});
		Self {
			server_socket,
			server_id,
			sessions_by_address: HashMap::new(),
			sessions: Vec::new(),
			protocol_acceptor: Box::new(protocol_acceptor) as Box<_>,
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
			message_receiver,
			event_listener: RwLock::new(Box::new(event_listener) as Box<_>)
		}
	}

	pub fn get_raknet_time(&self) -> Duration {
		Instant::now().duration_since(self.start_time.unwrap())
	}

	pub fn get_port(&self) -> u16 {
		self.server_socket.read().unwrap().udp_socket.local_addr().unwrap().port()
	}

	pub fn get_max_mtu_size(&self) -> u16 {
		self.server_socket.read().unwrap().max_mtu_size
	}

	pub fn get_id(&self) -> u64 {
		self.server_id
	}

	pub fn get_name(&self) -> &String {
		&self.name
	}

	pub fn tick_processor(&mut self) {
		while !self.shutdown && self.sessions.len() > 0 {
			for _ in 0..100 {
				let message = self.message_receiver.receive();
				if self.shutdown || message.is_none() {
					break;
				} else {
					//self.handle_message(message.unwrap());
				}
			}
		}
	}

	fn receive_packet(&mut self) -> bool {

		let mut server_socket = self.server_socket.write().unwrap();

		let address = match server_socket.recv_raw_packet() {
			Ok(t) => {
				server_socket.receive_bytes += t.0;
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
				let header = server_socket.buffer[0];
				if (header & Datagram::FLAG_VALID) != 0 {
					if (header & Datagram::FLAG_ACK) != 0 {
						session.handle_ack(Packet::decode(&mut server_socket.buffer.as_slice()).payload);
					} else if (header & Datagram::FLAG_NAK) != 0 {
						session.handle_nack(Packet::decode(&mut server_socket.buffer.as_slice()).payload);
					} else {
						session.handle_datagram(Datagram::decode(&mut server_socket.buffer.as_slice()));
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

	pub fn send_packet(&self, packet: impl Encode, address: &SocketAddr) {
		let mut server_socket = self.server_socket.write().unwrap();
		server_socket.buffer.clear();
		packet.encode(&mut server_socket.buffer);
		match server_socket.send_to(&server_socket.buffer, address) {
			Ok(send) => server_socket.send_bytes += send,
			Err(e) => debug!("{}", e)
		}
	}



	pub fn create_session(&mut self, address: SocketAddr, client_id: u64, mtu_size: u16) {
		//TODO self.check_sessions
		let next_session_id = self.next_session_id.pop_back();

	}
}