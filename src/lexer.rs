use regex::Regex;
use std::error::Error;
use std::fmt;
use std::ops::RangeBounds;
use std::sync::LazyLock;

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

pub struct Lexer {
    token_type: TokenType,
    expr: regex::Regex,
}

impl Lexer {
    pub fn new(token_type: TokenType, expr: &str) -> Self {
        Self {
            token_type,
            expr: Regex::new(expr).unwrap(),
        }
    }
}

static TOKENS: LazyLock<Vec<Lexer>> = LazyLock::new(|| {
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
    ]
});

struct Line {
    data: String,
    start: usize,
    current: usize,
}

impl Line {
    fn new(data: String) -> Self {
        Self {
            data,
            start: 0,
            current: 0,
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn advance(&mut self, n: usize) {
        if n >= self.data.len() {
            self.data = String::new();
        } else {
            self.data = self.data[n..].to_string();
        }
    }

    fn peek(&self) -> Option<char> {
        if self.current == self.data.len() - 1 {
            None
        } else {
            self.data.chars().nth(self.current)
        }
    }

    fn peek_next(&self) -> Option<char> {
        if self.current + 1 >= self.data.len() {
            None
        } else {
            self.data.chars().nth(self.current + 1)
        }
    }

    fn skip_whitespace(&mut self) {
        self.data = self.data.trim_start().to_string();
    }

    fn get_token(&mut self) -> String {
        let str = self.data[self.start..self.current].to_string();
        // self.start = self.current;
        // self.current += 1;
        str.clone()
    }

    fn as_str(&self) -> &str {
        self.data.as_str()
    }

    fn to_string(&self) -> String {
        self.data.clone()
    }

    fn slice_to_string(&self, range: impl RangeBounds<usize>) -> String {
        use std::ops::Bound;

        let len = self.data.len();
        let start = match range.start_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n + 1,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&n) => n + 1,
            Bound::Excluded(&n) => n,
            Bound::Unbounded => len,
        };
        self.data[start..end].to_string()
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}
/// Run the lexer on the input string.
pub fn run_lexer(data: &String) -> Result<Vec<TokenMatch>, Box<dyn Error>> {
    let mut tokens: Vec<TokenMatch> = Vec::new();

    for data in data.lines() {
        let mut line = Line::new(data.to_string());
        let mut cnt = 0;
        loop {
            line.skip_whitespace();
            if line.is_empty() {
                break;
            }
            cnt += 1;
            if cnt == 10 {
                break;
            }

            let mut max_len = 0;
            let mut max_match: Option<regex::Match> = None;
            let mut token_type = TokenType::Unknown;

            let token_ref = line.to_string();
            let mut token_start = 0;

            for token_rx in TOKENS.iter() {
                if let Some(matched) = token_rx.expr.find(token_ref.as_str()) {
                    if matched.start() == 0 && matched.len() > max_len {
                        max_len = matched.len();
                        max_match = Some(matched);
                        token_type = token_rx.token_type.clone();
                    } else if matched.start() > 0 {
                        if token_start > 0 && matched.start() < token_start {
                            token_start = matched.start();
                        }
                        if token_start == 0 {
                            token_start = matched.start();
                        }
                    }
                }
            }

            if max_match.is_some() {
                let matched = max_match.unwrap();
                tokens.push(TokenMatch::from_match(
                    line.to_string(),
                    token_type,
                    matched.clone(),
                ));
                line.advance(max_len);

                continue;
            }

            if max_match.is_none() {
                if token_start > 0 {
                    tokens.push(TokenMatch::new(
                        line.to_string().split_at(token_start).0.to_string(),
                        TokenType::Unknown,
                        Some(line.to_string().split_at(token_start).0.to_string()),
                    ));
                    line.advance(token_start);
                } else {
                    tokens.push(TokenMatch::new(
                        line.to_string(),
                        TokenType::Unknown,
                        Some(line.to_string()),
                    ));
                    break;
                }
            }
        }
    }
    Ok(tokens)
}
