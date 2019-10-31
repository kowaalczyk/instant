#[derive(Debug, PartialEq)]
pub enum Opcode {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        op: Opcode,
        right: Box<Expr>,
    },
    Number { val: i32 },
    Variable { var: String },
}

#[derive(Debug)]
pub enum Stmt {
    Expr { expr: Box<Expr> },
    Decl { var: String, expr: Box<Expr> },
}

#[derive(Debug)]
pub struct Prog {
    pub stmts: Vec<Box<Stmt>>,
}
