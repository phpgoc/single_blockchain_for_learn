use crate::global::{MILLISECOND_COUNTER, NUM, NONCE_COUNTER, TARGET_EVERY_N_MILLIS_TO_GENERATE_BLOCK, DIFFICULTY};
use std::sync::atomic::Ordering;

pub(crate) fn calculate_difficulty(new_block_mills : usize, nonce:usize){
    MILLISECOND_COUNTER.fetch_add(new_block_mills , Ordering::Relaxed);
    let millisecond_count = MILLISECOND_COUNTER.load(Ordering::Relaxed);
    NUM.fetch_add(1,Ordering::Relaxed);
    let num = NUM.load(Ordering::Relaxed);
    NONCE_COUNTER.fetch_add(nonce,Ordering::Relaxed);
    let nonce_count = NONCE_COUNTER.load(Ordering::Relaxed);
    if num % 10 == 0{
        println!("num = {}, time/num = {}", num, millisecond_count/num);
    }
    let target_milliseconds = if (num +1)* TARGET_EVERY_N_MILLIS_TO_GENERATE_BLOCK > millisecond_count{
         (num +1)* TARGET_EVERY_N_MILLIS_TO_GENERATE_BLOCK  - millisecond_count
    }else{
        DIFFICULTY.store(4,Ordering::Relaxed);
        return;
    };
    let target_nonce = nonce_count as u128 * target_milliseconds as u128 / millisecond_count as u128;
    // dbg!(nonce_count,target_milliseconds,millisecond_count,target_nonce);
    let difficulty = (target_nonce as f64).log(2.0) as usize;
    DIFFICULTY.store(difficulty,Ordering::Relaxed);

}