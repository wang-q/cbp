# vcpkg build environment

```bash
cd
# Download and extract vcpkg
curl -L https://github.com/microsoft/vcpkg/archive/refs/tags/2025.02.14.tar.gz |
    tar xvz &&
    mv vcpkg-* vcpkg

cd vcpkg
./bootstrap-vcpkg.sh -disableMetrics

# Set environment variables
export VCPKG_ROOT=$HOME/vcpkg
export PATH=$VCPKG_ROOT:$PATH

```

```bash
# Create compiler shims
mkdir -p $HOME/bin

cat > $HOME/bin/zig-cc <<'EOF'
#!/bin/bash
exec zig cc "$@"
EOF

cat > $HOME/bin/zig-c++ <<'EOF'
#!/bin/bash
exec zig c++ "$@"
EOF

cat > $HOME/bin/zig-ar <<'EOF'
#!/bin/bash
exec zig ar "$@"
EOF

cat > $HOME/bin/zig-ranlib <<'EOF'
#!/bin/bash
exec zig ranlib "$@"
EOF

chmod +x $HOME/bin/zig-*

```

```bash
# Create custom triplet file
cat > $VCPKG_ROOT/triplets/community/x64-linux-zig.cmake <<'EOF'
set(VCPKG_TARGET_ARCHITECTURE x64)
set(VCPKG_CRT_LINKAGE static)
set(VCPKG_LIBRARY_LINKAGE static)

set(VCPKG_CMAKE_SYSTEM_NAME Linux)
set(VCPKG_BUILD_TYPE release)

# Set target architecture for zig
set(CMAKE_C_COMPILER_TARGET x86_64-linux-gnu.2.17)
set(CMAKE_CXX_COMPILER_TARGET x86_64-linux-gnu.2.17)

set(VCPKG_CHAINLOAD_TOOLCHAIN_FILE ${CMAKE_CURRENT_LIST_DIR}/zig-toolchain.cmake)
EOF

cat > $VCPKG_ROOT/triplets/community/arm64-macos-zig.cmake <<'EOF'
set(VCPKG_TARGET_ARCHITECTURE arm64)
set(VCPKG_CRT_LINKAGE static)
set(VCPKG_LIBRARY_LINKAGE static)

set(VCPKG_CMAKE_SYSTEM_NAME Darwin)
set(VCPKG_OSX_ARCHITECTURES arm64)
set(VCPKG_BUILD_TYPE release)

# Set target architecture for zig
set(CMAKE_C_COMPILER_TARGET aarch64-macos-none)
set(CMAKE_CXX_COMPILER_TARGET aarch64-macos-none)

set(VCPKG_CHAINLOAD_TOOLCHAIN_FILE ${CMAKE_CURRENT_LIST_DIR}/zig-toolchain.cmake)
EOF

# Create zig toolchain file
cat > $VCPKG_ROOT/triplets/community/zig-toolchain.cmake <<'EOF'
# Configure compilers
set(CMAKE_C_COMPILER zig-cc)
set(CMAKE_CXX_COMPILER zig-c++)

# Specify compiler details
set(CMAKE_C_COMPILER_ID Clang)
set(CMAKE_CXX_COMPILER_ID Clang)
set(CMAKE_C_COMPILER_FRONTEND_VARIANT GNU)
set(CMAKE_CXX_COMPILER_FRONTEND_VARIANT GNU)

# Configure toolchain programs
set(CMAKE_AR zig-ar)
set(CMAKE_RANLIB zig-ranlib)

# Configure static library creation and indexing commands
set(CMAKE_C_ARCHIVE_CREATE "<CMAKE_AR> -crs <TARGET> <OBJECTS>")
set(CMAKE_CXX_ARCHIVE_CREATE "<CMAKE_AR> -crs <TARGET> <OBJECTS>")

set(CMAKE_C_ARCHIVE_FINISH "<CMAKE_RANLIB> <TARGET>")
set(CMAKE_CXX_ARCHIVE_FINISH "<CMAKE_RANLIB> <TARGET>")

# Disable compiler checks
set(CMAKE_TRY_COMPILE_TARGET_TYPE "STATIC_LIBRARY")
EOF

```

```bash
# vcpkg remove --debug zlib:x64-linux-zig
vcpkg install --debug zlib:x64-linux-zig
vcpkg install --debug libdeflate:x64-linux-zig

vcpkg install --debug expat:x64-linux-zig
vcpkg install --debug argtable2:x64-linux-zig

vcpkg install --debug sqlite3:x64-linux-zig

vcpkg install --debug libpng:x64-linux-zig
vcpkg install --debug pixman:x64-linux-zig
# vcpkg install --debug cairo:x64-linux-zig

# vcpkg install --debug libxcrypt:x64-linux-zig

vcpkg install --debug c-ares:x64-linux-zig

vcpkg install --debug hdf5:x64-linux-zig

# vcpkg install --debug gsl:x64-linux-zig

# Install zlib with custom target
# vcpkg install zlib:x64-linux-zig \
#     --cmake-args="-DCMAKE_C_COMPILER_TARGET=aarch64-macos-none" \
#     --cmake-args="-DCMAKE_CXX_COMPILER_TARGET=aarch64-macos-none"

vcpkg install --debug zlib:x64-macos-zig
vcpkg install --debug bzip2:x64-macos-zig

```
