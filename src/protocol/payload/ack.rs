use crate::protocol::payload::AcknowledgePacket;
use std::ops::{Deref, DerefMut};

#[derive(Default, Deref, DerefMut)]
pub struct ACK {
	pub acknowledge: AcknowledgePacket
}