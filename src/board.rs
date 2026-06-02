use std::fmt;

pub const ROWS: usize = 6;
pub const COLS: usize = 7;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Piece {
    Player1,
    Player2,
}

impl Piece {
    pub fn opponent(self) -> Self {
        match self {
            Piece::Player1 => Piece::Player2,
            Piece::Player2 => Piece::Player1,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Piece(Piece),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board {
    pub(crate) cells: [[Cell; COLS]; ROWS],
    pub(crate) heights: [usize; COLS],
    pub(crate) move_count: u32,
}

impl Board {
    pub fn new() -> Self {
        Board {
            cells: [[Cell::Empty; COLS]; ROWS],
            heights: [0; COLS],
            move_count: 0,
        }
    }

    pub fn cols(&self) -> usize {
        COLS
    }

    pub fn rows(&self) -> usize {
        ROWS
    }

    pub fn cell(&self, row: usize, col: usize) -> Cell {
        self.cells[row][col]
    }

    pub fn height(&self, col: usize) -> usize {
        self.heights[col]
    }

    pub fn move_count(&self) -> u32 {
        self.move_count
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in (0..ROWS).rev() {
            write!(f, "|")?;
            for col in 0..COLS {
                let c = match self.cells[row][col] {
                    Cell::Empty => ' ',
                    Cell::Piece(Piece::Player1) => 'X',
                    Cell::Piece(Piece::Player2) => 'O',
                };
                write!(f, "{}", c)?;
                if col < COLS - 1 {
                    write!(f, " ")?;
                }
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "+{}+", "-".repeat(2 * COLS - 1))?;
        write!(f, " ")?;
        for col in 0..COLS {
            write!(f, "{}", col + 1)?;
            if col < COLS - 1 {
                write!(f, " ")?;
            }
        }
        writeln!(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board_empty() {
        let board = Board::new();
        assert_eq!(board.move_count(), 0);
        for col in 0..COLS {
            assert_eq!(board.height(col), 0);
        }
    }

    #[test]
    fn test_opponent() {
        assert_eq!(Piece::Player1.opponent(), Piece::Player2);
        assert_eq!(Piece::Player2.opponent(), Piece::Player1);
    }
}
