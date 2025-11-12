use regex::Regex;
use std::error::Error;
use std::fmt;
use std::ops::RangeBounds;
use std::sync::LazyLock;

mod line;
pub mod token;

pub struct Lexer {
    token_type: token::TokenType,
    expr: regex::Regex,
}

impl Lexer {
    pub fn new(token_type: token::TokenType, expr: &str) -> Self {
        Self {
            token_type,
            expr: Regex::new(expr).unwrap(),
        }
    }
}

static TOKENS: LazyLock<Vec<Lexer>> = LazyLock::new(|| {
    vec![
        Lexer::new(token::TokenType::Int, r"int\b"),
        Lexer::new(token::TokenType::Main, r"main\b"),
        Lexer::new(token::TokenType::Void, r"void\b"),
        Lexer::new(token::TokenType::Return, r"return\b"),
        Lexer::new(token::TokenType::IntConst, r"[0-9]+\b"),
        Lexer::new(token::TokenType::OpenBrace, r"\{"),
        Lexer::new(token::TokenType::CloseBrace, r"\}"),
        Lexer::new(token::TokenType::OpenParen, r"\("),
        Lexer::new(token::TokenType::CloseParen, r"\)"),
        Lexer::new(token::TokenType::Semicolon, r";"),
    ]
});

/// Run the lexer on the input string.
pub fn run_lexer(data: &String) -> Result<Vec<token::TokenMatch>, Box<dyn Error>> {
    let mut tokens: Vec<token::TokenMatch> = Vec::new();

    for data in data.lines() {
        let mut line = line::Line::new(data.to_string());
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
            let mut token_type = token::TokenType::Unknown;

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
                tokens.push(token::TokenMatch::from_match(
                    line.to_string(),
                    token_type,
                    matched.clone(),
                ));
                line.advance(max_len);

                continue;
            }

            if max_match.is_none() {
                if token_start > 0 {
                    tokens.push(token::TokenMatch::new(
                        line.to_string().split_at(token_start).0.to_string(),
                        token::TokenType::Unknown,
                        Some(line.to_string().split_at(token_start).0.to_string()),
                    ));
                    line.advance(token_start);
                } else {
                    tokens.push(token::TokenMatch::new(
                        line.to_string(),
                        token::TokenType::Unknown,
                        Some(line.to_string()),
                    ));
                    break;
                }
            }
        }
    }
    Ok(tokens)
}
