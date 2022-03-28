#[derive(Debug)]
pub enum Token {
    ADD,
    SUB,
    DIV,
    MULT,
    NUM,
    COMMENT,
    UNKNOWN,
}

#[derive(Debug)]
pub struct Result {
    pub item: char,
    pub token_type: Token,
}