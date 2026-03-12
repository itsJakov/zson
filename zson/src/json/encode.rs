use std::borrow::Cow;
use std::collections::HashMap;
use itertools::Itertools;
use snailquote::escape;
use crate::Value;

pub fn encode_value(value: Value) -> Cow<'static, str> {
    match value {
        Value::None => Cow::Borrowed("null"),
        Value::Bool(b) => Cow::Borrowed(if b { "true" } else { "false" }),
        Value::Number(number) => Cow::Owned(number.to_string()),
        Value::String(s) => Cow::Owned(format!("\"{}\"", escape(&s))), // TODO: snailquote is temporary
        Value::Array(vec) => encode_array(vec),
        Value::Object(map) => encode_object(map)
    }
}

fn encode_array(vec: Vec<Value>) -> Cow<'static, str> {
    if vec.is_empty() {
        return Cow::Borrowed("[]")
    }

    let values = vec
        .into_iter()
        .map(encode_value)
        .join(", ");

    Cow::Owned(format!("[{}]", values))
}

fn encode_object(map: HashMap<String, Value>) -> Cow<'static, str> {
    if map.is_empty() {
        return Cow::Borrowed("{}");
    }

    let pairs = map
        .into_iter()
        .map(|(key, value)| format!("\"{}\" : {}", key, encode_value(value)))
        .join(", ");

    Cow::Owned(format!("{{ {} }}", pairs))
}