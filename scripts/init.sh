#!/usr/bin/env bash

# Detect OS and architecture
OS_TYPE=""
ARCH=""

if [ $(uname -s) = "Darwin" ]; then
    OS_TYPE="macos"
elif [ $(uname -s) = "Linux" ]; then
    OS_TYPE="linux"
    # Check glibc version
    glibc_version=$(ldd --version 2>&1 | head -n1 | grep -oP '(?<=\s)\d+\.\d+')
    if (( $(echo "$glibc_version < 2.17" | bc -l) )); then
        echo "Requires glibc version >= 2.17"
        exit 1
    fi
else
    echo "cbp currently supports Linux and macOS only"
    exit 1
fi

if [ $(uname -m) = "x86_64" ]; then
    ARCH="x86_64"
elif [ $(uname -m) = "arm64" ] || [ $(uname -m) = "aarch64" ]; then
    ARCH="aarch64"
else
    echo "cbp currently supports x86_64 and ARM64 architectures only"
    exit 1
fi

# Check required dependencies
for cmd in curl jq; do
    if ! command -v $cmd >/dev/null 2>&1; then
        echo "Missing required dependency: $cmd"
        exit 1
    fi
done

# Create necessary directories
mkdir -p "$HOME/.cbp/cache"
mkdir -p "$HOME/.cbp/binaries"

# Download latest version of cbp
echo "Downloading cbp..."
latest_version=$(curl -s https://api.github.com/repos/wang-q/cbp/releases/latest | jq -r .tag_name)
download_url="https://github.com/wang-q/cbp/releases/download/${latest_version}/cbp.${OS_TYPE}"

curl -L -o "$HOME/bin/cbp" "$download_url"
chmod +x "$HOME/bin/cbp"

# Add to PATH
for rc in "$HOME/.bashrc" "$HOME/.bash_profile" "$HOME/.zshrc"; do
    if [ -f "$rc" ]; then
        if ! grep -q '# .cbp' "$rc"; then
            echo '# .cbp' >> "$rc"
            echo 'export PATH="$HOME/bin:$PATH"' >> "$rc"
        fi
    fi
done

echo "cbp installation completed!"
echo "To make the environment variables take effect, run:"
echo "    source ~/.bashrc  # or restart your terminal"
echo "To verify installation:"
echo "    cbp --version"
