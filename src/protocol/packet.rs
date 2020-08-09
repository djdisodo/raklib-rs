use crate::protocol::Payload;
use crate::protocol::message_identifiers;
use bytes::{Bytes, BytesMut, BufMut, Buf};
use std::io::Cursor;

#[derive(Default, Deref, DerefMut)]
pub struct Packet<T: Payload> {
    pub payload: T
}

impl<T: Payload> From<&[u8]> for Packet<T> {
	fn from(mut buffer: &[u8]) -> Self {
		if T::ID != buffer.get_u8() {
			panic!("message identifier doesn't match");
		}
		Self {
			payload: T::decode(&mut buffer)
		}
	}
}

impl<T: Payload> Into<Vec<u8>> for &Packet<T> {
	fn into(self) -> Vec<u8> {
		let mut buffer = Vec::new();
		buffer.put_u8(T::ID);
		self.payload.encode(&mut buffer);
		buffer
	}
}