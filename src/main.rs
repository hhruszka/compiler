use crate::lexer::run_lexer;
use std::process::ExitCode;
use std::fs;

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
        },
        Ok(content) => content,
    };
    if args.get_command() == cli::Command::Lex {
        // println!("Running lexer...");
        match run_lexer(&data) {
            Err(e) => {
                // eprintln!("Lexer error: {}", e);
                return ExitCode::FAILURE;
            },
            Ok(tokens) => {
                // for token in tokens.iter() {
                //     println!("{}", token);
                // }
            }
        }
    }

    ExitCode::SUCCESS
}
