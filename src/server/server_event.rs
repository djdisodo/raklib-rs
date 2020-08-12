use crate::protocol::EncapsulatedPacket;
use std::net::SocketAddr;
use std::time::Duration;

pub enum ServerEvent {
	Encapsulated(Encapsulated),
	OpenSession(OpenSession),
	CloseSession(CloseSession),
	AckNotification(AckNotification),
	ReportBandwidthStats(ReportBandwidthStats),
	Raw(Raw),
	ReportPing(ReportPing)
}

pub struct Encapsulated {
	pub session_id: u32,
	pub packet: Vec<u8>
}

pub struct OpenSession {
	pub session_id: u32,
	pub address: SocketAddr,
	pub client_id: u64
}

pub struct CloseSession {
	pub session_id: u32,
	pub reason: String
}

pub struct AckNotification {
	pub session_id: u32,
	pub identifier_ack: u32,
}

pub struct ReportBandwidthStats {
	pub bytes_sent_diff: usize,
	pub bytes_received_diff: usize
}

pub struct Raw {
	pub address: SocketAddr,
	pub payload: Vec<u8>
}

pub struct ReportPing {
	pub session_id: u32,
	pub latency: Duration
}