pub mod parser;
pub mod utils;
pub mod lexer;
pub mod error;

pub use utils::*;

mod tests {
    #[test]
    fn colon_test() {
        let result = std::panic::catch_unwind(|| crate::parser::parse("1"));
        assert!(result.is_err());
    }

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

}