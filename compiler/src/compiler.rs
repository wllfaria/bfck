use lexer::token::Token;
use parser::instruction_handler::InstructionHandler;

use crate::fasm_boilerplate::{
    BREAK, FASM_BOILERPLATE, MULTI_DEC, MULTI_INC, MULTI_LEFT, MULTI_RIGHT, WRITE,
};

#[derive(Debug, Default)]
pub struct Compiler {
    has_multi_increment: bool,
    has_multi_decrement: bool,
    has_multi_move_left: bool,
    has_multi_move_right: bool,
    has_write: bool,
    jump_count: usize,
    jump_list: Vec<String>,
}

impl Compiler {
    pub fn new<W>(writer: &mut W) -> Compiler
    where
        W: std::io::Write,
    {
        _ = write!(writer, "{}", FASM_BOILERPLATE);
        Compiler {
            has_multi_increment: false,
            has_multi_decrement: false,
            has_multi_move_left: false,
            has_multi_move_right: false,
            has_write: false,
            jump_count: 0,
            jump_list: vec![],
        }
    }
}

impl<W, R> InstructionHandler<W, R> for Compiler
where
    W: std::io::Write,
    R: std::io::Read,
{
    fn move_ptr_left(
        &mut self,
        count: usize,
        _: &[Token],
        writer: &mut W,
        instruction_ptr: &mut usize,
    ) {
        if count > 1 {
            self.has_multi_move_left = true;
            _ = writeln!(writer, "    mov ecx, {}", count);
            _ = writeln!(writer, "    call _l");
        } else {
            _ = writeln!(writer, "    dec ebx");
        }

        *instruction_ptr += 1;
    }

    fn move_ptr_right(
        &mut self,
        count: usize,
        _: &[Token],
        writer: &mut W,
        instruction_ptr: &mut usize,
    ) {
        if count > 1 {
            self.has_multi_move_right = true;
            _ = writeln!(writer, "    mov ecx, {}", count);
            _ = writeln!(writer, "    call _r");
        } else {
            _ = writeln!(writer, "    inc ebx");
        }
        *instruction_ptr += 1;
    }

    fn increment_ptr(
        &mut self,
        count: usize,
        _: &[Token],
        writer: &mut W,
        instruction_ptr: &mut usize,
    ) {
        if count > 1 {
            self.has_multi_increment = true;
            _ = writeln!(writer, "    mov ecx, {}", count);
            _ = writeln!(writer, "    call _u");
        } else {
            _ = writeln!(writer, "    inc byte [ebx]");
        }
        *instruction_ptr += 1;
    }

    fn decrement_ptr(
        &mut self,
        count: usize,
        _: &[Token],
        writer: &mut W,
        instruction_ptr: &mut usize,
    ) {
        if count > 1 {
            self.has_multi_decrement = true;
            _ = writeln!(writer, "    mov ecx, {}", count);
            _ = writeln!(writer, "    call _d");
        } else {
            _ = writeln!(writer, "    dec byte [ebx]");
        }
        *instruction_ptr += 1;
    }

    fn write_ptr(
        &mut self,
        count: usize,
        _: &[Token],
        writer: &mut W,
        instruction_ptr: &mut usize,
    ) {
        self.has_write = true;
        for _ in 0..count {
            _ = writeln!(writer, "    call _w");
        }
        *instruction_ptr += 1;
    }

    fn read_ptr(
        &mut self,
        _: usize,
        _: &[Token],
        _: &mut W,
        _: &mut R,
        instruction_ptr: &mut usize,
    ) {
        *instruction_ptr += 1;
    }

    fn jump_if_zero(
        &mut self,
        _: usize,
        tokens: &[Token],
        writer: &mut W,
        instruction_ptr: &mut usize,
    ) {
        let jump_header_name = format!("_j{}z", self.jump_count);
        let jump_body_name = format!("_j{}b", self.jump_count);
        self.jump_count += 1;
        _ = writeln!(writer, "    call {}", jump_header_name);

        let mut jump_header = String::new();
        jump_header.push_str(&format!("{}:\n", jump_header_name));
        jump_header.push_str("    cmp byte [ebx], 0\n");
        jump_header.push_str("    je _b\n");
        jump_header.push_str(&format!("    jne {}", jump_body_name));

        let start = *instruction_ptr;
        let mut open_jumps = 1;
        *instruction_ptr += 1;

        let mut body_writer = vec![];
        let mut cursor = std::io::Cursor::new(&mut body_writer);

        while open_jumps != 0 {
            if *instruction_ptr >= tokens.len() {
                panic!("jump if zero defined at {start} is never closed");
            }

            match tokens[*instruction_ptr] {
                Token::MoveLeft(count) => {
                    <crate::Compiler as parser::instruction_handler::InstructionHandler<
                        std::io::Cursor<&mut Vec<_>>,
                        R,
                    >>::move_ptr_left(
                        self, count, tokens, &mut cursor, instruction_ptr
                    );
                }
                Token::MoveRight(count) => {
                    <crate::Compiler as parser::instruction_handler::InstructionHandler<
                        std::io::Cursor<&mut Vec<_>>,
                        R,
                    >>::move_ptr_right(
                        self, count, tokens, &mut cursor, instruction_ptr
                    );
                }
                Token::Increment(count) => {
                    <crate::Compiler as parser::instruction_handler::InstructionHandler<
                        std::io::Cursor<&mut Vec<_>>,
                        R,
                    >>::increment_ptr(
                        self, count, tokens, &mut cursor, instruction_ptr
                    );
                }
                Token::Decrement(count) => {
                    <crate::Compiler as parser::instruction_handler::InstructionHandler<
                        std::io::Cursor<&mut Vec<_>>,
                        R,
                    >>::decrement_ptr(
                        self, count, tokens, &mut cursor, instruction_ptr
                    );
                }
                Token::Write(count) => {
                    <crate::Compiler as parser::instruction_handler::InstructionHandler<
                        std::io::Cursor<&mut Vec<_>>,
                        R,
                    >>::write_ptr(
                        self, count, tokens, &mut cursor, instruction_ptr
                    );
                }
                Token::JumpIfZero(count) => {
                    <crate::Compiler as parser::instruction_handler::InstructionHandler<
                        std::io::Cursor<&mut Vec<_>>,
                        R,
                    >>::jump_if_zero(
                        self, count, tokens, &mut cursor, instruction_ptr
                    );
                }
                Token::JumpUnlessZero(count) => {
                    open_jumps -= count;
                    *instruction_ptr += 1;
                }
                Token::Read(_) => *instruction_ptr += 1,
            };
        }

        let mut jump_body = String::new();
        jump_body.push_str(&format!("{}:\n", jump_body_name));

        let resulting = String::from_utf8(body_writer).unwrap();
        jump_body.push_str(&resulting);
        jump_body.push_str("    cmp byte [ebx], 0\n");
        jump_body.push_str(&format!("    jne {}\n", jump_body_name));
        jump_body.push_str("    je _b");

        self.jump_list.push(jump_header);
        self.jump_list.push(jump_body);
    }

    fn jump_unless_zero(&mut self, _: usize, _: &[Token], _: &mut W, instruction_ptr: &mut usize) {
        // we handle the entire jump on the start of the jump, so we should
        // never reach this block
        unreachable!();
    }

    fn finish(&mut self, writer: &mut W) {
        _ = writeln!(writer, "    jmp _e");

        for jump in self.jump_list.iter() {
            _ = writeln!(writer, "{}", jump);
        }

        if self.has_multi_increment {
            _ = writeln!(writer, "{}", MULTI_INC);
        }

        if self.has_multi_decrement {
            _ = writeln!(writer, "{}", MULTI_DEC);
        }

        if self.has_multi_move_right {
            _ = writeln!(writer, "{}", MULTI_RIGHT);
        }

        if self.has_multi_move_left {
            _ = writeln!(writer, "{}", MULTI_LEFT);
        }

        if self.has_write {
            _ = writeln!(writer, "{}", WRITE);
        }

        if self.jump_count > 0 {
            _ = writeln!(writer, "{}", BREAK);
        }
    }
}
