## Binary Packages

This section describes the binary packages distributed through the "Binaries" release.

### Platform Support

* Linux: x86_64 with glibc 2.17+ (CentOS 7 compatible)
* macOS: ARM64 (Apple Silicon)

### Package Format

All packages are distributed as gzipped tarballs with the following naming convention:

* Linux: package.linux.tar.gz
* macOS: package.macos.tar.gz

### Installation

These binaries are meant to be installed via the `cbp` package manager. Manual installation is
possible but not recommended.

### Technical Notes

* All binaries are statically linked where possible
* Dynamic dependencies are limited to system libraries
* Binaries are built with Zig for consistent cross-platform compatibility
* This is a rolling release - packages are updated independently of `cbp` versions
