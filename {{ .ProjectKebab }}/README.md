# {{ .Project }}

{{ .Scaffold.description }}

## Installation

{{- if .Computed.feature_homebrew_tap }}

```sh
brew tap {{ .Scaffold.homebrew_owner }}/{{ .Scaffold.homebrew_name | trimPrefix "homebrew-" }}
brew install {{ .ProjectKebab }}
```
{{- end }}

Download a binary from the [releases page](https://{{ .Scaffold.repository }}/releases), or build from source:

```sh
cargo install --locked --git https://{{ .Scaffold.repository }}
```

## Usage

```sh
{{ .ProjectKebab }} --help
{{- range .Scaffold.commands }}
{{ $.ProjectKebab }} {{ . }}
{{- end }}
```

Shell completions:

```sh
{{ .ProjectKebab }} completions zsh > ~/.zfunc/_{{ .ProjectKebab }}
```
{{- if .Computed.feature_config_file }}

### Configuration

{{ .ProjectKebab }} reads `~/.config/{{ .ProjectKebab }}/config.yaml` (or `$XDG_CONFIG_HOME/{{ .ProjectKebab }}/config.yaml`) when it exists. Pass `--config <path>` to use a different file. Flags take precedence over the file.

```yaml
log_level: debug
{{- if .Computed.feature_file_logging }}
log_file: /tmp/{{ .ProjectKebab }}.log
{{- end }}
```
{{- end }}

## Development

Tools come from [mise](https://mise.jdx.dev), and Rust comes from [rustup](https://rustup.rs) at the version in `rust-toolchain.toml`.

```sh
mise install       # install the dev tools
mise run check     # everything CI runs
mise tasks         # list all tasks
```

`mise run check` runs these checks, and CI runs the same task:

| Task | Tool | What it checks |
|---|---|---|
| `fmt:check` | rustfmt | Formatting, per `rustfmt.toml` |
| `lint` | clippy | Lints from `[lints]` in `Cargo.toml`, with warnings as errors |
| `test` | cargo test | Unit tests in `src/` and CLI tests in `tests/` |
| `docs` | rustdoc | Doc comments build without warnings |
| `deny` | [cargo-deny](https://github.com/EmbarkStudios/cargo-deny) | Security advisories, licenses and crate sources, per `deny.toml` |
| `tidy` | [cargo-shear](https://github.com/Boshen/cargo-shear) | Dependencies in `Cargo.toml` that the code never uses |
| `typos` | [typos](https://github.com/crate-ci/typos) | Spelling |

`mise run coverage` writes an `lcov.info` coverage report with [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov).

### Tests

- Unit tests live next to the code in `#[cfg(test)]` modules. Commands write to a `&mut impl Write`, so a test passes a `Vec<u8>` and checks what was written.
- CLI tests in `tests/cli.rs` run the built binary with [assert_cmd](https://docs.rs/assert_cmd).
- [insta](https://insta.rs) is included so snapshot tests are easy to add later. Today it backs only the small inline snapshots in the command tests. It earns its place on large outputs such as help text, JSON, or rendered files. When output changes on purpose, run `cargo insta review` to accept the new snapshot instead of editing the expected text by hand.

### Adding a command

```sh
scaffold new command
```

This generates `src/commands/<name>.rs` and registers the command in `src/cli.rs`, `src/commands.rs` and `src/main.rs`.

### Releases

{{- if .Computed.feature_gh_actions }}

Run the **Build and Publish** workflow from the Actions tab and pick a semver bump. It tags the release and runs GoReleaser, which cross-compiles with [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild).
{{- else }}

GoReleaser builds the release binaries, cross-compiling with [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild).
{{- end }} Releases build on Linux. The macOS binaries link against zig's bundled macOS stubs, which covers pure-Rust code; a dependency that links Apple frameworks needs a macOS SDK (set `SDKROOT`).

`mise run build` builds a release binary for this machine into `dist/`.
