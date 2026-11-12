#!/usr/bin/env bash
#
# ArchWiz installer.
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/abhaypratap08/ArchWiz/main/install.sh | bash
#
# Optional:
#   ARCHWIZ_INSTALL_DIR=/path/to/bin  (default: $HOME/.local/bin)
#
set -euo pipefail

REPO="abhaypratap08/ArchWiz"
APP="archwiz"

log() { printf '%s\n' "$*" >&2; }
die() { log "error: $*"; exit 1; }

# --- operating system and architecture ---------------------------------------
os="$(uname -s 2>/dev/null || true)"
case "$os" in
  Linux) ;;
  *) die "unsupported operating system: ${os:-unknown}. ArchWiz provides Linux x86_64 and aarch64 binaries." ;;
esac

arch_raw="$(uname -m 2>/dev/null || true)"
case "$arch_raw" in
  x86_64|amd64) arch="x86_64" ;;
  aarch64|arm64) arch="aarch64" ;;
  *) die "unsupported architecture: ${arch_raw:-unknown}. Supported: x86_64, aarch64." ;;
esac

target="${arch}-unknown-linux-gnu"
asset="${APP}-${target}.tar.xz"
checksum_file="${asset}.sha256"

# --- distribution and package manager detection ------------------------------
distro_id=""
id_like=""
if [ -r /etc/os-release ]; then
  # shellcheck disable=SC1091
  . /etc/os-release
  distro_id="${ID:-}"
  id_like="${ID_LIKE:-}"
fi

pm=""
case "$distro_id" in
  arch|manjaro|endeavouros|artix) pm="pacman" ;;
  debian|ubuntu|linuxmint|pop|elementary|zorin) pm="apt" ;;
  fedora) pm="dnf" ;;
  rhel|centos|rocky|almalinux|ol) pm="dnf" ;;
  opensuse*|suse|sles) pm="zypper" ;;
  alpine) pm="apk" ;;
esac

if [ -z "$pm" ]; then
  case "$id_like" in
    *arch*) pm="pacman" ;;
    *debian*|*ubuntu*) pm="apt" ;;
    *rhel*|*fedora*) pm="dnf" ;;
    *suse*) pm="zypper" ;;
  esac
fi

# Fall back to whatever package manager is actually installed.
if [ -z "$pm" ]; then
  if command -v pacman >/dev/null 2>&1; then pm="pacman"
  elif command -v apt-get >/dev/null 2>&1; then pm="apt"
  elif command -v dnf >/dev/null 2>&1; then pm="dnf"
  elif command -v yum >/dev/null 2>&1; then pm="yum"
  elif command -v zypper >/dev/null 2>&1; then pm="zypper"
  elif command -v apk >/dev/null 2>&1; then pm="apk"
  fi
fi

# Normalize unsupported dnf to yum when dnf is absent.
if [ "$pm" = "dnf" ] && ! command -v dnf >/dev/null 2>&1 && command -v yum >/dev/null 2>&1; then
  pm="yum"
fi

# --- dependency detection -----------------------------------------------------
missing_tools=()
command -v curl >/dev/null 2>&1 || missing_tools+=("curl")
command -v tar >/dev/null 2>&1 || missing_tools+=("tar")
command -v install >/dev/null 2>&1 || missing_tools+=("install")
command -v xz >/dev/null 2>&1 || missing_tools+=("xz")
if ! command -v sha256sum >/dev/null 2>&1 && ! command -v shasum >/dev/null 2>&1; then
  missing_tools+=("sha256sum")
fi

