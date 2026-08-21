# AI-Paired TDD Lab

This repository is a small Rust exercise for practicing strict
test-driven development while alternating between a human and an LLM. The
current feature calculates summary statistics for a slice of `f32` values.

## Prerequisites

Install [Rustup](https://rustup.rs/). The repository's
`rust-toolchain.toml` selects the required nightly toolchain automatically.

## Run the tests

```sh
cargo test
```

## Continue the exercise

1. Read [CONTRIBUTING.md](CONTRIBUTING.md) for the complete pairing rules.
2. Check `git status` before doing anything. An uncommitted test may be an
   intentional red step; do not discard or bypass it.
3. Open [TEST_LIST.md](TEST_LIST.md) and choose the first unchecked behavior.
4. State which item is being worked on and whether it is the human's or the
   LLM's cycle.
5. Follow one Red -> Green -> Refactor cycle:
   - add exactly one failing test and confirm it fails for the intended reason;
   - write only enough production code to make it pass;
   - refactor only when the green code asks for it.
6. Check the item in `TEST_LIST.md` only after the behavior is green and
   committed. Commit a refactor separately.
7. Alternate ownership of the next cycle. On a human cycle, the human writes
   the code and runs the tests while the LLM reviews.

To resume after a break, inspect `git status`, `git log --oneline`, and the
test list. AI-authored cycle commits contain a `Co-authored-by: Copilot`
trailer; human-authored cycle commits do not.

## Project layout

- `src/lib.rs` defines the public summary result.
- `src/scalar.rs` contains the scalar implementation under test.
- `tests/summarize.rs` contains integration tests.
- `TEST_LIST.md` tracks completed and upcoming behaviors.
- `CONTRIBUTING.md` defines the TDD and pairing discipline.
