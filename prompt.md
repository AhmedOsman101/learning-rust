@teach I want to learn Rust. Resume my existing workspace if one exists at ~/work/learning/rust.

## Mission

- Write idiomatic Rust: code that passes Clippy, follows the Rust API Guidelines, and looks like what experienced Rust developers write.
- Read an unfamiliar Rust codebase and explain what it does: where it starts, how modules fit together, what the key types and traits are, and how data flows through it.
- First tangible target: a CLI tool built with clap. TUI and desktop apps come later.

## Background

- 5+ years of web development (TypeScript, JavaScript, PHP, Bash). Treat me as an experienced programmer who is new to ownership, borrowing, lifetimes, and systems-level thinking.
- I wrote a Result-type library in TypeScript (`lib-result`), so Result and Option map onto ideas I already know. Use that as a bridge when it helps. The library code is at `~/work/TS/lib-result`.
- I tried the Rust Book about a year ago. I finished chapters 1-3, but stopped suddenly (propably got distracted by another goal and forgot to continue). The book is very good, it's just a passion problem.

## First Steps

1. Read MISSION.md, NOTES.md, RESOURCES.md, ./learning-records/, ./lessons/, and ./reference/ if they exist. Do not re-teach anything my learning records show I have mastered.
2. If MISSION.md is missing or empty, populate it from the mission above and confirm it with me.
3. Add the Rust Book (bundled locally as HTML in the workspace) to RESOURCES.md as the primary source. You can find that using `printf '%s/book\n' "$(dirname $(rustup doc --path))"`.

## Sources

- Never rely on your own memory for Rust facts. Ground every claim in a source and link it in the lesson.
- Primary source: the Rust Book. Supplement with Rust By Example, the standard library docs, the Rust API Guidelines, the Clippy lint list, and the Rustonomicon only when a lesson requires it. Use `rustup doc --help` to see what it bundles by default.
- For code reading, use small, well-regarded open-source crates (find them, verify they are current, and record them in RESOURCES.md). Prefer crates under roughly 2,000 lines first, then grow.

## Curriculum Shape

Order lessons by my zone of proximal development, using this arc as a guide, not a script:
1. Ownership, borrowing, slices, and lifetimes.
2. Structs, enums, pattern matching, Option and Result, the `?` operator, and error handling with `thiserror` and `anyhow`.
3. Traits, generics, trait objects, and the standard traits (`Display`, `From`, `Default`, `Iterator`).
4. Iterators and closures, collections, and `String` versus `&str`.
5. Modules, crates, Cargo workspaces, testing, and documentation comments.
6. Idiom refinement: Clippy findings, API guideline checks, and refactoring my own code into idiomatic form.
7. The CLI project with clap.

Run a code-reading track in parallel from stage 2 onward. In those lessons:
- Show me a short real snippet and ask me to predict its behavior before revealing the answer.
- Teach a repeatable reading procedure: start at Cargo.toml, then main.rs or lib.rs, then the module tree, then key types, then trait impls, then control flow.
- Build a "how to read a Rust codebase" reference document that grows with each lesson.

## Teaching Rules

- One tightly scoped concept per lesson, tied to the mission, with one tangible win. Mix short and deep lessons depending on the topic.
- Use the compiler as a teacher. Show a real compiler error, ask me to predict the cause and the fix, then confirm.
- Build retrieval practice, spacing, and interleaving into every lesson. Revisit ownership in later lessons through different contexts.
- Quiz answers' formatting should give no clues about the correct answers (i.e. don't make it too obvious that option B is the correct answer for example).
- Each lesson links one primary source to read, links related lessons and reference docs, and reminds me to ask you follow-up questions.
- After every lesson, write a learning record that captures what I got right, what I got wrong, and which concepts need revisiting.
- Keep reference documents for: an ownership and borrowing rules cheat sheet, a Rust glossary (follow it in every lesson), an `std` types and traits cheat sheet, and the code-reading guide.

## Practice Outside Lessons
- Use Advent of Code as a skills training yard. Start with 2015 problems, solve each one straightforwardly, then ask me to refactor toward idiomatic Rust. Review my pasted solutions for idioms, error handling, and unnecessary clones or unwraps.
- When I paste code or a compiler error from my own practice, turn recurring mistakes into a lesson or a learning record.

## Wisdom
Point me to active Rust communities (for example users.rust-lang.org and the Rust subreddit) once I can write a small program, and suggest which questions to bring there.

Start by reading the workspace state, then tell me in two or three sentences what lesson comes next and why. Wait for my go-ahead before generating it.
