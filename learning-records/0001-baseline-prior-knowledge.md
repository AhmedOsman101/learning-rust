# Baseline: where Othman stands at workspace creation

Othman is an experienced TypeScript programmer (author of a Result-type library) who finished Rust Book chapters 1–3 a year ago and has now redone chapters 2 (guessing_game with `match`, `parse`, loop, shadowing) and 4.1 (ownership moves in playground). Variables, mutability, functions, control flow, and move semantics are therefore treated as prior knowledge; borrowing, references, and slices are not yet evidenced and are the current frontier.

## Evidence

Stated background plus workspace code: `guessing_game/src/main.rs` (uses `rand`, `Ordering`, `io::stdin().read_line(&mut res)`, `match` on `parse`) and `playground/src/main.rs` (move semantics `takes_and_gives_back`).

## Implications

Start the lesson arc at Book chapter 4.2 (references and borrowing), not chapter 1. Never re-teach moves from scratch; use them as the contrast that motivates borrowing. Bridge `Result`/`Option` from `lib-result` when reaching stage 2.
