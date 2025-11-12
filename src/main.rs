// use crate::lexer::run_lexer;
use std::fs;
use std::process::ExitCode;

mod cli;
mod lexer;

fn main() -> ExitCode {
    let args = cli::Args::new();
    // println!("File name: {}", args.get_filename());
    // println!("Command: {}", args.get_command());

    let result = fs::read_to_string(args.get_filename());
    let data = match result {
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return ExitCode::FAILURE;
        }
        Ok(content) => content,
    };
    let mut unknown_tokens = false;
    if args.get_command() == cli::Command::Lex {
        // println!("Running lexer...");
        match lexer::run_lexer(&data) {
            Err(e) => {
                eprintln!("Lexer error: {}", e);
                return ExitCode::FAILURE;
            }
            Ok(tokens) => {
                for token in tokens.iter() {
                    // eprintln!("Found tokens: {} type: {}", token, token.token_type());
                    if token.token_type() == lexer::token::TokenType::Unknown {
                        unknown_tokens = true;
                        break;
                    }
                }
            }
        }
    }

    if unknown_tokens {
        eprintln!("Ilegal tokens detected");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
