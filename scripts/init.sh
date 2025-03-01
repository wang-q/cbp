#!/usr/bin/env bash

# Detect OS and architecture
OS_TYPE=""

if [ "$(uname -s)" = "Darwin" ]; then
    if [ "$(uname -m)" = "arm64" ]; then
        OS_TYPE="macos"
    else
        echo "cbp currently supports Apple Silicon only on macOS"
        exit 1
    fi
elif [ "$(uname -s)" = "Linux" ]; then
    if [ "$(uname -m)" = "x86_64" ]; then
        OS_TYPE="linux"
        # Check glibc version
        glibc_version=$(ldd --version 2>&1 | head -n1 | grep -oP '(?<=\s)\d+\.\d+')
        major=$(echo "$glibc_version" | cut -d. -f1)
        minor=$(echo "$glibc_version" | cut -d. -f2)
        if [ "$major" -lt 2 ] || [ "$major" -eq 2 -a "$minor" -lt 17 ]; then
            echo "Requires glibc version >= 2.17"
            exit 1
        fi
    else
        echo "cbp currently supports x86_64 only on Linux"
        exit 1
    fi
else
    echo "cbp currently supports Linux and macOS only"
    exit 1
fi

# Check required dependencies
for cmd in curl; do
    if ! command -v $cmd >/dev/null 2>&1; then
        echo "Missing required dependency: $cmd"
        exit 1
    fi
done

# Create necessary directories
mkdir -p "$HOME/.cbp/bin"
mkdir -p "$HOME/.cbp/cache"
mkdir -p "$HOME/.cbp/records"

# Download latest version of cbp
echo "Downloading cbp..."
download_url="https://github.com/wang-q/cbp/releases/latest/download/cbp.${OS_TYPE}"

curl -L -o "$HOME/.cbp/bin/cbp" "$download_url"
chmod +x "$HOME/.cbp/bin/cbp"

# Add to PATH
for rc in "$HOME/.bashrc" "$HOME/.bash_profile" "$HOME/.zshrc"; do
    if [ -f "$rc" ]; then
        if ! grep -q '# .cbp' "$rc"; then
            echo '# .cbp' >> "$rc"
            echo 'export PATH="$HOME/.cbp/bin:$PATH"' >> "$rc"
        fi
    fi
done

echo "cbp installation completed!"
echo "To make the environment variables take effect, run:"
echo "    source ~/.bashrc  # or restart your terminal"
echo "To verify installation:"
echo "    cbp help"
