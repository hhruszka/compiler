use super::tokens;
use regex::Regex;

pub struct Lexer {
    pub token_type: tokens::TokenType,
    pub expr: regex::Regex,
}

impl Lexer {
    pub fn new(token_type: tokens::TokenType, expr: &str) -> Self {
        Self {
            token_type,
            expr: Regex::new(expr).unwrap(),
        }
    }
}
