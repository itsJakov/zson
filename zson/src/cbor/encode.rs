use crate::{Value};
use crate::cbor::types::{MajorType, U8_ARG};

fn emit_type(buffer: &mut Vec<u8>, major: MajorType, argument: u8) {
    buffer.push(major as u8 | argument);
}

fn emit_type_len(buffer: &mut Vec<u8>, major: MajorType, length: u64) {
    if length <= 23 {
        emit_type(buffer, major, length as u8)
    } else if length < 2_u64.pow(8) {
        emit_type(buffer, major, U8_ARG);
        buffer.push(length as u8);
    } else {
        todo!();
    }
}

pub fn encode_value(buffer: &mut Vec<u8>, value: Value) {
    match value {
        Value::None => emit_type(buffer, MajorType::Primitive, 22),
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
    emit_type(buffer, MajorType::Primitive, key);
}

fn encode_number(buffer: &mut Vec<u8>, value: i64) {
    if value >= 0 {
        emit_type_len(buffer, MajorType::UnsignedInt, value as u64);
    } else {
        let negative = value.abs() - 1;
        emit_type_len(buffer, MajorType::NegativeInt, negative as u64);
    }
}