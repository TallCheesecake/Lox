use anyhow::{self, Error, anyhow};
use clap::{Args, Parser, Subcommand, ValueEnum};
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
                match str::from_utf8(fs::read(path).unwrap().as_slice()) {
                    Err(_) => Ok(()),
                    Ok(x) => pull_out_token(x),
                }
            }
        }
    }
}

fn pull_out_token(input: &str) -> Result<scanner::Token<'_>> {
    let mut i = 0;
    i += 1;
    println!("it ran: {} times ", i);
    let scanner = scanner::Scanner::new(input).generator();
    match scanner {
        Some(Ok(x)) => return Ok(x),
        Some(Err(e)) => return Err(miette::Report::from_err(e)),
        None => return todo!(),
    };
}
