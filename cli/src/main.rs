mod args;

fn main() {
    match args::Args::parse() {
        args::RunMode::Repl => repl::run(),
        args::RunMode::Assemble(source, output) => compiler::assemble(source, output),
        args::RunMode::Compile(source, output) => compiler::compile(source, output),
    }
}
