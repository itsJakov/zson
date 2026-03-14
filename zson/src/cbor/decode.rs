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
        _ => todo!()
    }
}

fn decode_unsigned(cursor: &mut Cursor, argument: u8) -> Option<i64> {
    let value = decode_type_len(cursor, argument)?;
    Some(value as i64)
}