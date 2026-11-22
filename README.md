# Rust clap Project Scaffold

A [scaffold](https://github.com/hay-kot/scaffold) template for a Rust CLI built on [clap](https://docs.rs/clap). It is the Rust counterpart of [scaffold-go-cli](https://github.com/hay-kot/scaffold-go-cli) and asks the same questions.

```sh
scaffold new https://github.com/hay-kot/scaffold-rust-cli
```

The post-scaffold hook runs `cargo generate-lockfile` and `cargo fmt`, so it needs Rust and network access. Allow it when prompted, or pass `--run-hooks=always`.

## Features

- clap derive CLI with one module per command, plus a `completions` command
- `tracing` logs to stderr, with stdout reserved for command output
- Optional features: file logging, YAML config file (XDG paths on every platform), JSON output
- Version, commit and date injection at build time
- Strict lints: clippy `pedantic`, `nursery` and `cargo` groups plus selected `restriction` lints, all errors in CI. Exceptions need `#[expect(lint, reason = "...")]` at the use site.
- `mise run check` runs rustfmt, clippy, tests, rustdoc, cargo-deny, cargo-shear and typos
- Unit tests next to the code, CLI tests with assert_cmd, and insta for snapshot tests
- GoReleaser with cargo-zigbuild: static Linux musl, macOS and Windows binaries from one Linux runner
  - Multi-architecture Docker images on distroless
  - Homebrew tap publishing
- `AGENTS.md` with the conventions and checks for coding agents
- A `command` sub-scaffold: `scaffold new command` adds a command and registers it

### GitHub features

- Issue and PR templates
- PR checks through `mise run check`
- Manual release workflow with a semver bump
- Renovate configuration for cargo, mise and GitHub Actions

## Developing the template

```sh
mise run test:snapshot          # compare the rendered test preset to snapshots/test.snapshot
mise run test:snapshot:update   # accept the current rendering
mise run test:run               # render each preset, run its full checks and its first command
```

The `test` preset uses the default features. The `full` preset turns on every CLI feature. Both must pass `mise run check` after rendering.

A weekly workflow bumps crate majors, Rust stable and GitHub Action majors (`mise run update:deps`, `mise run update:actions`), then opens a PR.
