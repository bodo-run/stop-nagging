#!/usr/bin/env bash
set -euo pipefail

REPO_OWNER="bodo-run"
REPO_NAME="stop-nagging"

# Determine a sensible default install directory
# We'll check for a directory in PATH that is writable.
# If none is found, we fall back to "$HOME/.local/bin".
fallback_dir="$HOME/.local/bin"

# Split PATH on ":" into an array
IFS=':' read -ra path_entries <<<"$PATH"
install_candidates=("/usr/local/bin" "${path_entries[@]}")
install_dir=""

for dir in "${install_candidates[@]}"; do
    # Skip empty paths
    [ -z "$dir" ] && continue

    # Check if directory is writable
    if [ -d "$dir" ] && [ -w "$dir" ]; then
        install_dir="$dir"
        break
    fi
done

# If we didn't find a writable dir in PATH, fallback to $HOME/.local/bin
if [ -z "$install_dir" ]; then
    install_dir="$fallback_dir"
fi

mkdir -p "$install_dir"

echo "Selected install directory: $install_dir"

# Detect OS and ARCH to choose the correct artifact
OS=$(uname -s)
ARCH=$(uname -m)

case "${OS}_${ARCH}" in
Linux_x86_64)
    TARGET="x86_64-unknown-linux-gnu"
    ;;
Darwin_x86_64)
    TARGET="x86_64-apple-darwin"
    ;;
Darwin_arm64)
    TARGET="aarch64-apple-darwin"
    ;;
*)
    echo "Unsupported OS/ARCH combo: ${OS} ${ARCH}"
    echo "Please check the project's releases for a compatible artifact or build from source."
    exit 1
    ;;
esac

ASSET_NAME="stop-nagging-${TARGET}.tar.gz"
echo "OS/ARCH => ${TARGET}"
echo "Asset name => ${ASSET_NAME}"

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
echo "Moving binary to ${install_dir}..."
mv "stop-nagging-${TARGET}/stop-nagging" "${install_dir}/stop-nagging"

echo "Making the binary executable..."
chmod +x "${install_dir}/stop-nagging"

# Cleanup
rm -rf "stop-nagging-${TARGET}" "${ASSET_NAME}"

echo "Installation complete!"

# Check if install_dir is in PATH
if ! echo "$PATH" | tr ':' '\n' | grep -Fx "$install_dir" >/dev/null; then
    echo "NOTE: $install_dir is not in your PATH. Add it by running:"
    echo "  export PATH=\"\$PATH:$install_dir\""
fi

echo "Now you can run: stop-nagging --help"
