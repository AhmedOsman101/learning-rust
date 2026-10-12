# Notes

- Experienced programmer: 5+ years TypeScript/JavaScript/PHP/Bash. Author of `lib-result` (TypeScript Result-type library) at `~/work/TS/lib-result` — use as bridge for Option/Result.
- Rust history: finished Book chapters 1–3 ~a year ago, stopped; restarted. Book is liked, motivation is the blocker — keep lessons short with one tangible win.
- Existing code in workspace: `guessing_game/` (Book ch.2 program + `quit` handling, `match` on parse) and `playground/` (ownership move example, Book ch.4.1).
- Teaching prefs: one concept per lesson; compiler-as-teacher (predict error cause/fix); retrieval practice + spacing + interleaving; quiz answers give no positional/formatting clues; each lesson links one primary source + related lessons/refs; learning record after every lesson.
- Reference docs to maintain: ownership & borrowing cheat sheet, Rust glossary (binding in every lesson), `std` types & traits cheat sheet, code-reading guide.
- Practice yard: Advent of Code 2015, straightforward solve then idiomatic refactor; review pasted solutions for idioms, error handling, clones/unwraps.
- Toolchain: rustc/cargo 1.99.0, edition 2024. Local docs under `/home/othman/.local/share/rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/doc/rust/html/`.
- Reset (2026-10-13): user asked to restart from Book Chapter 1, reporting chapters 1–3 forgotten. Early lessons move fast and lead with retrieval; anything recalled correctly is promoted, not re-taught. See LR-0002.
- Lesson authoring: button quizzes (`quiz-buttons.js`) need exactly one `data-correct` per quiz or every answer scores wrong — verify with a grep before shipping (bitten once in Lesson 0001, quiz 1).
