# Change Log

## Unreleased - ReleaseDate

## 0.4.0 - 2026-05-11

- New Commands
  - Added `snap` command for file snapshot management with `save`, `load`, `list`, and `delta` subcommands.
    - `save` — Create snapshots of files/directories with gzip comment-based path tracking.
    - `load` — Restore files from snapshots to original or custom locations.
    - `list` — Inspect snapshot contents and source paths.
    - `delta` — Show and pack files modified since a snapshot was taken.
    - Added `--exclude` flag to `snap save` for glob-based file exclusion.
  - Enhanced `dot` command with dotfiles management using templates and filename conventions.
    - Template rendering with Tera engine (Jinja2 compatible syntax).
    - Filename prefix parsing (`private_`, `executable_`, `dot_`, `xdg_config/`, `xdg_data/`, etc.).
    - Permission setting via `private_` and `executable_` prefixes.
    - Multi-source support for `dot` command.
    - Only set Unix permissions when explicitly specified via filename prefixes.
  - Removed `kb` command and its associated documentation viewing functionality.
- Code Quality
  - Replaced `println!`/`eprintln!` with `tracing` macros (`info!`, `warn!`, `debug!`, `error!`) across all modules.
  - Added `tracing-subscriber` for structured application logging.
  - Replaced `unwrap()` with proper error handling in file path and other fallible operations.
  - Refactored function signatures: replaced `PathBuf` with `Path` where ownership is not needed.
  - Centralized GitHub URL handling with utility functions (`api`, `raw`, `release`).
  - Implemented `CbpDirs::from_arg_matches` for consistent directory handling.
  - Extracted common file path construction and extraction checks into shared library functions.
  - Moved snap utility functions to shared `utils` module with comprehensive tests.
  - Simplified path handling using `strip_prefix` and direct iterator methods.
  - Improved user feedback messages for various commands (install, list, check, dot).
  - Added file count display in snap creation and delta messages.
  - Improved verbose output formatting and consistency across snap and dot commands.
  - Allow overriding download URLs via `CBP_DOWNLOAD_BASE` environment variable in build commands.
  - Added public function documentation across multiple modules.
  - Fix: remove existing file before writing in dot command to prevent symlink issues.
- Documentation
  - Moved all documentation from `doc/` to `docs/` directory.
  - Added comprehensive mdBook documentation structure with `SUMMARY.md`.
  - Added help documentation files for all commands in `docs/help/`.
  - Moved command help text from inline code to external markdown files using `include_str!`.
  - Added detailed help text style guide with formatting specifications.
  - Added comprehensive `homedir.md` documentation for `dot` and `snap` commands.
  - Updated `binaries.md` with new package build instructions.
  - Fixed incorrect file paths in documentation references.
  - Standardized file endings with trailing newlines.
  - Updated `build` command help with subcommand descriptions.
  - Added Windows path handling documentation for snap commands.
  - Replaced `AGENTS.md` with more comprehensive `CLAUDE.md` as AI assistant guide.
- Package Management
  - Added packages: `coreutils`, `libdivsufsort`, `maple-mono`, `minigraph`, `miniprot`, `ms`, `pgr`, `rush`, `tva`, `twemoji`.
  - Migrated `miniprot` from make-based build to vcpkg port.
- Build System
  - Extracted common file path and extraction logic into shared library functions.
  - Added utility functions for target path generation and line ending normalization.
- CI/CD
  - Added GitHub workflow (`docs.yml`) for building and deploying documentation to GitHub Pages.
  - Added build script for documentation with mdBook.
  - Updated `book.toml` edit URL template from `master` to `main` branch.
  - Added mdBook output and coverage directories to `.gitignore`.
- Repository Housekeeping
  - Updated README badges and links (fixed license URL, added documentation badge).
  - Updated various package configurations and binary tarballs.

## 0.3.14 - 2025-11-03

- CLI Improvements
  - Added `--force` option to `build upload` to skip MD5 checks and force uploads.
- Documentation Updates
  - Use zvm 0.14.1 for Zig, bump vcpkg tag to `2025.10.17`, unify PowerShell fences.
  - Developer guide (`doc/developer.md`): clarified Zig versions (install 0.13.0 and 0.14.1, use 0.13.0); added PATH setup snippets for Rust/VCPKG; bumped vcpkg tag to `2025.10.17`; restored detailed TOC format.
- Package Management
  - Added packages: `bbtools`, `gatk`, `graphicsmagick`, `jellyfish`, `quorum`, `spades`, and `superreads`.
- Build System and Ports
  - General cleanups and small fixes in build documentation.
- Repository Housekeeping
  - Added `logo.svg`; updated README header to display logo and added Logo section.

## 0.3.13 - 2025-06-05

- Removed created_at field from upload.rs
- Updated test dependency versions (assert_cmd, predicates, mockito)
- Build System Improvements
  - Moved unused tool scripts from `scripts/tools/` to `doc/attempts/tools/`

## 0.3.12 - 2025-05-22

- Package Management Changes
  - Added some packages
