use lexer::token::Token;
use parser::instruction_handler::InstructionHandler;

use std::collections::VecDeque;

static CAPACITY: usize = 32;
static INCREMENT: usize = CAPACITY / 2;

#[derive(Debug)]
pub struct Interpreter {
    /// the pointer to the current cell that should be acted upon
    data_ptr: usize,
    /// the tape of available cells to perform operations, this is initialized as an
    /// VecDeque of 0u8 with capacity `CAPACITY`, and we simulate an infinite tape by
    /// increasing its capacity by `INCREMENT` everytime we attempt to move out of
    /// bounds
    tape: VecDeque<u8>,
}

impl Default for Interpreter {
    fn default() -> Interpreter {
        Interpreter::new()
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            tape: vec![0; CAPACITY].into_iter().collect(),
            data_ptr: INCREMENT,
        }
    }

    /// increments the current tape `amount` `INCREMENT` stops to the left
    /// ```rust
    /// use std::collections::VecDeque;
    ///
    /// // lets say we have this queue
    /// let mut queue: VecDeque<u8> = [1, 2, 3, 4].into_iter().collect();
    ///
    /// // after extending 4, we would have
    /// // [1, 2, 3, 4, 0, 0, 0, 0]
    /// //  ^
    /// //  illustrative `data_ptr`
    /// queue.extend(vec![0; 4]);
    ///
    /// // then, when we rotate it to the right by the amount of items added
    /// // we would end up with
    /// // [0, 0, 0, 0, 1, 2, 3, 4]
    /// //  ^
    /// //  illustrative `data_ptr`
    /// queue.rotate_right(4);
    /// ```
    /// as you can see, the `data_ptr` needs to be adjusted so that it stays
    /// pointing to the same cell as it was before, this is done by just
    /// adding n where n is the amount of items added to the queue to the
    /// `data_ptr`
    fn increment_left(&mut self, amount: usize) {
        let data_ptr_points_to = self.tape[self.data_ptr];
        let total_increment = INCREMENT * amount;
        self.tape.extend(vec![0; total_increment]);
        self.tape.rotate_right(total_increment);
        self.data_ptr += total_increment;
        assert_eq!(self.tape[self.data_ptr], data_ptr_points_to, "after incrementing the queue to the left, the data pointer ended up in a different cell");
    }

    /// increments the current tape `amount` `INCREMENT` stops to the right
    /// ```rust
    /// use std::collections::VecDeque;
    ///
    /// // lets say we have this queue
    /// let mut queue: VecDeque<u8> = [1, 2, 3, 4].into_iter().collect();
    ///
    /// // after extending 4, we would have
    /// // [1, 2, 3, 4, 0, 0, 0, 0]
    /// //  ^
    /// //  illustrative `data_ptr`
    /// queue.extend(vec![0; 4]);
    /// ```
    ///
    /// unlike incrementing left, we dont need to change the data_ptr since
    /// growing to the right doesnt need any rotation, so we can just increase
    /// by the amount of `INCREMENT` stops needed
    fn increment_right(&mut self, amount: usize) {
        let increment = INCREMENT * amount;
        self.tape.extend(vec![0; increment]);
    }
}

