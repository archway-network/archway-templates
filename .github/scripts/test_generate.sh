#!/bin/bash
set -u -o errexit -o nounset -o pipefail
command -v shellcheck > /dev/null && shellcheck "$0"

TMP_DIR=$(mktemp -d "${TMPDIR:-/tmp}/XXXXXXXXX")

echo "Navigating to $TMP_DIR"
cd "$TMP_DIR"

function test-template() {
  TEMPLATE="${1}"
  PROJECT_NAME="${TEMPLATE//\//-}"
  echo "#######################################"
  echo "# Testing template $TEMPLATE"
  echo "#######################################"
  (
    GIT_BRANCH=$(git -C "$GITHUB_SERVER_URL/$GITHUB_REPOSITORY" branch --show-current)

    echo "Generating project from local repository (branch $GIT_BRANCH) ..."
    cargo generate --git "$GITHUB_SERVER_URL/$GITHUB_REPOSITORY" --name "$PROJECT_NAME" --branch "$GIT_BRANCH" "$TEMPLATE"

    (
      cd "$PROJECT_NAME"
      echo "This is what was generated"
      ls -lA

      # Check formatting
      echo "Checking formatting ..."
      cargo fmt -- --check

      echo "Running clippy ..."
      cargo clippy -- -D warnings

      # Debug builds first to fail fast
      echo "Running unit tests ..."
      cargo unit-test
      echo "Creating schema ..."
      cargo schema

      echo "Building wasm ..."
      cargo wasm
    )
  )
}

find "$GITHUB_SERVER_URL/$GITHUB_REPOSITORY" -name Cargo.toml -exec dirname {} \; | while read -r TEMPLATE; do
  test-template "${TEMPLATE//$GITHUB_SERVER_URL/$GITHUB_REPOSITORY\//}"
  echo
done
