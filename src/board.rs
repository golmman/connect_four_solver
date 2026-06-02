#![allow(dead_code)]

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
    cells: [[Cell; COLS]; ROWS],
    heights: [usize; COLS],
    move_count: u32,
}

impl Board {
    pub fn new() -> Self {
        Board {
            cells: [[Cell::Empty; COLS]; ROWS],
            heights: [0; COLS],
            move_count: 0,
        }
    }

    #[allow(dead_code)]
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

    pub fn is_valid_move(&self, col: usize) -> bool {
        col < COLS && self.heights[col] < ROWS
    }

    pub fn valid_moves(&self) -> Vec<usize> {
        (0..COLS).filter(|&col| self.is_valid_move(col)).collect()
    }

    pub fn drop_piece(&self, col: usize, piece: Piece) -> Option<Board> {
        if !self.is_valid_move(col) {
            return None;
        }
        let mut new_board = self.clone();
        let row = new_board.heights[col];
        new_board.cells[row][col] = Cell::Piece(piece);
        new_board.heights[col] += 1;
        new_board.move_count += 1;
        Some(new_board)
    }

    pub fn check_win(&self, piece: Piece) -> bool {
        let directions = [(0, 1), (1, 0), (1, 1), (1, -1)];
        for row in 0..ROWS {
            for col in 0..COLS {
                if self.cells[row][col] != Cell::Piece(piece) {
                    continue;
                }
                for &(dr, dc) in &directions {
                    let mut count = 1;
                    for step in 1..4 {
                        let r = row as isize + dr * step;
                        let c = col as isize + dc * step;
                        if r < 0 || r >= ROWS as isize || c < 0 || c >= COLS as isize {
                            break;
                        }
                        if self.cells[r as usize][c as usize] != Cell::Piece(piece) {
                            break;
                        }
                        count += 1;
                    }
                    if count == 4 {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn is_full(&self) -> bool {
        self.move_count as usize == ROWS * COLS
    }

    pub fn is_terminal(&self) -> bool {
        self.is_full() || self.check_win(Piece::Player1) || self.check_win(Piece::Player2)
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
        assert!(!board.is_full());
        assert!(!board.is_terminal());
        assert_eq!(board.move_count(), 0);
        for col in 0..COLS {
            assert_eq!(board.height(col), 0);
            assert!(board.is_valid_move(col));
        }
    }

    #[test]
    fn test_drop_piece() {
        let board = Board::new();
        let board = board.drop_piece(0, Piece::Player1).unwrap();
        assert_eq!(board.height(0), 1);
        assert_eq!(board.cell(0, 0), Cell::Piece(Piece::Player1));
    }

    #[test]
    fn test_drop_piece_full_column() {
        let mut board = Board::new();
        for _ in 0..ROWS {
            board = board.drop_piece(0, Piece::Player1).unwrap();
        }
        assert!(!board.is_valid_move(0));
        assert!(board.drop_piece(0, Piece::Player1).is_none());
    }

    #[test]
    fn test_horizontal_win() {
        let mut board = Board::new();
        for col in 0..4 {
            board = board.drop_piece(col, Piece::Player1).unwrap();
            if col < 3 {
                board = board.drop_piece(col, Piece::Player2).unwrap();
            }
        }
        assert!(board.check_win(Piece::Player1));
    }

    #[test]
    fn test_vertical_win() {
        let mut board = Board::new();
        for _ in 0..4 {
            board = board.drop_piece(0, Piece::Player1).unwrap();
            board = board.drop_piece(1, Piece::Player2).unwrap();
        }
        assert!(board.check_win(Piece::Player1));
    }

    #[test]
    fn test_diagonal_win_up_right() {
        let mut board = Board::new();
        for col in 0..4 {
            for _ in 0..col {
                board = board.drop_piece(col, Piece::Player2).unwrap();
            }
            board = board.drop_piece(col, Piece::Player1).unwrap();
        }
        assert!(board.check_win(Piece::Player1));
    }

    #[test]
    fn test_diagonal_win_down_right() {
        let mut board = Board::new();
        for col in 0..4 {
            for _ in 0..(3 - col) {
                board = board.drop_piece(col, Piece::Player2).unwrap();
            }
            board = board.drop_piece(col, Piece::Player1).unwrap();
        }
        assert!(board.check_win(Piece::Player1));
    }

    #[test]
    fn test_no_win() {
        let board = Board::new();
        assert!(!board.check_win(Piece::Player1));
        assert!(!board.check_win(Piece::Player2));
    }

    #[test]
    fn test_is_terminal_full() {
        let mut board = Board::new();
        for i in 0..(ROWS * COLS) {
            let col = i % COLS;
            let piece = if (i / COLS) % 2 == 0 {
                Piece::Player1
            } else {
                Piece::Player2
            };
            board = board.drop_piece(col, piece).unwrap();
        }
        assert!(board.is_full());
        assert!(board.is_terminal());
    }

    #[test]
    fn test_valid_moves() {
        let board = Board::new();
        assert_eq!(board.valid_moves().len(), COLS);
    }

    #[test]
    fn test_opponent() {
        assert_eq!(Piece::Player1.opponent(), Piece::Player2);
        assert_eq!(Piece::Player2.opponent(), Piece::Player1);
    }
}
