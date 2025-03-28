cmake_minimum_required(VERSION 3.15)
project(seqtk C)

find_package(ZLIB REQUIRED)

add_executable(seqtk seqtk.c)

target_link_libraries(seqtk PRIVATE ZLIB::ZLIB m)
target_compile_definitions(seqtk PRIVATE
    _GNU_SOURCE
)
target_compile_options(seqtk PRIVATE
    -O2
    -Wall
    -Wno-unused-function
    -Wno-format
)

if(WIN32)
    set_target_properties(seqtk
        PROPERTIES
        SUFFIX ".exe"
    )
endif()

install(TARGETS seqtk)
