#!/bin/sh
set -eu

REPOSITORY="https://github.com/abhaypratap08/ArchWiz"
BRANCH="main"
BINARY="archwiz"

INSTALL_ROOT="${CARGO_INSTALL_ROOT:-${CARGO_HOME:-$HOME/.cargo}}"
BIN_DIR="$INSTALL_ROOT/bin"

printf '\n'
printf '  ◈ ARCHWIZ INSTALLER\n'
printf '  ===================\n\n'

if ! command -v cargo >/dev/null 2>&1; then
  printf '[ERROR] Cargo is required.\n'
  printf 'Install Rust from https://rustup.rs\n'
  exit 1
fi

if ! command -v rustc >/dev/null 2>&1; then
  printf '[ERROR] Rust compiler was not found.\n'
  exit 1
fi

printf '[INFO] Installing ARCHWIZ...\n'

cargo install \
  --root "$INSTALL_ROOT" \
  --git "$REPOSITORY" \
  --branch "$BRANCH" \
  --locked \
  --force \
  --bin "$BINARY"

if [ ! -x "$BIN_DIR/$BINARY" ]; then
  printf '[ERROR] ARCHWIZ binary was not found after installation.\n'
  exit 1
fi

printf '\n[SUCCESS] ARCHWIZ installed.\n'

# Automatically configure Fish when available.
if command -v fish >/dev/null 2>&1; then
  if fish -c 'fish_add_path "$argv[1]"' "$BIN_DIR"; then
    printf '[SUCCESS] Fish PATH configured automatically.\n'
  else
    printf '[WARNING] Could not configure Fish PATH automatically.\n'
  fi
else
  printf '[INFO] Fish was not detected.\n'
  printf '[INFO] Add this directory to your shell PATH: %s\n' "$BIN_DIR"
fi

printf '\n[VERSION] '
"$BIN_DIR/$BINARY" --version

printf '\n[READY] Run: archwiz\n\n'
