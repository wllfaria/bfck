use clap::{Arg, ArgAction, Command};

#[derive(Debug)]
pub enum RunMode {
    Assemble(String, Option<String>),
    Repl,
    Compile(String, Option<String>),
}

pub struct Args;

impl Args {
    pub fn parse() -> RunMode {
        let matches = Command::new("bfck")
            .version("0.1.0")
            .about("Interpreter and compiler for brainfuck")
            .after_help("When no options are passed, a REPL is initialized instead")
            .arg(
                Arg::new("assemble")
                    .short('s')
                    .long("assemble")
                    .help("Generate the assembly output of the Brainfuck code")
                    .action(ArgAction::SetTrue)
                    .requires("source")
                    .required(false),
            )
            .arg(
                Arg::new("source")
                    .help("Source file to compile")
                    .required(false),
            )
            .arg(Arg::new("output").help("Output file name").required(false))
            .get_matches();

        let assemble = matches.get_flag("assemble");
        let source = matches.get_one::<String>("source");

        match (source, assemble) {
            (Some(source), true) => {
                let output = matches.get_one::<String>("output");
                RunMode::Assemble(source.clone(), output.cloned())
            }
            (Some(source), false) => {
                let output = matches.get_one::<String>("output");
                RunMode::Compile(source.clone(), output.cloned())
            }
            (None, false) => RunMode::Repl,
            (None, true) => unreachable!(),
        }
    }
}
