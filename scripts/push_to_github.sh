#!/usr/bin/env bash
# push_to_github.sh — verify tests and push LitTCG repo to GitHub
# Usage: ./scripts/push_to_github.sh
set -euo pipefail

REPO_DIR="/home/joshua/LitTCG/LitTCG"

echo "=== Verifying LitTCG crate ==="
cd "$REPO_DIR"
cargo test

echo ""
echo "=== Checking git status ==="
git status --short

echo ""
echo "=== Removing any tracked dist/ build artifacts ==="
git rm -r --cached dist/ 2>/dev/null || true

echo "=== Adding all changes ==="
git add -A

echo ""
echo "=== Committing ==="
git commit -m "Rebrand project to LitTCG (Literary Trading Card Game)

- Rename crate identifiers: communication-class -> lit-tcg, communication_class -> lit_tcg
- Rename desktop binary: communication-class-desktop -> lit-tcg-desktop
- Rename Android package: com.communicationclass.game -> com.littcg.game
- Update index.html, AndroidManifest.xml, CI workflow, and integration tests
- Rebrand workspace docs (README, GDD, AGENTS, CLAUDE, LOCAL_AGENT_SETUP, CONTRIBUTING, itch_page)
- Expand .gitignore for Rust/WASM/Android artifacts and logs
- Add GitHub-focused README inside the repo
- Add GitHub remote origin for LitTCG" || true

echo ""
echo "=== Pushing to origin ==="
git push -u origin master

echo ""
echo "=== Done ==="
