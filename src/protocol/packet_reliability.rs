
#[derive(Debug, TryFromPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum PacketReliability {
	Unreliable = 0,
	UnreliableSequenced = 1,
	Reliable = 2,
	ReliableOrdered = 3,
	ReliableSequenced = 4,
	/* The following reliabilities are used in RakNet internals, but never sent on the wire. */
	UnreliableWithAckReceipt = 5,
	ReliableWithAckReceipt = 6,
	ReliableOrderedWithAckReceipt = 7,
}

impl PacketReliability {
	pub fn is_reliable(&self) -> bool {
		match self {
			PacketReliability::Reliable => true,
			PacketReliability::ReliableOrdered => true,
			PacketReliability::ReliableSequenced => true,
			PacketReliability::ReliableWithAckReceipt => true,
			PacketReliability::ReliableOrderedWithAckReceipt => true,
			_ => false
		}
	}

	pub fn is_sequenced(&self) -> bool {
		match self {
			PacketReliability::ReliableSequenced => true,
			PacketReliability::UnreliableSequenced => true,
			_ => false
		}
	}

	pub fn is_ordered(&self) -> bool {
		match self {
			PacketReliability::ReliableOrdered => true,
			PacketReliability::ReliableOrderedWithAckReceipt => true,
			_ => false,
		}
	}
}

impl Default for PacketReliability {
	fn default() -> Self {
		PacketReliability::Unreliable
	}
}