use super::line;
use super::token_match;
use super::tokens;

// use crate::tokens;
use std::error::Error;

/// Run the lexer on the input string.
pub fn tokenize(data: &String) -> Result<Vec<token_match::TokenMatch>, Box<dyn Error>> {
    let mut tokens: Vec<token_match::TokenMatch> = Vec::new();

    for data in data.lines() {
        let mut line = line::Line::new(data.to_string());

        loop {
            line.skip_whitespace();
            if line.is_empty() {
                break;
            }

            let mut max_len = 0;
            let mut max_match: Option<regex::Match> = None;
            let mut token_type = tokens::TokenType::Unknown;

            let token_ref = line.remaining();
            let mut next_token_start: Option<usize> = None;

            for token_rx in tokens::TOKENS.iter() {
                if let Some(matched) = token_rx.expr.find(token_ref.as_str()) {
                    if matched.start() == 0 && matched.len() > max_len {
                        max_len = matched.len();
                        max_match = Some(matched);
                        token_type = token_rx.token_type;
                    } else if matched.start() > 0 {
                        next_token_start = match next_token_start {
                            Some(start) => Some(start.min(matched.start())),
                            None => Some(matched.start()),
                        };
                    }
                }
            }

            match max_match {
                Some(matched) => {
                    tokens.push(token_match::TokenMatch::from_match(
                        line.to_string(),
                        token_type,
                        matched.clone(),
                    ));
                    line.advance(max_len);

                    continue;
                }
                None => match next_token_start {
                    Some(pos) => {
                        // there is a garbage in from of the next token
                        tokens.push(token_match::TokenMatch::new(
                            line.to_string().split_at(pos).0.to_string(),
                            tokens::TokenType::Unknown,
                            Some(line.to_string().split_at(pos).0.to_string()),
                        ));
                        line.advance(pos);
                    }
                    None => {
                        // no more tokens to process in this line but some garbage present
                        tokens.push(token_match::TokenMatch::new(
                            line.to_string(),
                            tokens::TokenType::Unknown,
                            Some(line.to_string()),
                        ));
                        break;
                    }
                },
            }
        }
    }
    Ok(tokens)
}
