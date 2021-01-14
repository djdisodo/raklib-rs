use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::net::SocketAddr;
use std::time::Duration;
use crate::server::{ServerEventListener, ServerEvent};

pub struct RaknetToUserThreadEventSender {
	channel: Arc<Mutex<VecDeque<ServerEvent>>>
}

impl RaknetToUserThreadEventSender {
	pub fn new(channel: Arc<Mutex<VecDeque<ServerEvent>>>) -> Self {
		Self {
			channel
		}
	}
}

impl ServerEventListener for RaknetToUserThreadEventSender {
	#[inline]
	fn handle_event(&mut self, event: ServerEvent) {
		self.channel.lock().unwrap().push_back(event);
	}

	#[inline]
	fn on_client_connect(&mut self, session_id: usize, address: SocketAddr, client_id: u64) {
		self.handle_event(ServerEvent::ClientConnect {
			session_id,
			address,
			client_id
		})
	}

	#[inline]
	fn on_client_disconnect(&mut self, session_id: usize, reason: &str) {
		self.handle_event(ServerEvent::ClientDisconnect {
			session_id,
			reason: reason.to_owned()
		})
	}

	#[inline]
	fn on_packet_receive(&mut self, session_id: usize, packet: &[u8]) {
		self.handle_event(ServerEvent::PacketReceive {
			session_id,
			packet: packet.to_owned()
		})
	}

	#[inline]
	fn on_raw_packet_receive(&mut self, address: SocketAddr, payload: &[u8]) {
		self.handle_event(ServerEvent::RawPacketReceive {
			address,
			payload: payload.to_owned()
		})
	}

	#[inline]
	fn on_packet_ack(&mut self, session_id: usize, identifier_ack: u32) {
		self.handle_event(ServerEvent::PacketAck {
			session_id,
			identifier_ack
		})
	}

	#[inline]
	fn on_bandwidth_stats_update(&mut self, bytes_sent_diff: usize, bytes_received_diff: usize) {
		self.handle_event(ServerEvent::BandwidthStatsUpdate {
			bytes_sent_diff,
			bytes_received_diff
		})
	}

	#[inline]
	fn on_ping_measure(&mut self, session_id: usize, latency: Duration) {
		self.handle_event(ServerEvent::PingMeasure {
			session_id,
			latency
		})
	}
}