use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use crate::server::ipc::UserToRaknetMessage;
use crate::server::ServerInterface;
use std::net::{SocketAddr, IpAddr};
use regex::bytes::Regex;
use std::time::Duration;
use crate::protocol::EncapsulatedPacket;
use crate::server::ipc::user_to_raknet_message::{Encapsulated, BlockAddress};
use crate::server::server_event::Raw;

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
		self.channel.lock().unwrap().push_back(message);
	}

	#[inline]
	fn send_encapsulated(&mut self, session_id: u32, packet: EncapsulatedPacket, immediate: bool) {
		self.handle_message(UserToRaknetMessage::Encapsulated(Encapsulated {
			session_id,
			packet,
			immediate
		}));
	}

	#[inline]
	fn send_raw(&mut self, address: SocketAddr, payload: Vec<u8>) {
		self.handle_message(UserToRaknetMessage::Raw(Raw {
			address,
			payload
		}));
	}

	#[inline]
	fn close_session(&mut self, session_id: u32) {
		self.handle_message(UserToRaknetMessage::CloseSession(session_id));
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
		self.handle_message(UserToRaknetMessage::BlockAddress(BlockAddress {
			address,
			timeout
		}));
	}

	#[inline]
	fn unblock_address(&mut self, address: IpAddr) {
		self.handle_message(UserToRaknetMessage::UnblockAddress(address));
	}

	#[inline]
	fn add_raw_packet_filter(&mut self, regex: Regex) {
		self.handle_message(UserToRaknetMessage::RawFilter(regex));
	}

	#[inline]
	fn shutdown(&mut self) {
		self.handle_message(UserToRaknetMessage::Shutdown);
	}
}