use std::{ffi::OsString, fs};

use crate::analysis::Visitor;
mod analysis;
mod parser;
mod scanner;
mod test;
mod vm;
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

fn hello(args: Args) -> Result<(), miette::Report> {
    if args.file.is_some() {
        let contents = match fs::read_to_string(args.file.unwrap()) {
            Ok(r) => r,
            Err(_) => return Err(miette::miette!("main io error")),
        };
        let mut parse = parser::Parser::new(contents)?;
        let mut scope = analysis::Stack::new();
        let temp = parse.parse_program()?;
        println!("____________");
        println!("{:?}", temp);
        println!("____________");
        // for i in temp {
        //     scope.visit_stmnt(&i);
        // }
        // println!("val {:?}", scope.scope);
    } else {
        eprintln!("Must Provide a pos argument: rlox PATH_TO_FILE");
    }
    Ok(())
}

fn main() -> Result<(), miette::Report> {
    match parse_args() {
        Ok(x) => {
            return Ok(hello(x)?);
        }
        Err(_) => return Err(miette::miette!("hello")),
    };
}
