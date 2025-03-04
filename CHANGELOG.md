# Change Log

## Unreleased - ReleaseDate

- Added `init` command for initializing the package manager
- Added `prefix` command for getting the package manager prefix
- Added Windows support
- Improved proxy support
- Improved core utilities

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
