use std::{io, thread, time::Instant};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

mod constants;
mod game;
mod piece;
mod render;
mod terminal;

use constants::{FALL_INTERVAL, FRAME_TIME};
use game::Game;
use render::render;
use terminal::TerminalGuard;

fn main() -> io::Result<()> {
    let mut terminal = TerminalGuard::new()?;
    let mut game = Game::new();
    let mut last_tick = Instant::now();

    loop {
        render(terminal.stdout(), &game)?;

        if event::poll(FRAME_TIME)? {
            let Event::Key(key_event) = event::read()? else {
                continue;
            };

            if key_event.kind != KeyEventKind::Press {
                continue;
            }

            match key_event.code {
                KeyCode::Left if !game.game_over => {
                    game.try_move(-1, 0);
                }
                KeyCode::Right if !game.game_over => {
                    game.try_move(1, 0);
                }
                KeyCode::Down if !game.game_over && !game.try_move(0, 1) => {
                    game.lock_piece();
                }
                KeyCode::Up if !game.game_over => {
                    game.try_rotate();
                }
                KeyCode::Char(' ') if !game.game_over => {
                    game.hard_drop();
                }
                KeyCode::Char('q') => break,
                _ => {}
            }
        }

        if !game.game_over && last_tick.elapsed() >= FALL_INTERVAL {
            game.tick();
            last_tick = Instant::now();
        }

        thread::sleep(FRAME_TIME);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{BOARD_HEIGHT, BOARD_WIDTH};
    use crate::game::clear_lines;
    use crate::piece::{Piece, TetrominoKind};

    #[test]
    fn clears_completed_lines_and_keeps_empty_rows_on_top() {
        let mut board = [[None; BOARD_WIDTH]; BOARD_HEIGHT];
        board[BOARD_HEIGHT - 1] = [Some(TetrominoKind::I); BOARD_WIDTH];
        board[BOARD_HEIGHT - 2][0] = Some(TetrominoKind::O);

        let cleared = clear_lines(&mut board);

        assert_eq!(cleared, 1);
        assert_eq!(board[0], [None; BOARD_WIDTH]);
        assert_eq!(board[BOARD_HEIGHT - 1][0], Some(TetrominoKind::O));
    }

    #[test]
    fn rejects_piece_outside_board() {
        let game = Game::new();
        let piece = Piece::new(TetrominoKind::O).moved(-5, 0);

        assert!(!game.is_valid_position(piece));
    }

    #[test]
    fn locks_piece_when_downward_move_fails() {
        let mut game = Game::new();
        game.current = Piece::new(TetrominoKind::O).moved(0, (BOARD_HEIGHT - 2) as i32);

        game.tick();

        assert!(game.board[BOARD_HEIGHT - 1][4].is_some());
        assert!(game.board[BOARD_HEIGHT - 2][4].is_some());
    }
}
