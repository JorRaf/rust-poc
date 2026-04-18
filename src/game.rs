use rand::thread_rng;

use crate::{
    constants::{BOARD_HEIGHT, BOARD_WIDTH},
    piece::{Piece, TetrominoKind},
};

pub type Board = [[Option<TetrominoKind>; BOARD_WIDTH]; BOARD_HEIGHT];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameCommand {
    MoveLeft,
    MoveRight,
    SoftDrop,
    Rotate,
    HardDrop,
}

pub struct Game {
    pub board: Board,
    pub current: Piece,
    pub next: TetrominoKind,
    pub score: u32,
    pub lines_cleared: u32,
    pub game_over: bool,
}

impl Game {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let current_kind = TetrominoKind::random(&mut rng);
        let next = TetrominoKind::random(&mut rng);

        Self {
            board: [[None; BOARD_WIDTH]; BOARD_HEIGHT],
            current: Piece::new(current_kind),
            next,
            score: 0,
            lines_cleared: 0,
            game_over: false,
        }
    }

    pub fn is_valid_position(&self, piece: Piece) -> bool {
        piece.cells().iter().all(|&(x, y)| {
            let (Ok(xu), Ok(yu)) = (usize::try_from(x), usize::try_from(y)) else {
                return false;
            };

            xu < BOARD_WIDTH && yu < BOARD_HEIGHT && self.board[yu][xu].is_none()
        })
    }

    pub fn try_move(&mut self, dx: i32, dy: i32) -> bool {
        let moved = self.current.moved(dx, dy);
        if self.is_valid_position(moved) {
            self.current = moved;
            true
        } else {
            false
        }
    }

    pub fn try_rotate(&mut self) {
        let rotated = self.current.rotated();
        let kicks = [0, -1, 1, -2, 2];

        for kick in kicks {
            let candidate = rotated.moved(kick, 0);
            if self.is_valid_position(candidate) {
                self.current = candidate;
                break;
            }
        }
    }

    pub fn hard_drop(&mut self) {
        while self.try_move(0, 1) {}
        self.lock_piece();
    }

    pub fn handle_command(&mut self, command: GameCommand) {
        if self.game_over {
            return;
        }

        match command {
            GameCommand::MoveLeft => {
                self.try_move(-1, 0);
            }
            GameCommand::MoveRight => {
                self.try_move(1, 0);
            }
            GameCommand::SoftDrop => {
                if !self.try_move(0, 1) {
                    self.lock_piece();
                }
            }
            GameCommand::Rotate => {
                self.try_rotate();
            }
            GameCommand::HardDrop => {
                self.hard_drop();
            }
        }
    }

    pub fn tick(&mut self) {
        if !self.try_move(0, 1) {
            self.lock_piece();
        }
    }

    pub fn lock_piece(&mut self) {
        let cells = self.current.cells();
        for (x, y) in cells {
            let (Ok(xu), Ok(yu)) = (usize::try_from(x), usize::try_from(y)) else {
                self.game_over = true;
                return;
            };

            if xu >= BOARD_WIDTH || yu >= BOARD_HEIGHT {
                self.game_over = true;
                return;
            }

            self.board[yu][xu] = Some(self.current.kind);
        }

        let cleared = clear_lines(&mut self.board);
        self.lines_cleared += cleared;
        self.score += match cleared {
            1 => 100,
            2 => 300,
            3 => 500,
            4 => 800,
            _ => 0,
        };

        self.spawn_next_piece();
    }

    pub fn occupied_cell(&self, x: usize, y: usize) -> bool {
        self.board[y][x].is_some()
            || self
                .current
                .cells()
                .iter()
                .any(|&(px, py)| px == x as i32 && py == y as i32)
    }

    fn spawn_next_piece(&mut self) {
        let mut rng = thread_rng();
        self.current = Piece::new(self.next);
        self.next = TetrominoKind::random(&mut rng);

        if !self.is_valid_position(self.current) {
            self.game_over = true;
        }
    }
}

pub fn clear_lines(board: &mut Board) -> u32 {
    let mut kept_rows = Vec::with_capacity(BOARD_HEIGHT);
    let mut cleared = 0;

    for row in *board {
        if row.iter().all(Option::is_some) {
            cleared += 1;
        } else {
            kept_rows.push(row);
        }
    }

    while kept_rows.len() < BOARD_HEIGHT {
        kept_rows.insert(0, [None; BOARD_WIDTH]);
    }

    board.copy_from_slice(&kept_rows);
    cleared
}
