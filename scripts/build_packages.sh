#!/usr/bin/env bash
# ==============================================================================
#  ⚡ Mimo Local Packaging & Build Script
#  Run this script locally to build release archives & binary packages
# ==============================================================================

set -e

VERSION="1.3.1"
ARCH="$(uname -m)"
OUTPUT_DIR="dist-release"

echo "🔨 Building Mimo v${VERSION} release binary..."
cargo build --release

mkdir -p "${OUTPUT_DIR}"

# 1. Create tar.gz release bundle
BUNDLE_DIR="mimo-${ARCH}"
mkdir -p "${BUNDLE_DIR}"
cp target/release/mimo "${BUNDLE_DIR}/"
cp README.md LICENSE "${BUNDLE_DIR}/" 2>/dev/null || true

TARBALL="${OUTPUT_DIR}/mimo-${VERSION}-${ARCH}.tar.gz"
tar -czvf "${TARBALL}" "${BUNDLE_DIR}"
rm -rf "${BUNDLE_DIR}"

# 2. Check if NFPM is installed locally for .deb / .rpm
if command -v nfpm >/dev/null 2>&1; then
    echo "📦 Packaging DEB & RPM via NFPM..."
    DEB_ARCH="amd64"
    if [ "$ARCH" = "aarch64" ]; then DEB_ARCH="arm64"; fi

    VERSION="${VERSION}" ARCH="${DEB_ARCH}" BIN_PATH="target/release/mimo" nfpm package --config packaging/nfpm.yaml --target "${OUTPUT_DIR}/mimo_${VERSION}_${DEB_ARCH}.deb" || true
    VERSION="${VERSION}" ARCH="${ARCH}" BIN_PATH="target/release/mimo" nfpm package --config packaging/nfpm.yaml --target "${OUTPUT_DIR}/mimo-${VERSION}-1.${ARCH}.rpm" || true
fi

# 3. Generate Checksums
cd "${OUTPUT_DIR}"
sha256sum * > SHA256SUMS.txt
cd ..

echo "🎉 Build complete! Release files are located in '${OUTPUT_DIR}/':"
ls -lh "${OUTPUT_DIR}"
