mod encode;
mod types;
mod decode;

use itertools::Itertools;
use crate::cbor::encode::encode_value;
use crate::{Decodable, Encodable};
use crate::cbor::decode::{decode_value, Cursor};

pub fn encode_cbor<T: Encodable>(value: &T) -> Vec<u8> {
    let mut buffer = Vec::new();
    encode_value(&mut buffer, value.encode());
    buffer
}

pub fn decode_cbor<T: Decodable>(buffer: &[u8]) -> Option<T> {
    let mut cursor = Cursor::new(buffer);
    let value = decode_value(&mut cursor)?;
    T::decode(value)
}

pub fn cbor_as_string(buffer: &[u8]) -> String {
    buffer.iter()
        .map(|&b| format!("{b:02X}"))
        .join(" ")
}