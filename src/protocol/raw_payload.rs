use crate::protocol::PacketPayload;
use std::convert::TryFrom;
use serde::{Serialize, Deserialize};
use serde::export::fmt::Debug;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct RawPayload {
    buffer: Vec<u8>
}