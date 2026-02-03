use clap::{Parser, Subcommand};
use miette::Result;
mod scanner;
use std::{
    ffi::{OsStr, OsString},
    fs,
};

use crate::scanner::TokenType;
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
    // Flag {
    //     flags: String,
    // },
}

fn main() -> Result<()> {
    let args = Cli::parse();
    println!("{:?}", args.command);
    match args.command {
        Commands::Input { path } => {
            if path.is_none() {
                unreachable!();
            } else {
                let path = path.as_deref().unwrap_or_else(|| OsStr::new(""));
                //match only for valid path
                match str::from_utf8(fs::read(path).unwrap().as_slice()) {
                    Err(_) => Ok(()),
                    Ok(x) => pull_out_tokens(x),
                }
            }
        }
    }
}

fn pull_out_tokens(input: &str) -> Result<()> {
    let mut scanner = scanner::Scanner::new(input); // mutable because generator mutates it

    while let Some(token_result) = scanner.generator() {
        match token_result {
            Ok(token) => {
                if token.kind == TokenType::Eof {
                    break;
                }
                println!("{:?}", token)
            }
            Err(e) => return Err(e.into()),
        }
    }
    Ok(())
}
