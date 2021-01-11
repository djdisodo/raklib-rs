use crate::protocol::payload::{ACK, NACK};
use crate::protocol::Datagram;
use crate::server::Server;
use std::net::SocketAddr;
use std::time::{SystemTime, Duration};

pub struct Session<'a> {
	//TODO
	server: &'a Server<'a>,

	address: SocketAddr,

	state: SessionState, //default SessionState::Connecting

	id: u64,

	last_update: SystemTime,

	is_temporal: bool, //default true

	is_active: bool, //default false

	last_ping_time: SystemTime, //default -1

	last_ping_measure: Duration, //default 1

	internal_id: u64,

	




}

impl Session<'_> {

	pub const MAX_SPLIT_PART_COUNT: usize = 128;
	pub const MAX_CONCURRENT_SPLIT_COUNT: usize = 4;

	pub const MIN_MTU_SIZE: u16 = 400;

	pub fn handle_datagram(&mut self, datagram: Datagram) {
		unimplemented!()
	}

	pub fn handle_ack(&mut self, ack: ACK) {
		unimplemented!()
	}

	pub fn handle_nack(&mut self, nack: NACK) {
		unimplemented!()
	}
}

enum SessionState {
	Connecting,
	Connected,
	Disconnecting,
	Disconnected(SystemTime)
}