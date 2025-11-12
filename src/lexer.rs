// use std::sync::OnceLock;
use regex::Regex;
use std::sync::LazyLock;
use std::error::Error;

pub enum Token {
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
}

pub struct Lexer {
    token: Token,
    expr: regex::Regex,
}

impl Lexer {
   pub fn new(token: Token, expr: &str) -> Self {
        Self {
            token,
            expr: Regex::new(expr).unwrap(),
        }
    }
}

// static TOKENS: OnceLock<Vec<Lexer>> = OnceLock::new();
//
// fn get_patterns() -> &'static Vec<Lexer> {
//     TOKENS.get_or_init(|| {
//         vec![
//             Lexer::new(Token::Int, r"int\b"),
//             Lexer::new(Token::Main, r"main\b"),
//             Lexer::new(Token::Void, r"void\b"),
//             Lexer::new(Token::Return, r"return\b"),
//             Lexer::new(Token::IntConst, r"\d+\b"),
//             Lexer::new(Token::OpenBrace, r"\{"),
//             Lexer::new(Token::CloseBrace, r"\}"),
//             Lexer::new(Token::OpenParen, r"\("),
//             Lexer::new(Token::CloseParen, r"\)"),
//             Lexer::new(Token::Semicolon, r";"),
//         ]
//     })
// }

static TOKENS: LazyLock<Vec<Lexer>> = LazyLock::new(|| {
    vec![
        Lexer::new(Token::Int, r"int\b"),
        Lexer::new(Token::Main, r"main\b"),
        Lexer::new(Token::Void, r"void\b"),
        Lexer::new(Token::Return, r"return\b"),
        Lexer::new(Token::IntConst, r"[0-9]+\b"),
        Lexer::new(Token::OpenBrace, r"\{"),
        Lexer::new(Token::CloseBrace, r"\}"),
        Lexer::new(Token::OpenParen, r"\("),
        Lexer::new(Token::CloseParen, r"\)"),
        Lexer::new(Token::Semicolon, r";"),
    ]
});

pub fn run_lexer(data: &String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut tokens :Vec<String>= Vec::new();

    let mut token = String::new();

    for line in data.lines() {
        for char in line.chars(){
            if char.is_whitespace(){
                if !token.is_empty() {
                    // eprintln!("Unexpected token {}", token);
                    Err(format!("Unexpected token {}", token))?;
                }
                continue;
            }
            token.push(char);
            if !token.is_empty() {
                for lexer in TOKENS.iter() {
                    if let Some(matched) = lexer.expr.find(&token) {
                        if matched.start() != 0 {
                            // eprintln!("Unexpected token {}", &token[0..matched.start()]);
                            return Err(format!("Unexpected token {}", &token[0..matched.start()]).into()); 
                        }
                        tokens.push(token.clone()); 
                        token.clear();
                        break;
                    }
                }
            }
        }
    }
    Ok(tokens)
}