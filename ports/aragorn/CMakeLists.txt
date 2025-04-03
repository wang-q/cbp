cmake_minimum_required(VERSION 3.0)
project(aragorn C)

add_executable(aragorn aragorn.c)

install(
    TARGETS aragorn
    RUNTIME DESTINATION bin
)
