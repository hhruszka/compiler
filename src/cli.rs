use clap::{ArgGroup, Parser};
use std::fmt;

#[derive(PartialEq)]
pub enum Command {
    Lex,
    Parse,
    Codegen,
    None,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Lex => write!(f, "lex"),
            Command::Parse => write!(f, "parse"),
            Command::Codegen => write!(f, "code-gen"),
            Command::None => write!(f, "none"),
        }
    }
}

/// Simple program to greet a person
#[derive(Parser)]
#[command(name = "compiler")]
#[command(version, about, long_about = None)]
#[command(group(
    ArgGroup::new("command")
        .required(true)
        .args(&["lex", "parse", "codegen"])
))]
pub struct Args {
    /// run lexer only
    #[arg(long)]
    lex: bool,

    /// run parser only
    #[arg(long)]
    parse: bool,

    /// run lexer, parser and then generate assembly only
    #[arg(long = "code-gen")]
    codegen: bool,

    /// file name
    #[arg(required = true, help = "file name")]
    file_name: String,
}

impl Args {
    pub fn new() -> Self {
        let args = Self::parse();
        return args;
    }

    pub fn get_command(&self) -> Command {
        if self.lex {
            return Command::Lex;
        }
        if self.parse {
            return Command::Parse;
        }
        if self.codegen {
            return Command::Codegen;
        }
        return Command::None;
    }

    pub fn get_filename(&self) -> &str {
        return self.file_name.as_str();
    }
}
