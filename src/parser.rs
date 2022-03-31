use crate::lexer::*;
use crate::utils::*;
use crate::error::*;

pub fn parse (input: &str) -> Vec<TokenRes> {
    let mut lexer = Lexer::new(input);
    let mut current_num_list: Vec<char> = Vec::new();
    let mut final_list: Vec<TokenRes> = Vec::new();
    for _i in input.chars() {
        if lexer.fetch_cur().item.is_numeric() {
            current_num_list.push(lexer.fetch_cur().item);
        }
        if !lexer.fetch_cur().item.is_numeric() {
            final_list.push(TokenRes {item: current_num_list.iter().collect::<String>() , token_type: Token::NUM});
            current_num_list.clear();
            final_list.push(TokenRes {item: lexer.fetch_cur().item.to_string(), token_type: get_type(lexer.fetch_cur().item)});
        }
        if lexer.fetch_cur().item == 's' {
            if lexer.peek().item == 'e' {
                final_list.push(TokenRes {item: "set".to_string(), token_type: Token::SET})
            }
        }
        if lexer.fetch_cur().item == 'g' {
            if lexer.peek().item == 'e' {
               final_list.push(TokenRes {item: "get".to_string(), token_type: Token::GET}) 
            }
        }
        lexer.advance(); 
    }
    final_list = clean_input(&final_list);
    check_error(&final_list);

    //println!("{:?}" , final_list);
    return final_list
}