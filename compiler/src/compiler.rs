use lexer::token::Token;
use parser::instruction_handler::InstructionHandler;

use crate::fasm_boilerplate::{
    FASM_BOILERPLATE, MULTI_DEC, MULTI_INC, MULTI_LEFT, MULTI_RIGHT, WRITE,
};

#[derive(Debug)]
pub struct Compiler {
    has_multi_increment: bool,
    has_multi_decrement: bool,
    has_multi_move_left: bool,
    has_multi_move_right: bool,
    has_write: bool,
    assembly_file: String,
}

impl Default for Compiler {
    fn default() -> Compiler {
        Compiler::new()
    }
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            has_multi_increment: false,
            has_multi_decrement: false,
            has_multi_move_left: false,
            has_multi_move_right: false,
            has_write: false,
            assembly_file: String::from(FASM_BOILERPLATE),
        }
    }
}

impl<W, R> InstructionHandler<W, R> for Compiler
where
    W: std::io::Write,
    R: std::io::Read,
{
    fn move_ptr_left(&mut self, count: usize, _: &[Token], _: &mut W, instruction_ptr: &mut usize) {
        if count > 1 {
            self.has_multi_move_left = true;
            self.assembly_file
                .push_str(&format!("    mov ecx, {}\n", count));
            self.assembly_file.push_str("    call _l\n");
        } else {
            self.assembly_file.push_str("    dec ebx\n");
        }

        *instruction_ptr += 1;
    }

    fn move_ptr_right(
        &mut self,
        count: usize,
        _: &[Token],
        _: &mut W,
        instruction_ptr: &mut usize,
    ) {
        if count > 1 {
            self.has_multi_move_right = true;
            self.assembly_file
                .push_str(&format!("    mov ecx, {}\n", count));
            self.assembly_file.push_str("    call _r\n");
        } else {
            self.assembly_file.push_str("    inc ebx\n");
        }
        *instruction_ptr += 1;
    }

    fn increment_ptr(&mut self, count: usize, _: &[Token], _: &mut W, instruction_ptr: &mut usize) {
        if count > 1 {
            self.has_multi_increment = true;
            self.assembly_file
                .push_str(&format!("    mov ecx, {}\n", count));
            self.assembly_file.push_str("    call _u\n");
        } else {
            self.assembly_file.push_str("    inc byte [ebx]\n");
        }
        *instruction_ptr += 1;
    }

    fn decrement_ptr(&mut self, count: usize, _: &[Token], _: &mut W, instruction_ptr: &mut usize) {
        if count > 1 {
            self.has_multi_decrement = true;
            self.assembly_file
                .push_str(&format!("    mov ecx, {}\n", count));
            self.assembly_file.push_str("    call _d\n");
        } else {
            self.assembly_file.push_str("    dec byte [ebx]\n");
        }
        *instruction_ptr += 1;
    }

    fn write_ptr(&mut self, count: usize, _: &[Token], _: &mut W, instruction_ptr: &mut usize) {
        self.has_write = true;
        for _ in 0..count {
            self.assembly_file.push_str("    call _w\n");
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

    fn jump_if_zero(&mut self, _: usize, _: &[Token], _: &mut W, instruction_ptr: &mut usize) {
        *instruction_ptr += 1;
    }

    fn jump_unless_zero(&mut self, _: usize, _: &[Token], _: &mut W, instruction_ptr: &mut usize) {
        *instruction_ptr += 1;
    }

    fn finish(&mut self) {
        self.assembly_file.push_str("    call _e\n");

        if self.has_multi_increment {
            self.assembly_file.push_str(MULTI_INC);
        }

        if self.has_multi_decrement {
            self.assembly_file.push_str(MULTI_DEC);
        }

        if self.has_multi_move_right {
            self.assembly_file.push_str(MULTI_RIGHT);
        }

        if self.has_multi_move_left {
            self.assembly_file.push_str(MULTI_LEFT);
        }

        if self.has_write {
            self.assembly_file.push_str(WRITE);
        }

        _ = std::fs::write("./output.s", self.assembly_file.as_bytes());
    }
}
