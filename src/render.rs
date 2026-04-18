use std::io::{self, Stdout, Write};

use crossterm::{
    cursor, queue,
    terminal::{self, ClearType},
};

use crate::{constants::BOARD_WIDTH, game::Game};

pub fn render(stdout: &mut Stdout, game: &Game) -> io::Result<()> {
    queue!(
        stdout,
        cursor::MoveTo(0, 0),
        terminal::Clear(ClearType::All)
    )?;
    queue!(stdout, cursor::MoveTo(0, 0))?;
    write!(stdout, "Terminal Tetris")?;
    queue!(stdout, cursor::MoveTo(0, 1))?;
    write!(
        stdout,
        "Left/Right: move  Up: rotate  Down: soft drop  Space: hard drop  q: quit"
    )?;
    queue!(stdout, cursor::MoveTo(0, 2))?;
    write!(
        stdout,
        "Score: {}   Lines: {}",
        game.score, game.lines_cleared
    )?;

    for y in 0..crate::constants::BOARD_HEIGHT {
        queue!(stdout, cursor::MoveTo(0, (y + 4) as u16))?;
        write!(stdout, "|")?;
        for x in 0..BOARD_WIDTH {
            let cell = if game.occupied_cell(x, y) { "[]" } else { " ." };
            write!(stdout, "{cell}")?;
        }
        write!(stdout, "|")?;
    }

    queue!(
        stdout,
        cursor::MoveTo(0, (crate::constants::BOARD_HEIGHT + 4) as u16)
    )?;
    write!(stdout, "+")?;
    for _ in 0..BOARD_WIDTH {
        write!(stdout, "--")?;
    }
    write!(stdout, "+")?;

    let sidebar_x = (BOARD_WIDTH * 2 + 4) as u16;
    queue!(stdout, cursor::MoveTo(sidebar_x, 4))?;
    write!(stdout, "Next:")?;
    for (index, line) in game.next.preview().iter().enumerate() {
        queue!(stdout, cursor::MoveTo(sidebar_x, (5 + index) as u16))?;
        write!(stdout, "{line}")?;
    }

    if game.game_over {
        queue!(stdout, cursor::MoveTo(sidebar_x, 11))?;
        write!(stdout, "Game over. Press q to exit.")?;
    }

    stdout.flush()?;
    Ok(())
}
