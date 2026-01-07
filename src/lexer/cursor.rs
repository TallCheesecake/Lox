use std::str::Chars;
pub struct Cursor<'a> {
    len_remaining: usize,
    chars: Chars<'a>,
}
impl<'a> Cursor<'a> {
    pub fn new(input_string: &'a str) -> Self {
        Cursor {
            len_remaining: input_string.len(),
            chars: input_string.chars(),
        }
    }
    //returns the amount of the token consumed
    pub fn length_consumed(&self) -> usize {
        self.chars.as_str().len() - self.len_remaining
    }
    //the rust lexer just clones it I dont see how thats ok since cursor can litteraly be the
    //whole input string
    pub fn next_char_in_token(&mut self) -> char {
        self.chars.next().unwrap_or('\0')
    }
    //this is heavily inspired by the rustc_lexer ad you can tell
    pub fn bump(&mut self) -> Option<char> {
        self.chars.next()
    }
    //this function is meant the search until you find a char in a &str
    pub fn eat_until() {
        todo!();
        //i wanna try and impl the way memchr::memchr search does it
    }
    pub fn third(&self) -> char {
        let mut temp = self.chars.clone();
        temp.next();
        temp.next();
        //TODO: change return error type
        temp.next().unwrap_or('e')
    }
    pub fn second(&self) -> char {
        let mut temp = self.chars.clone();
        temp.next();
        //TODO: change return error type
        temp.next().unwrap_or('e')
    }
    //NOTE: since cloning a char iterator is only a pointer into a STR you only clone the iterator body
    //which comes out to about 2 words
    pub fn first(&self) -> char {
        //TODO: change return error type
        self.chars.clone().next().unwrap_or('e')
    }
}
