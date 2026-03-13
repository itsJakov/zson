use crate::{ObjectMap, Value};

pub trait Decodable: Sized {
    fn decode(value: Value) -> Option<Self>;
}

pub trait MapDecodable: Decodable {
    fn decode_from_map(map: &mut ObjectMap, key: &str) -> Option<Self> {
        let value = map.remove(key)?;
        Self::decode(value)
    }
}

impl Decodable for Value {
    fn decode(value: Value) -> Option<Self> {
        Some(value)
    }
}
impl MapDecodable for Value {}

impl Decodable for bool {
    fn decode(value: Value) -> Option<Self> {
        match value {
            Value::Bool(b) => Some(b),
            _ => None,
        }
    }
}
impl MapDecodable for bool {}

impl Decodable for i64 {
    fn decode(value: Value) -> Option<Self> {
        match value {
            Value::Number(n) => Some(n),
            _ => None,
        }
    }
}
impl MapDecodable for i64 {}

impl Decodable for String {
    fn decode(value: Value) -> Option<Self> {
        match value {
            Value::String(s) => Some(s),
            _ => None,
        }
    }
}
impl MapDecodable for String {}

impl<T: Decodable> Decodable for Option<T> {
    fn decode(value: Value) -> Option<Self> {
        match value {
            Value::None => Some(None),
            _ => Some(T::decode(value)),
        }
    }
}
impl<T: Decodable> MapDecodable for Option<T> {
    fn decode_from_map(map: &mut ObjectMap, key: &str) -> Option<Self> {
        match map.remove(key) {
            Some(value) => Self::decode(value),
            _ => Some(None),
        }
    }
}

impl<T: Decodable> Decodable for Vec<T> {
    fn decode(value: Value) -> Option<Self> {
        match value {
            Value::Array(a) => a.into_iter()
                .map(T::decode)
                .collect(),
            _ => None,
        }
    }
}
impl<T: Decodable> MapDecodable for Vec<T> {}