use std::{fs, process::exit, path::Path, io::Write, process::Command};
use instant_parser::ast;
use instant_compiler::stack::{compile};
use instant_compiler::jasmin::translate;
use instant_utils::{parse_arg, parse_env, parse_program, write_file, check_exit_code};


fn compile_jasmin_file(
    parsed_ast: &ast::Prog, jasmin_output_dir: &String, java_class_name: &String
) {
    let compiled_program = match compile(&parsed_ast) {
        Ok(stack_representation) => stack_representation,
        Err(e) => {
            println!("Failed to compile: {:?}", e);
            exit(1);
        }
    };
    let jasmin_output = translate(&compiled_program, &java_class_name);
    let mut jasmin_file_path = Path::new(jasmin_output_dir).to_path_buf();
    jasmin_file_path.push(
        Path::new(java_class_name).with_extension("j")
    );

    let mut jasmin_file = match fs::File::create(jasmin_file_path) {
        Ok(f) => f,
        Err(e) => {
            println!("Failed to create output file: {:?}", e);
            exit(1);
        }
    };
    write_file(&mut jasmin_file, &jasmin_output);
}

fn compile_jvm_file(
    jasmin_source_path: &String, class_output_dir: &String, jasmin_jar_path: &String
) {
    let compilation_status = Command::new("java")
        .arg("-jar").arg(&jasmin_jar_path)
        .arg("-d").arg(&class_output_dir)
        .arg(&jasmin_source_path)
        .status();
    check_exit_code("java", &compilation_status);
}

fn main() {
    let input_filename = parse_arg();
    let jasmin_jar_path = parse_env("JASMIN_COMPILER", "jasmin.jar");

    let jasmin_output_filename= String::from(
        Path::new(&input_filename).with_extension("j").to_str().unwrap()
    );
    let output_dir = String::from(
        Path::new(&input_filename)
            .parent().unwrap()
            .to_str().unwrap()
    );
    let output_class_name = String::from(
        Path::new(&input_filename)
            .file_stem().unwrap()
            .to_str().unwrap()
    );

    let parsed_ast = parse_program(&input_filename);
    compile_jasmin_file(&parsed_ast, &output_dir, &output_class_name);
    compile_jvm_file(
        &jasmin_output_filename,
        &output_dir,
        &jasmin_jar_path
    );
}
