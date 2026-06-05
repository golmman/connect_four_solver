#![allow(dead_code)]

mod board;
mod eval;
mod game_test;
mod move_exec;
mod move_gen;
mod search;

fn main() {
    let board = board::Board::new();
    let result = search::solve_verbose(&board, board::Piece::Player1);
    println!("{}", board);
    println!("Result: {:?}", result);
}
