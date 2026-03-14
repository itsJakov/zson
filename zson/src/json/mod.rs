mod encode;
mod decode;
mod lexer;

use crate::json::decode::parse;
use crate::{Coder, Encodable, Decodable};
use encode::encode_value;

pub struct JSON;
impl Coder for JSON {
    type Type = str;

    fn encode<T: Encodable>(value: &T) -> String {
        encode_value(value.encode()).into_owned()
    }

    fn decode<T: Decodable>(value: &str) -> Option<T> {
        T::decode(parse(&value)?)
    }
}