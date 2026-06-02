#[cfg(test)]
mod tests {
    use crate::board::{Board, Cell, Piece, COLS, ROWS};
    use crate::move_gen::{is_valid_move, valid_moves};
    use crate::move_exec::drop_piece;
    use crate::eval::{check_win, is_full, is_terminal};

    // ── Column validity ────────────────────────────────────────────────────

    #[test]
    fn test_valid_columns() {
        let board = Board::new();
        for col in 0..COLS {
            assert!(is_valid_move(&board, col), "col {col} must be valid");
        }
        assert!(!is_valid_move(&board, COLS), "col {COLS} must be out of range");
        assert!(!is_valid_move(&board, usize::MAX));
        assert_eq!(valid_moves(&board).len(), COLS, "all 7 cols must be valid initially");
    }

    // ── Column capacity ────────────────────────────────────────────────────

    #[test]
    fn test_column_capacity() {
        let mut board = Board::new();
        for i in 0..ROWS {
            assert!(drop_piece(&mut board, 0, Piece::Player1),
                "drop {} into col 0 must succeed", i + 1);
        }
        assert!(!drop_piece(&mut board, 0, Piece::Player1),
            "7th drop into a full col must be rejected");
        assert_eq!(board.height(0), ROWS);
        assert!(!valid_moves(&board).contains(&0), "full col must not appear in valid moves");
    }

    // ── Alternating players ────────────────────────────────────────────────

    #[test]
    fn test_alternating_players() {
        let moves: [(usize, Piece); 10] = [
            (0, Piece::Player1),
            (1, Piece::Player2),
            (0, Piece::Player1),
            (1, Piece::Player2),
            (0, Piece::Player1),
            (2, Piece::Player2),
            (0, Piece::Player1),
            (2, Piece::Player2),
            (3, Piece::Player1),
            (3, Piece::Player2),
        ];

        let mut board = Board::new();
        for (i, &(col, expected)) in moves.iter().enumerate() {
            assert!(drop_piece(&mut board, col, expected),
                "move {} should succeed", i + 1);
        }

        assert_eq!(board.height(0), 4);
        assert_eq!(board.height(1), 2);
        assert_eq!(board.height(2), 2);
        assert_eq!(board.height(3), 2);
        assert_eq!(board.move_count(), 10);

        assert_eq!(board.cell(0, 0), Cell::Piece(Piece::Player1));
        assert_eq!(board.cell(1, 0), Cell::Piece(Piece::Player1));
        assert_eq!(board.cell(2, 0), Cell::Piece(Piece::Player1));
        assert_eq!(board.cell(3, 0), Cell::Piece(Piece::Player1));
        assert_eq!(board.cell(0, 1), Cell::Piece(Piece::Player2));
        assert_eq!(board.cell(0, 2), Cell::Piece(Piece::Player2));
        assert_eq!(board.cell(0, 3), Cell::Piece(Piece::Player1));
        assert_eq!(board.cell(1, 3), Cell::Piece(Piece::Player2));
    }

    // ── Win detection ──────────────────────────────────────────────────────

    #[test]
    fn test_horizontal_win_correct() {
        // Player1: cols 0-3 with opponent interleaved to fill below
        let mut board = Board::new();
        drop_piece(&mut board, 0, Piece::Player1);
        drop_piece(&mut board, 0, Piece::Player2);
        drop_piece(&mut board, 1, Piece::Player1);
        drop_piece(&mut board, 1, Piece::Player2);
        drop_piece(&mut board, 2, Piece::Player1);
        drop_piece(&mut board, 2, Piece::Player2);
        drop_piece(&mut board, 3, Piece::Player1);

        assert!(check_win(&board, Piece::Player1));
        assert!(!check_win(&board, Piece::Player2));
        assert!(is_terminal(&board));
    }

    #[test]
    fn test_vertical_win_correct() {
        // Player1 stacks four in col 2, Player2 gets three in col 0 (not enough)
        let mut board = Board::new();
        drop_piece(&mut board, 2, Piece::Player1);
        drop_piece(&mut board, 0, Piece::Player2);
        drop_piece(&mut board, 2, Piece::Player1);
        drop_piece(&mut board, 0, Piece::Player2);
        drop_piece(&mut board, 2, Piece::Player1);
        drop_piece(&mut board, 0, Piece::Player2);
        drop_piece(&mut board, 2, Piece::Player1);

        assert!(check_win(&board, Piece::Player1));
        assert!(!check_win(&board, Piece::Player2));
        assert!(is_terminal(&board));
    }

    #[test]
    fn test_diagonal_up_right_correct() {
        // Diagonal from (0,0) → (1,1) → (2,2) → (3,3)
        let mut board = Board::new();
        for col in 0..4 {
            for _ in 0..col {
                drop_piece(&mut board, col, Piece::Player2);
            }
            drop_piece(&mut board, col, Piece::Player1);
        }

        assert!(check_win(&board, Piece::Player1));
        assert!(!check_win(&board, Piece::Player2));
        assert!(is_terminal(&board));
    }

