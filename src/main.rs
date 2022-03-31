use brain::parser::*;
use brain::compiler::*;

fn main() {
    let res1 = compile(&parse("set 100;")); 
    let res = compile(&parse("get;"));
    println!("{} : {:?}" , res1.res , res1.comp_type);
    println!("{} : {:?}" , res.res , res.comp_type)
    //println!("{:?}" , parse("set 10;"))
}