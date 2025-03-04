# Rust Packages Build Process


This document describes the build process for Rust-based command-line utilities. The process
uses `cargo zigbuild` to create optimized binaries and packages them into
platform-specific `.tar.gz` archives for distribution through GitHub releases.

## Prerequisites

* Rust toolchain
* Zig 
* cargo-zigbuild

## Builds on Linux

```bash
bash scripts/rust.sh eza
bash scripts/rust.sh fd
bash scripts/rust.sh ripgrep
bash scripts/rust.sh hyperfine
bash scripts/rust.sh tealdeer
bash scripts/rust.sh tokei

bash scripts/rust.sh fd windows
bash scripts/rust.sh ripgrep windows
bash scripts/rust.sh hyperfine windows
bash scripts/rust.sh tealdeer windows
bash scripts/rust.sh tokei windows

```

## Builds on macOS

```bash
bash scripts/rust.sh eza macos
bash scripts/rust.sh fd macos
bash scripts/rust.sh ripgrep macos
bash scripts/rust.sh hyperfine macos
bash scripts/rust.sh tealdeer macos
bash scripts/rust.sh tokei macos

```

### My bioinformatics utilities

```bash
bash scripts/rust.sh intspan
bash scripts/rust.sh nwr
bash scripts/rust.sh hnsm
bash scripts/rust.sh pgr
bash scripts/rust.sh anchr

```
