use lexer::token::Token;

/// A instruction handler is anything that is capable of handling every
/// brainfuck instruction, we made this a generic trait as we have a
/// interpreter, which handler brainfuck code in runtime. And a compiler
/// that turns brainfuck into assembly.
pub trait InstructionHandler<W, R>
where
    W: std::io::Write,
    R: std::io::Read,
{
    fn move_ptr_left(
        &mut self,
        count: usize,
        tokens: &[Token],
        writer: &mut W,
        instruction_ptr: &mut usize,
    );
    fn move_ptr_right(
        &mut self,
        count: usize,
        tokens: &[Token],
        writer: &mut W,
        instruction_ptr: &mut usize,
    );
    fn increment_ptr(
        &mut self,
        count: usize,
        tokens: &[Token],
        writer: &mut W,
        instruction_ptr: &mut usize,
    );
    fn decrement_ptr(
        &mut self,
        count: usize,
        tokens: &[Token],
        writer: &mut W,
        instruction_ptr: &mut usize,
    );
    fn write_ptr(
        &mut self,
        count: usize,
        tokens: &[Token],
        writer: &mut W,
        instruction_ptr: &mut usize,
    );
    fn read_ptr(
        &mut self,
        count: usize,
        tokens: &[Token],
        writer: &mut W,
        reader: &mut R,
        instruction_ptr: &mut usize,
    );
    fn jump_if_zero(
        &mut self,
        count: usize,
        tokens: &[Token],
        writer: &mut W,
        instruction_ptr: &mut usize,
    );
    fn jump_unless_zero(
        &mut self,
        count: usize,
        tokens: &[Token],
        writer: &mut W,
        instruction_ptr: &mut usize,
    );
    fn finish(&mut self) {}
}
