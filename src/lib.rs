#[macro_use] extern crate derive_deref;
#[macro_use] extern crate num_enum;

use std::time::Duration;

pub mod generic;
pub mod protocol;
pub mod server;

pub const DEFAULT_PROTOCOL_VERSION: u8 = 6;
pub static SYSTEM_ADDRESS_COUNT: usize = 20;

pub type RaknetTime = Duration;

#[cfg(test)]
mod tests {
    use crate::protocol::packet::IncompatibleProtocolVersion;
    use crate::protocol::EncodeBody;

    #[test]
    fn it_works() {
        let mut buffer = Vec::new();
        let aa = IncompatibleProtocolVersion::default();
        aa.encode_payload(&mut buffer);
        println!("done");
    }
}


