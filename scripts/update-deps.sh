#!/usr/bin/env bash
# Bumps the template to current dependency releases:
#   - crate requirements in Cargo.toml move to the latest major (or 0.x minor)
#   - rust-toolchain.toml and `rust-version` move to the latest stable Rust
# Requirements name only the major ("1", "0.3"), so minor and patch releases
# need no change here; the generated project's Cargo.lock picks them up.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEMPLATE_DIR="$(cd "$SCRIPT_DIR/.." && pwd)/{{ .ProjectKebab }}"

python3 - "$TEMPLATE_DIR" <<'PY'
import json
import re
import sys
import urllib.request
from pathlib import Path

template = Path(sys.argv[1])
cargo_toml = template / "Cargo.toml"
toolchain = template / "rust-toolchain.toml"
user_agent = {"User-Agent": "scaffold-rust-cli-update-deps"}


def fetch(url):
    request = urllib.request.Request(url, headers=user_agent)
    with urllib.request.urlopen(request, timeout=30) as response:
        return response.read().decode()


def requirement(version):
    major, minor, _ = version.split(".", 2)
    return major if major != "0" else f"0.{minor}"


text = cargo_toml.read_text()
section = None
lines = []
changed = 0
dep_line = re.compile(r'^([A-Za-z0-9_-]+) = (?:"([^"]+)"|\{ version = "([^"]+)")')
for line in text.splitlines(keepends=True):
    header = re.match(r"^\[([^\]]+)\]", line)
    if header:
        section = header.group(1)
    match = dep_line.match(line)
    if match and section in ("dependencies", "dev-dependencies"):
        name = match.group(1)
        current = match.group(2) or match.group(3)
        crate = json.loads(fetch(f"https://crates.io/api/v1/crates/{name}"))["crate"]
        wanted = requirement(crate["max_stable_version"])
        if wanted != current:
            print(f"  UPDATE {name} {current} -> {wanted}")
            line = line.replace(f'"{current}"', f'"{wanted}"', 1)
            changed += 1
        else:
            print(f"  OK     {name} {current}")
    lines.append(line)
text = "".join(lines)

channel = fetch("https://static.rust-lang.org/dist/channel-rust-stable.toml")
stable = re.search(r'\[pkg\.rust\]\nversion = "(\d+)\.(\d+)\.', channel)
rust = f"{stable.group(1)}.{stable.group(2)}"
text, rust_changes = re.subn(r'^rust-version = "[^"]+"', f'rust-version = "{rust}"', text, flags=re.M)
cargo_toml.write_text(text)

toolchain_text = toolchain.read_text()
new_toolchain = re.sub(r'^channel = "[^"]+"', f'channel = "{rust}"', toolchain_text, flags=re.M)
if new_toolchain != toolchain_text:
    print(f"  UPDATE rust toolchain -> {rust}")
    toolchain.write_text(new_toolchain)
    changed += 1
else:
    print(f"  OK     rust toolchain {rust}")

print(f"\n{changed} update(s).")
PY
