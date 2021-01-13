use crate::protocol::{Payload, Encode, Decode, PayloadExt};
use bytes::{BufMut, Buf};

#[derive(Deref, DerefMut, Debug)]
pub struct Packet<T: PayloadExt + ?Sized> {
    pub payload: Box<T>
}

impl<T: PayloadExt + 'static> Packet<T> {
	pub fn wrap(payload: T) -> Self {
		Self {
			payload: Box::new(payload)
		}
	}

	pub fn into_dyn(self) -> Packet<dyn PayloadExt> {
		Packet {
			payload: self.payload
		}
	}
}

impl<T: Payload> Decode for Packet<T> {
	fn decode(serializer: &mut &[u8]) -> Self where Self: Sized {
		if T::ID as u8 != serializer.get_u8() {
			panic!("message identifier doesn't match");
		}
		Self {
			payload: Box::new(T::decode(serializer))
		}
	}
}

impl<T: Payload> Encode for Packet<T> {
	fn encode(&self, mut serializer: &mut Vec<u8>) {
		serializer.put_u8(self.get_id() as u8);
		self.payload.encode(&mut serializer);
	}
}

impl<T: Payload + Default> Default for Packet<T> {
	fn default() -> Self {
		Self {
			payload: Box::new(T::default())
		}
	}
}