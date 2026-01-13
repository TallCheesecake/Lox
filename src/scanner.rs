use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Display;
use std::str::Chars;
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]

pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error,
    Eof,
}

#[derive(Copy, Clone)]
pub struct Token<'a> {
    pub kind: TokenType,
    pub line: usize,
    pub lexeme: &'a str,
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.lexeme)
    }
}

impl<'a> Token<'a> {
    pub fn default(text: &'a str) -> Token<'a> {
        Token {
            kind: TokenType::Error,
            lexeme: text,
            line: 0,
        }
    }
}

pub struct Scanner<'a> {
    keywords: HashMap<&'a str, TokenType>,
    code: &'a str,
    end: usize,
    chars: Chars<'a>,
    line: usize,
}
impl<'a> Scanner<'a> {
    pub fn new(code: &'a str) -> Scanner {
        let mut keywords = HashMap::with_capacity(16);
        keywords.insert("else", TokenType::Else);
        keywords.insert("and", TokenType::And);
        keywords.insert("for", TokenType::For);
        keywords.insert("fun", TokenType::Fun);
        keywords.insert("class", TokenType::Class);
        keywords.insert("return", TokenType::Return);
        keywords.insert("if", TokenType::If);
        keywords.insert("nil", TokenType::Nil);
        keywords.insert("or", TokenType::Or);
        keywords.insert("true", TokenType::True);
        keywords.insert("false", TokenType::False);
        keywords.insert("print", TokenType::Print);
        keywords.insert("super", TokenType::Super);
        keywords.insert("this", TokenType::This);
        keywords.insert("var", TokenType::Var);
        keywords.insert("while", TokenType::While);

        Scanner {
            keywords,
            code,
            end: 0,
            chars: code.chars(),
            line: 1,
        }
    }
    pub fn generator(&mut self) -> Option<Token<'a>> {
        pub enum ThirdState {
            OrEquals,
            String,
            Number,
            Iden,
            Invalid,
        }
        println!("----------");
        println!("{}, {} ", self.chars.as_str(), self.code);
        let generate = |kind: TokenType, line: usize, lexeme: &'a str| -> Option<Token<'a>> {
            println!("lexeme: {:?}", lexeme);
            Some(Token { kind, line, lexeme })
        };
        let counter: usize = 0;
        self.end = self.chars.as_str().len();
        self.chars = self.chars.as_str().trim_start().chars();
        println!("----------");
        println!("post sanitaiton: {}, {} ", self.chars.as_str(), self.code);
        let c = self.current();
        println!("we have called current: and gotten: {}", c);
        let third_state = match c {
            '(' => return generate(TokenType::LeftParen, counter, self.lexeme()),
            ')' => return generate(TokenType::RightParen, counter, self.lexeme()),
            '{' => return generate(TokenType::LeftBrace, counter, self.lexeme()),
            '}' => return generate(TokenType::RightBrace, counter, self.lexeme()),
            ';' => return generate(TokenType::Semicolon, counter, self.lexeme()),
            ',' => return generate(TokenType::Comma, counter, self.lexeme()),
            '.' => return generate(TokenType::Dot, counter, self.lexeme()),
            '-' => return generate(TokenType::Minus, counter, self.lexeme()),
            '+' => return generate(TokenType::Plus, counter, self.lexeme()),
            '/' => return generate(TokenType::Slash, counter, self.lexeme()),
            '*' => return generate(TokenType::Star, counter, self.lexeme()),
            '!' | '=' | '<' | '>' => ThirdState::OrEquals,
            'A'..='Z' | 'a'..='z' => ThirdState::Iden,
            '0'..='9' => ThirdState::Number,
            '"' => ThirdState::String,
            _ => ThirdState::Invalid,
        };
        match third_state {
            ThirdState::OrEquals => {
                //NOTE: there is a much easirer way to do this !!!
                if let Some(x) = self.bump() {
                    match x {
                        '<' => {
                            return { generate(TokenType::LessEqual, counter, self.lexeme()) };
                        }
                        '>' => {
                            return { generate(TokenType::GreaterEqual, counter, self.lexeme()) };
                        }
                        '!' => {
                            return { generate(TokenType::BangEqual, counter, self.lexeme()) };
                        }
                        '=' => {
                            return { generate(TokenType::EqualEqual, counter, self.lexeme()) };
                        }
                        _ => {}
                    }
                }
                // return Some(Token::default(self.lexeme()));
            }
            ThirdState::String => {
                if let Some(_) = self.chars.find(|&x| x != '"') {
                    generate(TokenType::String, self.line, self.lexeme());
                } else {
                    //i dont think this is technacly correct
                    generate(TokenType::String, self.line, self.lexeme());
                }
            }
            ThirdState::Number => todo!(),
            ThirdState::Iden => {
                //Views the underlying data as a subslice of the original data.
                // let start = self.chars.as_str().chars();
                // I belive that we already call next for c
                while let Some(x) = self.chars.next() {
                    if !x.is_ascii_alphabetic() {
                        break;
                    }
                    self.bump();
                }
                let index = self.lexeme();
                if let Some(c) = self.keywords.get_mut(&(index)) {
                    return generate(*c, counter, self.lexeme());
                } else {
                    {}
                }
                return Some(Token::default(self.lexeme()));
            }
            ThirdState::Invalid => todo!(),
        };
        None
    }
    fn trim_whitespace(&mut self) {
        while matches!(self.current(), '\u{0009}' | '\u{0020}') {
            self.chars = self.chars.as_str().trim_start().chars();
        }
    }

    fn lexeme(&mut self) -> &'a str {
        println!(
            "method lexeme: {:?}",
            //dont ask me abt the -1 at the self.end ...
            &self.code[(self.code.len() - self.chars.as_str().len() - 1)..self.end - 1]
        );
        &self.code[(self.code.len() - self.chars.as_str().len() - 1)..self.end - 1]
    }

    fn current(&mut self) -> char {
        self.chars.next().unwrap_or('0')
    }

    fn bump(&mut self) -> Option<char> {
        self.chars.next()
    }
    fn second(&self) -> char {
        let mut temp = self.code.chars();
        temp.next();
        temp.next().unwrap_or('0')
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_it_works_char() {
        let mut result = Scanner::new("({");
        if let Some(x) = result.generator() {
            assert_eq!(x.kind, TokenType::LeftParen);
            assert_eq!(x.lexeme, "(")
        }
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}
