pub mod parser;
pub mod utils;
pub mod lexer;
pub mod error;
pub mod compiler;
pub use utils::*;

mod tests {

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
        let result = crate::compiler::compile(&crate::parser::parse("100+100;"));
        assert_eq!(result.res, "200")
    }

    #[test]
    fn sub_test() {
        let result = crate::compiler::compile(&crate::parser::parse("200-100;"));
        assert_eq!(result.res, "100")
    }

    #[test]
    fn mult_test() {
        let result = crate::compiler::compile(&crate::parser::parse("100*10;"));
        assert_eq!(result.res, "1000") 
    }

    #[test]
    fn div_test() {
        let result = crate::compiler::compile(&crate::parser::parse("100/2;"));
        assert_eq!(result.res, "50")
    }

}