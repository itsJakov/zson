use crate::{Value};
use crate::cbor::types::MajorType;

pub fn encode_value(buffer: &mut Vec<u8>, value: Value) {
    match value {
        Value::None => buffer.push(((MajorType::Primitive as u8) << 5) | 22),
        _ => {}
    }
}