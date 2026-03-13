#[repr(u8)]
pub enum MajorType {
    UnsignedInt,
    NegativeInt,
    ByteStr,
    TextStr,
    Array,
    Map,
    Tag,
    Primitive
}

