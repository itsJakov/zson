use std::borrow::Cow;
use crate::json::lexer::{Lexer, Token};
use crate::{ObjectMap, Value};
use std::collections::HashMap;
use snailquote::unescape;

pub fn parse(input: &str) -> Option<Value> {
    let mut lexer = Lexer::new(input);
    parse_value(&mut lexer)
}

fn parse_value(lexer: &mut Lexer) -> Option<Value> {
    match lexer.advance()? {
        Token::Null => Some(Value::None),
        Token::False => Some(Value::Bool(false)),
        Token::True => Some(Value::Bool(true)),
        Token::Number => lexer.slice().parse::<i64>().ok().map(Value::Number),
        Token::String => parse_string(lexer).map(Value::String),
        Token::ArrayOpen => parse_array(lexer).map(Value::Array),
        Token::ObjectOpen => parse_object(lexer).map(Value::Object),
        _ => None // Unexpected or missing token
    }
}

fn parse_string(lexer: &mut Lexer) -> Option<String> {
    unescape(lexer.slice()).ok()
}

fn parse_array(lexer: &mut Lexer) -> Option<Vec<Value>> {
    if let Token::ArrayClose = lexer.peek()? {
        lexer.advance();
        return Some(Vec::new())
    }

    let mut array: Vec<Value> = Vec::new();

    loop {
        let value = parse_value(lexer)?;
        array.push(value);

        match lexer.advance()? {
            Token::Comma => continue,
            Token::ArrayClose => break,
            _ => return None // Expected comma or array close
        }
    }

    Some(array)
}

fn parse_object(lexer: &mut Lexer) -> Option<ObjectMap> {
    if let Token::ObjectClose = lexer.peek()? {
        lexer.advance();
        return Some(HashMap::new());
    }

    let mut object = ObjectMap::new();

    loop {
        let key = parse_value(lexer)?;
        let Value::String(key) = key else {
            // Object keys must be strings
            return None;
        };

        let Token::Colon = lexer.advance()? else {
            // Expected colon after key
            return None;
        };

        let value = parse_value(lexer)?;
        object.insert(Cow::Owned(key), value);

        match lexer.advance()? {
            Token::Comma => continue,
            Token::ObjectClose => break,
            _ => return None // Expected comma or object close
        }
    }

    Some(object)
}