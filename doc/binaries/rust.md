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
bash scripts/rust.sh ripgrep
bash scripts/rust.sh hyperfine
bash scripts/rust.sh tealdeer
bash scripts/rust.sh tokei
bash scripts/rust.sh jnv

# My bioinformatics utilities
bash scripts/rust.sh intspan
bash scripts/rust.sh nwr
bash scripts/rust.sh hnsm
bash scripts/rust.sh pgr
bash scripts/rust.sh anchr

```

## Builds on macOS

```bash
bash scripts/rust.sh eza native
bash scripts/rust.sh fd native
bash scripts/rust.sh ripgrep native
bash scripts/rust.sh bat native
bash scripts/rust.sh hyperfine native
bash scripts/rust.sh tealdeer native
bash scripts/rust.sh tokei native
bash scripts/rust.sh jnv native

bash scripts/rust.sh intspan native
bash scripts/rust.sh nwr native
bash scripts/rust.sh hnsm native
bash scripts/rust.sh pgr native
bash scripts/rust.sh anchr native

```

## Builds on Windows

```powershell
winget install 7zip.7zip

.\scripts\rust.ps1 eza
.\scripts\rust.ps1 fd
.\scripts\rust.ps1 ripgrep
.\scripts\rust.ps1 bat
.\scripts\rust.ps1 hyperfine
.\scripts\rust.ps1 tealdeer
.\scripts\rust.ps1 tokei
.\scripts\rust.ps1 jnv

cbp local -l eza

.\scripts\rust.ps1 intspan
.\scripts\rust.ps1 nwr
.\scripts\rust.ps1 hnsm
.\scripts\rust.ps1 pgr
.\scripts\rust.ps1 anchr

$files = Get-ChildItem "binaries\*.tar.gz" | Select-Object -ExpandProperty FullName
cbp upload $files

```
