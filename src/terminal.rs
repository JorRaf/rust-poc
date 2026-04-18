use std::io::{self, Stdout, stdout};

use crossterm::{
    cursor, execute,
    terminal::{self, ClearType},
};

use crate::constants::BOARD_HEIGHT;

pub struct TerminalGuard {
    stdout: Stdout,
}

impl TerminalGuard {
    pub fn new() -> io::Result<Self> {
        let mut stdout = stdout();
        terminal::enable_raw_mode()?;
        execute!(
            stdout,
            cursor::Hide,
            cursor::MoveTo(0, 0),
            terminal::Clear(ClearType::All)
        )?;
        Ok(Self { stdout })
    }

    pub fn stdout(&mut self) -> &mut Stdout {
        &mut self.stdout
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = execute!(
            self.stdout,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, (BOARD_HEIGHT + 10) as u16),
            cursor::Show
        );
        let _ = terminal::disable_raw_mode();
    }
}
