use clap::{Args, Parser, Subcommand, ValueEnum};
use miette::Result;
mod scanner;
use std::{
    ffi::{OsStr, OsString},
    fs,
};
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
                    Ok(x) => scan(x),
                    Err(_) => Ok(()),
                }
            }
        }
    }
}

fn scan(input: &str) -> Result<()> {
    loop {
        let scanner = scanner::Scanner::new(input).generator();
        match scanner {
            Some(Ok(_)) => return Ok(()),
            Some(Err(e)) => return Err(miette::Report::from_err(e)),
            None => return Ok(()),
        };
    }
}