    #[test]
    fn test_diagonal_down_right_correct() {
        // Diagonal from (3,0) → (2,1) → (1,2) → (0,3)
        let mut board = Board::new();
        for col in 0..4 {
            for _ in 0..(3 - col) {
                drop_piece(&mut board, col, Piece::Player2);
            }
            drop_piece(&mut board, col, Piece::Player1);
        }

        assert!(check_win(&board, Piece::Player1));
        assert!(!check_win(&board, Piece::Player2));
        assert!(is_terminal(&board));
    }

    #[test]
    fn test_no_false_positive_almost_win() {
        // Three in a row in all directions, but no four
        let mut board = Board::new();
        drop_piece(&mut board, 0, Piece::Player1);
        drop_piece(&mut board, 0, Piece::Player2);
        drop_piece(&mut board, 1, Piece::Player1);
        drop_piece(&mut board, 1, Piece::Player2);
        drop_piece(&mut board, 2, Piece::Player1);
        drop_piece(&mut board, 2, Piece::Player2);

        assert!(!check_win(&board, Piece::Player1));
        assert!(!check_win(&board, Piece::Player2));
        assert!(!is_terminal(&board));
    }

    // ── Draw (full board, no winner) ───────────────────────────────────────

    #[test]
    fn test_draw_full_board() {
        let mut board = Board::new();
        // Fill each column with paired alternation so no 4-in-a-row occurs
        // in any direction. Even cols: P1,P1,P2,P2,P1,P1. Odd cols: P2,P2,P1,P1,P2,P2.
        for col in 0..COLS {
            let (a, b) = if col % 2 == 0 {
                (Piece::Player1, Piece::Player2)
            } else {
                (Piece::Player2, Piece::Player1)
            };
            for _ in 0..2 { drop_piece(&mut board, col, a); }
            for _ in 0..2 { drop_piece(&mut board, col, b); }
            for _ in 0..2 { drop_piece(&mut board, col, a); }
        }

        assert!(is_full(&board), "all 42 cells must be filled");
        assert!(!check_win(&board, Piece::Player1), "no winner");
        assert!(!check_win(&board, Piece::Player2), "no winner");
        assert!(is_terminal(&board), "full board is terminal");
    }

    // ── Board display ──────────────────────────────────────────────────────

    #[test]
    fn test_board_display_correct() {
        let mut board = Board::new();
        drop_piece(&mut board, 0, Piece::Player1);
        drop_piece(&mut board, 1, Piece::Player2);
        drop_piece(&mut board, 0, Piece::Player1);

        let output = format!("{board}");
        assert!(output.contains('X'), "display must show Player1 as X");
        assert!(output.contains('O'), "display must show Player2 as O");
        assert!(output.starts_with('|'), "display must start with board border");
        assert!(output.contains("1 2 3 4 5 6 7"), "display must have column numbers");
    }

    // ── Full game walkthrough ──────────────────────────────────────────────

    #[test]
    fn test_full_game_sequence() {
        let mut board = Board::new();

        // Simulate a plausible 9-move game snapshot.
        let seq: [(usize, Piece); 9] = [
            (3, Piece::Player1),
            (3, Piece::Player2),
            (4, Piece::Player1),
            (2, Piece::Player2),
            (4, Piece::Player1),
            (2, Piece::Player2),
            (5, Piece::Player1),
            (1, Piece::Player2),
            (5, Piece::Player1),
        ];

        for (i, &(col, expected)) in seq.iter().enumerate() {
            assert!(drop_piece(&mut board, col, expected),
                "move {} failed", i + 1);
            assert_eq!(board.move_count(), (i + 1) as u32);
            // After each move, the other player hasn't won
            let other = expected.opponent();
            assert!(!check_win(&board, other),
                "player {} must not win on opponent's move", i + 1);
        }

        // After these 9 moves the board looks like:
        //  col 1: P2
        //  col 2: P2, P2
        //  col 3: P1, P2
        //  col 4: P1, P1
        //  col 5: P1, P1
        assert_eq!(board.cell(0, 3), Cell::Piece(Piece::Player1)); // first drop in center
        assert_eq!(board.cell(1, 3), Cell::Piece(Piece::Player2));
        assert_eq!(board.cell(0, 4), Cell::Piece(Piece::Player1));
        assert_eq!(board.cell(0, 2), Cell::Piece(Piece::Player2));
        assert_eq!(board.cell(1, 4), Cell::Piece(Piece::Player1));

        assert!(!is_terminal(&board), "game must still be in progress");
    }

    // ── Opponent helper ────────────────────────────────────────────────────

    #[test]
    fn test_piece_opponent_consistency() {
        assert_eq!(Piece::Player1.opponent(), Piece::Player2);
        assert_eq!(Piece::Player2.opponent(), Piece::Player1);
        assert_eq!(Piece::Player1.opponent().opponent(), Piece::Player1);
    }
}
