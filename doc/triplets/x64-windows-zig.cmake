set(VCPKG_TARGET_ARCHITECTURE x64)
set(VCPKG_CRT_LINKAGE static)
set(VCPKG_LIBRARY_LINKAGE static)
set(VCPKG_ENV_PASSTHROUGH PATH)

set(VCPKG_CMAKE_SYSTEM_NAME MinGW)
set(VCPKG_BUILD_TYPE release)

# Set target architecture for zig
set(CMAKE_C_COMPILER_TARGET x86_64-windows-gnu)
set(CMAKE_CXX_COMPILER_TARGET x86_64-windows-gnu)

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
set(CMAKE_C_ARCHIVE_CREATE   "<CMAKE_AR> -crs <TARGET> <OBJECTS>")
set(CMAKE_CXX_ARCHIVE_CREATE "<CMAKE_AR> -crs <TARGET> <OBJECTS>")

set(CMAKE_C_ARCHIVE_FINISH   "<CMAKE_RANLIB> <TARGET>")
set(CMAKE_CXX_ARCHIVE_FINISH "<CMAKE_RANLIB> <TARGET>")

# Configure library naming
set(CMAKE_STATIC_LIBRARY_PREFIX "lib")
set(CMAKE_STATIC_LIBRARY_SUFFIX ".a")

# Disable compiler checks
set(CMAKE_TRY_COMPILE_TARGET_TYPE "STATIC_LIBRARY")
