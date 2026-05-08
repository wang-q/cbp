## Binary Packages

This section describes the binary packages distributed through the "Binaries" release.

### Platform Support

* Linux: x86_64 with glibc 2.17+ (CentOS 7 compatible)
* macOS: arm64 (Apple Silicon)
* Windows: x86_64

### Package Format

All packages are distributed as gzipped tarballs with the following naming convention:

* Linux: package.linux.tar.gz
* macOS: package.macos.tar.gz
* Windows: package.windows.tar.gz
* Fonts: package.font.tar.gz

### Installation

These binaries are meant to be installed via the `cbp` package manager. Manual installation is
possible but not recommended.

### Technical Notes

* All binaries are statically linked where possible
* Dynamic dependencies are limited to system libraries
* Most binaries are built using Zig to ensure cross-platform compatibility
* Packages follow a rolling release model and are updated independently of cbp versions
