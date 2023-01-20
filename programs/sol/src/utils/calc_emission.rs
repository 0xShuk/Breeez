use anchor_lang::prelude::*;

pub fn calc_emission(stake_time: i64, emission: u64) -> u64 {
    let clock = Clock::get().unwrap();
    let current_time = clock.unix_timestamp;

    let reward_time = (current_time - stake_time) as u64;

    let reward = reward_time * emission * 1000000000;

    reward / 3600
}