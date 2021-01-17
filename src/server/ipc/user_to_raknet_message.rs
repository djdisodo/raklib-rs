use regex::bytes::Regex;
use crate::protocol::EncapsulatedPacket;
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;

pub enum UserToRaknetMessage {
	Encapsulated {
		session_id: usize,
		packet: Box<EncapsulatedPacket>,
		immediate: bool
	},
	CloseSession {
		session_id: usize
	},
	Raw {
		address: SocketAddr,
		payload: Vec<u8>
	},
	BlockAddress {
		address: IpAddr,
		timeout: Duration
	},
	UnblockAddress(IpAddr),
	RawFilter(Regex),
	SetName(String),
	SetPortCheck(bool),
	SetPacketsPerTickLimit(usize)
}