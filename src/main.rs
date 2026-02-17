use std::{ffi::OsString, fs};
mod exe;
mod parser;
mod scanner;

struct Args {
    file: Option<OsString>,
}
fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;
    let mut file = None;
    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Value(val) if file.is_none() => {
                file = Some(val);
            }
            Long("help") => {
                println!(
                    "Usage: provide a file-path as the first pos arg \nExample: rlox main.lox"
                );
            }
            _ => return Err(arg.unexpected()),
        }
    }
    Ok(Args {
        file: file.or_else(|| None),
    })
}

fn hello(args: Args) -> Result<(), miette::Error> {
    if args.file.is_some() {
        let contents = match fs::read_to_string(args.file.unwrap()) {
            Ok(r) => r,
            Err(_) => return Err(miette::miette!("io error")),
        };
        let mut p = parser::Parser::new(contents.as_str())?;
        let out = p.parse_statment()?;
        println!("{:?}", out);
    } else {
        eprintln!("Must Provide a pos argument: rlox PATH_TO_FILE");
    }
    Ok(())
}
fn main() -> Result<(), miette::Error> {
    match parse_args() {
        Ok(x) => {
            return Ok(hello(x)?);
        }
        Err(_) => return Err(miette::miette!("hello")),
    };
}
