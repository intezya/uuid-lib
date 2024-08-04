extern crate md5;
extern crate uuid;

use md5::Context;
use uuid::Uuid;

fn generate_uuid_v3(namespace: Uuid, name: &str) -> Uuid {
    let mut context = Context::new();
    context.consume(namespace.as_bytes());
    context.consume(name.as_bytes());

    let hash = context.compute();
    let mut bytes = [0u8; 16];

    bytes.copy_from_slice(&hash.0);

    bytes[6] = (bytes[6] & 0x0F) | 0x30;
    bytes[8] = (bytes[8] & 0x3F) | 0x80;

    Uuid::from_bytes(bytes)
}

pub fn generate(namespace: Uuid, name: &str) -> Uuid {
    let uuid3 = generate_uuid_v3(namespace, name);
    return uuid3
}
