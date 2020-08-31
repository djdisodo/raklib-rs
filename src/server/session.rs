use crate::protocol::payload::{ACK, NACK};
use crate::protocol::Datagram;

pub struct Session {
	//TODO
}

impl Session {

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