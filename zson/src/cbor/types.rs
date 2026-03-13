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

pub const U8_ARG: u8 = 24;
pub const U16_ARG: u8 = 25;
pub const U32_ARG: u8 = 26;
pub const U64_ARG: u8 = 27;
pub const INDEFINITE_ARG: u8 = 28;