extern crate sha1;
extern crate uuid;

use sha1::{Digest, Sha1};
use uuid::Uuid;

fn generate_uuid_v5(namespace: Uuid, name: &str) -> Uuid {
    let mut hasher = Sha1::new();
    hasher.update(namespace.as_bytes());
    hasher.update(name.as_bytes());

    let hash = hasher.finalize();
    let mut bytes = [0u8; 16];

    bytes.copy_from_slice(&hash[..16]);

    bytes[6] = (bytes[6] & 0x0F) | 0x50;
    bytes[8] = (bytes[8] & 0x3F) | 0x80;

    Uuid::from_bytes(bytes)
}

pub fn uuid5(namespace: Uuid, name: &str) -> Uuid {
    let uuid3 = generate_uuid_v5(namespace, name);
    return uuid3
}
