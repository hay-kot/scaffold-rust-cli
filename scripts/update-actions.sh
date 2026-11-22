#!/usr/bin/env bash
# Pins every GitHub Action in this repo's workflows and in the template's
# workflows to the commit SHA of its latest release, with the tag as a
# comment:
#
#   uses: actions/checkout@<sha> # v7.0.1
#
# Generated projects then start pinned, and their Renovate config keeps the
# pins current. This script exists because Renovate cannot parse the
# template's workflow files, which contain scaffold [[ ]] blocks.
set -euo pipefail

if ! command -v gh &>/dev/null; then
	echo "error: gh CLI is required"
	exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

python3 - "$REPO_ROOT/.github/workflows" "$REPO_ROOT/{{ .ProjectKebab }}/.github/workflows" <<'PY'
import json
import re
import subprocess
import sys
from pathlib import Path

USES = re.compile(r"(uses:\s+)([A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+)@([^\s#]+)(\s+#\s*\S+)?")


def gh(path):
    result = subprocess.run(["gh", "api", path], check=True, capture_output=True, text=True)
    return json.loads(result.stdout)


def latest(repo):
    """Returns (tag, commit sha) for the repo's latest release, or its newest tag."""
    try:
        tag = gh(f"repos/{repo}/releases/latest")["tag_name"]
    except subprocess.CalledProcessError:
        tag = gh(f"repos/{repo}/tags?per_page=1")[0]["name"]
    ref = gh(f"repos/{repo}/git/ref/tags/{tag}")["object"]
    # Annotated tags point at a tag object; follow it to the commit.
    while ref["type"] == "tag":
        ref = gh(f"repos/{repo}/git/tags/{ref['sha']}")["object"]
    return tag, ref["sha"]


files = [f for d in sys.argv[1:] for f in sorted(Path(d).glob("*.yml"))]
repos = sorted({m.group(2) for f in files for m in USES.finditer(f.read_text())})

pins = {}
for repo in repos:
    pins[repo] = latest(repo)
    print(f"  {repo}@{pins[repo][1]} # {pins[repo][0]}")

for f in files:
    text = f.read_text()
    new = USES.sub(lambda m: f"{m.group(1)}{m.group(2)}@{pins[m.group(2)][1]} # {pins[m.group(2)][0]}", text)
    if new != text:
        f.write_text(new)
        print(f"updated {f}")
PY

echo ""
echo "Run 'mise run test:snapshot:update' to regenerate the snapshot."
