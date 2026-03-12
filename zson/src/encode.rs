use crate::Value;

pub trait Encodable {
    fn encode(&self) -> Value;
}

impl Encodable for Value {
    fn encode(&self) -> Value {
        self.clone()
    }
}

impl Encodable for bool {
    fn encode(&self) -> Value {
        Value::Bool(*self)
    }
}

impl Encodable for i64 {
    fn encode(&self) -> Value {
        Value::Number(*self)
    }
}

impl Encodable for String {
    fn encode(&self) -> Value {
        Value::String(self.clone())
    }
}

impl<T: Encodable> Encodable for Option<T> {
    fn encode(&self) -> Value {
        match self {
            Some(v) => v.encode(),
            None => Value::None
        }
    }
}

impl<T: Encodable> Encodable for Vec<T> {
    fn encode(&self) -> Value {
        Value::Array(self.iter().map(T::encode).collect())
    }
}