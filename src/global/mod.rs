use std::sync::atomic::AtomicUsize;
use std::collections::HashMap;
use std::sync::Mutex;
use crate::structs::Block;

pub(crate) const ZERO: &str = "0";
pub(crate) const TARGET_EVERY_N_MILLIS_TO_GENERATE_BLOCK : usize= 5000;
pub(crate) static MILLISECOND_COUNTER: AtomicUsize = AtomicUsize::new(0);
pub(crate) static NONCE_COUNTER: AtomicUsize = AtomicUsize::new(0);
pub(crate) static NUM: AtomicUsize = AtomicUsize::new(0);
pub(crate) static DIFFICULTY: AtomicUsize = AtomicUsize::new(15); //bit 8 is two zero

lazy_static::lazy_static! {
    pub static ref BLOCK_CHAIN : Mutex<HashMap<String, Block>> = {
        Mutex::new(HashMap::new())
    };
    pub static ref LAST_HASH : Mutex<String> = Mutex::new(ZERO.to_string());
}