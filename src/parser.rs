use crate::*;

fn get_type(item: char) -> Result {
    if item == '+' {
        return Result {item: item, token_type: Token::ADD}
    }
    else if item == '-' {
        return Result {item: item, token_type: Token::SUB}
    }
    else if item == '*' {
        return Result {item: item, token_type: Token::MULT}
    }
    else if item == '/' {
        return Result {item: item, token_type: Token::DIV}
    }
    else if item.is_numeric() {
        return Result {item: item, token_type: Token::NUM}
    }
    else if item == '#' {
        return Result {item: item, token_type: Token::COMMENT}
    }
    else {
        return Result {item: item, token_type: Token::UNKNOWN}
    }
}

pub fn parse (input: &str) {
    println!("Expression : {}" , input);
    println!("____________________________");
    println!("TOKENS");
    for character in input.chars() {
        println!("{:#?}" , get_type(character));
    }
    println!("____________________________");
}