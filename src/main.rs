use clap::Parser;
use miette::Result;
mod scanner;

#[derive(Parser)]
struct Args {
    #[arg(short)]
    input: String,
}
fn main() -> Result<()> {
    let args = Args::parse();
    let scanner = scanner::Scanner::new(args.input.as_str()).generator();
    match scanner {
        Some(Ok(_)) => Ok(()),
        Some(Err(e)) => Err(e.into()),
        None => Ok(()),
    }
}
