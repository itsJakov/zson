#[repr(u8)]
pub enum MajorType {
    UnsignedInt = 0 << 5,
    NegativeInt = 1 << 5,
    ByteStr = 2 << 5,
    TextStr = 3 << 5,
    Array = 4 << 5,
    Map = 5 << 5,
    Tag = 6 << 5,
    Primitive = 7 << 5,
}

impl TryFrom<u8> for MajorType {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match (value & 0xE0) >> 5 {
            0 => Ok(MajorType::UnsignedInt),
            1 => Ok(MajorType::NegativeInt),
            2 => Ok(MajorType::ByteStr),
            3 => Ok(MajorType::TextStr),
            4 => Ok(MajorType::Array),
            5 => Ok(MajorType::Map),
            6 => Ok(MajorType::Tag),
            7 => Ok(MajorType::Primitive),
            _ => Err(())
        }
    }
}

pub const U8_ARG: u8 = 24;
pub const U16_ARG: u8 = 25;
pub const U32_ARG: u8 = 26;
pub const U64_ARG: u8 = 27;
pub const INDEFINITE_ARG: u8 = 28;