use std::str::Chars;
pub struct Cursor<'a> {
    pub len_remaining: usize,
    pub input: &'a str,
    pub chars: Chars<'a>,
}
impl<'a> Cursor<'a> {
    pub fn new(input_string: &'a str) -> Self {
        Cursor {
            len_remaining: input_string.len(),
            input: input_string,
            chars: input_string.chars(),
        }
    }
    //returns the amount of the token consumed
    pub fn length_consumed(&self) -> usize {
        self.input.len() - self.len_remaining
    }
    pub fn next_char_in_token(&mut self) -> char {
        self.chars.next().unwrap_or('\0')
    }
    fn len_remaining_update(&mut self) {
        self.len_remaining = self.chars.as_str().len();
    }
    //this is heavily inspired by the rustc_lexer if you can't tell
    pub fn bump(&mut self) -> Option<char> {
        let temp = self.chars.next();
        self.len_remaining_update();
        temp
    }

    pub fn eof(&mut self) -> bool {
        match self.bump() {
            Some(_) => false,
            None => true,
        }
    }
    pub fn current(&mut self) -> Option<char> {
        let result = &self.input[self.len_remaining..];
        //NOTE: the first call to next effectivly indexes into the iterator
        result.chars().next()
    }
    //eat while a pred holds so like if F taking a char is true then call bump
    pub fn eat_while(&mut self, f: impl Fn(char) -> bool) {
        while let Some(c) = self.current() {
            if f(c) {
                self.bump();
            } else {
                break;
            }
        }
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
        self.chars.clone().next().unwrap_or('e')
    }
}
