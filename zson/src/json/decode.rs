use std::collections::HashMap;
use crate::json::lexer::{Lexer, Token};
use crate::Value;

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
        Token::String => Some(Value::String(lexer.slice().to_owned())),
        Token::ArrayOpen => parse_array(lexer).map(Value::Array),
        _ => None // Unexpected or missing token
    }
}

fn parse_array(lexer: &mut Lexer) -> Option<Vec<Value>> {
    if let Token::ArrayClose = lexer.peak()? {
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

fn parse_object(lexer: &mut Lexer) -> Option<HashMap<String, Value>> {
    if let Token::ObjectClose = lexer.peak()? {
        lexer.advance();
        return Some(HashMap::new());
    }

    let mut object: HashMap<String, Value> = HashMap::new();

    // TODO: Implement object parsing

    Some(object)
}