use crate::protocol::PacketPayload;
use crate::protocol::raw_payload::RawPayload;
use crate::protocol::message_identifiers;
use bincode::Error;

pub struct Packet {
    buffer: Vec<u8>
}

impl Packet {
    pub fn from(buffer: Vec<u8>) -> Result<Self, &'static str> {
        if buffer.len() < 1 {
            return Err("buffer length should be longer than 1");
        }
        Ok(Self {
            buffer
        })
    }
    pub fn get_id(&self) -> u8 {
        self.buffer[0]
    }

    pub fn decode_payload<T: PacketPayload>(&self) -> Result<T, &str> {
        return if self.buffer.len() - message_identifiers::SIZE < T::MIN_SIZE as usize {
            Err("buffer size is not enough to decode")
        } else {
            match T::decode(&self.buffer[1..]) {
                Ok(result) => Ok(result),
                Err(e) => Err(e.to_string().as_str())
            }
        }
    }

    pub fn encode_payload<T: PacketPayload>(payload: &T) -> Result<Self, Error> {
        let mut buffer = Vec::with_capacity(T::MIN_SIZE as usize);
        buffer.push(T::ID);
        match payload.encode() {
            Ok(mut bytes) => buffer.append(&mut bytes),
            Err(e) => return Err(e)
        };
        Ok(Self {
            buffer
        })
    }

}