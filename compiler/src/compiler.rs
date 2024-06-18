use lexer::token::Token;
use parser::instruction_handler::InstructionHandler;

#[derive(Debug)]
pub struct Compiler {}

impl Default for Compiler {
    fn default() -> Compiler {
        Compiler::new()
    }
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {}
    }
}

impl<W, R> InstructionHandler<W, R> for Compiler
where
    W: std::io::Write,
    R: std::io::Read,
{
    fn move_ptr_left(&mut self, count: usize, writer: &mut W, instruction_ptr: &mut usize) {}
    fn move_ptr_right(&mut self, count: usize, writer: &mut W, instruction_ptr: &mut usize) {}
    fn increment_ptr(&mut self, count: usize, writer: &mut W, instruction_ptr: &mut usize) {}
    fn decrement_ptr(&mut self, count: usize, writer: &mut W, instruction_ptr: &mut usize) {}
    fn write_ptr(&mut self, count: usize, writer: &mut W, instruction_ptr: &mut usize) {}
    fn read_ptr(
        &mut self,
        count: usize,
        writer: &mut W,
        reader: &mut R,
        instruction_ptr: &mut usize,
    ) {
    }
    fn jump_if_zero(
        &mut self,
        count: usize,
        tokens: &[Token],
        writer: &mut W,
        instruction_ptr: &mut usize,
    ) {
    }
    fn jump_unless_zero(
        &mut self,
        count: usize,
        tokens: &[Token],
        writer: &mut W,
        instruction_ptr: &mut usize,
    ) {
    }
}
