// use clap::{Parser, Subcommand};
// use miette::{Context, Error, Result};
mod parser;
mod scanner;
// use std::{
//     ffi::{OsStr, OsString},
//     fs,
// };
//
//
// /// A fictional versioning CLI
// #[derive(Debug, Parser)] // requires `derive` feature
// #[command(name = "git")]
// #[command(about = "A fictional versioning CLI", long_about = None)]
// struct Cli {
//     #[command(subcommand)]
//     command: Commands,
// }
// #[derive(Debug, Subcommand)]
// enum Commands {
//     #[command(arg_required_else_help = true)]
//     Input {
//         #[arg(value_name = "PATH")]
//         path: Option<OsString>,
//     },
// }
//
fn main() {
    println!("before ");
    let mut p = parser::Parser::new("1");
    println!("{:?}", p);
    let output = p.parse_expresion(0).unwrap();
    println!("{output}");
}
