# Specify compiler details
set(CMAKE_C_COMPILER_ID Clang)
set(CMAKE_CXX_COMPILER_ID Clang)
set(CMAKE_C_COMPILER_FRONTEND_VARIANT GNU)
set(CMAKE_CXX_COMPILER_FRONTEND_VARIANT GNU)

# Configure compilers
# Force CMake to use our compiler
set(CMAKE_C_COMPILER "zig-cc.cmd" )
set(CMAKE_CXX_COMPILER "zig-c++.cmd" )

# Set target architecture for zig
set(CMAKE_C_COMPILER_TARGET x86_64-windows-gnu)
set(CMAKE_CXX_COMPILER_TARGET x86_64-windows-gnu)

# Configure toolchain programs
set(CMAKE_AR "zig-ar.cmd")
set(CMAKE_RANLIB "zig-ranlib.cmd")

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
