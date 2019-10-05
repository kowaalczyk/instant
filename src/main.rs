use calculator_parser::calculator;

fn main() {
    // TODO
    let parser = calculator::StmtParser::new();
    let test_exprs = vec!["10 + 2 * 7;", "10 * 2 + 7"];
    for expr in test_exprs {
        let parsed_expr = parser.parse("10 + 2 * 7;").unwrap();
        println!("{:#?}", parsed_expr);
    }
}
