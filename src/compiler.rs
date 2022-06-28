use std::collections::HashMap;


use crate::utils::*;
use crate::utils::VarPrintRes;
use crate::MulRet;
use crate::error::compile_error;

pub fn compile(input: &Vec<TokenRes>, v: &mut HashMap<String, Variable>) -> MulRet {
    let mut arithmetics: Vec<Token> = Vec::new();

    arithmetics.push(Token::ADD);
    arithmetics.push(Token::SUB);
    arithmetics.push(Token::MULT);
    arithmetics.push(Token::DIV);

    for (index , i) in input.iter().enumerate() {
        if arithmetics.contains(&i.token_type) {
            let current_index = input.iter().position(|r| *r == *i).unwrap();
            let lhs: i32 = input[current_index - 1].item.parse().unwrap();
            let rhs: i32 = input[current_index + 1].item.parse().unwrap();
            match &i.token_type {
                Token::ADD => {
                    let res = lhs + rhs;
                    return MulRet::COMP(CompRes {res: res.to_string() , comp_type: CompType::ARTH})
                },
                Token::SUB => {
                    let res = lhs - rhs;
                    return MulRet::COMP(CompRes {res: res.to_string() , comp_type: CompType::ARTH})
                },
                Token::MULT => {
                    let res = lhs * rhs;
                    return MulRet::COMP(CompRes {res: res.to_string() , comp_type: CompType::ARTH})
                },
                Token::DIV => {
                    let res = lhs / rhs;
                    return MulRet::COMP(CompRes {res: res.to_string() , comp_type: CompType::ARTH})
                },
                _ => {
                    return MulRet::COMP(CompRes {res: "".to_string() , comp_type: CompType::UNKNOWN})
                }
            }
        }
        if i.token_type == Token::SET {
            #[allow(unused_assignments)]
            let mut vtype:DataTypes = DataTypes::UNKNOWN;
            if input[index + 3].token_type == Token::UNKNOWN {
                compile_error(CompErr {reason: "Tried to set unknown token as variable.".to_string() , input: input.to_vec()}); 
            }
            else if arithmetics.contains(&input[index + 3].token_type) {
                compile_error(CompErr {reason: "Can't set arithmetic operators as variable".to_string(), input: input.to_vec()})
            }
            
            let is_letter = input[index + 4].item.to_string().parse::<i32>().is_err();
            if is_letter {
                vtype = DataTypes::STR;
            }
            else if !is_letter {
                vtype = DataTypes::INT;
            }
            else {
                vtype = DataTypes::UNKNOWN;
            }

            if !input[index + 3].item.to_string().parse::<i32>().is_err() {
                panic!("Can't set variable name to integer")
            }

            let var: Variable = Variable {value: input[index + 4].item.to_string() , data_type: vtype};
            v.insert(input[index + 3].item.to_string(), var);
            //variables.push();
            let res = format!("{:?}" , v);
            return MulRet::COMP(CompRes {res: res.to_string() , comp_type: CompType::VARIABLE})
        }
        if i.token_type == Token::PRINT {
            let res = v.get(&input[index + 5].item.to_string());
            let m = &res.expect("msg").value;
            return MulRet::VPRINT(VarPrintRes {val: m.to_string(), vtype: DataTypes::BOOL})
            //return CompRes {res: format!("{:?}", res.expect("CompRes GET Err")), comp_type: CompType::VARIABLE}
        }
    }
    return MulRet::COMP(CompRes {res: "".to_string() , comp_type: CompType::UNKNOWN})
}