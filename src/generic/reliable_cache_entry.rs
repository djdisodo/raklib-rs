use crate::protocol::EncapsulatedPacket;
use std::time::SystemTime;

pub struct ReliableCacheEntry {
	pub packets: Vec<Box<EncapsulatedPacket>>,
	pub timestamp: SystemTime
}

impl ReliableCacheEntry {
	pub fn new(packets: Vec<Box<EncapsulatedPacket>>) -> Self {
		Self {
			packets,
			timestamp: SystemTime::now()
		}
	}
}