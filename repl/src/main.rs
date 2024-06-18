fn main() {
    let code = include_str!("../../samples/hello_world_pretty.bf");
    let tokens = lexer::Lexer::tokenize(code);
    let instruction_handler = interpreter::Interpreter::default();
    parser::Parser::new(std::io::stdout(), std::io::stdin(), instruction_handler).interpret(tokens);
    let tokens = lexer::Lexer::tokenize(code);
    let instruction_handler = compiler::Compiler::default();
    parser::Parser::new(std::io::stdout(), std::io::stdin(), instruction_handler).interpret(tokens);
}
