cmake_minimum_required(VERSION 3.15)
project(faops C)

find_package(ZLIB REQUIRED)

add_executable(faops faops.c)

target_link_libraries(faops PRIVATE ZLIB::ZLIB)
target_compile_options(faops PRIVATE
    -O2
    -Wall
    -std=gnu99
    -Wno-unused-function
)

if(WIN32)
    set_target_properties(faops
        PROPERTIES
        SUFFIX ".exe"
    )
endif()

install(TARGETS faops)