if [ "${#missing_tools[@]}" -gt 0 ]; then
  if [ -z "$pm" ]; then
    die "missing required tools: ${missing_tools[*]}. Unsupported distribution (${distro_id:-unknown}); install curl, tar, coreutils (providing install and sha256sum) manually, then rerun."
  fi

  packages=()
  for tool in "${missing_tools[@]}"; do
    case "$tool" in
      curl) packages+=("curl") ;;
      tar) packages+=("tar") ;;
      xz)
        if [ "$pm" = "apt" ]; then packages+=("xz-utils"); else packages+=("xz"); fi
        ;;
      install|sha256sum) packages+=("coreutils") ;;
    esac
  done

  # De-duplicate package names.
  unique_packages=()
  for pkg in "${packages[@]}"; do
    found=0
    for existing in "${unique_packages[@]:-}"; do
      if [ "$existing" = "$pkg" ]; then found=1; break; fi
    done
    if [ "$found" -eq 0 ]; then unique_packages+=("$pkg"); fi
  done
  packages=("${unique_packages[@]}")

  log "Missing required tools: ${missing_tools[*]}"
  log "The following packages need installation: ${packages[*]}"

  if [ "$(id -u)" -ne 0 ]; then
    if ! command -v sudo >/dev/null 2>&1; then
      die "sudo is required to install packages but was not found. Install ${packages[*]} manually, then rerun."
    fi
    sudo_cmd="sudo"
  else
    sudo_cmd=""
  fi

  if [ -r /dev/tty ]; then
    printf 'Install missing packages now? [y/N] ' >/dev/tty
    read -r reply </dev/tty || reply=""
  else
    reply="n"
  fi
  case "$reply" in
    y|Y|yes|YES) ;;
    *) die "package installation declined. Install ${packages[*]} manually, then rerun." ;;
  esac

  log "Installing missing packages with ${pm}..."
  case "$pm" in
    pacman)
      # shellcheck disable=SC2086
      $sudo_cmd pacman -Sy --needed --noconfirm "${packages[@]}" || die "failed to install packages: ${packages[*]}"
      ;;
    apt)
      # shellcheck disable=SC2086
      $sudo_cmd apt-get update || die "failed to update package lists"
      # shellcheck disable=SC2086
      $sudo_cmd apt-get install -y "${packages[@]}" || die "failed to install packages: ${packages[*]}"
      ;;
    dnf)
      # shellcheck disable=SC2086
      $sudo_cmd dnf install -y "${packages[@]}" || die "failed to install packages: ${packages[*]}"
      ;;
    yum)
      # shellcheck disable=SC2086
      $sudo_cmd yum install -y "${packages[@]}" || die "failed to install packages: ${packages[*]}"
      ;;
    zypper)
      # shellcheck disable=SC2086
      $sudo_cmd zypper --non-interactive install "${packages[@]}" || die "failed to install packages: ${packages[*]}"
      ;;
    apk)
      # shellcheck disable=SC2086
      $sudo_cmd apk add --no-cache "${packages[@]}" || die "failed to install packages: ${packages[*]}"
      ;;
    *) die "unsupported package manager: ${pm}" ;;
  esac

  # Re-check.
  command -v curl >/dev/null 2>&1 || die "curl is still missing after package installation"
  command -v tar >/dev/null 2>&1 || die "tar is still missing after package installation"
  command -v xz >/dev/null 2>&1 || die "xz is still missing after package installation"
  command -v install >/dev/null 2>&1 || die "install is still missing after package installation"
  if ! command -v sha256sum >/dev/null 2>&1 && ! command -v shasum >/dev/null 2>&1; then
    die "sha256sum is still missing after package installation"
  fi
fi

install_dir="${ARCHWIZ_INSTALL_DIR:-$HOME/.local/bin}"
[ -n "$install_dir" ] || die "ARCHWIZ_INSTALL_DIR must not be empty"

# --- temp workspace -----------------------------------------------------------
tmpdir="$(mktemp -d 2>/dev/null)" || die "could not create temporary directory"
cleanup() { rm -rf "$tmpdir"; }
trap cleanup EXIT HUP INT TERM

# --- resolve latest stable release -------------------------------------------
log "Resolving latest ArchWiz release..."
latest_url="$(curl -fsSL --retry 3 --retry-delay 2 -o /dev/null -w '%{url_effective}' \
  "https://github.com/${REPO}/releases/latest")" || die "could not resolve latest release"
tag="${latest_url##*/}"
case "$tag" in
  v[0-9]*) ;;
  *) die "could not determine a stable release tag (got '${tag}')" ;;
esac
log "Latest release: ${tag}"

base_url="https://github.com/${REPO}/releases/download/${tag}"

# --- download asset + checksum ------------------------------------------------
log "Downloading ${asset}..."
curl -fsSL --retry 3 --retry-delay 2 \
  -o "${tmpdir}/${asset}" "${base_url}/${asset}" \
  || die "failed to download ${asset} (asset may be missing for ${target})"

log "Downloading ${checksum_file}..."
curl -fsSL --retry 3 --retry-delay 2 \
  -o "${tmpdir}/${checksum_file}" "${base_url}/${checksum_file}" \
  || die "failed to download ${checksum_file}"

# --- verify checksum ----------------------------------------------------------
log "Verifying SHA-256 checksum..."
if command -v sha256sum >/dev/null 2>&1; then
  (cd "$tmpdir" && sha256sum -c "$checksum_file" >/dev/null 2>&1) || die "checksum verification failed for ${asset}"
else
  (cd "$tmpdir" && shasum -a 256 -c "$checksum_file" >/dev/null 2>&1) || die "checksum verification failed for ${asset}"
fi

# --- extract ------------------------------------------------------------------
log "Extracting..."
mkdir -p "${tmpdir}/extract"
tar -xJf "${tmpdir}/${asset}" -C "${tmpdir}/extract" \
  || die "failed to extract ${asset}"

binary=""
for candidate in "${tmpdir}"/extract/*/archwiz; do
  if [ -f "$candidate" ]; then
    binary="$candidate"
    break
  fi
done
[ -n "$binary" ] && [ -f "$binary" ] || die "archwiz binary not found in archive"

# --- install ------------------------------------------------------------------
mkdir -p "$install_dir" || die "could not create installation directory: ${install_dir}"
if [ ! -w "$install_dir" ]; then
  die "installation directory is not writable: ${install_dir}"
fi

target_binary="${install_dir}/${APP}"
log "Installing to ${target_binary}..."
install -m 0755 "$binary" "$target_binary" || die "failed to install ${APP}"

# --- PATH hint ----------------------------------------------------------------
case ":${PATH}:" in
  *":${install_dir}:"*) ;;
  *) log "Note: ${install_dir} is not on your PATH. Add it to your shell configuration manually." ;;
esac

log "ArchWiz ${tag} installed successfully."
log "Run '${APP} --help' to get started."