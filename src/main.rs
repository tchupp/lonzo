#[macro_use]
extern crate lalrpop_util;

mod ast;

lalrpop_mod!(pub calculator);

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use ast::Expr;
    use ast::Opcode;

    use super::calculator;

    #[test]
    fn parser_can_handle_int_with_parentheses() {
        let parser = calculator::ExprParser::new();

        assert_eq!(parser.parse("22").unwrap(), Box::new(Expr::Number(22)));
        assert_eq!(parser.parse("(14)").unwrap(), Box::new(Expr::Number(14)));
        assert_eq!(parser.parse("((((7))))").unwrap(), Box::new(Expr::Number(7)));

        assert!(parser.parse("((91)").is_err());
    }

    #[test]
    fn parser_can_handle_addition() {
        let parser = calculator::ExprParser::new();

        assert_eq!(
            parser.parse("11+1").unwrap(),
            Box::new(
                Expr::Op(
                    Box::new(Expr::Number(11)),
                    Opcode::Add,
                    Box::new(Expr::Number(1)),
                )
            )
        );
        assert_eq!(
            parser.parse("(10  ) +  12").unwrap(),
            Box::new(
                Expr::Op(
                    Box::new(Expr::Number(10)),
                    Opcode::Add,
                    Box::new(Expr::Number(12)),
                )
            )
        );

        assert!(parser.parse("1 + (21").is_err());
    }

    #[test]
    fn parser_can_handle_subtraction() {
        let parser = calculator::ExprParser::new();

        assert_eq!(
            parser.parse("23-1").unwrap(),
            Box::new(
                Expr::Op(
                    Box::new(Expr::Number(23)),
                    Opcode::Sub,
                    Box::new(Expr::Number(1)),
                )
            )
        );
        assert_eq!(
            parser.parse("40   -  (  12)").unwrap(),
            Box::new(
                Expr::Op(
                    Box::new(Expr::Number(40)),
                    Opcode::Sub,
                    Box::new(Expr::Number(12)),
                )
            )
        );

        assert!(parser.parse("35 - (12").is_err());
    }

    #[test]
    fn parser_can_handle_multiplication() {
        let parser = calculator::ExprParser::new();

        assert_eq!(
            parser.parse("11 * 2").unwrap(),
            Box::new(
                Expr::Op(
                    Box::new(Expr::Number(11)),
                    Opcode::Mul,
                    Box::new(Expr::Number(2)),
                )
            )
        );
        assert_eq!(
            parser.parse("3        * (7)").unwrap(),
            Box::new(
                Expr::Op(
                    Box::new(Expr::Number(3)),
                    Opcode::Mul,
                    Box::new(Expr::Number(7)),
                )
            )
        );

        assert!(parser.parse("(5 * 6").is_err());
    }

    #[test]
    fn parser_can_handle_division() {
        let parser = calculator::ExprParser::new();

        assert_eq!(
            parser.parse("12 / 2").unwrap(),
            Box::new(
                Expr::Op(
                    Box::new(Expr::Number(12)),
                    Opcode::Div,
                    Box::new(Expr::Number(2)),
                )
            )
        );
        assert_eq!(
            parser.parse("26        / (2)").unwrap(),
            Box::new(
                Expr::Op(
                    Box::new(Expr::Number(26)),
                    Opcode::Div,
                    Box::new(Expr::Number(2)),
                )
            )
        );

        assert!(parser.parse("(5 / 1").is_err());
    }
}