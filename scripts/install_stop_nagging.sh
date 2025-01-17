#!/usr/bin/env bash
set -e

# This script installs stop-nagging locally.
# Usage:
#   curl -s https://raw.githubusercontent.com/mohsen1/stop-nagging/main/scripts/install_stop_nagging.sh | bash

REPO_URL="https://github.com/mohsen1/stop-nagging"
INSTALL_DIR="$HOME/.local/bin"

if [[ ! -d "$INSTALL_DIR" ]]; then
    mkdir -p "$INSTALL_DIR"
fi

echo "Fetching latest release artifact from GitHub..."
# For a real implementation, you might want to fetch a release tarball or binary from GitHub Releases.
# Below is a placeholder that clones + builds the project on the user's machine. This can be replaced with direct binary downloads.

if [[ -d "stop-nagging-tmp" ]]; then
    rm -rf stop-nagging-tmp
fi

git clone --depth=1 "$REPO_URL" stop-nagging-tmp
cd stop-nagging-tmp

echo "Building stop-nagging..."
cargo build --release

echo "Moving stop-nagging binary to $INSTALL_DIR..."
mv target/release/stop-nagging "$INSTALL_DIR"

cd ..
rm -rf stop-nagging-tmp

echo "Installation complete!"
echo "Ensure $INSTALL_DIR is in your PATH. Example:"
echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
echo "Now you can run: stop-nagging --help"
