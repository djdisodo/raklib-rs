use std::convert::{TryFrom, TryInto};
use bincode::Error;
use serde::{Serialize, Deserialize};
use serde::export::fmt::Debug;
pub trait PacketPayload: Serialize + Deserialize + PartialEq + Debug {
	const ID: u8;
	const MIN_SIZE: u16 = 0;

	fn decode(buffer: &[u8]) -> Result<Self, Error> {
		bincode::deserialize(buffer)
	}

	fn encode(&self) -> Result<Vec<u8>, Error> {
		bincode::serialize(self)
	}
}