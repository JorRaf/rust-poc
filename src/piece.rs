use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TetrominoKind {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl TetrominoKind {
    pub fn random(rng: &mut impl Rng) -> Self {
        match rng.gen_range(0..7) {
            0 => Self::I,
            1 => Self::O,
            2 => Self::T,
            3 => Self::S,
            4 => Self::Z,
            5 => Self::J,
            _ => Self::L,
        }
    }

    pub fn blocks(self, rotation: usize) -> [(i32, i32); 4] {
        match self {
            Self::I => match rotation % 4 {
                0 | 2 => [(0, 1), (1, 1), (2, 1), (3, 1)],
                _ => [(2, 0), (2, 1), (2, 2), (2, 3)],
            },
            Self::O => [(1, 0), (2, 0), (1, 1), (2, 1)],
            Self::T => match rotation % 4 {
                0 => [(1, 0), (0, 1), (1, 1), (2, 1)],
                1 => [(1, 0), (1, 1), (2, 1), (1, 2)],
                2 => [(0, 1), (1, 1), (2, 1), (1, 2)],
                _ => [(1, 0), (0, 1), (1, 1), (1, 2)],
            },
            Self::S => match rotation % 4 {
                0 | 2 => [(1, 0), (2, 0), (0, 1), (1, 1)],
                _ => [(1, 0), (1, 1), (2, 1), (2, 2)],
            },
            Self::Z => match rotation % 4 {
                0 | 2 => [(0, 0), (1, 0), (1, 1), (2, 1)],
                _ => [(2, 0), (1, 1), (2, 1), (1, 2)],
            },
            Self::J => match rotation % 4 {
                0 => [(0, 0), (0, 1), (1, 1), (2, 1)],
                1 => [(1, 0), (2, 0), (1, 1), (1, 2)],
                2 => [(0, 1), (1, 1), (2, 1), (2, 2)],
                _ => [(1, 0), (1, 1), (0, 2), (1, 2)],
            },
            Self::L => match rotation % 4 {
                0 => [(2, 0), (0, 1), (1, 1), (2, 1)],
                1 => [(1, 0), (1, 1), (1, 2), (2, 2)],
                2 => [(0, 1), (1, 1), (2, 1), (0, 2)],
                _ => [(0, 0), (1, 0), (1, 1), (1, 2)],
            },
        }
    }

    pub fn preview(self) -> [&'static str; 4] {
        match self {
            Self::I => ["[][][][]", "        ", "        ", "        "],
            Self::O => ["  [][]  ", "  [][]  ", "        ", "        "],
            Self::T => ["  []    ", "[][][]  ", "        ", "        "],
            Self::S => ["  [][]  ", "[][]    ", "        ", "        "],
            Self::Z => ["[][]    ", "  [][]  ", "        ", "        "],
            Self::J => ["[]      ", "[][][]  ", "        ", "        "],
            Self::L => ["    []  ", "[][][]  ", "        ", "        "],
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub kind: TetrominoKind,
    pub rotation: usize,
    pub x: i32,
    pub y: i32,
}

impl Piece {
    pub fn new(kind: TetrominoKind) -> Self {
        Self {
            kind,
            rotation: 0,
            x: 3,
            y: 0,
        }
    }

    pub fn cells(self) -> [(i32, i32); 4] {
        let mut cells = self.kind.blocks(self.rotation);
        for cell in &mut cells {
            cell.0 += self.x;
            cell.1 += self.y;
        }
        cells
    }

    pub fn moved(self, dx: i32, dy: i32) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
            ..self
        }
    }

    pub fn rotated(self) -> Self {
        Self {
            rotation: (self.rotation + 1) % 4,
            ..self
        }
    }
}
