# AGENTS.md

## Project Overview

A Connect Four solver written in Rust (edition 2024). The board is 7 columns × 6 rows. The project is a single binary crate with no external dependencies.

## Module Layout

| Module | File | Purpose |
|--------|------|---------|
| `board` | `src/board.rs` | Core types (`Piece`, `Cell`, `Board`), board accessors, `Display` |
| `move_gen` | `src/move_gen.rs` | `is_valid_move()`, `valid_moves()` |
| `move_exec` | `src/move_exec.rs` | `drop_piece()` |
| `eval` | `src/eval.rs` | `check_win()`, `is_full()`, `is_terminal()` |
| `game_test` | `src/game_test.rs` | Integration tests demonstrating game rules |

All game-logic functions are standalone (not `Board` methods) and operate on `&Board` or `&mut Board`.

## Setup Commands

- Build: `cargo build`
- Run: `cargo run`
- Test: `cargo test`
- Test with output: `cargo test -- --nocapture`

## Code Style

- Standard Rust edition 2024 idioms
- Use `camelCase` for enum variants (`Player1`, `Player2`)
- No external dependencies
- Tests live in `#[cfg(test)] mod tests` blocks inside each module file
- Public API functions use `pub`, test helpers use `pub(crate)` or stay private
- Board fields are `pub(crate)` for sibling module access
- Avoid comments unless the code is non-obvious
- Use 4-space indentation

## Testing

- Run full suite: `cargo test`
- Run a specific test: `cargo test <test_name>`
- Unit tests are co-located with their module (e.g. `src/eval.rs` has `mod tests`)
- Integration tests are in `src/game_test.rs`
- All game rule invariants are tested (column validity, capacity, alternation, win detection, draw)

## PR Instructions

- Run `cargo test` and `cargo build` before committing
- Keep module boundaries clean: move_gen, move_exec, eval are separate concerns
- Add or update tests for any new functionality
