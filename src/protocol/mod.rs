pub mod payload;
mod packet_reliability;
mod datagram;
mod encapsulated_packet;
mod split_packet_info;

pub use payload::Payload;
pub use packet::Packet;
pub use packet_reliability::PacketReliability;
pub use datagram::Datagram;
pub use encapsulated_packet::EncapsulatedPacket;
pub use split_packet_info::SplitPacketInfo;
pub use message_identifiers::MessageIdentifiers;

mod packet;
mod message_identifiers;

pub trait Encode {
	fn encode(&self, serializer: &mut Vec<u8>);
}

pub trait Decode {
	fn decode(serializer: &mut &[u8]) -> Self;
}