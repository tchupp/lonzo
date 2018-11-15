#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub calculator);

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::calculator;

    #[test]
    fn parser_can_handle_int_with_parentheses() {
        assert_eq!(calculator::ExprParser::new().parse("22").unwrap(), 22);
        assert_eq!(calculator::ExprParser::new().parse("(14)").unwrap(), 14);
        assert_eq!(calculator::ExprParser::new().parse("((((7))))").unwrap(), 7);

        assert!(calculator::ExprParser::new().parse("((91)").is_err());
    }

    #[test]
    fn parser_can_handle_addition() {
        assert_eq!(calculator::ExprParser::new().parse("11+1").unwrap(), 12);
        assert_eq!(calculator::ExprParser::new().parse("(10  ) +  12").unwrap(), 22);

        assert!(calculator::ExprParser::new().parse("1 + (21").is_err());
    }

    #[test]
    fn parser_can_handle_subtraction() {
        assert_eq!(calculator::ExprParser::new().parse("23-1").unwrap(), 22);
        assert_eq!(calculator::ExprParser::new().parse("40   -  (  12)").unwrap(), 28);

        assert!(calculator::ExprParser::new().parse("35 - (12").is_err());
    }

    #[test]
    fn parser_can_handle_multiplication() {
        assert_eq!(calculator::ExprParser::new().parse("11 * 2").unwrap(), 22);
        assert_eq!(calculator::ExprParser::new().parse("3        * (7)").unwrap(), 21);

        assert!(calculator::ExprParser::new().parse("(5 * 6").is_err());
    }

    #[test]
    fn parser_can_handle_division() {
        assert_eq!(calculator::ExprParser::new().parse("12 / 2").unwrap(), 6);
        assert_eq!(calculator::ExprParser::new().parse("26        / (2)").unwrap(), 13);

        assert!(calculator::ExprParser::new().parse("(5 / 1").is_err());
    }
}