use brain::parser::*;
use brain::compiler::*;
use std::collections::HashMap;
use brain::*;

fn main() {
    let mut v: HashMap<String, Variable> = HashMap::new();
    let set_res = compile(&parse("set a 100;"), &mut v); 
    let get_res = compile(&parse("print a;"), &mut v);
    //println!("{} : {:?}" , setRes.res , setRes.comp_type);
    
    match set_res {
        MulRet::COMP(set_res) => {
            println!("{} : {:?}", set_res.res, set_res.comp_type)
        }
        MulRet::VPRINT(_set_res) => {}
    }
    
    match get_res {
        MulRet::VPRINT(get_res) => {
            println!("{} : {:?}" , get_res.val , get_res.vtype)        
        },
        MulRet::COMP(_get_res) => {},
    };
}