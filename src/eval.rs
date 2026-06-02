use crate::board::{Board, Cell, Piece, COLS, ROWS};

impl Board {
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

#[cfg(test)]
mod tests {
    use crate::board::{Board, Piece, ROWS, COLS};

    #[test]
    fn test_no_win() {
        let board = Board::new();
        assert!(!board.check_win(Piece::Player1));
        assert!(!board.check_win(Piece::Player2));
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
        assert!(!board.check_win(Piece::Player2));
    }

    #[test]
    fn test_vertical_win() {
        let mut board = Board::new();
        board = board.drop_piece(0, Piece::Player1).unwrap();
        board = board.drop_piece(1, Piece::Player2).unwrap();
        board = board.drop_piece(0, Piece::Player1).unwrap();
        board = board.drop_piece(1, Piece::Player2).unwrap();
        board = board.drop_piece(0, Piece::Player1).unwrap();
        board = board.drop_piece(2, Piece::Player2).unwrap();
        board = board.drop_piece(0, Piece::Player1).unwrap();
        assert!(board.check_win(Piece::Player1));
        assert!(!board.check_win(Piece::Player2));
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
    fn test_is_terminal_on_win() {
        let mut board = Board::new();
        for col in 0..4 {
            board = board.drop_piece(col, Piece::Player1).unwrap();
            if col < 3 {
                board = board.drop_piece(col, Piece::Player2).unwrap();
            }
        }
        assert!(board.is_terminal());
    }

    #[test]
    fn test_not_terminal_midgame() {
        let board = Board::new()
            .drop_piece(0, Piece::Player1).unwrap()
            .drop_piece(1, Piece::Player2).unwrap();
        assert!(!board.is_full());
        assert!(!board.is_terminal());
    }
}
