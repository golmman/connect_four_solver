use std::time::{Duration, Instant};

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
    let mut ctx = SearchCtx {
        nodes: 0,
        start: Instant::now(),
        next_report: Instant::now(),
        verbose: false,
    };
    let mut pn_limit = 1;
    let mut dn_limit = 1;
    let mut pv = Vec::new();

    loop {
        let mut b = board.clone();
        pv.clear();
        let (result, _, _) = df_pn(&mut b, player, true, pn_limit, dn_limit, &mut ctx, &mut pv, 0);
        if let Some(r) = result {
            return r;
        }
        pn_limit = pn_limit.saturating_mul(2).min(INF);
        dn_limit = dn_limit.saturating_mul(2).min(INF);
    }
}

pub fn solve_verbose(board: &Board, player: Piece) -> SearchResult {
    let mut ctx = SearchCtx {
        nodes: 0,
        start: Instant::now(),
        next_report: Instant::now(),
        verbose: true,
    };
    let mut pn_limit = 1;
    let mut dn_limit = 1;
    let mut pv = Vec::new();

    print_header();

    loop {
        let mut b = board.clone();
        pv.clear();
        let (result, node_pn, node_dn) = df_pn(
            &mut b, player, true, pn_limit, dn_limit,
            &mut ctx, &mut pv, 0,
        );
        if let Some(r) = result {
            report_final(&ctx, node_pn, node_dn, &pv, r);
            return r;
        }
        report_iteration(&ctx, node_pn, node_dn, &pv);
        pn_limit = pn_limit.saturating_mul(2).min(INF);
        dn_limit = dn_limit.saturating_mul(2).min(INF);
    }
}

struct SearchCtx {
    nodes: u64,
    start: Instant,
    next_report: Instant,
    verbose: bool,
}

fn report_final(ctx: &SearchCtx, node_pn: u32, node_dn: u32, pv: &[usize], result: SearchResult) {
    let elapsed = ctx.start.elapsed();
    let secs = elapsed.as_secs_f64();
    let nps = if secs > 0.0 { (ctx.nodes as f64 / secs) as u64 } else { ctx.nodes };
    println!();
    print_info(ctx.nodes, nps, 0, node_pn, node_dn, pv, "done");
    println!();
    println!("result: {:?}, {} nodes in {:.3}s ({}/s)", result, ctx.nodes, secs, nps);
}

fn report_iteration(ctx: &SearchCtx, node_pn: u32, node_dn: u32, pv: &[usize]) {
    let elapsed = ctx.start.elapsed();
    let secs = elapsed.as_secs_f64();
    let nps = if secs > 0.0 { (ctx.nodes as f64 / secs) as u64 } else { ctx.nodes };
    println!();
    print_info(ctx.nodes, nps, 0, node_pn, node_dn, pv, "iteration complete, deepening");
    println!();
}

fn print_header() {
    println!("{:>12} {:>10} {:>10} {:>6} {:>6}  pv", "nodes", "nps", "depth", "pn", "dn");
    println!("{}", "-".repeat(70));
}

fn print_info(nodes: u64, nps: u64, depth: usize, pn: u32, dn: u32, pv: &[usize], tag: &str) {
    let pv_str: Vec<String> = pv.iter().map(|c| (c + 1).to_string()).collect();
    let pv_fmt = if pv_str.is_empty() {
        String::new()
    } else {
        format!("  pv={}", pv_str.join(" "))
    };
    println!(
        "{:>12} {:>10} {:>10} {:>6} {:>6}{}{}",
        nodes, nps, depth, pn, dn, pv_fmt,
        if tag.is_empty() { String::new() } else { format!("  ({})", tag) }
    );
}

fn df_pn(
    board: &mut Board,
    prover: Piece,
    is_or: bool,
    pn_limit: u32,
    dn_limit: u32,
    ctx: &mut SearchCtx,
    pv: &mut Vec<usize>,
    depth: usize,
) -> (Option<SearchResult>, u32, u32) {
    ctx.nodes += 1;
    if ctx.verbose && ctx.nodes % 4097 == 0 {
        let now = Instant::now();
        if now >= ctx.next_report {
            let elapsed = now.duration_since(ctx.start);
            let secs = elapsed.as_secs_f64();
            let nps = if secs > 0.0 { (ctx.nodes as f64 / secs) as u64 } else { ctx.nodes };
            print_info(ctx.nodes, nps, depth, pn_limit, dn_limit, pv, "");
            ctx.next_report = now + Duration::from_millis(250);
        }
    }

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
        let pv_len = pv.len();
        pv.push(moves[best_idx]);
        let (result, new_pn, new_dn) = df_pn(
            &mut child_board,
            prover,
            !is_or,
            child_pn_limit,
            child_dn_limit,
            ctx,
            pv,
            depth + 1,
        );
        pv.truncate(pv_len);

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
        drop_piece(&mut board, 6, Piece::Player1);
        drop_piece(&mut board, 0, Piece::Player2);
        drop_piece(&mut board, 5, Piece::Player1);
        drop_piece(&mut board, 1, Piece::Player2);
        drop_piece(&mut board, 4, Piece::Player1);
        drop_piece(&mut board, 2, Piece::Player2);
        drop_piece(&mut board, 6, Piece::Player1);
        drop_piece(&mut board, 3, Piece::Player2);
        assert_eq!(solve(&board, Piece::Player1), SearchResult::Loss);
    }
}
