use crate::{ObjectMap, Value};

pub trait Decodable: Sized {
    fn decode(value: Value) -> Option<Self>;

    fn decode_from_object(map: &mut ObjectMap, key: &str) -> Option<Self> {
        let value = map.remove(key)?;
        Self::decode(value)
    }
}

impl Decodable for Value {
    fn decode(value: Value) -> Option<Self> {
        Some(value)
    }
}

impl Decodable for bool {
    fn decode(value: Value) -> Option<Self> {
        match value {
            Value::Bool(b) => Some(b),
            _ => None,
        }
    }
}

impl Decodable for i64 {
    fn decode(value: Value) -> Option<Self> {
        match value {
            Value::Number(n) => Some(n),
            _ => None,
        }
    }
}

impl Decodable for String {
    fn decode(value: Value) -> Option<Self> {
        match value {
            Value::String(s) => Some(s),
            _ => None,
        }
    }
}

impl<T: Decodable> Decodable for Option<T> {
    fn decode(value: Value) -> Option<Self> {
        match value {
            Value::None => Some(None),
            _ => Some(T::decode(value)),
        }
    }

    fn decode_from_object(map: &mut ObjectMap, key: &str) -> Option<Self> {
        match map.remove(key) {
            Some(value) => Self::decode(value),
            None => Some(None),
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