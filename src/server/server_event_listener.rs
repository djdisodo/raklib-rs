use std::net::SocketAddr;
use std::time::Duration;
use crate::server::ServerEvent;

pub trait ServerEventListener {
	fn handle_event(&mut self, event: ServerEvent) {
		match event {
			ServerEvent::OpenSession(open_session) => self.open_session(
				open_session.session_id,
				open_session.address,
				open_session.client_id
			),
			ServerEvent::CloseSession(close_session) => self.close_session(
				close_session.session_id,
				close_session.reason
			),
			ServerEvent::Encapsulated(encapsulated) => self.handle_encapsulated(
				encapsulated.session_id,
				encapsulated.packet
			),
			ServerEvent::Raw(raw) => self.handle_raw(
				raw.address,
				raw.payload
			),
			ServerEvent::AckNotification(ack_notification) => self.notify_ack(
				ack_notification.session_id,
				ack_notification.identifier_ack
			),
			ServerEvent::ReportBandwidthStats(report_bandwidth_stats) => self.handle_bandwidth_stats(
				report_bandwidth_stats.bytes_sent_diff,
				report_bandwidth_stats.bytes_received_diff
			),
			ServerEvent::ReportPing(report_ping) => self.update_ping(
				report_ping.session_id,
				report_ping.latency
			)
		}
	}

	fn open_session(&mut self, session_id: u32, address: SocketAddr, client_id: u64);
	fn close_session(&mut self, session_id: u32, reason: String);
	fn handle_encapsulated(&mut self, session_id: u32, packet: Vec<u8>);
	fn handle_raw(&mut self, address: SocketAddr, payload: Vec<u8>);
	fn notify_ack(&mut self, session_id: u32, identifier_ack: u32);
	fn handle_bandwidth_stats(&mut self, bytes_sent_diff: usize, bytes_received_diff: usize);
	fn update_ping(&mut self, session_id: u32, latency: Duration);
}