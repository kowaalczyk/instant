#[derive(Debug)]
pub enum Opcode {
    Add,
    Sub,
    Mul,
    // TODO: Div
}

#[derive(Debug)]
pub enum Expr {
    Binary { left: Box<Expr>, op: Opcode, right: Box<Expr> },
    Nested { expr: Box<Expr> },
    Number { val: i32 },  // TODO: Parse minus if present before the int
    // TODO: variables
}

#[derive(Debug)]
pub enum Stmt {
    Expr { expr: Box<Expr> },
    // TODO: declaration
}

#[derive(Debug)]
pub struct Prog {
    pub stmts: Vec<Box<Stmt>>,
}
