use crate::utils::*;

pub struct Lexer {
    pub chars: Vec<char>,
    pub cursor: usize
}

pub fn get_type(item: char) -> Token {
    if item == '+' {
        return Token::ADD
    }
    else if item == '-' {
        return Token::SUB
    }
    else if item == '*' {
        return Token::MULT
    }
    else if item == '/' {
        return Token::DIV
    }
    else if item.is_numeric() {
        return Token::NUM
    }
    else if item == '#' {
        return Token::COMMENT
    }
    else if item == ';' {
        return Token::COLON
    }
    else {
        return Token::UNKNOWN
    }
}

impl Lexer {
    pub fn new(source : &str) -> Self {
        Self {
            chars: source.chars().collect(),
            cursor: 0
        }
    }

    pub fn advance(&mut self) -> () {
        match self.chars.get_mut(self.cursor + 1) {
            Some(_) => {self.cursor += 1;}
            None => {self.cursor = self.cursor}
        }
    }

    pub fn _advance_index(&mut self , index: usize) {
        self.cursor += index
    }

    pub fn _retreat(&mut self) {
        self.cursor -= 1
    }

    pub fn fetch_cur(&self) -> Result {
        Result {item: *self.chars.get(self.cursor).unwrap() , token_type: get_type(*self.chars.get(self.cursor).unwrap())}
    }

    pub fn _fetch_index(&self, index: usize) -> Result {
        if index > self.chars.len() {
            Result {item: *self.chars.get(self.cursor).unwrap(), token_type: Token::UNKNOWN}
        }
        else {
            Result {item: *self.chars.get(index).unwrap(), token_type: get_type(*self.chars.get(self.cursor).unwrap())}
        }
    }

    pub fn _peek(&mut self) -> Result {
        match self.chars.get_mut(self.cursor + 1) {
            Some(_) => Result {item: *self.chars.get(self.cursor + 1).unwrap(), token_type: get_type(*self.chars.get(self.cursor + 1).unwrap())},
            None => Result {item: ' ' , token_type: Token::UNKNOWN}
        }
    }
}