#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    ADD,
    SUB,
    DIV,
    MULT,
    NUM,
    COMMENT,
    COLON,
    UNKNOWN,
}

#[derive(Debug)]
pub struct Result {
    pub item: char,
    pub token_type: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenRes {
    pub item: String,
    pub token_type: Token,
}

pub fn clean_input(tokens: &Vec<TokenRes>) -> Vec<TokenRes> {
    let mut tokens_m = tokens.clone();
    tokens_m.retain(|token| token.item != "");
    return tokens_m
}