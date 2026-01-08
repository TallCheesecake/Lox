pub use cursor::Cursor;
use std::fmt::{self};
mod cursor;
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
