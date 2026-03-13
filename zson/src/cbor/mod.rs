mod encode;
mod types;

use itertools::Itertools;
use crate::cbor::encode::encode_value;
use crate::Encodable;

pub fn encode_cbor<T: Encodable>(value: &T) -> Vec<u8> {
    let mut buffer = Vec::new();
    let value = value.encode();
    encode_value(&mut buffer, value);
    buffer
}

pub fn cbor_as_string(buffer: &[u8]) -> String {
    buffer.iter()
        .map(|&b| format!("{b:02X}"))
        .join(" ")
}