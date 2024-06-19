use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, terminal,
};
use std::io::Write;

pub fn run() {
    let mut stdout = std::io::stdout();
    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )
    .expect("failed to flush stdout");

    let mut events = vec![];

    loop {
        stdout.flush().expect("failed to flush stdout");
        let event = event::read().expect("failed to read stdin");
        events.push(event.clone());
        if let Event::Key(KeyEvent {
            code: KeyCode::Enter,
            ..
        }) = event
        {
            let code = events.iter().fold(String::default(), |mut acc, event| {
                if let Event::Key(KeyEvent {
                    code: KeyCode::Char(c),
                    ..
                }) = event
                {
                    acc.push(*c);
                };
                acc
            });

            let tokens = lexer::Lexer::tokenize(&code);
            let interpreter = interpreter::Interpreter::default();
            let mut parser = parser::Parser::new(std::io::stdout(), std::io::stdin(), interpreter);
            parser.interpret(tokens);
            events.clear();
            execute!(stdout, cursor::MoveToNextLine(1)).expect("failed to write to stdout");
        }
    }
}
