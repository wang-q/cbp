# Change Log

## Unreleased - ReleaseDate

- Added `local` command for package management
  - Install packages from local binaries directory
  - Support both local builds and cached packages
  - Prevent duplicate installations
- Added `list` command for package management
  - List all installed packages
  - Show files in specific package
- Added `untracked` command to find unmanaged files
  - Skip system generated files (macOS/Windows/Linux)
  - Ignore package management directories
- Added `remove` command for package management
  - Remove installed packages
  - Handle files and symlinks properly
  - Skip directory removal
- Added core utilities
  - Platform detection (macOS/Linux)
  - Directory structure management
  - Package listing and formatting
- Added `scripts/tools/` directory containing Bash prototypes for package management

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
