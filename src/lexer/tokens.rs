use super::lexer::Lexer;
use std::fmt;
use std::sync::LazyLock;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum TokenType {
    Int,
    Main,
    Void,
    Return,
    IntConst,
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    Semicolon,
    Comment,
    Unknown,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub static TOKENS: LazyLock<Vec<Lexer>> = LazyLock::new(|| {
    vec![
        Lexer::new(TokenType::Int, r"int\b"),
        Lexer::new(TokenType::Main, r"main\b"),
        Lexer::new(TokenType::Void, r"void\b"),
        Lexer::new(TokenType::Return, r"return\b"),
        Lexer::new(TokenType::IntConst, r"[0-9]+\b"),
        Lexer::new(TokenType::OpenBrace, r"\{"),
        Lexer::new(TokenType::CloseBrace, r"\}"),
        Lexer::new(TokenType::OpenParen, r"\("),
        Lexer::new(TokenType::CloseParen, r"\)"),
        Lexer::new(TokenType::Semicolon, r";"),
        Lexer::new(TokenType::Comment, r"//[\s\S]*"),
    ]
});
