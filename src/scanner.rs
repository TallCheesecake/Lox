use anyhow::Error;
use anyhow::{Result, anyhow};
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
    current: usize,
    start: usize,
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
            current: 0,
            //NOTE: end is poorly named, end is actually the end of the consumed characters
            //currently before the next loop through the lexeme
            start: 0,
            chars: code.chars(),
            line: 1,
        }
    }

    pub fn generator(&mut self) -> Option<Result<Token<'a>, Error>> {
        pub enum ThirdState {
            OrEquals(char),
            String,
            Number,
            Iden,
        }
        let generate =
            |kind: TokenType, line: usize, lexeme: &'a str| -> Option<Result<Token<'a>, Error>> {
                Some(Ok(Token { kind, line, lexeme }))
            };
        self.start = self.current;
        let counter: usize = 0;

        let c = self.chars.next().unwrap_or('\0');
        //yes im aware of code re-use and i dont care
        let third_state = match c {
            '(' => {
                self.current = c.len_utf8();
                return generate(TokenType::LeftParen, counter, self.lexeme());
            }
            ')' => {
                self.current = c.len_utf8();
                return generate(TokenType::RightParen, counter, self.lexeme());
            }
            '{' => {
                self.current = c.len_utf8();
                return generate(TokenType::LeftBrace, counter, self.lexeme());
            }
            '}' => {
                self.current = c.len_utf8();
                return generate(TokenType::RightBrace, counter, self.lexeme());
            }
            ';' => {
                self.current = c.len_utf8();
                return generate(TokenType::Semicolon, counter, self.lexeme());
            }
            ',' => {
                self.current = c.len_utf8();
                return generate(TokenType::Comma, counter, self.lexeme());
            }
            '.' => {
                self.current = c.len_utf8();
                return generate(TokenType::Dot, counter, self.lexeme());
            }
            '-' => {
                self.current = c.len_utf8();
                return generate(TokenType::Minus, counter, self.lexeme());
            }
            '+' => {
                self.current = c.len_utf8();
                return generate(TokenType::Plus, counter, self.lexeme());
            }
            '/' => {
                self.current = c.len_utf8();
                return generate(TokenType::Slash, counter, self.lexeme());
            }
            '*' => {
                self.current = c.len_utf8();
                return generate(TokenType::Star, counter, self.lexeme());
            }
            '!' | '=' | '<' | '>' => ThirdState::OrEquals(c),
            'A'..='Z' | 'a'..='z' => ThirdState::Iden,
            '0'..='9' => ThirdState::Number,
            '"' => ThirdState::String,
            _ => {
                return Some(Err(anyhow!(
                    "invalid character line: {}, this token: {}",
                    counter,
                    self.lexeme()
                )));
            }
        };

        let mut comparator_handle =
            |kind: TokenType, kind2: TokenType, ch: char| -> Option<Result<Token<'a>, Error>> {
                {
                    println!("comparitor length: {:?}", self.chars.as_str().len());
                    if self.first() == '=' {
                        self.chars.next()?;
                        self.current = self.code[self.chars.as_str().len()..].len();
                        return generate(kind, counter, self.lexeme());
                    } else {
                        self.current = self.code[self.chars.as_str().len()..].len();
                        return generate(kind2, counter, self.lexeme());
                    }
                }
            };
        match third_state {
            ThirdState::OrEquals(c) => match c {
                '<' => {
                    return comparator_handle(TokenType::LessEqual, TokenType::Less, c);
                }
                '!' => {
                    return comparator_handle(TokenType::BangEqual, TokenType::Bang, c);
                }
                '>' => {
                    return comparator_handle(TokenType::GreaterEqual, TokenType::Greater, c);
                }
                '=' => {
                    return comparator_handle(TokenType::EqualEqual, TokenType::Equal, c);
                }
                _ => {
                    return Some(Err(anyhow!(
                        "invalid character line: {}, this token: {}",
                        counter,
                        self.lexeme()
                    )));
                }
            },

            ThirdState::String => {
                if let Some(c) = self.chars.find(|&x| x == '"') {
                    if c == '"' {
                        self.current = self.code[self.chars.as_str().len()..].len();
                        return generate(TokenType::String, counter, self.lexeme());
                    } else {
                        return generate(TokenType::String, counter, self.lexeme());
                    }
                }
            }

            ThirdState::Number => {
                //TODO: add accumulator for 2 . count
                while self.first() != '.' {
                    if let Some(_) = self.chars.find_map(|c| {
                        if matches!(c, '0'..='9') {
                            Some(())
                        } else {
                            None
                        }
                    }) {}
                }
            }

            ThirdState::Iden => {
                while self.first().is_ascii_alphanumeric() {
                    self.chars.next();
                }
                self.current = self.code[self.chars.as_str().len()..].len();
                let index = self.lexeme();
                if let Some(c) = self.keywords.get_mut(&(index)) {
                    return generate(*c, counter, self.lexeme());
                } else {
                    return generate(TokenType::Identifier, counter, self.lexeme());
                }
            }
        };
        None
    }

    fn lexeme(&mut self) -> &'a str {
        &self.code[self.start..self.current]
    }
    fn first(&self) -> char {
        let mut temp = self.chars.clone();
        temp.next().unwrap_or('\0')
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string() {
        let mut scanner = Scanner::new("\"hello\"");

        let token = scanner
            .generator()
            .expect("expected a token")
            .expect("lexer returned error");

        assert_eq!(token.kind, TokenType::String);
        assert_eq!(token.lexeme, "\"hello\"");
    }
    #[test]
    #[ignore]
    fn chart() {
        let mut scanner = Scanner::new("{");

        let token = scanner
            .generator()
            .expect("expected a token")
            .expect("lexer returned error");

        assert_eq!(token.kind, TokenType::LeftBrace);
        assert_eq!(token.lexeme, "{");
    }
    #[test]
    #[ignore]
    fn orelse_first() {
        let mut scanner = Scanner::new("<=fdkjdsalkjs");

        let token = scanner
            .generator()
            .expect("expected a token")
            .expect("lexer returned error");

        assert_eq!(token.kind, TokenType::LessEqual);
        assert_eq!(token.lexeme, "<=");
    }
    #[test]
    #[ignore]
    fn orelse_second() {
        let mut scanner = Scanner::new("!=fdkjdsalkjs");

        let token = scanner
            .generator()
            .expect("expected a token")
            .expect("lexer returned error");

        assert_eq!(token.kind, TokenType::BangEqual);
        assert_eq!(token.lexeme, "!=");
        // println!("self.chars.next:");
    }
    #[test]
    #[ignore]
    fn for_key() {
        let mut scanner = Scanner::new("false");

        let token = scanner
            .generator()
            .expect("expected a token")
            .expect("lexer returned error");

        assert_eq!(token.kind, TokenType::False);
        assert_eq!(token.lexeme, "false");
    }
    #[test]
    #[ignore]
    fn iden_key() {
        let mut scanner = Scanner::new("hello");
        let token = scanner
            .generator()
            .expect("expected a token")
            .expect("lexer returned error");
        assert_eq!(token.kind, TokenType::Identifier);
        assert_eq!(token.lexeme, "hello");
    }
    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}
