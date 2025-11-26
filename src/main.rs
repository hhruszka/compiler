mod cli;
mod lexer;
mod parser;

use lexer::token_match::TokenMatch;
use lexer::tokenizer::tokenize;
use lexer::tokens::TokenType;
use parser::parser::Parser;
use prettytable::format::{self};
use prettytable::{Table, row};
use std::fs;
use std::process::ExitCode;

fn print(tokens: Vec<TokenMatch>) -> ExitCode {
    let mut table = Table::new();
    let mut exit_code = ExitCode::SUCCESS;

    table.set_format(*format::consts::FORMAT_CLEAN);
    table.add_row(row!["Token Type", "Value"]);

    for token in tokens.iter() {
        if token.token_type() == TokenType::Unknown {
            exit_code = ExitCode::FAILURE;
        } else {
            table.add_row(row![token.token_type(), token]);
        }
    }
    if exit_code == ExitCode::FAILURE {
        eprintln!("Ilegal tokens detected");
    }
    table.printstd();
    exit_code
}

fn main() -> ExitCode {
    let args = cli::Args::new();
    let result = fs::read_to_string(args.get_filename());

    let data = match result {
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return ExitCode::FAILURE;
        }
        Ok(content) => content,
    };

    if args.get_command() == cli::Command::Lex {
        return match tokenize(&data) {
            Err(e) => {
                eprintln!("Lexer error: {}", e);
                ExitCode::FAILURE
            }
            Ok(tokens) => print(tokens),
        };
    }
    if args.get_command() == cli::Command::Parse {
        return match tokenize(&data) {
            Err(e) => {
                eprintln!("Lexer error: {}", e);
                return ExitCode::FAILURE;
            }
            Ok(tokens) => {
                let mut parser = Parser::new(tokens);
                match parser.parse_program() {
                    Err(e) => {
                        eprintln!("Parser error: {}", e);
                         ExitCode::FAILURE
                    },
                    Ok(nodes    ) => {
                        print!("{}",nodes);
                        ExitCode::SUCCESS
                    }
                }
            }
        };
    }
    ExitCode::SUCCESS
}
