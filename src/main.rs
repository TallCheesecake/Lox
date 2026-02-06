use clap::{Parser, Subcommand};
use miette::{Context, Error, Result};
mod parser;
mod scanner;
use std::{
    ffi::{OsStr, OsString},
    fs,
};

use crate::parser::{self as parse};
use crate::scanner::{MyBad, Token, TokenType};

/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "git")]
#[command(about = "A fictional versioning CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Input {
        #[arg(value_name = "PATH")]
        path: Option<OsString>,
    },
}

fn main() -> Result<()> {
    let args = Cli::parse();
    println!("{:?}", args.command);

    let Commands::Input { path } = args.command;
    let path = path.as_deref().unwrap_or_else(|| OsStr::new(""));

    let bytes = fs::read(path).unwrap_or_else(|_| {
        panic!("failed to read file {:?}", path);
    });

    let input = std::str::from_utf8(&bytes).unwrap();

    test(input)?;

    Ok(())
}

fn test(input: &str) -> miette::Result<Vec<Token<'_>>> {
    parse::Parser::construct(input)
}
