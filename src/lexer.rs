use std::fmt::{self};
mod cursor;
pub use cursor::Cursor;
#[derive(PartialEq, Debug, Clone, Copy)]

pub struct Token<'de> {
    pub t: TokenItem,
    pub original_string: &'de str,
    pub len: usize,
}
impl<'de> Token<'de> {
    pub fn new(t: TokenItem, original_string: &'de str, len: usize) -> Token<'_> {
        Token {
            t,
            original_string,
            len,
        }
    }
}
//you will need to have a constructed cursor for this
pub fn is_whitespace(c: char) -> bool {
    match c {
        //tab and space respectivly
        '\u{0009}' | ' ' => return true,

        _ => return false,
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TokenItem {
    // LineComment,
    // BlockComment,
    LeftParen,
    Equal,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
    //ABOVE IS SINGLE CHAR
    Bang,
    BangEqual,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Iden,
    String,
    Number(f64), //above is combos, strings, User identofiers, and numbers
    And,
    Class,
    Else,
    False,
    Fun,
    For,
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
    Eof,
    //above is Keywords
}
impl Cursor<'_> {
    // Parses a token from the input string.
    pub fn advance_token(&mut self) -> Token<'_> {
        let Some(first_char) = self.bump() else {
            return Token::new(TokenItem::Eof, "\0", 0);
        };
        pub fn literal_token_create(t: TokenItem, original_string: &str) -> Token {
            Token {
                t,
                original_string,
                len: 4,
            }
        }

        let token_kind = match first_char {
            '(' => final_token(TokenItem::LeftParen),
            ')' => final_token(TokenItem::RightParen),
            '{' => final_token(TokenItem::LeftBrace),
            '}' => final_token(TokenItem::RightBrace),
            ',' => final_token(TokenItem::Comma),
            '.' => final_token(TokenItem::Dot),
            '-' => final_token(TokenItem::Minus),
            '+' => final_token(TokenItem::Plus),
            ';' => final_token(TokenItem::Semicolon),
            '*' => final_token(TokenItem::Star),
            'a'..='z' | '_' | 'A'..='Z' => SecondState::Iden,
            '<' => SecondState::LessEqual,
            '!' => SecondState::BangEqual,
            '=' => SecondState::EqualEqual,
            '>' => SecondState::GreaterEqual,
            '"' => SecondState::String,
            c if c.is_whitespace() => continue,
            x => {
                return Some(Err(println!("error at {x}")));
            }
        };
    }
    pub fn number_lex(&mut self) -> Token {
        let mut c = self.first();
        match c {
            //NOTE: add a c.is_valid_number()
            '0'..='9' => while self.first() != '.' {},
        }
    }
}

//TODO: the string representation will have to be a Cow<'_, &str>
impl<'de> fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let origin = self.original_string;
        //the write! and Write trait are pretty much just compiler stuff and VERY confusing
        match self.t {
            TokenItem::LeftParen => write!(f, "token: {origin} LEFT_PAREN"),
            TokenItem::RightParen => write!(f, "token: {origin} RIGHT_PAREN"),
            TokenItem::LeftBrace => write!(f, "token: {origin} LEFT_BRACE"),
            TokenItem::RightBrace => write!(f, "token: {origin} RIGHT_BRACE"),
            TokenItem::Comma => write!(f, "token: {origin} COMMA"),
            TokenItem::Dot => write!(f, "token: {origin} DOT"),
            TokenItem::Minus => write!(f, "token: {origin} MINUS"),
            TokenItem::Plus => write!(f, "token: {origin} PLUS"),
            TokenItem::Semicolon => write!(f, "token: {origin} SEMICOLON"),
            TokenItem::Star => write!(f, "token: {origin} STAR"),
            TokenItem::Bang => write!(f, "token: {origin} todo!()"),
            TokenItem::BangEqual => write!(f, "token: {origin} todo!()"),
            TokenItem::EqualEqual => write!(f, "token: {origin} todo!()"),
            TokenItem::Greater => write!(f, "token: {origin} todo!()"),
            TokenItem::GreaterEqual => write!(f, "token: {origin} todo!()"),
            TokenItem::Less => write!(f, "token: {origin} todo!()"),
            TokenItem::LessEqual => write!(f, "token: {origin} todo!()"),
            TokenItem::Number(_) => write!(f, "token: {origin} todo!()"),
            TokenItem::And => write!(f, "token: {origin} todo!()"),
            TokenItem::Class => write!(f, "token: {origin} todo!()"),
            TokenItem::Else => write!(f, "token: {origin} todo!()"),
            TokenItem::False => write!(f, "token: {origin} FALSE"),
            TokenItem::Fun => write!(f, "token: {origin} FUN"),
            TokenItem::For => write!(f, "token: {origin} FOR"),
            TokenItem::If => write!(f, "token: {origin} IF"),
            TokenItem::Nil => write!(f, "token: {origin} todo!()"),
            TokenItem::Or => write!(f, "token: {origin} todo!()"),
            TokenItem::Print => write!(f, "token: {origin} todo!()"),
            TokenItem::Return => write!(f, "token: {origin} todo!()"),
            TokenItem::Super => write!(f, "token: {origin} todo!()"),
            TokenItem::This => write!(f, "token: {origin} THIS"),
            TokenItem::True => write!(f, "token: {origin} TRUE"),
            TokenItem::Var => write!(f, "token: {origin} todo!()"),
            TokenItem::While => write!(f, "token: {origin} todo!()"),
            TokenItem::Eof => write!(f, "token: {origin} EOF"),
            TokenItem::Equal => write!(f, "token: {origin} todo!()"),
            TokenItem::Iden => write!(f, "token: {origin} todo!()"),
            TokenItem::String => write!(f, "token: {origin} todo!()"),
        }
    }
}

pub struct Lexer<'de> {
    len_remaining: &'de str,
    // whole: &'de str,
    byte: usize,
}
impl<'de> Lexer<'de> {
    pub fn new(input: &'de str) -> Self {
        Self {
            len_remaining: input,
            byte: 0,
        }
    }
}

impl<'de> Iterator for Lexer<'de> {
    type Item = Result<Token<'de>, error::Error>;
    fn next(&mut self) -> Option<Self::Item> {
        //we assign the entirety of the char stream
        // enum SecondState {
        //     Iden,
        //     Number,
        //     BangEqual,
        //     EqualEqual,
        //     GreaterEqual,
        //     LessEqual,
        //     String,
        // }
        //
        let mut chars = self.len_remaining.chars();
        loop {
            let c = chars.next()?;
            let c_output_string = &self.len_remaining[..c.len_utf8()];
            let c_cont = self.len_remaining;
            self.len_remaining = chars.as_str();
            self.byte += c.len_utf8();
            let state1 = match c {};

            break match state1 {};
        }
        None
    }
}
