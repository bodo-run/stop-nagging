#!/usr/bin/env bash
set -euo pipefail

# This script installs the latest stop-nagging release from GitHub.
# Usage:
#   curl -s https://raw.githubusercontent.com/bodo-run/stop-nagging/main/scripts/install_stop_nagging.sh | bash

REPO_OWNER="bodo-run"
REPO_NAME="stop-nagging"
INSTALL_DIR="$HOME/.local/bin"

# Detect OS and ARCH to choose the correct artifact
OS=$(uname -s)
ARCH=$(uname -m)

# Map OS/ARCH to a known target triple from your GitHub Actions build matrix
# Extend this if you build for more combos.

case "${OS}_${ARCH}" in
Linux_x86_64)
    TARGET="x86_64-unknown-linux-gnu"
    ;;
Darwin_x86_64)
    TARGET="x86_64-apple-darwin"
    ;;
Darwin_arm64)
    # macOS Apple Silicon
    TARGET="aarch64-apple-darwin"
    ;;
*)
    echo "Unsupported OS/ARCH combo: ${OS} ${ARCH}"
    echo "Please check the project's releases for a compatible artifact or build from source."
    exit 1
    ;;
esac

# The asset name we use in the GitHub Actions build matrix
ASSET_NAME="stop-nagging-${TARGET}.tar.gz"

# Ensure the install directory exists
mkdir -p "${INSTALL_DIR}"

echo "Determined OS/ARCH => ${TARGET}"
echo "Will download asset: ${ASSET_NAME}"

echo "Fetching latest release info from GitHub..."
LATEST_URL=$(
    curl -s "https://api.github.com/repos/${REPO_OWNER}/${REPO_NAME}/releases/latest" |
        grep "browser_download_url" |
        grep "${ASSET_NAME}" |
        cut -d '"' -f 4
)

if [ -z "${LATEST_URL}" ]; then
    echo "Failed to find a release asset named ${ASSET_NAME} in the latest release."
    echo "Check that your OS/ARCH is built or consider building from source."
    exit 1
fi

echo "Downloading from: ${LATEST_URL}"
curl -L -o "${ASSET_NAME}" "${LATEST_URL}"

echo "Extracting archive..."
tar xzf "${ASSET_NAME}"

# The tar will contain a folder named something like: stop-nagging-${TARGET}/stop-nagging
# Move the binary to INSTALL_DIR.
echo "Moving binary to ${INSTALL_DIR}..."
mv "stop-nagging-${TARGET}/stop-nagging" "${INSTALL_DIR}/stop-nagging"

# Cleanup
rm -rf "stop-nagging-${TARGET}" "${ASSET_NAME}"

echo "Installation complete!"
echo "Ensure ${INSTALL_DIR} is in your PATH. For example:"
echo "  export PATH=\"\$PATH:${INSTALL_DIR}\""
echo "Now you can run: stop-nagging --help"
