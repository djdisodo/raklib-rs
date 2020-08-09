mod payload;
mod datagram;
mod encapsulated_packet;

pub use payload::Payload;
pub use datagram::Datagram;
pub use encapsulated_packet::EncapsulatedPacket;

mod packet;
pub mod message_identifiers;

pub trait Encode {
	fn encode(&self, serializer: &mut Vec<u8>);
}

pub trait Decode {
	fn decode(serializer: &mut &[u8]) -> Self;
}