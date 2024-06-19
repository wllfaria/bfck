mod compiler;
mod fasm_boilerplate;
use compiler::Compiler;

use std::path::Path;

pub fn assemble<P>(source: P, output: Option<String>)
where
    P: AsRef<Path>,
{
    let code = get_source_code(source);
    let tokens = tokenize(&code);
    let mut writer = create_output(output.unwrap_or("output.s".to_string()));
    let compiler = Compiler::new(&mut writer);
    parser::Parser::new(writer, std::io::stdin(), compiler).interpret(tokens)
}

pub fn compile<P>(source: P, output: Option<String>)
where
    P: AsRef<Path>,
{
    assemble(source.as_ref(), Some("temp.s".to_string()));

    if let Err(e) = std::process::Command::new("fasm")
        .arg("temp.s")
        .arg(output.unwrap_or("output".to_string()))
        .output()
    {
        eprintln!("{}", e);
    }

    if std::fs::remove_file("temp.s").is_err() {
        eprintln!("ERROR: failed to do cleanup");
        std::process::exit(1);
    }
}

fn tokenize(source: &str) -> Vec<lexer::token::Token> {
    lexer::Lexer::tokenize(source)
}

fn get_source_code<P>(path: P) -> String
where
    P: AsRef<Path>,
{
    if !path.as_ref().exists() {
        eprintln!(
            "ERROR: cannot find {}: No such file or directory",
            path.as_ref().to_string_lossy()
        );
        std::process::exit(1);
    }

    let Ok(code) = std::fs::read_to_string(path.as_ref()) else {
        eprintln!("ERROR: failed to read {}", path.as_ref().to_string_lossy());
        std::process::exit(1);
    };

    code
}

fn create_output<P>(path: P) -> impl std::io::Write
where
    P: AsRef<Path>,
{
    let Ok(cwd) = std::env::current_dir() else {
        eprintln!("ERROR: failed to get current working directory");
        std::process::exit(1);
    };

    let output = cwd.join(path.as_ref());
    let Ok(output) = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .read(true)
        .write(true)
        .open(&output)
    else {
        eprintln!(
            "ERROR: failed to create output file: {}",
            output.to_string_lossy()
        );
        std::process::exit(1);
    };

    output
}
