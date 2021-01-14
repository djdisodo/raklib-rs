use crate::protocol::EncapsulatedPacket;
use std::net::{SocketAddr, IpAddr};
use std::time::Duration;
use regex::bytes::Regex;
use crate::server::ipc::UserToRaknetMessage;

pub trait ServerInterface {

	fn handle_message(&mut self, message: UserToRaknetMessage) {
		match message {
			UserToRaknetMessage::Encapsulated {
				session_id,
				packet,
				immediate
			} => self.send_encapsulated(
				session_id,
				packet,
				immediate
			),
			UserToRaknetMessage::Raw {
				address,
				payload
			} => self.send_raw(
				address,
				payload
			),
			UserToRaknetMessage::CloseSession {
				session_id
			} => self.close_session(session_id),
			UserToRaknetMessage::SetName(name) => self.set_name(name),
			UserToRaknetMessage::SetPortCheck(port_check) => self.set_port_check(port_check),
			UserToRaknetMessage::SetPacketsPerTickLimit(limit) => self.set_packet_per_tick_limit(limit),
			UserToRaknetMessage::BlockAddress {
				address,
				timeout
			} => self.block_address(
				address,
				timeout
			),
			UserToRaknetMessage::UnblockAddress(address) => self.unblock_address(address),
			UserToRaknetMessage::RawFilter(regex) => self.add_raw_packet_filter(regex),
			UserToRaknetMessage::Shutdown => self.shutdown()
		}
	}

	fn send_encapsulated(&mut self, session_id: usize, packet: EncapsulatedPacket, immediate: bool);
	fn send_raw(&mut self, address: SocketAddr, payload: Vec<u8>);
	fn close_session(&mut self, session_id: usize);
	fn set_name(&mut self, name: String);
	fn set_port_check(&mut self, value: bool);
	fn set_packet_per_tick_limit(&mut self, limit: usize);
	fn block_address(&mut self, address: IpAddr, timeout: Duration);
	fn unblock_address(&mut self, address: IpAddr);
	fn add_raw_packet_filter(&mut self, regex: Regex);
	fn shutdown(&mut self);
}