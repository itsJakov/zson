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

