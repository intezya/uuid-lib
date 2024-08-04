extern crate rand;
extern crate uuid;

use std::time::{SystemTime, UNIX_EPOCH};
use chrono::prelude::*;
use rand::Rng;
use uuid::Uuid;

fn generate_uuid_v6() -> Uuid {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
    let timestamp_ms = now.as_millis();

    let timestamp = (timestamp_ms & 0xFFFFFFFFFFFFF) << 4;

    let mut rng = rand::thread_rng();
    let random_bits: u64 = rng.gen();
    let random_62_bits = random_bits & 0x3FFFFFFFFFFFFFFF;

    let version: u16 = 6 << 12;
    let time_high_and_version = ((timestamp >> 48) as u16) | version;
    let time_mid = ((timestamp >> 32) & 0xFFFF) as u16;
    let time_low = (timestamp & 0xFFFFFFFF) as u32;

    let clock_seq = ((random_62_bits >> 48) & 0x3FFF) as u16;
    let node = (random_62_bits & 0xFFFFFFFFFFFF) as u64;

    let mut d4 = [0u8; 8];
    d4[0] = (clock_seq >> 8) as u8;
    d4[1] = clock_seq as u8;
    d4[2..].copy_from_slice(&node.to_be_bytes()[2..]);

    let uuid = Uuid::from_fields(time_low, time_mid, time_high_and_version, &d4);
    uuid
}

pub fn uuid6() -> Uuid {
    let uuid6 = generate_uuid_v6();
    return uuid6
}
