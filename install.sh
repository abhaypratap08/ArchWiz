#!/bin/sh
set -eu

REPOSITORY="https://github.com/abhaypratap08/ArchWiz"
BRANCH="main"
BINARY="archwiz"

INSTALL_ROOT="${CARGO_INSTALL_ROOT:-${CARGO_HOME:-$HOME/.cargo}}"
BIN_DIR="$INSTALL_ROOT/bin"

printf '\n'
printf '  ARCHWIZ INSTALLER\n'
printf '  =================\n\n'

if ! command -v cargo >/dev/null 2>&1; then
  printf '[ERROR] Rust and Cargo are required.\n'
  printf 'Install Rust: https://rustup.rs\n'
  exit 1
fi

if ! command -v rustc >/dev/null 2>&1; then
  printf '[ERROR] rustc was not found. Install the Rust toolchain.\n'
  exit 1
fi

printf '[INFO] Repository: %s\n' "$REPOSITORY"
printf '[INFO] Installing %s from branch %s...\n\n' "$BINARY" "$BRANCH"

cargo install \
  --root "$INSTALL_ROOT" \
  --git "$REPOSITORY" \
  --branch "$BRANCH" \
  --locked \
  --force \
  --bin "$BINARY"

if [ ! -x "$BIN_DIR/$BINARY" ]; then
  printf '\n[ERROR] Installation completed but binary was not found.\n'
  exit 1
fi

printf '\n[SUCCESS] ARCHWIZ installed!\n\n'
printf 'Version:\n'
"$BIN_DIR/$BINARY" --version

case ":${PATH:-}:" in
*":$BIN_DIR:"*)
  printf '\n[READY] Run: archwiz\n'
  ;;
*)
  printf '\n[INFO] Add ARCHWIZ to your Fish PATH:\n\n'
  printf '  fish_add_path "%s"\n' "$BIN_DIR"
  ;;
esac

printf '\n'
