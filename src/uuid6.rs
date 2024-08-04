extern crate chrono;
extern crate rand;
extern crate uuid;

use chrono::prelude::*;
use rand::Rng;
use uuid::Uuid;

fn generate_uuid_v6() -> Uuid {
    let utc_now = Utc::now().timestamp_nanos_opt().unwrap() as u64;

    let time_high = (utc_now >> 28) & 0xFFFFFFF;
    let time_mid = (utc_now >> 12) & 0xFFFF;
    let time_low = (utc_now & 0xFFF) as u16;

    let mut rng = rand::thread_rng();
    let clock_seq: u16 = rng.random();

    // Устанавливаем вариант UUID
    let clock_seq_hi_and_res = ((clock_seq >> 8) & 0x3F) | 0x80;
    let clock_seq_low = (clock_seq & 0xFF) as u8;

    let mut node_id = [0u8; 6];
    rng.fill(&mut node_id);

    let uuid_bytes: uuid::Bytes = [
        (time_high >> 24) as u8,
        (time_high >> 16) as u8,
        (time_high >> 8) as u8,
        time_high as u8,
        (time_mid >> 8) as u8,
        time_mid as u8,
        (time_low >> 8) as u8,
        time_low as u8,
        clock_seq_hi_and_res as u8,
        clock_seq_low,
        node_id[0], node_id[1], node_id[2], node_id[3], node_id[4], node_id[5]
    ];

    Uuid::from_bytes(uuid_bytes)
}

pub fn uuid6() -> Uuid {
    let uuid6 = generate_uuid_v6();
    return uuid6
}
