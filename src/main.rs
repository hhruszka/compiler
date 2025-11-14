mod cli;
mod lexer;

use lexer::tokenizer::tokenize;
use prettytable::format::{self};
use prettytable::{Table, row};
use std::fs;
use std::process::ExitCode;
use lexer::tokens::TokenType;

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
        // println!("Running lexer...");
        match tokenize(&data) {
            Err(e) => {
                eprintln!("Lexer error: {}", e);
                return ExitCode::FAILURE;
            }
            Ok(tokens) => {
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
                return exit_code;
            }
        }
    }
    return ExitCode::SUCCESS;
}
