mod encode;
mod decode;
mod lexer;

use crate::Encodable;
use encode::encode_value;
use crate::Decodable;
use crate::json::decode::parse;

pub fn encode_json<T: Encodable>(value: &T) -> String {
    encode_value(value.encode()).into_owned()
}

pub fn decode_json<T: Decodable>(json: &str) -> Option<T> {
    T::decode(parse(json)?)
}