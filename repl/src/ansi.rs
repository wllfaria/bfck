use std::io::Write;

/// convenience macro to shape the sequences to be executed, appending them
/// after the ansi prefix `\x1b (ESC)`
macro_rules! ansi {
    ($($sequence:tt),+) => {
        _ = std::io::stdout().write_all(format!("\x1b[{}\n", format!($($sequence),+)).as_bytes());
        _ = std::io::stdout().flush();
    };
}

/// clear the entire terminal viewport; note that this will not change the cursor
/// position, therefore, if needed, adjust the cursor position using `move_cursor`
///
/// example:
/// ```rust
/// // clear the screen, maybe before starting the application
/// clear_screen();
/// // moves the cursor to the (0,0) (top left) of the terminal, first row and column
/// move_cursor(0, 0);
/// ```
pub fn clear_screen() {
    ansi!("2J");
}

/// move the cursor to the x, y (col, row) of the terminal window, if the position
/// specified is out of bounds, the cursor will move to the farthest position it
/// is allowed to.
///
/// technically one might have a bigger terminal screen that doesn't fit in
/// a u16, but its not realistic.
///
/// example:
/// ```rust
/// move_cursor(10, 3);
/// ```
///
/// the example above would move the cursor to line 3, column 10 of the viewport
pub fn move_cursor(x: u16, y: u16) {
    ansi!("{x};{y}H");
}
