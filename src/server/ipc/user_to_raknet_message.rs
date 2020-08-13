pub use crate::server::server_event::Raw;

use regex::bytes::Regex;
use crate::protocol::EncapsulatedPacket;
use std::net::IpAddr;
use std::time::Duration;

pub enum UserToRaknetMessage {
	Encapsulated(Encapsulated),
	CloseSession(u32),
	Raw(Raw),
	BlockAddress(BlockAddress),
	UnblockAddress(IpAddr),
	RawFilter(Regex),
	SetName(String),
	SetPortCheck(bool),
	SetPacketsPerTickLimit(usize),
	Shutdown
}

pub struct Encapsulated {
	pub session_id: u32,
	pub packet: EncapsulatedPacket,
	pub immediate: bool
}

pub struct BlockAddress {
	pub address: IpAddr,
	pub timeout: Duration
}