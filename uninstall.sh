#!/usr/bin/env bash
#
# ArchWiz uninstaller.
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/abhaypratap08/ArchWiz/main/uninstall.sh | bash
#
# Optional:
#   ARCHWIZ_INSTALL_DIR=/path/to/bin  (default: $HOME/.local/bin)
#
set -euo pipefail

APP="archwiz"

log() { printf '%s\n' "$*" >&2; }
die() { log "error: $*"; exit 1; }

install_dir="${ARCHWIZ_INSTALL_DIR:-$HOME/.local/bin}"
[ -n "$install_dir" ] || die "ARCHWIZ_INSTALL_DIR must not be empty"

target="${install_dir}/${APP}"

if [ ! -e "$target" ] && [ ! -L "$target" ]; then
  log "ArchWiz is not installed at ${target}."
  exit 0
fi

if [ -w "$install_dir" ] || [ -w "$target" ]; then
  rm -f "$target" || die "failed to remove ${target}"
else
  if ! command -v sudo >/dev/null 2>&1; then
    die "cannot remove ${target}: permission denied and sudo is unavailable"
  fi
  sudo rm -f "$target" || die "failed to remove ${target}"
fi

log "ArchWiz removed from ${target}."
log "Shell configuration and PATH were not modified."