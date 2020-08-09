#![feature(in_band_lifetimes)]
#[macro_use] extern crate derive_deref;

pub mod protocol;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


