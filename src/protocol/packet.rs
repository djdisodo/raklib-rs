use crate::protocol::{Payload, Encode, Decode};
use bytes::{BufMut, Buf};

#[derive(Deref, DerefMut)]
pub struct Packet<T: Payload> {
    pub payload: T
}

impl<T: Payload> Packet<T> {
	pub fn wrap(payload: T) -> Self {
		Self {
			payload
		}
	}
}

impl<T: Payload> Decode for Packet<T> {
	fn decode(serializer: &mut &[u8]) -> Self {
		if T::ID as u8 != serializer.get_u8() {
			panic!("message identifier doesn't match");
		}
		Self {
			payload: T::decode(serializer)
		}
	}
}

impl<T: Payload> Encode for Packet<T> {
	fn encode(&self, mut serializer: &mut Vec<u8>) {
		serializer.put_u8(T::ID as u8);
		self.payload.encode(&mut serializer);
	}
}

impl<T: Payload + Default> Default for Packet<T> {
	fn default() -> Self {
		Self {
			payload: T::default()
		}
	}
}