use crate::protocol::EncapsulatedPacket;
use std::time::SystemTime;

pub struct ReliableCacheEntry {
	packets: Vec<Box<EncapsulatedPacket>>,
	timestamp: SystemTime
}

impl ReliableCacheEntry {
	pub fn new(packets: Vec<Box<EncapsulatedPacket>>) -> Self {
		Self {
			packets,
			timestamp: SystemTime::now()
		}
	}

	pub fn get_packets(&self) -> &Vec<Box<EncapsulatedPacket>> {
		&self.packets
	}

	pub fn get_timestamp(&self) -> &SystemTime {
		&self.timestamp
	}
}