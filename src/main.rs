#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub calculator); // synthesized by LALRPOP

fn main() {
    println!("Hello, world!");
}

#[test]
fn calculator1() {
    assert!(calculator::TermParser::new().parse("22").is_ok());
    assert!(calculator::TermParser::new().parse("(22)").is_ok());
    assert!(calculator::TermParser::new().parse("((((22))))").is_ok());
    assert!(calculator::TermParser::new().parse("((22)").is_err());
}
