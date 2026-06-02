use crate::board::{Board, Cell, Piece};

pub fn drop_piece(board: &mut Board, col: usize, piece: Piece) -> bool {
    if !board.is_valid_move(col) {
        return false;
    }
    let row = board.heights[col];
    board.cells[row][col] = Cell::Piece(piece);
    board.heights[col] += 1;
    board.move_count += 1;
    true
}

#[cfg(test)]
mod tests {
    use crate::board::{Board, Cell, Piece, ROWS};
    use super::drop_piece;

    #[test]
    fn test_drop_piece() {
        let mut board = Board::new();
        assert!(drop_piece(&mut board, 0, Piece::Player1));
        assert_eq!(board.height(0), 1);
        assert_eq!(board.cell(0, 0), Cell::Piece(Piece::Player1));
    }

    #[test]
    fn test_drop_piece_full_column() {
        let mut board = Board::new();
        for _ in 0..ROWS {
            assert!(drop_piece(&mut board, 0, Piece::Player1));
        }
        assert!(!drop_piece(&mut board, 0, Piece::Player1));
    }

    #[test]
    fn test_drop_piece_stacks() {
        let mut board = Board::new();
        assert!(drop_piece(&mut board, 0, Piece::Player1));
        assert!(drop_piece(&mut board, 0, Piece::Player2));
        assert_eq!(board.height(0), 2);
        assert_eq!(board.cell(0, 0), Cell::Piece(Piece::Player1));
        assert_eq!(board.cell(1, 0), Cell::Piece(Piece::Player2));
    }
}
