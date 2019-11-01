use std::{fs, env, process::exit, path::Path, process::Command};
use instant_parser::ast;
use instant_compiler::llvm::{compile_llvm};
use instant_utils::{parse_arg, parse_env, parse_program, write_file, check_exit_code};

fn compile_llvm_file(parsed_ast: &ast::Prog, output_path: &String) {
    let compiled_code = match compile_llvm(parsed_ast) {
        Ok(code) => code,
        Err(e) => {
            println!("Failed to compile: {:?}", e);
            exit(1);
        }
    };
    let mut llvm_file = match fs::File::create(output_path) {
        Ok(f) => f,
        Err(e) => {
            println!("Failed to create output file: {:?}", e);
            exit(1);
        }
    };
    write_file(&mut llvm_file, &compiled_code);
}

fn compile_binary_file(
    llvm_assembler: &String, llvm_linker: &String, llvm_runtime: &String,
    llvm_compiled_program: &String, binary_output_path: &String
) {
    let mut compilation_output_dir = env::temp_dir().to_path_buf();
    compilation_output_dir.push("instant_program_out.bc");
    let compilation_output_file = compilation_output_dir.to_str().unwrap();

    let compilation_status = Command::new(llvm_assembler)
        .arg("-o")
        .arg(compilation_output_file)
        .arg(llvm_compiled_program)
        .status();
    check_exit_code(llvm_assembler, &compilation_status);

    let linking_status = Command::new(llvm_linker)
        .arg("-o")
        .arg(binary_output_path)
        .arg(llvm_runtime)
        .arg(compilation_output_file)
        .status();
    check_exit_code(llvm_linker, &linking_status);
}

fn main() {
    let input_filename = parse_arg();
    let llvm_assembler = parse_env("LLVM_ASSEMBLER", "llvm-as");
    let llvm_linker = parse_env("LLVM_LINKER", "llvm-link");
    let llvm_runtime = parse_env("LLVM_RUNTIME", "runtime.bc");

    let llvm_output_filename= String::from(
        Path::new(&input_filename).with_extension("ll").to_str().unwrap()
    );
    let binary_output_filename = String::from(
        Path::new(&input_filename).with_extension("bc").to_str().unwrap()
    );

    let parsed_ast = parse_program(&input_filename);
    compile_llvm_file(&parsed_ast, &llvm_output_filename);
    compile_binary_file(
        &llvm_assembler,
        &llvm_linker,
        &llvm_runtime,
        &llvm_output_filename,
        &binary_output_filename
    );
}
