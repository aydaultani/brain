#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    ADD,
    SUB,
    DIV,
    MULT,
    NUM,
    COMMENT,
    COLON,
    SET,
    GET,
    LETTER,
    UNKNOWN,
}

#[derive(Debug, PartialEq, Clone)]
pub enum DataTypes {
    INT,
    STR,
    FLOAT,
    BOOL,
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

#[derive(Debug, PartialEq, Clone)]
pub enum CompType {
    ARTH,
    UNKNOWN,
    VARIABLE,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CompRes {
    pub res: String,
    pub comp_type: CompType
}

#[derive(Debug, PartialEq, Clone)]
pub struct CompErr {
    pub reason: String,
    pub input: Vec<TokenRes>
}

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub value: String,
    pub data_type: DataTypes,
}

pub fn clean_input(tokens: &Vec<TokenRes>) -> Vec<TokenRes> {
    let mut tokens_m = tokens.clone();
    tokens_m.retain(|token| token.token_type != Token::UNKNOWN);
    tokens_m.retain(|token| token.item != "");
    return tokens_m
}