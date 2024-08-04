use rand::Rng;
use uuid::Uuid;

fn generate_uuid8() -> Uuid {
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 16];

    rng.fill(&mut bytes);

    bytes[6] = (bytes[6] & 0x0F) | 0x80;
    bytes[8] = (bytes[8] & 0x3F) | 0x80;

    let uuid = Uuid::from_bytes(bytes);

    return uuid.hyphenated().into_uuid()
}

pub fn uuid8() -> Uuid {
    let uuid8 = generate_uuid8();
    return uuid8
}
