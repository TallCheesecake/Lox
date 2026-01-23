use miette::Result;
mod scanner;
use scanner::example_main;
fn main() -> Result<()> {
    example_main()
}
