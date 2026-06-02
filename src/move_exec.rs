use crate::board::{Board, Cell, Piece};

impl Board {
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
}

#[cfg(test)]
mod tests {
    use crate::board::{Board, Cell, Piece};

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
        for _ in 0..crate::board::ROWS {
            board = board.drop_piece(0, Piece::Player1).unwrap();
        }
        assert!(board.drop_piece(0, Piece::Player1).is_none());
    }

    #[test]
    fn test_drop_piece_stacks() {
        let mut board = Board::new();
        board = board.drop_piece(0, Piece::Player1).unwrap();
        board = board.drop_piece(0, Piece::Player2).unwrap();
        assert_eq!(board.height(0), 2);
        assert_eq!(board.cell(0, 0), Cell::Piece(Piece::Player1));
        assert_eq!(board.cell(1, 0), Cell::Piece(Piece::Player2));
    }
}
