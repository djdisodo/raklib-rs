#![feature(in_band_lifetimes)]
#[macro_use] extern crate derive_deref;

pub mod protocol;

pub const DEFAULT_PROTOCOL_VERSION: u8 = 6;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


