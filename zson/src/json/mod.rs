mod encode;
use crate::Encodable;
use encode::encode_value;

pub fn encode_json<T: Encodable>(value: &T) -> String {
    encode_value(value.encode()).into_owned()
}
