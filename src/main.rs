use std::{fs, env, process::exit};
use calculator_parser::{ast, calculator};
use calculator_compiler::stack::compile;

fn parse_str(source: &String) -> ast::Prog {
    let parser = calculator::ProgParser::new();
    parser.parse(&source)
        .expect("parsing error")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if env::args().count() != 2 {
        println!("Usage: {} {}", &args[0], "[input_filename]");
        exit(1);
    }
    let unparsed_file = fs::read_to_string(&args[1])
        .expect("Cannot read file");

    let ast = parse_str(&unparsed_file);
    println!("{:#?}", &ast);

    let compiled_commands = compile(&ast);
    println!("{:#?}", &compiled_commands);
}
