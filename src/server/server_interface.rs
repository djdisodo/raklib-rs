use crate::protocol::EncapsulatedPacket;
use std::net::{SocketAddr, IpAddr};
use std::time::Duration;
use regex::bytes::Regex;

pub trait ServerInterface {
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