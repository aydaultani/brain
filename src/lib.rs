pub mod parser;
pub mod utils;
pub mod lexer;
pub mod error;
pub mod compiler;
pub use utils::*;

mod tests {
    use std::collections::HashMap;


    #[test]
    fn mid_colon_test() {
        let result = std::panic::catch_unwind(|| crate::parser::parse(";1+1"));
        assert!(result.is_err());
    }

    #[test]
    fn arithmetic_parse_test() {
        let result = std::panic::catch_unwind(|| crate::parser::parse("++1;"));
        assert!(result.is_err())
    }

    #[test]
    fn colon_test() {
        let result = std::panic::catch_unwind(|| crate::parser::parse("1"));
        assert!(result.is_err());
    }

    #[test]
    fn arth_add() {
        let mut v: HashMap<String, crate::utils::Variable> = HashMap::new();
        let result = crate::compiler::compile(&crate::parser::parse("100+100;"), &mut v);
        assert_eq!(result.res, "200")
    }

    #[test]
    fn sub_test() {
        let mut v: HashMap<String, crate::utils::Variable> = HashMap::new();
        let result = crate::compiler::compile(&crate::parser::parse("200-100;"), &mut v);
        assert_eq!(result.res, "100")
    }

    #[test]
    fn mult_test() {
        let mut v: HashMap<String, crate::utils::Variable> = HashMap::new();
        let result = crate::compiler::compile(&crate::parser::parse("100*10;"), &mut v);
        assert_eq!(result.res, "1000") 
    }

    #[test]
    fn div_test() {
        let mut v: HashMap<String, crate::utils::Variable> = HashMap::new();
        let result = crate::compiler::compile(&crate::parser::parse("100/2;"), &mut v);
        assert_eq!(result.res, "50")
    }

    #[test]
    fn var_set_test_str() {
        let mut v: HashMap<String, crate::utils::Variable> = HashMap::new();
        let setRes = crate::compiler::compile(&crate::parser::parse("set a 100;"), &mut v); 
        let getRes = crate::compiler::compile(&crate::parser::parse("print a;"), &mut v);
        let foo = false;
        if getRes.res != "100".to_string() {}
    }

}