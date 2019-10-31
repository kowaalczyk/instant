use crate::stack::{CompiledCode, Instruction};

trait JVMOutput {
    fn translate(&self) -> Vec<String>;
}

impl JVMOutput for CompiledCode {
    fn translate(&self) -> Vec<String> {
        let mut instruction_vec: Vec<String> = vec![
            String::from(".method public static main([Ljava/lang/String;)V"),
            format!(".limit stack {}", self.stack_limit),
            format!(".limit locals {}", self.locals_limit),
        ];

        for instr in self.instructions.iter() {
            let mut instr_jvm_format = instr.translate();
            instruction_vec.append(&mut instr_jvm_format);
        }

        instruction_vec.push(String::from("return"));
        instruction_vec.push(String::from(".end method"));
        instruction_vec
    }
}

impl JVMOutput for Instruction {
    fn translate(&self) -> Vec<String> {
        let mut instruction_vec: Vec<String> = vec![];
        match self {
            Instruction::PUSH { val } => {
                let instr = match *val {
                    -1 => String::from("iconst_m1"),
                    0..=5 => format!("iconst_{}", val),
                    5..=127 => format!("bipush {}", val),
                    _ => format!("sipush {} {}", ((val >> 16) / (2 << 8)) as u8, (val >> 24) as u8),
                    // larger are truncated without any warning TODO: fix this
                };
                instruction_vec.push(instr);
            },
            Instruction::MUL => {
                instruction_vec.push(String::from("imul"));
            },
            Instruction::ADD => {
                instruction_vec.push(String::from("iadd"));
            },
            Instruction::SUB => {
                instruction_vec.push(String::from("isub"));
            },
            Instruction::DIV => {
                instruction_vec.push(String::from("idiv"));
            },
            Instruction::PRINT => {
                let mut print_instructions = vec![
                    String::from("getstatic java/lang/System/out Ljava/io/PrintStream;"),
                    String::from("swap"),
                    String::from("invokevirtual java/io/PrintStream/println(I)V"),
                ];
                instruction_vec.append(&mut print_instructions);
            },
            Instruction::STORE { addr } => {
                let instr = match *addr {
                    0..=3 => format!("istore_{}", addr),
                    _ => format!("istore {}", addr),
                };
                instruction_vec.push(instr);
            },
            Instruction::LOAD { addr } => {
                let instr = match *addr {
                    0..=3 => format!("iload_{}", addr),
                    _ => format!("iload {}", addr),
                };
                instruction_vec.push(instr);
            },
            Instruction::SWAP => {
                instruction_vec.push(String::from("swap"));
            },
        };
        instruction_vec
    }
}

pub fn translate(compiled_program: &CompiledCode, name: &String) -> Vec<String> {
    let mut jasmin_representation = vec![
        String::from(".bytecode 47.0"),
        format!(".class public {}", name),
        String::from(".super java/lang/Object"),
        String::from(".method public <init>()V"),
        String::from("aload_0"),
        String::from("invokenonvirtual java/lang/Object/<init>()V"),
        String::from("return"),
        String::from(".end method"),
    ];
    jasmin_representation.append(&mut compiled_program.translate());
    jasmin_representation
}
