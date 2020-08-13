use crate::protocol::EncapsulatedPacket;
use std::net::{SocketAddr, IpAddr};
use std::time::Duration;
use regex::bytes::Regex;
use crate::server::ipc::UserToRaknetMessage;

pub trait ServerInterface {

	fn handle_message(&mut self, message: UserToRaknetMessage) {
		match message {
			UserToRaknetMessage::Encapsulated(encapsulated) => self.send_encapsulated(
				encapsulated.session_id,
				encapsulated.packet,
				encapsulated.immediate
			),
			UserToRaknetMessage::Raw(raw) => self.send_raw(
				raw.address,
				raw.payload
			),
			UserToRaknetMessage::CloseSession(session_id) => self.close_session(session_id),
			UserToRaknetMessage::SetName(name) => self.set_name(name),
			UserToRaknetMessage::SetPortCheck(port_check) => self.set_port_check(port_check),
			UserToRaknetMessage::SetPacketsPerTickLimit(limit) => self.set_packet_per_tick_limit(limit),
			UserToRaknetMessage::BlockAddress(block_address) => self.block_address(
				block_address.address,
				block_address.timeout
			),
			UserToRaknetMessage::UnblockAddress(address) => self.unblock_address(address),
			UserToRaknetMessage::RawFilter(regex) => self.add_raw_packet_filter(regex),
			UserToRaknetMessage::Shutdown => self.shutdown()
		}
	}

	fn send_encapsulated(&mut self, session_id: u32, packet: EncapsulatedPacket, immediate: bool);
	fn send_raw(&mut self, address: SocketAddr, payload: Vec<u8>);
	fn close_session(&mut self, session_id: u32);
	fn set_name(&mut self, name: String);
	fn set_port_check(&mut self, value: bool);
	fn set_packet_per_tick_limit(&mut self, limit: usize);
	fn block_address(&mut self, address: IpAddr, timeout: Duration);
	fn unblock_address(&mut self, address: IpAddr);
	fn add_raw_packet_filter(&mut self, regex: Regex);
	fn shutdown(&mut self);
}