use crate::protocol::{EncodeBody, DecodeBody, EncapsulatedPacket, EncodeHeader, CommonEncodePacket, DecodePacket};
use bytes::{BufMut, Buf};

#[derive(Default, Debug)]
pub struct Datagram {
	pub header_flags: u8,
	pub packets: Vec<Box<EncapsulatedPacket>>,
	pub sequence_number: Option<u32>
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

	pub const HEADER_SIZE: usize = 1 + 3; //header flags (1) + sequence number (3)
}

impl EncodeHeader for Datagram {
	fn encode_header(&self) -> u8 {
		unimplemented!()
	}
}

impl EncodeBody for Datagram {
	fn encode_body(&self, mut _serializer: &mut dyn BufMut) {
		unimplemented!()
	}
}

impl CommonEncodePacket for Datagram {}

impl DecodePacket for Datagram {
	fn decode_packet(serializer: &mut dyn Buf) -> Self {
		unimplemented!()
	}
}