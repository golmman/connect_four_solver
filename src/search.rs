use crate::board::{Board, Piece};
use crate::eval::{check_win, is_full};
use crate::move_exec::drop_piece;
use crate::move_gen::valid_moves;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchResult {
    Win,
    Loss,
    Draw,
}

const INF: u32 = u32::MAX / 2;

pub fn solve(board: &Board, player: Piece) -> SearchResult {
    let mut pn_limit = 1;
    let mut dn_limit = 1;

    loop {
        let mut b = board.clone();
        let (result, _, _) = df_pn(&mut b, player, true, pn_limit, dn_limit);
        if let Some(r) = result {
            return r;
        }
        pn_limit = pn_limit.saturating_mul(2).min(INF);
        dn_limit = dn_limit.saturating_mul(2).min(INF);
    }
}

fn df_pn(
    board: &mut Board,
    prover: Piece,
    is_or: bool,
    pn_limit: u32,
    dn_limit: u32,
) -> (Option<SearchResult>, u32, u32) {
    if check_win(board, prover) {
        return (Some(SearchResult::Win), 0, INF);
    }
    if check_win(board, prover.opponent()) {
        return (Some(SearchResult::Loss), INF, 0);
    }
    if is_full(board) {
        return (Some(SearchResult::Draw), INF, INF);
    }

    let moves = valid_moves(board);
    if moves.is_empty() {
        return (Some(SearchResult::Draw), INF, INF);
    }

    let current_player = if is_or { prover } else { prover.opponent() };
    let n = moves.len();
    let mut child_pn = vec![1u32; n];
    let mut child_dn = vec![1u32; n];

    loop {
        let (node_pn, node_dn) = if is_or {
            let pn = *child_pn.iter().min().unwrap_or(&INF);
            let dn = child_dn.iter().fold(0u32, |a, &b| a.saturating_add(b)).min(INF);
            (pn, dn)
        } else {
            let pn = child_pn.iter().fold(0u32, |a, &b| a.saturating_add(b)).min(INF);
            let dn = *child_dn.iter().min().unwrap_or(&INF);
            (pn, dn)
        };

        if node_pn == 0 {
            return (Some(SearchResult::Win), 0, INF);
        }
        if node_dn == 0 {
            return (Some(SearchResult::Loss), INF, 0);
        }
        if node_pn > pn_limit || node_dn > dn_limit {
            return (None, node_pn, node_dn);
        }

        let best_idx = if is_or {
            child_pn
                .iter()
                .enumerate()
                .min_by_key(|&(_, pn)| *pn)
                .map(|(i, _)| i)
                .unwrap()
        } else {
            child_dn
                .iter()
                .enumerate()
                .min_by_key(|&(_, dn)| *dn)
                .map(|(i, _)| i)
                .unwrap()
        };

        let (child_pn_limit, child_dn_limit) = if is_or {
            let sibling_dn_sum = child_dn
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != best_idx)
                .fold(0u32, |a, (_, &dn)| a.saturating_add(dn))
                .min(INF);
            let remaining_dn = if node_dn > dn_limit {
                1u32
            } else {
                dn_limit.saturating_sub(sibling_dn_sum).max(1)
            };
            (pn_limit, remaining_dn)
        } else {
            let sibling_pn_sum = child_pn
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != best_idx)
                .fold(0u32, |a, (_, &pn)| a.saturating_add(pn))
                .min(INF);
            let remaining_pn = if node_pn > pn_limit {
                1u32
            } else {
                pn_limit.saturating_sub(sibling_pn_sum).max(1)
            };
            (remaining_pn, dn_limit)
        };

        let mut child_board = board.clone();
        drop_piece(&mut child_board, moves[best_idx], current_player);
        let (result, new_pn, new_dn) = df_pn(
            &mut child_board,
            prover,
            !is_or,
            child_pn_limit,
            child_dn_limit,
        );

        child_pn[best_idx] = new_pn;
        child_dn[best_idx] = new_dn;

        match result {
            Some(SearchResult::Win) => {
                if is_or {
                    return (Some(SearchResult::Win), 0, INF);
                }
            }
            Some(SearchResult::Loss) => {
                if !is_or {
                    return (Some(SearchResult::Loss), INF, 0);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::board::{Board, Piece};
    use crate::move_exec::drop_piece;
    use crate::search::{solve, SearchResult};

    #[test]
    fn test_one_move_horizontal_win() {
        let mut board = Board::new();
        drop_piece(&mut board, 0, Piece::Player1);
        drop_piece(&mut board, 0, Piece::Player2);
        drop_piece(&mut board, 1, Piece::Player1);
        drop_piece(&mut board, 1, Piece::Player2);
        drop_piece(&mut board, 2, Piece::Player1);
        drop_piece(&mut board, 2, Piece::Player2);
        assert_eq!(solve(&board, Piece::Player1), SearchResult::Win);
    }

    #[test]
    fn test_one_move_vertical_win() {
        let mut board = Board::new();
        drop_piece(&mut board, 0, Piece::Player1);
        drop_piece(&mut board, 1, Piece::Player2);
        drop_piece(&mut board, 0, Piece::Player1);
        drop_piece(&mut board, 1, Piece::Player2);
        drop_piece(&mut board, 0, Piece::Player1);
        drop_piece(&mut board, 1, Piece::Player2);
        assert_eq!(solve(&board, Piece::Player1), SearchResult::Win);
    }

    #[test]
    fn test_immediate_loss() {
        let mut board = Board::new();
        // Player2 gets 4 in a row at row 0 (cols 0-3)
        drop_piece(&mut board, 6, Piece::Player1);  // X at (0,6)
        drop_piece(&mut board, 0, Piece::Player2);  // O at (0,0)
        drop_piece(&mut board, 5, Piece::Player1);  // X at (0,5)
        drop_piece(&mut board, 1, Piece::Player2);  // O at (0,1)
        drop_piece(&mut board, 4, Piece::Player1);  // X at (0,4)
        drop_piece(&mut board, 2, Piece::Player2);  // O at (0,2)
        drop_piece(&mut board, 6, Piece::Player1);  // X at (1,6)
        drop_piece(&mut board, 3, Piece::Player2);  // O at (0,3) → Player2 wins
        assert_eq!(solve(&board, Piece::Player1), SearchResult::Loss);
    }
}
