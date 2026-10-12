# Lesson 0001 completed (self-reported)

Othman reported finishing Lesson 0001 ("Cargo runs the shop"). Hard evidence so far covers one item: he correctly recalled `cargo new`'s output (manifest, entry point, ignore rules) from memory — demonstrated when he picked the right answer in the recall quiz and flagged the scoring bug.

## Evidence

Self-reported completion plus the bug report, which proves the correct recall (he knew the right answer and noticed the component disagreed).

## Implications

Quiz scores (§4) and the hands-on outputs (`fmt`/`clippy` on `guessing_game/`) are still unknown — requested but not yet pasted. Do not promote §2–§4 material to "known" until that evidence arrives. Lesson 0002 (Chapter 2 via the guessing game) stays queued behind it.

## Outcomes (evidence arrived 2026-10-13)

- §4 quiz: **4/4**. Manifest/lockfile distinction, `cargo check` vs `build`, `target/` regenerability, and the macro `!` are promoted to **known** (glossary: Lockfile, Macro).
- `cargo clippy`: clean on both projects. Clippy-clean baseline established for real.
- `cargo fmt`: reformatted `guessing_game/` 2-space → 4-space, exposing that rustfmt ignores `~/.config/rustfmt.toml`. Root cause identified and answered separately (auto-discovery only walks up from each source file looking for `rustfmt.toml`; fix: workspace-root `rustfmt.toml` or `--config-path`).
