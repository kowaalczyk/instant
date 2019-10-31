use std::{fs, env, process::exit, path::Path, io::Write, process::Command};
use instant_parser::{ast, instant};
use instant_compiler::stack::{compile};
use instant_compiler::jasmin::translate;

/// parses command line and os environment to return (input_path, jasmin_jar_path)
fn parse_args() -> (String, String) {
    let args: Vec<String> = env::args().collect();

    // input file is read as a command line argument
    let input_file = match args.get(1) {
        Some(input_filename) => {
            String::from(input_filename)
        },
        None => {
            println!("Usage: {} {}", &args[0], "[input_filename]");
            exit(2);
        },
    };

    // path to jasmin.jar can be read from environment, defaults to current directory
    let jasmin_jar_path = match env::var_os("JASMIN_COMPILER") {
        Some(jasmin_jar_path) => jasmin_jar_path.into_string().unwrap(),
        None => String::from("jasmin.jar"),
    };
    (input_file, jasmin_jar_path)
}

fn parse_program(input_path: &String) -> ast::Prog {
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

fn compile_jasmin(parsed_ast: &ast::Prog, jasmin_output_dir: &String, java_class_name: &String) {
    let compiled_program = match compile(&parsed_ast) {
        Ok(stack_representation) => stack_representation,
        Err(e) => {
            println!("Failed to compile: {:?}", e);
            exit(1);
        }
    };
    let jasmin_output = translate(&compiled_program, &java_class_name);
    let mut jasmin_file_path = Path::new(jasmin_output_dir)
        .to_path_buf();
    jasmin_file_path.push(
        Path::new(java_class_name).with_extension("j")
    );

    let mut jasmin_file = match fs::File::create(jasmin_file_path) {
        Ok(f) => f,
        Err(e) => {
            println!("Failed to compile: {:?}", e);
            exit(1);
        }
    };
    for line in jasmin_output.iter() {
        match writeln!(jasmin_file, "{}", line) {
            Ok(res) => res,
            Err(e) => {
                println!("Failed to write file: {:?}", e);
                exit(1);
            },
        };
    }
}

fn compile_jvm(jasmin_source_path: &String, class_output_dir: &String, jasmin_jar_path: &String) {
    let compilation_status = Command::new("java")
        .arg("-jar").arg(&jasmin_jar_path)
        .arg("-d").arg(&class_output_dir)
        .arg(&jasmin_source_path)
        .status();

    match compilation_status {
        Ok(status) => {
            if !status.success() {
                println!("Program jasmin.jar exited with error code: {:?}", status);
                exit(1);
            }
        },
        Err(e) => {
            println!("Failed to execute jasmin.jar: {:?}", e);
            exit(1);
        }
    }
}

fn main() {
    let (input_filename, jasmin_jar_path) = parse_args();

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
    compile_jasmin(&parsed_ast, &output_dir, &output_class_name);
    compile_jvm(
        &jasmin_output_filename,
        &output_dir,
        &jasmin_jar_path
    );
}
