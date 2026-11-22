#!/usr/bin/env bash
# Generates Cargo.lock so `--locked` builds work from the first commit, then
# formats the code so command names of any length come out rustfmt-clean.
set -euo pipefail
cd "{{ .ProjectKebab }}"
cargo generate-lockfile
cargo fmt --all
