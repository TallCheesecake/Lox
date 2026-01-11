use std::collections::HashMap;
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
    keywords: HashMap<&'static str, TokenType>,
    code: &'a str,
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
            start: 0,
            chars: code.chars(),
            line: 1,
        }
    }
    pub fn generator(&mut self) -> Token<'a> {
        pub enum ThirdState {
            //I GOT THIS FROM Jon's impl rust this is very smart
            OrEquals(char),
            String,
            Number,
            Iden(char),
            Invalid,
        }

        let generate = |kind: TokenType, line: usize, lexeme: &'a str| -> Token<'a> {
            Token { kind, line, lexeme }
        };
        //resets start
        self.start = self.chars.as_str().len();
        self.trim_whitespace();

        let c = self.current();
        let third_state = match c {
            // '(' => return generate(TokenType::LeftParen, 0, self.lexeme()),
            // ')' => return generate(TokenType::RightParen),
            // '{' => return generate(TokenType::LeftBrace),
            // '}' => return generate(TokenType::RightBrace),
            // ';' => return generate(TokenType::Semicolon),
            // ',' => return generate(TokenType::Comma),
            // '.' => return generate(TokenType::Dot),
            // '-' => return generate(TokenType::Minus),
            // '+' => return generate(TokenType::Plus),
            // '/' => return generate(TokenType::Slash),
            // '*' => return generate(TokenType::Star),
            '!' | '=' | '<' | '>' => ThirdState::OrEquals(c),
            'A'..='Z' | 'a'..='z' => ThirdState::Iden(c),
            '0'..='9' => ThirdState::Number,
            '"' => ThirdState::String,
            _ => ThirdState::Invalid,
        };
        match third_state {
            ThirdState::OrEquals(c) => {
                return {
                    if let Some(x) = self.bump() {
                        match x {
                            // '<' => return { generate(TokenType::LessEqual) },
                            // '>' => return { generate(TokenType::GreaterEqual) },
                            // '!' => return { generate(TokenType::BangEqual) },
                            // '=' => return { generate(TokenType::EqualEqual) },
                        }
                    } else {
                        todo!();
                    }
                };
            }
            ThirdState::String => todo!(),
            ThirdState::Number => todo!(),
            ThirdState::Iden(c) => {
                while let Some(x) = self.chars.as_str().chars().next() {
                    self.bump();
                }
                if let Some(c) = self.keywords.get_mut(&(self.lexeme())) {
                    return generate(*c);
                } else {
                    {}
                }
                return Token::default(self.lexeme());
            }
            ThirdState::Invalid => todo!(),
        };
    }

    fn trim_whitespace(&mut self) {
        while matches!(self.current(), '\u{0009}' | '\u{0020}') {
            self.bump();
        }
    }

    fn lexeme(&mut self) -> &'a str {
        &self.code[self.start..(self.code.len() - self.chars.as_str().len())]
    }

    fn current(&mut self) -> char {
        self.chars.next().unwrap_or('0')
    }

    fn bump(&mut self) -> Option<char> {
        self.chars.next()
    }
    fn second(&self) -> char {
        let mut temp = self.code.clone().chars();
        temp.next();
        temp.next().unwrap_or('0')
    }
}
