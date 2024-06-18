use crate::instruction_handler::InstructionHandler;
use lexer::token::Token;

use std::io::{Read, Write};

#[derive(Debug)]
pub struct Parser<W, R, I>
where
    W: Write,
    R: Read,
    I: InstructionHandler<W, R> + std::fmt::Debug,
{
    /// the pointer to the current instruction to be executed within the
    /// vector of tokens
    instruction_ptr: usize,
    /// generic writer to make testing easier, anything that impls Write can be used
    /// here
    writer: W,
    /// generic reader to make testing easier, anything that impls Read can be used
    /// here
    reader: R,
    /// instruction handler that will handle each action, this is generic as we have
    /// a interpreter and a compiler.
    instruction_handler: I,
}

impl<W, R, I> Parser<W, R, I>
where
    W: Write,
    R: Read,
    I: InstructionHandler<W, R> + std::fmt::Debug,
{
    pub fn new(writer: W, reader: R, instruction_handler: I) -> Parser<W, R, I> {
        Parser {
            writer,
            reader,
            instruction_handler,
            instruction_ptr: 0,
        }
    }

    pub fn writer(&self) -> &W {
        &self.writer
    }

    pub fn instruction_ptr(&self) -> usize {
        self.instruction_ptr
    }

    pub fn instruction_handler(&self) -> &I {
        &self.instruction_handler
    }

    pub fn interpret(&mut self, tokens: Vec<Token>) {
        while self.instruction_ptr < tokens.len() {
            match &tokens[self.instruction_ptr] {
                Token::MoveLeft(count) => self.instruction_handler.move_ptr_left(
                    *count,
                    &tokens,
                    &mut self.writer,
                    &mut self.instruction_ptr,
                ),
                Token::MoveRight(count) => self.instruction_handler.move_ptr_right(
                    *count,
                    &tokens,
                    &mut self.writer,
                    &mut self.instruction_ptr,
                ),
                Token::Increment(count) => self.instruction_handler.increment_ptr(
                    *count,
                    &tokens,
                    &mut self.writer,
                    &mut self.instruction_ptr,
                ),
                Token::Decrement(count) => self.instruction_handler.decrement_ptr(
                    *count,
                    &tokens,
                    &mut self.writer,
                    &mut self.instruction_ptr,
                ),
                Token::Write(count) => self.instruction_handler.write_ptr(
                    *count,
                    &tokens,
                    &mut self.writer,
                    &mut self.instruction_ptr,
                ),
                Token::JumpIfZero(count) => self.instruction_handler.jump_if_zero(
                    *count,
                    &tokens,
                    &mut self.writer,
                    &mut self.instruction_ptr,
                ),
                Token::JumpUnlessZero(count) => self.instruction_handler.jump_unless_zero(
                    *count,
                    &tokens,
                    &mut self.writer,
                    &mut self.instruction_ptr,
                ),
                Token::Read(count) => self.instruction_handler.read_ptr(
                    *count,
                    &tokens,
                    &mut self.writer,
                    &mut self.reader,
                    &mut self.instruction_ptr,
                ),
            }
        }
        self.instruction_handler.finish();
    }
}