impl<W, R> InstructionHandler<W, R> for Interpreter
where
    W: std::io::Write,
    R: std::io::Read,
{
    /// move the `data_ptr` `count` cells to the left, account for out of bounds
    /// by increasing the tape by `INCREMENT` stops to simulate a "infinite" tape
    /// as the specification suggests.
    fn move_ptr_left(&mut self, count: usize, _: &[Token], _: &mut W, instruction_ptr: &mut usize) {
        if count > self.data_ptr {
            let difference = count - self.data_ptr;
            let amount_of_increments = difference.div_ceil(INCREMENT);
            self.increment_left(amount_of_increments);
        }
        self.data_ptr -= count;
        *instruction_ptr += 1;
    }

    /// move the `data_ptr` `count` cells to the right, account for out of bounds
    /// by increasing the tape by `INCREMENT` stops to simulate a "infinite" tape
    /// as the specification suggests.
    fn move_ptr_right(
        &mut self,
        count: usize,
        _: &[Token],
        _: &mut W,
        instruction_ptr: &mut usize,
    ) {
        if self.data_ptr + count >= self.tape.len() {
            let new_len = self.data_ptr + count - self.tape.len();
            let new_len = usize::max(new_len, 1);
            let amount_of_increments = new_len.div_ceil(INCREMENT);
            self.increment_right(amount_of_increments);
        }
        self.data_ptr += count;
        *instruction_ptr += 1;
    }

    /// increment the current cell on the `tape` pointed by `data_ptr`, wrapping
    /// when the value exceeds `u8::MAX`
    fn increment_ptr(&mut self, count: usize, _: &[Token], _: &mut W, instruction_ptr: &mut usize) {
        self.tape[self.data_ptr] = self.tape[self.data_ptr].wrapping_add(count as u8);
        *instruction_ptr += 1;
    }

    /// decrement the current cell on the `tape` pointed by `data_ptr`, wrapping
    /// when the value would underflow below 0
    fn decrement_ptr(&mut self, count: usize, _: &[Token], _: &mut W, instruction_ptr: &mut usize) {
        self.tape[self.data_ptr] = self.tape[self.data_ptr].wrapping_sub(count as u8);
        *instruction_ptr += 1;
    }

    /// writes to stdout the contents of the current cell pointed by `data_ptr`
    /// as a `char` `count` times.
    fn write_ptr(
        &mut self,
        count: usize,
        _: &[Token],
        writer: &mut W,
        instruction_ptr: &mut usize,
    ) {
        for _ in 0..count {
            _ = write!(writer, "{}", self.tape[self.data_ptr] as char);
        }
        *instruction_ptr += 1;
    }

    /// read from stdin one byte at a time and add the byte to the current pointed
    /// cell
    fn read_ptr(
        &mut self,
        count: usize,
        _: &[Token],
        _: &mut W,
        reader: &mut R,
        instruction_ptr: &mut usize,
    ) {
        for _ in 0..count {
            let mut byte = [0u8; 1];

            match reader.read_exact(&mut byte) {
                Ok(_) => {
                    self.tape[self.data_ptr] = byte[0];
                }
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                    panic!("unexpected end of input");
                }
                Err(e) => {
                    panic!("failed to read from reader: {:?}", e);
                }
            }
        }
        *instruction_ptr += 1;
    }

    /// if the current cell pointed by `data_ptr` is equals to 0, we should jump
    /// to the next instruction after the matching closing jump, denoted by
    /// `JumpUnlessZero` if the current cell is not zero, then we just skip to the
    /// next instruction
    ///
    /// since we accept a `count`, when cell is non-zero, we can skip all of the
    /// next repeated occurrences directly to the next non-repeated token
    fn jump_if_zero(&mut self, _: usize, tokens: &[Token], _: &mut W, instruction_ptr: &mut usize) {
        let curr_val = self.tape[self.data_ptr];
        if curr_val != 0 {
            *instruction_ptr += 1;
            return;
        }
        let mut open_jumps = 1;
        let start = *instruction_ptr;
        let mut index = *instruction_ptr + 1;
        while open_jumps != 0 {
            if index >= tokens.len() {
                panic!("jump if zero defined at {start} is never closed");
            }

            match tokens[index] {
                Token::JumpIfZero(count) => open_jumps += count,
                Token::JumpUnlessZero(count) => open_jumps -= count,
                _ => {}
            }

            index += 1;
        }

        *instruction_ptr = index;
    }

    /// if the current cell pointed by `data_ptr` is not equals to 0, we should
    /// jump to the next instruction after the matching opening jump, denoted by
    /// `JumpIfZero`; if the current cell is zero, then we just skip to the next
    /// instruction
    ///
    /// since we accept a `count`, when cell is zero, we can skip all of the
    /// next repeated occurrences directly to the next non-repeated token
    fn jump_unless_zero(
        &mut self,
        _: usize,
        tokens: &[Token],
        _: &mut W,
        instruction_ptr: &mut usize,
    ) {
        let curr_val = self.tape[self.data_ptr];
        if curr_val == 0 {
            *instruction_ptr += 1;
            return;
        }
        let mut open_jumps = 1;
        let mut index = *instruction_ptr - 1;
        while open_jumps != 0 {
            match tokens[index] {
                Token::JumpIfZero(count) => open_jumps -= count,
                Token::JumpUnlessZero(count) => open_jumps += count,
                _ => {}
            }

            index -= 1;
        }

        *instruction_ptr = index + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parser::Parser;

    #[derive(Debug, Default)]
    struct Writer {
        data: String,
    }

    impl std::io::Write for Writer {
        fn flush(&mut self) -> Result<(), std::io::Error> {
            Ok(())
        }
        fn write(&mut self, data: &[u8]) -> Result<usize, std::io::Error> {
            self.data
                .push_str(&data.iter().map(|&byte| byte as char).collect::<String>());
            Ok(data.len())
        }
    }

    fn check<R>(input: &str, reader: R) -> Parser<Writer, R, Interpreter>
    where
        R: std::io::Read,
    {
        let writer = Writer::default();
        let interpreter = Interpreter::default();
        let mut parser = Parser::<Writer, R, Interpreter>::new(writer, reader, interpreter);
        let tokens = lexer::Lexer::tokenize(input);
        parser.interpret(tokens);
        parser
    }

    #[test]
    fn move_out_of_bounds_should_grow() {
        let source = (0..20).map(|_| "<").collect::<String>();
        let parser = check(&source, std::io::Cursor::new(""));
        assert_eq!(parser.instruction_handler().data_ptr, 12);
        assert_eq!(parser.instruction_handler().tape.len(), 48);

        let source = (0..20).map(|_| ">").collect::<String>();
        let parser = check(&source, std::io::Cursor::new(""));
        assert_eq!(parser.instruction_handler().data_ptr, 36);
        assert_eq!(parser.instruction_handler().tape.len(), 48);
    }

    #[test]
    fn increment_and_decrement_wrapping_cells() {
        let source = "+++>--->---++++";
        let parser = check(source, std::io::Cursor::new(""));
        assert_eq!(parser.instruction_handler().data_ptr, 18);
        assert_eq!(parser.instruction_handler().tape[16], 3);
        assert_eq!(parser.instruction_handler().tape[17], 253);
        assert_eq!(parser.instruction_handler().tape[18], 1);
        assert_eq!(parser.instruction_handler().tape.len(), 32);
    }

    #[test]
    fn write_chars() {
        let uppercase_a = (0..65).map(|_| "+").collect::<String>();
        let uppercase_b = (0..66).map(|_| "+").collect::<String>();
        let uppercase_c = (0..67).map(|_| "+").collect::<String>();
        let source = format!("{uppercase_a}.>{uppercase_b}.>{uppercase_c}.");
        let parser = check(&source, std::io::Cursor::new(""));
        assert_eq!(parser.writer().data, "ABC");
    }

    #[test]
    fn jump_if_zero() {
        // here we just should jump directly to the end where we set the cell
        // to be 65 (ascii capital a) and write it;
        let uppercase_a = (0..65).map(|_| "+").collect::<String>();
        let source = format!("[<<<<>[>>]]{uppercase_a}.");
        let parser = check(&source, std::io::Cursor::new(""));
        assert_eq!(parser.writer().data, "A");
        assert_eq!(parser.instruction_ptr(), 8);

        // we first set the current cell to any non-zero value, so we should skip
        // the first jump
        //
        // we are skipping the all the jumps until the first non-jump instruction,
        // which should write "A" to the writer
        //
        // then, we should hit the jump-unless-zero, which should also be skipped
        // and then just write "A" again
        let source = format!("+[[>{uppercase_a}.>]]<.");
        let parser = check(&source, std::io::Cursor::new(""));
        assert_eq!(parser.writer().data, "AA");
        assert_eq!(parser.instruction_ptr(), 9);

        // we first set the current cell to any non-zero value, so we skip the loop
        //
        // he should skip all the jump-if-zero's until the first instruction inside
        // of the loop. where we just do arbitrary commands, ensuring that the cell
        // remains non-zero so we hit the jump-unless-zero, going back to the start
        let source = format!("+++++[-]{uppercase_a}.");
        // this might feel tricky, but this is the equivalent of:
        //
        // let mut i = 5;
        // while i != 0 {
        //     i -= 1;
        // }
        //
        // which is just a simple way to hit jump_unless_zero a couple times while
        // also ensuring the program finishes; after that we just write out "A" to
        // make sure we finished after the loop.
        let parser = check(&source, std::io::Cursor::new(""));
        assert_eq!(parser.writer().data, "A");
        assert_eq!(parser.instruction_ptr(), 6);
    }

    #[test]
    #[should_panic(expected = "jump if zero defined at 1 is never closed")]
    fn non_closed_jump_if_zero() {
        let source = "<<[<";
        check(source, std::io::Cursor::new(""));
    }

    #[test]
    fn hello_world() {
        let source = include_str!("../../samples/hello_world_pretty.bf");
        let parser = check(source, std::io::Cursor::new(""));

        assert_eq!(parser.writer().data, "Hello World!\n");

        let source = include_str!("../../samples/hello_world_inline.bf");
        let parser = check(source, std::io::Cursor::new(""));

        assert_eq!(parser.writer().data, "Hello World!\n");
    }

    #[test]
    fn add_two_and_five() {
        let source = include_str!("../../samples/add_2_and_5.bf");
        let parser = check(source, std::io::Cursor::new(""));

        assert_eq!(parser.writer().data, "7");
    }
}
