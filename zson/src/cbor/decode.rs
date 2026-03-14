use crate::cbor::types::{MajorType, U8_ARG, U16_ARG, U32_ARG, U64_ARG, INDEFINITE_ARG};
use crate::Value;

pub struct Cursor<'a> {
    buffer: &'a [u8],
    pos: usize
}

impl<'a> Cursor<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self { buffer, pos: 0 }
    }

    fn get_byte(&mut self) -> u8 {
        let byte = self.buffer[self.pos];
        self.pos += 1; // TODO: bounds checking pls
        byte
    }

    fn get_bytes(&mut self, bytes: usize) -> &'a [u8] {
        let slice = &self.buffer[self.pos..self.pos + bytes];
        self.pos += bytes; // TODO: bounds checking pls
        slice
    }

    fn get_type(&mut self) -> Option<(MajorType, u8)> {
        let byte = self.get_byte();
        let major_type = MajorType::try_from(byte).ok()?;
        let argument = byte & 0x1F;
        Some((major_type, argument))
    }
}

fn decode_type_len(cursor: &mut Cursor, argument: u8) -> Option<u64> {
    match argument {
        U8_ARG => Some(cursor.get_byte() as u64),
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
    String::from_utf8(cursor.get_bytes(length).to_vec()).ok()
}