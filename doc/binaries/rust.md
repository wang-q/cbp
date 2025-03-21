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
# CLI utilities
bash scripts/rust.sh eza
bash scripts/rust.sh fd
bash scripts/rust.sh dust
bash scripts/rust.sh ripgrep
bash scripts/rust.sh skim
bash scripts/rust.sh hyperfine
bash scripts/rust.sh tealdeer
bash scripts/rust.sh tokei
bash scripts/rust.sh jnv
bash scripts/rust.sh resvg

# Bioinformatics utilities
bash scripts/rust.sh intspan
bash scripts/rust.sh nwr
bash scripts/rust.sh hnsm
bash scripts/rust.sh pgr
bash scripts/rust.sh anchr

bash scripts/rust.sh wgatools

```

## Builds on macOS

```bash
bash scripts/rust.sh eza native
bash scripts/rust.sh fd native
bash scripts/rust.sh dust native
bash scripts/rust.sh skim native
bash scripts/rust.sh ripgrep native
bash scripts/rust.sh hyperfine native
bash scripts/rust.sh tealdeer native
bash scripts/rust.sh tokei native
bash scripts/rust.sh jnv native
bash scripts/rust.sh resvg native

bash scripts/rust.sh intspan native
bash scripts/rust.sh nwr native
bash scripts/rust.sh hnsm native
bash scripts/rust.sh pgr native
bash scripts/rust.sh anchr native

bash scripts/rust.sh wgatools native

```

## Builds on Windows

```powershell
winget install 7zip.7zip

.\scripts\rust.ps1 eza
.\scripts\rust.ps1 fd
.\scripts\rust.ps1 dust
.\scripts\rust.ps1 ripgrep
# .\scripts\rust.ps1 skim
.\scripts\rust.ps1 hyperfine
.\scripts\rust.ps1 tealdeer
.\scripts\rust.ps1 tokei
.\scripts\rust.ps1 jnv
.\scripts\rust.ps1 resvg

cbp local -l eza

.\scripts\rust.ps1 intspan
.\scripts\rust.ps1 nwr
.\scripts\rust.ps1 hnsm
.\scripts\rust.ps1 pgr
.\scripts\rust.ps1 anchr

# .\scripts\rust.ps1 wgatools

```
