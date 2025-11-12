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

pub static TOKENS: LazyLock<Vec<super::Lexer>> = LazyLock::new(|| {
    vec![
        super::Lexer::new(TokenType::Int, r"int\b"),
        super::Lexer::new(TokenType::Main, r"main\b"),
        super::Lexer::new(TokenType::Void, r"void\b"),
        super::Lexer::new(TokenType::Return, r"return\b"),
        super::Lexer::new(TokenType::IntConst, r"[0-9]+\b"),
        super::Lexer::new(TokenType::OpenBrace, r"\{"),
        super::Lexer::new(TokenType::CloseBrace, r"\}"),
        super::Lexer::new(TokenType::OpenParen, r"\("),
        super::Lexer::new(TokenType::CloseParen, r"\)"),
        super::Lexer::new(TokenType::Semicolon, r";"),
        super::Lexer::new(TokenType::Comment, r"//[\s\S]*"),
    ]
});
