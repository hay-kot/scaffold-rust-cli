# AGENTS.md

Guidance for coding agents working in this repository.

## Before you finish

Run `mise run check`. It is what CI runs, and it must pass: formatting, clippy with warnings as errors, tests, docs, cargo-deny, cargo-shear and typos.

## Lints

The lint set in `Cargo.toml` is strict on purpose. Fix the code rather than silencing the lint. When a lint is wrong for one spot, silence it there with a reason:

```rust
#[expect(clippy::cast_possible_truncation, reason = "value is checked to fit in u8 above")]
```

Never use `#[allow(...)]` (the `allow_attributes` lint rejects it), and never add a crate-level or `Cargo.toml` allow without asking.

## Conventions

- `main.rs` owns process concerns: parsing, logging setup, error reporting and the exit code. Commands never call `std::process::exit`.
- Commands write output to the `out: &mut impl Write` they are given, never with `println!`. Logs go to stderr through `tracing`.
- Library-style modules return `anyhow::Result` with context on every fallible call (`.with_context(|| ...)`). Use `thiserror` if a caller needs to match on error variants.
- No `unwrap`, `expect` or `panic!` outside tests.
- Add a command with `scaffold new command`, not by hand, so all registration points stay in sync.

## Tests

- Unit tests go in a `#[cfg(test)] mod tests` block in the same file. Integration tests that run the binary go in `tests/`.
- Every new command gets a unit test that captures its output and a CLI test in `tests/cli.rs`.
- Use `insta::assert_snapshot!` for multi-line or structured output, and `cargo insta review` to accept intended changes.
