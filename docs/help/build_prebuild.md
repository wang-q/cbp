Build prebuilt binary packages from source distributions.

The command downloads pre-compiled binaries from GitHub releases,
extracts them, and packages them into platform-specific cbp archives.

Examples:
1. Build for current platform:
   cbp build prebuild zlib

2. Build for specific platform:
   cbp build prebuild zlib -t linux

3. Build multiple packages:
   cbp build prebuild zlib bzip2

4. Specify base directory:
   cbp build prebuild zlib --base /path/to/project