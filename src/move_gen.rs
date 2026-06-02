use crate::board::{Board, COLS, ROWS};

pub fn is_valid_move(board: &Board, col: usize) -> bool {
    col < COLS && board.heights[col] < ROWS
}

pub fn valid_moves(board: &Board) -> Vec<usize> {
    (0..COLS).filter(|&col| is_valid_move(board, col)).collect()
}

#[cfg(test)]
mod tests {
    use crate::board::{Board, COLS, ROWS, Piece};
    use crate::move_exec::drop_piece;
    use super::{is_valid_move, valid_moves};

    #[test]
    fn test_valid_moves() {
        let board = Board::new();
        assert_eq!(valid_moves(&board).len(), COLS);
    }

    #[test]
    fn test_invalid_col_out_of_range() {
        let board = Board::new();
        assert!(!is_valid_move(&board, COLS));
        assert!(!is_valid_move(&board, usize::MAX));
    }

    #[test]
    fn test_column_fills_up() {
        let mut board = Board::new();
        for _ in 0..ROWS {
            assert!(drop_piece(&mut board, 0, Piece::Player1));
        }
        assert!(!is_valid_move(&board, 0));
        assert_eq!(valid_moves(&board).len(), COLS - 1);
    }
}
