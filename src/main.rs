mod parser;
mod utils;
use parser::parse;
use utils::*;

fn main() {
    parse("1+1/10*20#");
}
