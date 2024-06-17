mod ansi;

fn main() {
    ansi::clear_screen();
    ansi::move_cursor(0, 0);
    loop {
        let mut code = String::new();

        std::io::stdin()
            .read_line(&mut code)
            .expect("failed to read from stdin");

        let tokens = lexer::Lexer::tokenize(&code);
        tokens.into_iter().for_each(|token| println!("{}", token));
    }
}
