/*
Compiles calculator code to a simple stack-based vm:
PUSH
ADD
SUB
MUL
PRINT
*/

use calculator_parser::ast;

// TODO: Add proper error handling to the compiler

#[derive(Debug)]
pub enum Command {
    PUSH { val: i32 },
    ADD,
    SUB,
    MUL,
    PRINT
}

pub fn compile(program: &ast::Prog) -> Result<Vec<Command>, String> {
    let mut result: Vec<Command> = vec![];
    for stmt in program.stmts.iter() {
        let mut compiled_stmt = compile_stmt(stmt.as_ref())?;
        result.append(&mut compiled_stmt);
    }
    Ok(result)
}

fn compile_stmt(stmt: &ast::Stmt) -> Result<Vec<Command>, String> {
    match stmt {
        ast::Stmt::Expr {expr} => {
            let mut compiled_expr = compile_expr(expr.as_ref())?;
            compiled_expr.push(Command::PRINT);
            Ok(compiled_expr)
        },
    }
}

fn compile_expr(expr: &ast::Expr) -> Result<Vec<Command>, String> {
    match expr {
        ast::Expr::Number {val} => {
            let command = Command::PUSH {val: *val};
            Ok(vec![command])
        },
        ast::Expr::Nested {expr} => {
            compile_expr(expr.as_ref())
        },
        ast::Expr::Binary {left, op, right} => {
            let mut commands: Vec<Command> = vec![];
            // TODO: Optimize order of evaluation (lhs vs rhs - deeper first to limit stack size)

            let mut lhs = compile_expr(left.as_ref())?;
            commands.append(&mut lhs);

            let mut rhs = compile_expr(right.as_ref())?;
            commands.append(&mut rhs);

            match op {
                ast::Opcode::Add => {
                    commands.push(Command::ADD);
                },
                ast::Opcode::Sub => {
                    commands.push(Command::SUB);
                },
                ast::Opcode::Mul => {
                    commands.push(Command::MUL);
                }
            };
            Ok(commands)
        }
    }
}
