#[macro_use] extern crate derive_deref;
#[macro_use] extern crate num_enum;

pub mod protocol;
pub mod server;

pub const DEFAULT_PROTOCOL_VERSION: u8 = 6;
pub static SYSTEM_ADDRESS_COUNT: usize = 20;

#[cfg(test)]
mod tests {
    use crate::protocol::payload::IncompatibleProtocolVersion;
    use crate::protocol::Encode;

    #[test]
    fn it_works() {
        let mut buffer = Vec::new();
        let aa = IncompatibleProtocolVersion::default();
        aa.encode(&mut buffer);
        println!("done");
    }
}


