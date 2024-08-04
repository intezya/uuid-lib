extern crate rand;
extern crate uuid;

use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

fn generate_uuid_v7() -> Uuid {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
    let unix_ts_ms = now.as_millis();
    let timestamp = (unix_ts_ms & 0xFFFFFFFFFFFF) as u64;

    let mut rng = rand::thread_rng();
    let rand_a: u16 = rng.gen_range(0..0x1000);
    let rand_b: u64 = rng.gen_range(0..(1u64 << 62));

    let mut uuid_bytes = [0u8; 16];

    uuid_bytes[0] = (timestamp >> 40) as u8;
    uuid_bytes[1] = (timestamp >> 32) as u8;
    uuid_bytes[2] = (timestamp >> 24) as u8;
    uuid_bytes[3] = (timestamp >> 16) as u8;
    uuid_bytes[4] = (timestamp >> 8) as u8;
    uuid_bytes[5] = timestamp as u8;
    uuid_bytes[6] = ((7 << 4) | ((rand_a >> 8) & 0xF)) as u8;
    uuid_bytes[7] = rand_a as u8;
    uuid_bytes[8] = ((2 << 6) | ((rand_b >> 56) & 0x3F) as u8) as u8;
    uuid_bytes[9] = (rand_b >> 48) as u8;
    uuid_bytes[10] = (rand_b >> 40) as u8;
    uuid_bytes[11] = (rand_b >> 32) as u8;
    uuid_bytes[12] = (rand_b >> 24) as u8;
    uuid_bytes[13] = (rand_b >> 16) as u8;
    uuid_bytes[14] = (rand_b >> 8) as u8;
    uuid_bytes[15] = rand_b as u8;

    Uuid::from_bytes(uuid_bytes)
}

pub fn generate() -> Uuid{
    let uuid7 = generate_uuid_v7();
    return uuid7
}
