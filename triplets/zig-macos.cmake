# Specify compiler details
set(CMAKE_C_COMPILER_ID "Clang")
set(CMAKE_CXX_COMPILER_ID "Clang")
set(CMAKE_C_COMPILER_FRONTEND_VARIANT "GNU")
set(CMAKE_CXX_COMPILER_FRONTEND_VARIANT "GNU")

# Configure compilers
set(CMAKE_C_COMPILER "zig-cc")
set(CMAKE_CXX_COMPILER "zig-c++")

# Set target architecture for zig
set(CMAKE_C_COMPILER_TARGET aarch64-macos-none)
set(CMAKE_CXX_COMPILER_TARGET aarch64-macos-none)

# Configure toolchain programs
set(CMAKE_AR "zig-ar")
set(CMAKE_RANLIB "zig-ranlib")

# Configure static library creation and indexing commands
set(CMAKE_C_ARCHIVE_CREATE "<CMAKE_AR> -crs <TARGET> <OBJECTS>")
set(CMAKE_CXX_ARCHIVE_CREATE "<CMAKE_AR> -crs <TARGET> <OBJECTS>")

set(CMAKE_C_ARCHIVE_FINISH "<CMAKE_RANLIB> <TARGET>")
set(CMAKE_CXX_ARCHIVE_FINISH "<CMAKE_RANLIB> <TARGET>")

# Disable compiler checks
set(CMAKE_TRY_COMPILE_TARGET_TYPE "STATIC_LIBRARY")
