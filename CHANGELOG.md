# Change Log

## Unreleased - ReleaseDate

- Added `list` command for package management
  - List all installed packages
  - Show files in specific package
- Added core utilities
  - Platform detection (macOS/Linux)
  - Directory structure management
  - Package listing and formatting

## 0.1.0 - 2025-02-28

- Initial release
- Added `kb` command for documentation display
  - README.md
  - common.sh documentation
  - sources directory documentation
  - binaries directory documentation
  - developer guide
- Supported platforms
  - Linux (glibc 2.17+)
  - macOS (Apple Silicon)
- Added installation script `init.sh`
- Added build script framework
  - common.sh providing shared build functions
  - Cross-platform compilation support
