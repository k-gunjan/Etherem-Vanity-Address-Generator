use crossterm::{cursor, execute, queue, terminal, Result};
use regex::Regex;
use std::io::{stdout, Write};

pub fn cli_display(s: &str) -> Result<()> {
    // Create a new terminal screen
    let mut screen = stdout();
    // Hide the cursor
    execute!(screen, cursor::Hide)?;

    // Move the cursor to the beginning of the line
    queue!(screen, cursor::MoveToColumn(0))?;

    // Clear the current line
    queue!(screen, terminal::Clear(terminal::ClearType::CurrentLine))?;

    // Print the counter
    print!("generator: {}", s);

    // Flush the screen to display the updated output
    screen.flush()?;

    // Show the cursor again
    execute!(screen, cursor::Show)?;
    Ok(())
}

// check if the string is comprised of hex chars
pub fn is_hex_string(input: &str) -> bool {
    let hex_regex = Regex::new(r"^[0-9a-fA-F]+$").unwrap();
    hex_regex.is_match(input)
}
