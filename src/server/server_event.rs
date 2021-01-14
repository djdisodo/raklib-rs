use crate::protocol::EncapsulatedPacket;
use std::net::SocketAddr;
use std::time::Duration;

pub enum ServerEvent {
	PacketReceive {
		session_id: usize,
		packet: Vec<u8>
	},
	ClientConnect {
		session_id: usize,
		address: SocketAddr,
		client_id: u64
	},
	ClientDisconnect {
		session_id: usize,
		reason: String
	},
	PacketAck {
		session_id: usize,
		identifier_ack: u32,
	},
	BandwidthStatsUpdate {
		bytes_sent_diff: usize,
		bytes_received_diff: usize
	}
	,
	RawPacketReceive {
		address: SocketAddr,
		payload: Vec<u8>
	},
	PingMeasure {
		session_id: usize,
		latency: Duration
	}
}