use crate::{Value};
use crate::cbor::types::MajorType;

pub fn encode_value(buffer: &mut Vec<u8>, value: Value) {
    match value {
        Value::None => buffer.push(MajorType::Primitive as u8 | 22),
        Value::Bool(b) => encode_bool(buffer, b),
        Value::Number(n) => encode_number(buffer, n),
        _ => {}
    }
}

fn encode_bool(buffer: &mut Vec<u8>, value: bool) {
    let key = match value {
        false => 20,
        true => 21
    };
    buffer.push(MajorType::Primitive as u8 | key);
}

fn encode_number(buffer: &mut Vec<u8>, value: i64) {
    if value >= 0 && value <= 23 {
        buffer.push(MajorType::UnsignedInt as u8 | value as u8);
        return;
    }

    // If fits in one byte
    if value < 2_i64.pow(8) {
        buffer.push(MajorType::UnsignedInt as u8 | 24);
        buffer.push(value as u8);
        return;
    }

    todo!()
}