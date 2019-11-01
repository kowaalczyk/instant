use std::{env, fs, io::Result, io::Write};
use instant_parser::{ast, instant};
use std::process::{exit, ExitStatus};

pub fn parse_arg() -> String {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(input_filename) => {
            String::from(input_filename)
        },
        None => {
            println!("Usage: {} {}", &args[0], "[input_filename]");
            exit(2)
        },
    }
}

pub fn parse_env(key: &str, default: &str) -> String {
    match env::var_os(key) {
        Some(llvm_as) => llvm_as.into_string().unwrap(),
        None => String::from(default),
    }
}

pub fn parse_program(input_path: &String) -> ast::Prog {
    let source_code = match fs::read_to_string(input_path) {
        Ok(source_code) => source_code,
        Err(e) => {
            println!("Error reading file: {:?}", e);
            exit(1);
        }
    };

    let parser = instant::ProgParser::new();
    match parser.parse(&source_code) {
        Ok(parsed_program) => {
            parsed_program
        },
        Err(parsing_error) => {
            println!("Parsing error: {:?}", parsing_error);
            exit(1);
        }
    }
}

pub fn write_file(file: &mut fs::File, compiled_code: &Vec<String>) {
    for line in compiled_code.iter() {
        match writeln!(file, "{}", line) {
            Ok(res) => res,
            Err(e) => {
                println!("Failed to write file: {:?}", e);
                exit(1);
            },
        };
    };
}

pub fn check_exit_code(command_name: &str, status: &Result<ExitStatus>) {
    match status {
        Ok(status) => {
            if !status.success() {
                println!("{} exited with error code: {:?}", command_name, status);
                exit(1);
            }
        },
        Err(e) => {
            println!("{} failed to execute: {:?}", command_name, e);
            exit(1);
        }
    };
}