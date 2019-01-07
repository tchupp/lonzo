#[derive(Eq, PartialEq, Debug)]
pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
}

#[derive(Eq, PartialEq, Debug)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}