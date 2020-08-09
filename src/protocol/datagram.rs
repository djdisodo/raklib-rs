use crate::protocol::EncapsulatedPacket;
use crate::protocol::{Encode, Decode};

#[derive(Default, Debug)]
pub struct Datagram {
	pub header_flags: u8,
	pub packets: Vec<EncapsulatedPacket>,
	pub sequence_number: u32
}

impl Datagram {
	pub const FLAG_VALID: u8 = 0x80;
	pub const FLAG_ACK: u8 = 0x40;
	pub const FLAG_NAK: u8 = 0x20; // hasBAndAS for ACKs

	/*
	 * These flags can be set on regular datagrams, but they are useless as per the public version of RakNet
	 * (the receiving client will not use them or pay any attention to them).
	 */
	pub const FLAG_PACKET_PAIR: u8 = 0x10;
	pub const FLAG_CONTINUOUS_SEND: u8 = 0x08;
	pub const FLAG_NEEDS_B_AND_AS: u8 = 0x04;
}

impl Encode for Datagram {
	fn encode(&self, _serializer: &mut Vec<u8>) {
		unimplemented!()
	}
}

impl Decode for Datagram {
	fn decode(_serializer: &mut &[u8]) -> Self {
		unimplemented!()
	}
}