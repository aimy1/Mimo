#!/usr/bin/env bash
# ==============================================================================
#  ⚡ Mimo Universal Installer Script
#  One-line installation script for Linux distributions & CPU architectures
#  Usage: curl -fsSL https://raw.githubusercontent.com/aimy1/Mimo/main/scripts/install.sh | bash
# ==============================================================================

set -e

REPO="aimy1/Mimo"
INSTALL_DIR="/usr/local/bin"
BINARY_NAME="mimo"

# Styling Colors
BOLD="\033[1m"
GREEN="\033[32m"
CYAN="\033[36m"
RED="\033[31m"
YELLOW="\033[33m"
RESET="\033[0m"

log_info() { echo -e "${CYAN}➜${RESET} ${BOLD}$1${RESET}"; }
log_success() { echo -e "${GREEN}✔${RESET} ${BOLD}$1${RESET}"; }
log_warn() { echo -e "${YELLOW}⚠${RESET} ${BOLD}$1${RESET}"; }
log_error() { echo -e "${RED}✖${RESET} ${BOLD}$1${RESET}"; exit 1; }

echo -e "${BOLD}"
echo "   ⚡ Mimo (Minimalist Mihomo Manager) Installer"
echo "=================================================="
echo -e "${RESET}"

# 1. Detect OS & Kernel
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
if [ "$OS" != "linux" ]; then
    log_error "Mimo installer currently supports Linux operating systems only."
fi

# 2. Detect CPU Architecture
ARCH_RAW="$(uname -m)"
case "$ARCH_RAW" in
    x86_64|amd64)
        ARCH="x86_64"
        DEB_ARCH="amd64"
        RPM_ARCH="x86_64"
        ;;
    aarch64|arm64)
        ARCH="aarch64"
        DEB_ARCH="arm64"
        RPM_ARCH="aarch64"
        ;;
    armv7l|armhf)
        ARCH="armv7"
        DEB_ARCH="armhf"
        RPM_ARCH="armhfp"
        ;;
    *)
        log_error "Unsupported CPU architecture: $ARCH_RAW"
        ;;
esac

log_info "Detected system: OS=$OS, CPU Architecture=$ARCH"

# 3. Detect Package Manager & Distro Family
NEED_SUDO=""
if [ "$(id -u)" -ne 0 ]; then
    NEED_SUDO="sudo"
fi

# Fetch Latest Tag from GitHub API
LATEST_TAG=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
if [ -z "$LATEST_TAG" ]; then
    LATEST_TAG="v1.3.1"
fi
VERSION="${LATEST_TAG#v}"

log_info "Latest release version: ${LATEST_TAG}"

# Check for Package Manager preferencia (apt / dnf / pacman / direct binary fallback)
if command -v apt-get >/dev/null 2>&1; then
    DEB_URL="https://github.com/${REPO}/releases/download/${LATEST_TAG}/mimo_${VERSION}_${DEB_ARCH}.deb"
    log_info "Installing via DEB package (${DEB_URL})..."
    TMP_DEB="$(mktemp --suffix=.deb)"
    curl -fsSL "$DEB_URL" -o "$TMP_DEB"
    $NEED_SUDO dpkg -i "$TMP_DEB" || $NEED_SUDO apt-get install -f -y
    rm -f "$TMP_DEB"
elif command -v dnf >/dev/null 2>&1 || command -v rpm >/dev/null 2>&1; then
    RPM_URL="https://github.com/${REPO}/releases/download/${LATEST_TAG}/mimo-${VERSION}-1.${RPM_ARCH}.rpm"
    log_info "Installing via RPM package (${RPM_URL})..."
    $NEED_SUDO dnf install -y "$RPM_URL" || $NEED_SUDO rpm -Uvh "$RPM_URL"
elif command -v pacman >/dev/null 2>&1; then
    log_info "Arch Linux detected. Installing tar.gz binary bundle..."
    TAR_URL="https://github.com/${REPO}/releases/download/${LATEST_TAG}/mimo-${VERSION}-${ARCH}.tar.gz"
    TMP_DIR="$(mktemp -d)"
    curl -fsSL "$TAR_URL" | tar -xz -C "$TMP_DIR"
    $NEED_SUDO cp "${TMP_DIR}/mimo-${ARCH}/mimo" "${INSTALL_DIR}/${BINARY_NAME}"
    $NEED_SUDO chmod +x "${INSTALL_DIR}/${BINARY_NAME}"
    rm -rf "$TMP_DIR"
else
    # Fallback to direct tar.gz binary extraction
    TAR_URL="https://github.com/${REPO}/releases/download/${LATEST_TAG}/mimo-${VERSION}-${ARCH}.tar.gz"
    log_info "Downloading binary archive: ${TAR_URL}"
    TMP_DIR="$(mktemp -d)"
    curl -fsSL "$TAR_URL" | tar -xz -C "$TMP_DIR"
    $NEED_SUDO cp "${TMP_DIR}/mimo-${ARCH}/mimo" "${INSTALL_DIR}/${BINARY_NAME}"
    $NEED_SUDO chmod +x "${INSTALL_DIR}/${BINARY_NAME}"
    rm -rf "$TMP_DIR"
fi

# 4. Optional Capability Authorization (CAP_NET_ADMIN)
if command -v setcap >/dev/null 2>&1; then
    log_info "Granting CAP_NET_ADMIN capability for TUN mode..."
    $NEED_SUDO setcap cap_net_admin+ep "$(command -v mimo)" 2>/dev/null || true
fi

log_success "🎉 Mimo ${LATEST_TAG} installed successfully!"
echo -e "Run '${BOLD}mimo${RESET}' in your terminal to start."
