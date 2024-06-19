fn main() {
    //let code = include_str!("../../samples/hello_world_pretty.bf");
    //let tokens = lexer::Lexer::tokenize(code);
    //let instruction_handler = interpreter::Interpreter::default();
    //parser::Parser::new(std::io::stdout(), std::io::stdin(), instruction_handler).interpret(tokens);

    let code = include_str!("../../samples/romeo_and_juliet.bf");
    let tokens = lexer::Lexer::tokenize(code);

    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("output.s")
        .unwrap();
    let instruction_handler = compiler::Compiler::new(&mut file);
    parser::Parser::new(file, std::io::stdin(), instruction_handler).interpret(tokens);
}
