use std::net::SocketAddr;
use std::time::Duration;
use crate::server::ServerEvent;

pub trait ServerEventListener: Send + Sync {
	fn handle_event(&mut self, event: ServerEvent) {
		match event {
			ServerEvent::ClientConnect {
				session_id,
				address,
				client_id
			} => self.on_client_connect(
				session_id,
				address,
				client_id
			),
			ServerEvent::ClientDisconnect {
				session_id,
				reason
			} => self.on_client_disconnect(
				session_id,
				&reason
			),
			ServerEvent::PacketReceive {
				session_id,
				packet
			} => self.on_packet_receive(
				session_id,
				&packet
			),
			ServerEvent::RawPacketReceive {
				address,
				payload
			} => self.on_raw_packet_receive(
				address,
				&payload
			),
			ServerEvent::PacketAck {
				session_id,
				identifier_ack
			} => self.on_packet_ack(
				session_id,
				identifier_ack
			),
			ServerEvent::BandwidthStatsUpdate {
				bytes_sent_diff,
				bytes_received_diff
			} => self.on_bandwidth_stats_update(
				bytes_sent_diff,
				bytes_received_diff
			),
			ServerEvent::PingMeasure {
				session_id,
				latency
			} => self.on_ping_measure(
				session_id,
				latency
			)
		}
	}

	fn on_client_connect(&mut self, session_id: usize, address: SocketAddr, client_id: u64);
	fn on_client_disconnect(&mut self, session_id: usize, reason: &str);
	fn on_packet_receive(&mut self, session_id: usize, packet: &[u8]);
	fn on_raw_packet_receive(&mut self, address: SocketAddr, payload: &[u8]);
	fn on_packet_ack(&mut self, session_id: usize, identifier_ack: u64);
	fn on_bandwidth_stats_update(&mut self, bytes_sent_diff: usize, bytes_received_diff: usize);
	fn on_ping_measure(&mut self, session_id: usize, latency: Duration);
}