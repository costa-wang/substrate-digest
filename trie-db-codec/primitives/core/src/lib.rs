// extern crate blake2_rfc;

pub mod hashing;

pub use hashing::{ blake2_256 };

pub mod hash;
mod hasher;

pub use self::hasher::blake2::Blake2Hasher;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
