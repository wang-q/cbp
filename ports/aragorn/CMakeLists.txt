cmake_minimum_required(VERSION 3.0)
project(aragorn C)

add_executable(aragorn aragorn.c)
target_compile_options(aragorn PRIVATE -Wno-null-character)

install(
    TARGETS aragorn
    RUNTIME DESTINATION bin
)
