use std::sync::Arc;
use std::collections::VecDeque;
use crate::server::ipc::UserToRaknetMessage;
use crate::server::ServerInterface;
use std::net::{SocketAddr, IpAddr};
use regex::bytes::Regex;
use std::time::Duration;
use crate::protocol::EncapsulatedPacket;
use parking_lot::Mutex;

pub struct UserToRaknetMessageSender {
	channel: Arc<Mutex<VecDeque<UserToRaknetMessage>>>
}

impl UserToRaknetMessageSender {
	pub fn new(channel: Arc<Mutex<VecDeque<UserToRaknetMessage>>>) -> Self {
		Self {
			channel
		}
	}
}

impl ServerInterface for UserToRaknetMessageSender {
	#[inline]
	fn handle_message(&mut self, message: UserToRaknetMessage) {
		self.channel.lock().push_back(message);
	}

	#[inline]
	fn send_encapsulated(&mut self, session_id: usize, packet: EncapsulatedPacket, immediate: bool) {
		self.handle_message(UserToRaknetMessage::Encapsulated {
			session_id,
			packet: Box::new(packet),
			immediate
		});
	}

	#[inline]
	fn send_raw(&mut self, address: &SocketAddr, payload: &[u8]) {
		self.handle_message(UserToRaknetMessage::Raw {
			address: address.clone(),
			payload: payload.to_vec()
		});
	}

	#[inline]
	fn close_session(&mut self, session_id: usize) {
		self.handle_message(UserToRaknetMessage::CloseSession {
			session_id
		});
	}

	#[inline]
	fn set_name(&mut self, name: String) {
		self.handle_message(UserToRaknetMessage::SetName(name));
	}

	#[inline]
	fn set_port_check(&mut self, value: bool) {
		self.handle_message(UserToRaknetMessage::SetPortCheck(value));
	}

	#[inline]
	fn set_packet_per_tick_limit(&mut self, limit: usize) {
		self.handle_message(UserToRaknetMessage::SetPacketsPerTickLimit(limit));
	}

	#[inline]
	fn block_address(&mut self, address: IpAddr, timeout: Duration) {
		self.handle_message(UserToRaknetMessage::BlockAddress {
			address: address.clone(),
			timeout
		});
	}

	#[inline]
	fn unblock_address(&mut self, address: &IpAddr) {
		self.handle_message(UserToRaknetMessage::UnblockAddress(address.clone()));
	}

	#[inline]
	fn add_raw_packet_filter(&mut self, regex: Regex) {
		self.handle_message(UserToRaknetMessage::RawFilter(regex));
	}
}

impl UserToRaknetMessageSender {

	#[inline]
	fn send_raw_mv(&mut self, address: SocketAddr, payload: Vec<u8>) {
		self.handle_message(UserToRaknetMessage::Raw {
			address,
			payload
		});
	}

	#[inline]
	fn block_address_mv(&mut self, address: IpAddr, timeout: Duration) {
		self.handle_message(UserToRaknetMessage::BlockAddress {
			address,
			timeout
		});
	}

	#[inline]
	fn unblock_address_mv(&mut self, address: IpAddr) {
		self.handle_message(UserToRaknetMessage::UnblockAddress(address));
	}
}