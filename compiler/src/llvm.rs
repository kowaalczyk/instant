use instant_parser::ast;
use crate::common::CompilationError;

use std::collections::HashSet;
use instant_parser::ast::Stmt;

pub trait FormatLLVM {
    fn format_llvm(&self) -> String;
}

enum CompilationResult {
    Register { id: u32 },
    Constant { val: i32 },
    None,
}

impl FormatLLVM for CompilationResult {
    fn format_llvm(&self) -> String {
        match self {
            CompilationResult::Constant { val } => val.to_string(),
            CompilationResult::Register { id } => format!("%r{}", id),
            CompilationResult::None => String::from(""),
        }
    }
}

pub trait CompileLLVM {
    fn compile_llvm(
        &self,
        available_reg: &mut u32,
        variables: &mut HashSet<String>,
    ) -> Result<CompiledCode, CompilationError>;
}

pub struct CompiledCode {
    instructions: Vec<String>,
    result: CompilationResult,
}

impl CompileLLVM for ast::Prog {
    fn compile_llvm(
        &self,
        available_reg: &mut u32,
        variables: &mut HashSet<String>
    ) -> Result<CompiledCode, CompilationError> {
        let mut instructions: Vec<String> = vec![];
        for stmt in self.stmts.iter() {
            let mut compiled_stmt = stmt.compile_llvm(
                available_reg,
                variables,
            )?;
            instructions.append(&mut compiled_stmt.instructions);
        }
        let compiled_program = CompiledCode {
            instructions,
            result: CompilationResult::None
        };
        Ok(compiled_program)
    }
}

impl CompileLLVM for ast::Stmt {
    fn compile_llvm(
        &self,
        available_reg: &mut u32,
        variables: &mut HashSet<String>
    ) -> Result<CompiledCode, CompilationError> {
        match self {
            Stmt::Expr { expr } => {
                let mut compiled_expr = expr.compile_llvm(available_reg, variables)?;
                let print_instr = format!(
                    "call void @printInt(i32 {})",
                    compiled_expr.result.format_llvm(),
                );
                compiled_expr.instructions.push(print_instr);
                compiled_expr.result = CompilationResult::None;
                Ok(compiled_expr)
            },
            Stmt::Decl { var, expr } => {
                let mut compiled_expr = expr.compile_llvm(available_reg, variables)?;

                if !variables.contains(var) {
                    // allocate memory for the new variable
                    let alloc_instr = format!(
                        "%{}ptr = alloca i32", var
                    );
                    compiled_expr.instructions.push(alloc_instr);
                    variables.insert(var.clone());
                }

                // update value of existing variable
                let store_instr = format!(
                    "store i32 {}, i32* %{}ptr",
                    compiled_expr.result.format_llvm(),
                    var
                );
                compiled_expr.instructions.push(store_instr);
                compiled_expr.result = CompilationResult::None;
                Ok(compiled_expr)
            },
        }
    }
}

impl CompileLLVM for ast::Expr {
    fn compile_llvm(
        &self,
        available_reg: &mut u32,
        variables: &mut HashSet<String>
    ) -> Result<CompiledCode, CompilationError> {
        match self {
            ast::Expr::Binary { left, op, right } => {
                let mut compiled_instructions: Vec<String> = vec![];

                let mut lhs = left.compile_llvm(available_reg, variables)?;
                compiled_instructions.append(&mut lhs.instructions);
                let mut rhs = right.compile_llvm(available_reg, variables)?;
                compiled_instructions.append(&mut rhs.instructions);

                let current_reg = CompilationResult::Register { id: available_reg.clone() };
                let current_instr = format!(
                    "{} = {} {}, {}",
                    current_reg.format_llvm(),
                    op.format_llvm(),
                    lhs.result.format_llvm(),
                    rhs.result.format_llvm(),
                );
                compiled_instructions.push(current_instr);
                *available_reg += 1;

                let compiled_code = CompiledCode {
                    instructions: compiled_instructions,
                    result: current_reg
                };
                Ok(compiled_code)
            },
            ast::Expr::Number { val } => {
                let compiled_code = CompiledCode {
                    instructions: vec![],
                    result: CompilationResult::Constant { val: val.clone() }
                };
                Ok(compiled_code)
            },
            ast::Expr::Variable { var } => {
                if variables.contains(var) {
                    let current_reg = CompilationResult::Register { id: available_reg.clone() };
                    let current_instr = format!(
                        "{} = load i32, i32* %{}ptr",
                        current_reg.format_llvm(),
                        var
                    );
                    *available_reg += 1;
                    let compiled_code = CompiledCode {
                        instructions: vec![current_instr],
                        result: current_reg,
                    };
                    Ok(compiled_code)
                } else {
                    Err(CompilationError::UnidentifiedVariable { identifier: var.clone() })
                }
            },
        }
    }
}

impl FormatLLVM for ast::Opcode {
    fn format_llvm(&self) -> String {
        let op_str = match self {
            ast::Opcode::Add => {"add i32"},
            ast::Opcode::Sub => {"sub i32"},
            ast::Opcode::Mul => {"mul i32"},
            ast::Opcode::Div => {"sdiv i32"},
        };
        String::from(op_str)
    }
}

pub fn compile_llvm(program: &ast::Prog) -> Result<Vec<String>, CompilationError> {
    let mut instructions = vec![
        String::from("declare void @printInt(i32)"),
        String::from("define i32 @main() {"),
    ];
    let mut available_reg = 0 as u32;
    let mut used_variables: HashSet<String> = HashSet::new();
    let mut compilation_result = program.compile_llvm(
        &mut available_reg,
        &mut used_variables
    )?;
    instructions.append(&mut compilation_result.instructions);

    instructions.append(&mut vec![
        String::from("ret i32 0"),
        String::from("}"),
    ]);
    Ok(instructions)
}
