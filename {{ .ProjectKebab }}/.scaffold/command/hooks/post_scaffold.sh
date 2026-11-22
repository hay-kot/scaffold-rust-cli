#!/usr/bin/env bash
# rustfmt sorts the injected `mod` declaration into place.
set -euo pipefail
cargo fmt --all
