use std::collections::HashMap;

#[derive(Debug)]
pub enum Value {
    None,
    Bool(bool),
    Number(i64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>)
}