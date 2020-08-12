use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use crate::server::{ServerEvent, ServerEventListener, OpenSession, CloseSession, Encapsulated, Raw, AckNotification, ReportBandwidthStats, ReportPing};
use std::net::SocketAddr;
use std::time::Duration;

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
	fn open_session(&mut self, session_id: u32, address: SocketAddr, client_id: u64) {
		self.handle_event(ServerEvent::OpenSession(OpenSession {
			session_id,
			address,
			client_id
		}))
	}

	#[inline]
	fn close_session(&mut self, session_id: u32, reason: String) {
		self.handle_event(ServerEvent::CloseSession(CloseSession {
			session_id,
			reason
		}))
	}

	#[inline]
	fn handle_encapsulated(&mut self, session_id: u32, packet: Vec<u8>) {
		self.handle_event(ServerEvent::Encapsulated(Encapsulated {
			session_id,
			packet
		}))
	}

	#[inline]
	fn handle_raw(&mut self, address: SocketAddr, payload: Vec<u8>) {
		self.handle_event(ServerEvent::Raw(Raw {
			address,
			payload
		}))
	}

	#[inline]
	fn notify_ack(&mut self, session_id: u32, identifier_ack: u32) {
		self.handle_event(ServerEvent::AckNotification(AckNotification {
			session_id,
			identifier_ack
		}))
	}

	#[inline]
	fn handle_bandwidth_stats(&mut self, bytes_sent_diff: usize, bytes_received_diff: usize) {
		self.handle_event(ServerEvent::ReportBandwidthStats(ReportBandwidthStats {
			bytes_sent_diff,
			bytes_received_diff
		}))
	}

	#[inline]
	fn update_ping(&mut self, session_id: u32, latency: Duration) {
		self.handle_event(ServerEvent::ReportPing(ReportPing {
			session_id,
			latency
		}))
	}
}