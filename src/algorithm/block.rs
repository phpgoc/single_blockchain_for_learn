use crate::structs::Block;
use crate::global;
use std::sync::atomic::Ordering;
use crate::algorithm::sha256;
use std::time::Instant;
use crate::algorithm::moving_average::calculate_difficulty;

pub(crate) fn generate_block() -> Block{
    let mut last_hash = global::LAST_HASH.lock().unwrap();
    let data = "".to_string();
    let pre_hash = (*last_hash).clone();
    let  difficulty =  global::DIFFICULTY.load(Ordering::Relaxed);
    let (nonce, hash) = calculate_nonce(format!("{}{}",*last_hash,&data));
    *last_hash = hash.clone();
    Block{
        pre_hash,
        nonce,
        data,
        hash,
        difficulty
    }
}
pub(crate) fn calculate_nonce(input : String) -> (usize,String){
    let mut nonce = 0;
    let target = sha256::hard_2_target();
    let now = Instant::now();
    loop{
        let hash = sha256::sha256(format!("{}{}",input,nonce));
        if sha256::hex2_u256(&hash) > target {
            nonce += 1;
            continue;
        }
        calculate_difficulty(now.elapsed().as_millis() as usize, nonce);
        return (nonce,hash)
    }
}