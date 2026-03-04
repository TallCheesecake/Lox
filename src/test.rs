use super::*;

#[test]
fn test_func() -> Result<(), Box<dyn std::error::Error>> {
    let mut parse = parser::Parser::new(String::from(" fun hi(a + 1, b) { var a = 1; } "))?;
    let output = parse.parse_statment()?;
    insta::assert_debug_snapshot!(output);
    Ok(())
}
#[test]
fn test_class_simple() -> Result<(), Box<dyn std::error::Error>> {
    let mut parse = parser::Parser::new(String::from("class Hello {var i =1;} "))?;
    let output = parse.parse_statment()?;
    insta::assert_debug_snapshot!(output);
    Ok(())
}
#[test]
fn test_while() -> Result<(), Box<dyn std::error::Error>> {
    let mut parse = parser::Parser::new(String::from("while(a = 1) {var hello = \"hi\";}"))?;
    let output = parse.parse_statment()?;
    insta::assert_debug_snapshot!(output);
    Ok(())
}
#[test]
fn test_var() -> Result<(), Box<dyn std::error::Error>> {
    let mut parse = parser::Parser::new(String::from("var a= 1;"))?;
    let output = parse.parse_statment()?;
    insta::assert_debug_snapshot!(output);
    Ok(())
}
#[test]
fn test_print_statement() -> Result<(), Box<dyn std::error::Error>> {
    let mut parse = parser::Parser::new(String::from("print \"hello\";"))?;
    let output = parse.parse_statment()?;
    insta::assert_debug_snapshot!(output);
    Ok(())
}
