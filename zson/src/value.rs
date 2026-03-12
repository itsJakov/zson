use std::borrow::Cow;
use std::collections::HashMap;

pub type ObjectMap = HashMap<Cow<'static, str>, Value>;

#[derive(Debug, Clone)]
pub enum Value {
    None,
    Bool(bool),
    Number(i64),
    String(String),
    Array(Vec<Value>),
    Object(ObjectMap)
}