use std::net::SocketAddr;
use std::time::Duration;

pub trait ServerEventListener {
	fn on_open_session(&mut self, session_id: u32, address: &SocketAddr, client_id: u64);
	fn on_close_session(&mut self, session_id: u32, reason: &str);
	fn on_handle_encapsulated(&mut self, session_id: u32, packet: &[u8]);
	fn on_handle_raw(&mut self, address: &SocketAddr, payload: &[u8]);
	fn on_notify_ack(&mut self, session_id: u32, identifier_ack: u64);
	fn on_handle_bandwidth_stats(&mut self, bytes_sent_diff: usize, bytes_received_diff: usize);
	fn on_update_ping(&mut self, session_id: u32, ping_time: Duration);
}