- Build System Improvements
  - Enhanced Windows command execution handling
    - Added support for `.exe`, `.ps1`, `.bat`, `.cmd` extensions
    - Improved handling of built-in Windows commands
    - Added PowerShell script execution support
- Schema Validation Updates
  - Refactored `schema.json` downloads field validation rules
  - Simplified downloads object structure
  - Improved platform-specific download configurations
- Documentation Updates
  - Streamlined build instructions in binaries.md
  - Simplified vcpkg usage examples in developer.md
  - Updated Windows-specific build instructions

## 0.3.11 - 2025-03-31

- Improved Rust implementation
  - Improved path handling in match_files
  - Optimize file renaming logic
- Added new tools and scripts
  - Added package analysis scripts
- Enhanced package validation
  - Updated schema with flexible version format
  - Added `--schema` option and workflow
- Documentation improvements
  - Reorganized binaries.md
  - Updated command descriptions

## 0.3.10 - 2025-03-27

- Reorganized commands
  - Moved `upload` command under `build` subcommands
- Enhanced package validation
  - Added `--schema` option to `build validate` command
  - Added GitHub workflow for package schema validation

## 0.3.9 - 2025-03-27

- Added new commands
  - Added `build source` command for downloading sources
  - Added `build prebuild` command for prebuilt packages
  - Added `build font` command for font packages
  - Added `build test` command for package testing
  - Added `build validate` command for package configuration validation
- Enhanced package management
  - Added package schema validation with JSON Schema
  - Added package type classification
  - Migrated build scripts to package definitions
  - Added more package metadata and test cases
- Improved testing
  - Enhanced binary test workflow with separate standalone and package tests
  - Added more test coverage
- Documentation updates
- Added more packages

## 0.3.8 - 2025-03-24

- Enhanced package management
  - Added `info` command for package metadata display
  - Added `uninstall` as alias for `remove` command
  - Improved `collect` command
  - Added `prebuild.sh` script for unified prebuilt package handling
  - Migrate prebuild scripts to package definitions: `packages/`
- Improved binary testing workflow
- Updated build system
  - Added LLVM toolchain support
- Improved documentation
- Added more packages

## 0.3.7 - 2025-03-21

- Added automated binary testing workflow
- Added Python 3 support with uv package manager
- Added relocatable Perl 5 support
- Added more vcpkg ports and packages
- Added test scripts for various packages
- Improved build system and documentation

## 0.3.6 - 2025-03-13

- Improved path handling
  - `dunce` for Windows path normalization
  - Simplified executable path resolution
- Enhanced build system
  - Added CentOS 7 container support
  - Improved vcpkg script handling
  - Improved rust handling
- Added new packages
- Updated documentations

## 0.3.5 - 2025-03-10

- Refactored build system
  - Added more vcpkg ports support
  - Moved legacy build scripts to doc/attempts/
  - Updated binaries.md documentation with vcpkg instructions
- Updated development environment
  - Enhanced source package management
  - Improved symlink handling
- Added font management support
  - Added font installation instructions
  - Added font package scripts

## 0.3.4 - 2025-03-07

- Migrated to vcpkg-based build system
  - Added custom triplets for cross-compilation
  - Added vcpkg overlay ports support
- Added development environment setup
  - Added `--dev` option to `init` command
  - Added compiler shims for cross-compilation
  - Added vcpkg triplet configurations
- Added `collect` command for package management
- Enhanced Windows support
  - Added MSYS2 building environment
  - Improved toolchain configurations
- Reorganized documentation and build scripts
  - Moved legacy scripts to doc/attempts/
  - Added vcpkg usage guides

## 0.3.3 - 2025-03-05

- Improved Windows support
  - Cross compilation with Zig 0.14.0
  - Added PowerShell build scripts
  - Enhanced binary handling for Windows executables
- Added package content management
  - Added `--list` option to view installed packages
  - Added `--type` option to filter package types
  - Added package content preview
- Added new packages
- Reorganized documentation

## 0.3.2 - 2025-03-04

- Added package manager initialization
  - Added `init` command for environment setup
  - Added `prefix` command for installation paths
- Added Windows support
- Improved proxy support
- Enhanced core utilities

## 0.3.1 - 2025-03-02

- Fixed symlink handling in `tar` command
- Added more software packages
- Improved documentation and help messages

## 0.3.0 - 2025-03-02

- Added `tar` command for creating package archives
- Added `upload` command for managing GitHub releases
- Improved package information tracking with MD5 checksums

## 0.2.0 - 2025-03-01

- Added `install` command with proxy support
- Added `avail` command for package discovery
- Added `local` command for local package installation
- Added `list` command for package management
- Added `check` command for unmanaged files detection
- Added `remove` command with resource fork handling
- Added core utilities for platform detection and package management
- Improved documentation and help messages
- Added `scripts/tools/` directory

## 0.1.0 - 2025-02-28

- Initial release
- Added `kb` command for documentation display
- Supported Linux (glibc 2.17+) and macOS (Apple Silicon)
- Added installation script `init.sh`
- Added build script framework with cross-platform support
