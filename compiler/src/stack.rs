use instant_parser::ast;
use crate::common::CompilationError;

use std::collections::HashMap;
use std::cmp::{min, max};
use std::fmt::Debug;

#[derive(Debug)]
pub enum Instruction {
    PUSH { val: i32 },
    ADD,
    SUB,
    MUL,
    DIV,
    PRINT,
    STORE { addr: i32 },
    LOAD { addr: i32 },
    SWAP,
}

#[derive(Debug)]
pub struct CompiledCode {
    pub instructions: Vec<Instruction>,
    pub stack_limit: u32,
    pub locals_limit: u32,
}

pub trait CompileStack {
    fn compile_stack(&self, env: &mut HashMap<String, i32>) -> Result<CompiledCode, CompilationError>;
}

impl CompileStack for ast::Prog {
    fn compile_stack(&self, env: &mut HashMap<String, i32>) -> Result<CompiledCode, CompilationError> {
        let mut instructions: Vec<Instruction> = vec![];
        let mut stack_limit = 0;
        for stmt in self.stmts.iter() {
            let mut compiled_stmt = stmt.compile_stack(env)?;
            instructions.append(&mut compiled_stmt.instructions);
            stack_limit = max(stack_limit, compiled_stmt.stack_limit);
        }
        let locals_limit = env.len() as u32;
        let compiled_program = CompiledCode { instructions, stack_limit, locals_limit };
        Ok(compiled_program)
    }
}

impl CompileStack for ast::Stmt {
    fn compile_stack(&self, env: &mut HashMap<String, i32>) -> Result<CompiledCode, CompilationError> {
        match self {
            ast::Stmt::Expr {expr} => {
                let mut compiled_expr = expr.compile_stack(env)?;
                compiled_expr.instructions.push(Instruction::PRINT);

                // stack limit is increased by 1 to account for the 1st argument to print
                let compiled_stmt = CompiledCode {
                    instructions: compiled_expr.instructions,
                    stack_limit: 1 + compiled_expr.stack_limit,
                    locals_limit: compiled_expr.locals_limit,
                };
                Ok(compiled_stmt)
            },
            ast::Stmt::Decl {var, expr} => {
                let mut compiled_expr = expr.compile_stack(env)?;
                let variable_location = match env.get(var) {
                    Some(existing_location) => *existing_location,
                    None => {
                        let new_location = match env.values().max() {
                            Some(last_used_location) => last_used_location + 1,
                            None => 0,
                        };
                        env.insert(var.clone(), new_location);
                        compiled_expr.locals_limit += 1;
                        new_location
                    }
                };

                let store_command = Instruction::STORE { addr: variable_location };
                compiled_expr.instructions.push(store_command);

                let compiled_stmt = CompiledCode {
                    instructions: compiled_expr.instructions,
                    stack_limit: compiled_expr.stack_limit,
                    locals_limit: compiled_expr.locals_limit
                };
                Ok(compiled_stmt)
            }
        }
    }
}

impl CompileStack for ast::Expr {
    fn compile_stack(&self, env: &mut HashMap<String, i32>) -> Result<CompiledCode, CompilationError> {
        match self {
            ast::Expr::Number {val} => {
                let instruction = Instruction::PUSH {val: *val};
                let compiled_code = CompiledCode {
                    instructions: vec![instruction],
                    stack_limit: 1,
                    locals_limit: 0
                };
                Ok(compiled_code)
            },
            ast::Expr::Variable {var} => {
                match env.get(var) {
                    Option::None => {
                        Err(CompilationError::UnidentifiedVariable { identifier: var.to_string() })
                    },
                    Option::Some(variable_address) => {
                        let instruction = Instruction::LOAD {addr: *variable_address};
                        let compiled_code = CompiledCode {
                            instructions: vec![instruction],
                            stack_limit: 1,
                            locals_limit: 1,
                        };
                        Ok(compiled_code)
                    }
                }
            },
            ast::Expr::Binary {left, op, right} => {
                let mut lhs = left.compile_stack(env)?;
                let mut rhs = right.compile_stack(env)?;

                let mut instructions: Vec<Instruction> = vec![];
                if lhs.stack_limit >= rhs.stack_limit {
                    instructions.append(&mut lhs.instructions);
                    instructions.append(&mut rhs.instructions);
                } else {
                    instructions.append(&mut rhs.instructions);
                    instructions.append(&mut lhs.instructions);

                    if *op == ast::Opcode::Sub || *op == ast::Opcode::Div {
                        instructions.push(Instruction::SWAP);
                    }
                }

                match op {
                    ast::Opcode::Add => {
                        instructions.push(Instruction::ADD);
                    },
                    ast::Opcode::Sub => {
                        instructions.push(Instruction::SUB);
                    },
                    ast::Opcode::Mul => {
                        instructions.push(Instruction::MUL);
                    },
                    ast::Opcode::Div => {
                        instructions.push(Instruction::DIV)
                    }
                };

                let compiled_code = CompiledCode {
                    instructions,
                    stack_limit: max(
                        1 + min(lhs.stack_limit, rhs.stack_limit),
                        max(lhs.stack_limit, rhs.stack_limit)
                    ),
                    locals_limit: env.len() as u32,
                };
                Ok(compiled_code)
            },
        }
    }
}


/// compiles the program to a list of instructions on abstract stack-based machine
pub fn compile_stack(program: &ast::Prog) -> Result<CompiledCode, CompilationError> {
    let mut env: HashMap<String, i32> = HashMap::new();
    let compiled_program = program.compile_stack(&mut env)?;
    Ok(compiled_program)
}
