mod ack;
mod acknowledge_packet;
mod advertise_system;

pub use ack::ACK;
pub use acknowledge_packet::AcknowledgePacket;


use std::fmt::Debug;
use bytes::{BytesMut, Bytes};
use std::io::Error;

pub trait Payload: Debug + Encode + Decode + Default {
	const ID: u8;
	const MIN_SIZE: u16 = 0;
}

pub trait Encode {
	fn encode(&self, serializer: &mut Vec<u8>);
}

pub trait Decode {
	fn decode(serializer: &mut &[u8]) -> Self;
}