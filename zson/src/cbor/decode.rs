use crate::cbor::types::{MajorType, INDEFINITE_ARG, U16_ARG, U32_ARG, U64_ARG, U8_ARG};
use crate::{ObjectMap, Value};
use std::borrow::Cow;

pub struct Cursor<'a> {
    buffer: &'a [u8],
    pos: usize
}

impl<'a> Cursor<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self { buffer, pos: 0 }
    }

    fn get_byte(&mut self) -> Option<u8> {
        let byte = self.buffer.get(self.pos)?;
        self.pos += 1;
        Some(*byte)
    }

    fn get_bytes(&mut self, bytes: usize) -> Option<Vec<u8>> {
        let slice = &self.buffer.get(self.pos..self.pos + bytes)?;
        self.pos += bytes;
        Some(slice.to_vec())
    }

    fn get_type(&mut self) -> Option<(MajorType, u8)> {
        let byte = self.get_byte()?;
        let major_type = MajorType::try_from(byte).ok()?;
        let argument = byte & 0x1F; // Bottom 5 bits
        Some((major_type, argument))
    }
}

fn decode_type_len(cursor: &mut Cursor, argument: u8) -> Option<u64> {
    match argument {
        U8_ARG => Some(cursor.get_byte()? as u64),
        U16_ARG => None,
        U32_ARG => None,
        U64_ARG => None,
        INDEFINITE_ARG => None,
        _ => Some(argument as u64)
    }
}

pub fn decode_value(cursor: &mut Cursor) -> Option<Value> {
    match cursor.get_type()? {
        (MajorType::UnsignedInt, arg) => decode_unsigned(cursor, arg).map(Value::Number),
        (MajorType::NegativeInt, arg) => decode_negative(cursor, arg).map(Value::Number),
        (MajorType::TextStr, arg) => decode_string(cursor, arg).map(Value::String),
        (MajorType::Array, arg) => decode_array(cursor, arg).map(Value::Array),
        (MajorType::Map, arg) => decode_map(cursor, arg).map(Value::Object),
        (MajorType::Primitive, arg) => decode_primitive(cursor, arg),
        _ => None // Unsupported major type
    }
}

fn decode_unsigned(cursor: &mut Cursor, argument: u8) -> Option<i64> {
    let value = decode_type_len(cursor, argument)? as i64;
    Some(value)
}

fn decode_negative(cursor: &mut Cursor, argument: u8) -> Option<i64> {
    let value = decode_type_len(cursor, argument)? as i64;
    Some(-(value + 1))
}

fn decode_string(cursor: &mut Cursor, argument: u8) -> Option<String> {
    let length = decode_type_len(cursor, argument)? as usize;
    String::from_utf8(cursor.get_bytes(length)?).ok()
}

fn decode_array(cursor: &mut Cursor, argument: u8) -> Option<Vec<Value>> {
    let length = decode_type_len(cursor, argument)? as usize;

    let mut array = Vec::with_capacity(length);
    for _ in 0..length {
        array.push(decode_value(cursor)?);
    }

    Some(array)
}

fn decode_map(cursor: &mut Cursor, argument: u8) -> Option<ObjectMap> {
    let length = decode_type_len(cursor, argument)? as usize;

    let mut map = ObjectMap::with_capacity(length);
    for _ in 0..length {
        let Value::String(key) = decode_value(cursor)? else {
            // Keys have to be Strings
            return None;
        };

        map.insert(Cow::Owned(key), decode_value(cursor)?);
    }

    Some(map)
}

fn decode_primitive(cursor: &mut Cursor, argument: u8) -> Option<Value> {
    match argument {
        20 => Some(Value::Bool(false)),
        21 => Some(Value::Bool(true)),
        22 => Some(Value::None),
        _ => None // Unsupported primitive value
    }
}
