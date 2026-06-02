use crate::board::{Board, COLS, ROWS};

impl Board {
    pub fn is_valid_move(&self, col: usize) -> bool {
        col < COLS && self.heights[col] < ROWS
    }

    pub fn valid_moves(&self) -> Vec<usize> {
        (0..COLS).filter(|&col| self.is_valid_move(col)).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::board::{Board, COLS, ROWS, Piece};

    #[test]
    fn test_valid_moves() {
        let board = Board::new();
        assert_eq!(board.valid_moves().len(), COLS);
    }

    #[test]
    fn test_invalid_col_out_of_range() {
        let board = Board::new();
        assert!(!board.is_valid_move(COLS));
        assert!(!board.is_valid_move(usize::MAX));
    }

    #[test]
    fn test_column_fills_up() {
        let mut board = Board::new();
        for _ in 0..ROWS {
            board = board.drop_piece(0, Piece::Player1).unwrap();
        }
        assert!(!board.is_valid_move(0));
        assert_eq!(board.valid_moves().len(), COLS - 1);
    }
}
