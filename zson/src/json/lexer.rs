use logos::Logos;

#[derive(Debug, Logos)]
#[logos(skip r"[ \t\r\n\f]+")]
pub enum Token {
    #[token("null")]
    Null,

    #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?")]
    Number,

    #[regex(r#""([^"\\\x00-\x1F]|\\(["\\bnfrt/]|u[a-fA-F0-9]{4}))*""#)]
    String,

    #[token("false")]
    False,

    #[token("true")]
    True,

    #[token("{")]
    ObjectOpen,

    #[token("}")]
    ObjectClose,

    #[token("[")]
    ArrayOpen,

    #[token("]")]
    ArrayClose,

    #[token(":")]
    Colon,

    #[token(",")]
    Comma,
}

pub struct Lexer<'source> {
    logos: logos::Lexer<'source, Token>,
    peeked: Option<Token>,
}

impl Lexer<'_> {
    pub fn new(input: &'_ str) -> Lexer<'_> {
        Lexer {
            logos: Token::lexer(input),
            peeked: None,
        }
    }

    pub fn slice(&self) -> &str {
        self.logos.slice()
    }

    pub fn advance(&mut self) -> Option<Token> {
        if let Some(peaked) = self.peeked.take() {
            return Some(peaked);
        }
        self.logos.next()?.ok()
    }

    pub fn peek(&mut self) -> Option<&Token> {
        if self.peeked.is_none() {
            self.peeked = self.logos.next()?.ok();
        }
        self.peeked.as_ref()
    }
}