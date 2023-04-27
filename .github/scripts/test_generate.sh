#!/bin/bash
set -u -o errexit -o nounset -o pipefail
command -v shellcheck > /dev/null && shellcheck "$0"

REPO_ROOT="$(realpath "$(dirname "$0")/../..")"
TMP_DIR=$(mktemp -d "${TMPDIR:-/tmp}/XXXXXXXXX")

echo "Navigating to $TMP_DIR"
cd "$TMP_DIR"

function test-template() {
  TEMPLATE="${1}"
  PROJECT_NAME="${TEMPLATE//\//-}"
  echo
  echo "#######################################"
  echo "# Testing template $TEMPLATE"
  echo "#######################################"
  echo
  (
    GIT_BRANCH=$(git -C "$REPO_ROOT" branch --show-current)

    echo "Generating project from local repository (branch $GIT_BRANCH) ..."
    cargo generate --path "$REPO_ROOT" --name "$PROJECT_NAME" -d minimal=false "$TEMPLATE"

    (
      cd "$PROJECT_NAME"
      echo "This is what was generated"
      ls -lA

      # Check formatting
      echo "Checking formatting ..."
      if [ "$TEMPLATE" = "base-workspace" ]; then
        echo "Skipping formatting check because the base-workspace template doesn't have any packages"
      else
        cargo fmt -- --check
      fi

      echo "Running clippy ..."
      cargo clippy -- -D warnings -A clippy::derive_partial_eq_without_eq

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

if [[ -n "${1:-}" ]]; then
  test-template "${1}"
else
  find "$REPO_ROOT" -name Cargo.toml -exec dirname {} \; | while read -r TEMPLATE; do
    test-template "${TEMPLATE//$REPO_ROOT\//}"
    echo
  done
fi
