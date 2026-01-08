use std::fs;
use std::io;
use std::path::Path;
mod lexer;
fn main() {
    live_input()
}
fn live_input() {
    loop {
        print!("> ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        if input != "/0" {
            run(input);
        }
    }
}

fn run(input: String) {
    let mut lex = Cursor::new(input.as_str());
    if let Some(ref x) = lex.next() {
        match x {
            Ok(token) => println!("{token}"),
            Err(error) => eprintln!("{error}"),
        }
    }
}
fn run_file(p: &Path) {
    let input = fs::read_to_string(p).expect("file reading error");
    run(input);
}
