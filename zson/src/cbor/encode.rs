use crate::{ObjectMap, Value};
use crate::cbor::types::{MajorType, U8_ARG};

fn emit_type(buffer: &mut Vec<u8>, major: MajorType, argument: u8) {
    buffer.push(major as u8 | argument);
}

fn emit_type_len(buffer: &mut Vec<u8>, major: MajorType, length: u64) {
    if length <= 23 {
        emit_type(buffer, major, length as u8)
    } else if length < u8::MAX as u64 {
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
        Value::String(s) => encode_string(buffer, s),
        Value::Array(vec) => encode_array(buffer, vec),
        Value::Object(map) => encode_object(buffer, map),
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

fn encode_string(buffer: &mut Vec<u8>, value: String) {
    emit_type_len(buffer, MajorType::TextStr, value.len() as u64);
    for byte in value.as_bytes() {
        buffer.push(*byte);
    }
}

fn encode_array(buffer: &mut Vec<u8>, array: Vec<Value>) {
    emit_type_len(buffer, MajorType::Array, array.len() as u64);
    for value in array {
        encode_value(buffer, value);
    }
}

fn encode_object(buffer: &mut Vec<u8>, map: ObjectMap) {
    emit_type_len(buffer, MajorType::Map, map.len() as u64);
    for (key, value) in map {
        encode_string(buffer, key.into_owned());
        encode_value(buffer, value);
    }
}