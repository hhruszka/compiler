use std::fmt;

pub struct TokenMatch {
    token_type: TokenType,
    haystack: String,
    token: String,
    start: usize,
    end: usize,
}

impl TokenMatch {
    pub fn new(line: String, token_type: TokenType, token: Option<String>) -> Self {
        Self {
            token_type: token_type,
            haystack: line.clone(),
            token: token.unwrap_or_else(|| line.clone()),
            start: 0,
            end: line.len(),
        }
    }

    pub fn from_match(line: String, token_type: TokenType, m: regex::Match) -> Self {
        Self {
            token_type: token_type,
            haystack: line.clone(),
            token: m.as_str().to_string(),
            start: m.start(),
            end: m.end(),
        }
    }

    pub fn token_type(&self) -> TokenType {
        self.token_type
    }
}

impl fmt::Display for TokenMatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}

impl fmt::Debug for TokenMatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TokenMatch {{ haystack: {}, token: {}, start: {}, end: {} }}",
            self.haystack, self.token, self.start, self.end
        )
    }
}

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
    Unknown,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
