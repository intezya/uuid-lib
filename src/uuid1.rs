extern crate chrono;
extern crate rand;
extern crate uuid;

use chrono::prelude::*;
use rand::Rng;
use uuid::Uuid;

fn generate_uuid_v1() -> Uuid {
    let utc_now = Utc::now().timestamp_nanos_opt().unwrap() as u64;

    // Получаем младшие 32 бита временной метки UTC
    let time_low = (utc_now & 0xFFFFFFFF) as u32;

    // Получаем средние 16 битов временной метки UTC
    let time_mid = ((utc_now >> 32) & 0xFFFF) as u16;

    // Получаем старшие 12 битов временной метки UTC
    let time_hi = ((utc_now >> 48) & 0x0FFF) as u16;

    // Версия UUID (1)
    let time_hi_and_version = (1 << 12) | time_hi;

    // Генерируем случайные данные для ClockSequence
    let mut rng = rand::thread_rng();
    let clock_seq: u16 = rng.random();

    let clock_seq_hi_and_res = ((clock_seq >> 8) & 0x3F) | 0x80;
    let clock_seq_low = (clock_seq & 0xFF) as u8;

    let mut node_id = [0u8; 6];
    rng.fill(&mut node_id);

    let uuid_bytes = [
        (time_low >> 24) as u8,
        (time_low >> 16) as u8,
        (time_low >> 8) as u8,
        time_low as u8,
        (time_mid >> 8) as u8,
        time_mid as u8,
        (time_hi_and_version >> 8) as u8,
        time_hi_and_version as u8,
        clock_seq_hi_and_res as u8,
        clock_seq_low,
        node_id[0], node_id[1], node_id[2], node_id[3], node_id[4], node_id[5]
    ];

    Uuid::from_bytes(uuid_bytes)
}

pub fn uuid1() -> Uuid {
    let uuid1 = generate_uuid_v1();
    return uuid1
}