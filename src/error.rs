use crate::utils::*;

pub fn check_error(input: &Vec<TokenRes>) {

    let res = TokenRes {
        item: "noop".to_string(),
        token_type: Token::UNKNOWN
    };
    let last = match input.last() {
        Some(x) => {x}
        None => {&res}
    };

    if last.token_type != Token::COLON {
        panic!("Expressions must end with `;`")
    }


    for (index, i) in input.iter().enumerate() {
        if i.token_type == Token::COLON {
            if index != input.len() - 1 {
                panic!("`;` are only allowed at the end of expressions")
            }
        }

        if let Token::ADD | Token::SUB | Token::MULT | Token::DIV = i.token_type{
            let left = index.checked_sub(1).and_then(|i| input.get(i));
            let right = index.checked_add(1).and_then(|i| input.get(i));
            if let Some((left, right)) = left.zip(right) {
                if left.token_type != Token::NUM || right.token_type != Token::NUM {
                    panic!("Arithmetic is only allowed between numbers.")
                } 
            }
        }
    }
}