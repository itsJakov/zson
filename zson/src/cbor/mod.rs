mod encode;
mod types;
mod decode;

use std::ops::Deref;
use crate::cbor::decode::{decode_value, Cursor};
use crate::cbor::encode::encode_value;
use crate::{Coder, Decodable, Encodable};
use itertools::Itertools;

pub struct CBOR;
impl Coder for CBOR {
    type Type = [u8];

    fn encode<T: Encodable>(value: &T) -> Vec<u8> {
        let mut buffer = Vec::new();
        encode_value(&mut buffer, value.encode());
        buffer
    }

    fn decode<T: Decodable>(value: &[u8]) -> Option<T> {
        let mut cursor = Cursor::new(value);
        let value = decode_value(&mut cursor)?;
        T::decode(value)
    }
}

impl CBOR {
    pub fn hex_string(buffer: impl Deref<Target = [u8]>) -> String {
        buffer.iter()
            .map(|&b| format!("{b:02X}"))
            .join(" ")
    }
}