use  crate::structs::U256;
use sha2::Digest;
use crate::global;
use std::sync::atomic::Ordering;

pub(crate) fn sha256(input : String) -> String {
    hex::encode(sha2::Sha256::digest(input.as_bytes()))
}
pub(crate) fn hex2_u256(hex : &String) -> U256 {
    U256::from_str_radix(hex,16).unwrap()
}
pub(crate) fn hard_2_target() -> U256{
    let hard = global::DIFFICULTY.load(Ordering::Relaxed);
    let mut res = U256::MAX;
    for _ in 0..hard{
        res >>= 1;
    }
    res
}