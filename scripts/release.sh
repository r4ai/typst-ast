#!/usr/bin/env bash
set -euo pipefail

if [ $# -ne 1 ]; then
  echo "Usage: $0 <version>" >&2
  echo "  e.g. $0 1.2.3" >&2
  exit 1
fi

VERSION="$1"
TAG="v${VERSION}"

if ! echo "$VERSION" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+'; then
  echo "Error: version must be in semver format (e.g. 1.2.3)" >&2
  exit 1
fi

# Update Cargo.toml
sed -i "s/^version = \".*\"/version = \"${VERSION}\"/" Cargo.toml
echo "Updated Cargo.toml version to ${VERSION}"

# Update Cargo.lock
cargo update -p typst-ast
echo "Updated Cargo.lock"

# Commit, tag, push
git add Cargo.toml Cargo.lock
git commit -m "chore: release ${TAG}"
git tag "${TAG}"

echo ""
echo "Ready to release ${TAG}. Run the following commands to push:"
echo ""
echo "  git push origin HEAD"
echo "  git push origin ${TAG}"
echo ""
