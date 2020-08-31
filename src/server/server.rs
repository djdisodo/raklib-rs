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
use std::sync::Mutex;
use std::rc::Rc;
use crate::server::unconnected_message_handler::UnconnectedMessageHandler;

pub struct Server<'a, EventListener: ServerEventListener, PA: ProtocolAcceptor> {

	socket: Mutex<ServerSocket>,

	server_id: u64,

	sessions: Vec<Session>,
	sessions_by_address: HashMap<SocketAddr, usize/* index in sessions */>,

	protocol_acceptor: PA,

	name: String,

	packet_limit: usize, //default 200

	shutdown: bool,

	ticks: u8, //default 0
	
	block: HashMap<IpAddr, Duration>, // duration = unblock time; TODO it might be SystemTime
	ip_sec: HashMap<IpAddr, usize>,

	raw_packet_filters: Vec<Regex>,

	pub port_checking: bool, //default false

	start_time: Option<Instant>,

	max_mtu_size: u16,

	reusable_address: SocketAddr,

	next_session_id: VecDeque<usize>,

	message_receiver: UserToRaknetMessageReceiver, // event_source
	event_listener: Mutex<EventListener>,

	//trace_cleaner
}

pub struct ServerSocket {
	socket: UdpSocket,
	receive_bytes: usize,
	send_bytes: usize,
	buffer: Vec<u8>
}

impl ServerSocket {
	pub fn send_packet(&mut self, packet: impl Encode, address: &SocketAddr) {
		self.buffer.clear();
		packet.encode(&mut self.buffer);
		match self.socket.send_to(&self.buffer, address) {
			Ok(send) => self.send_bytes += send,
			Err(e) => debug!("{}", e)
		}
	}
}

impl<EventListener: ServerEventListener, PA: ProtocolAcceptor> Server<'_, EventListener, PA> {
	pub fn new(
		server_id: u64,
		socket: UdpSocket,
		max_mtu_size: u16,
		protocol_acceptor: PA,
		message_receiver: UserToRaknetMessageReceiver,
		event_listener: EventListener,
	) -> Self {
		let socket = Mutex::new(ServerSocket {
			socket,
			receive_bytes: 0,
			send_bytes: 0,
			buffer: Vec::with_capacity(max_mtu_size as usize)
		});
		Self {
			socket: socket,
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
			max_mtu_size,
			reusable_address: socket.local_addr().as_ref().unwrap().clone(),
			next_session_id: VecDeque::new(),
			message_receiver,
			event_listener: Mutex::new(event_listener)
		}
	}

	pub fn get_raknet_time(&self) -> Duration {
		Instant::now().duration_since(self.start_time.unwrap())
	}

	pub fn get_port(&self) -> u16 {
		self.socket.local_addr().unwrap().port()
	}

	pub fn get_max_mtu_size(&self) -> u16 {
		self.max_mtu_size
	}

	pub fn get_id(&self) -> u64 {
		self.server_id
	}

	pub fn get_name(&self) -> &String {
		&self.name
	}

	pub fn tick_processor(&mut self) {
		while !self.shutdown && self.sessions.len() > 0 {
			loop {
				for _ in 0..100 {
					let message = self.message_receiver.receive();
					if self.shutdown || message.is_none() {
						break;
					} else {
						self.handle_message(message.unwrap());
					}
				}

			}
		}
	}

	pub fn receive_packet(&mut self) -> bool {
		if self.buffer.capacity() != self.max_mtu_size as usize {
			self.buffer = Vec::with_capacity(self.max_mtu_size as usize);
		}
		let address = match self.socket.recv_from(&mut self.buffer) {
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
						session.handle_ack(Packet::decode(&mut self.buffer.as_slice()).payload);
					} else if (header & Datagram::FLAG_NAK) != 0 {
						session.handle_nack(Packet::decode(&mut self.buffer.as_slice()).payload);
					} else {
						session.handle_datagram(Datagram::decode(&mut self.buffer.as_slice()));
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



	pub fn create_session(&mut self, address: SocketAddr, client_id: u64, mtu_size: u16) {
		//TODO self.check_sessions
		let next_session_id = self.next_session_id.pop_back();

	}
}