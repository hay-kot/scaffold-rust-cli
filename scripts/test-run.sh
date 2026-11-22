#!/usr/bin/env bash
# Renders every preset with hooks enabled, then runs the generated project's
# full `mise run check` and its first command. PRESETS overrides the list.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
OUT_DIR="$(mktemp -d)"
trap 'rm -rf "$OUT_DIR"' EXIT

presets="${PRESETS:-test config logging full}"

for preset in $presets; do
	echo "==> rendering preset '$preset'"
	scaffold new --output-dir="$OUT_DIR/$preset" --preset="$preset" --no-prompt --run-hooks=always "$REPO_ROOT"

	project_dir="$(find "$OUT_DIR/$preset" -mindepth 1 -maxdepth 1 -type d | head -n 1)"
	cd "$project_dir"

	# typos and cargo-shear read .gitignore only inside a git repository.
	git init -q

	mise trust --quiet .
	mise install --quiet

	echo "==> checking preset '$preset'"
	mise run check

	output="$(cargo run --quiet --locked -- --no-color hello 2>/dev/null)"
	if [ "$output" != "Hello World!" ]; then
		echo "error: preset '$preset' printed '$output', want 'Hello World!'"
		exit 1
	fi
	echo "==> preset '$preset' OK"
done
