cmake_minimum_required(VERSION 3.15)
project(sickle C)

find_package(ZLIB REQUIRED)

set(SOURCES
    src/sliding.c
    src/trim_single.c
    src/trim_paired.c
    src/sickle.c
    src/print_record.c
)

add_executable(sickle ${SOURCES})

target_link_libraries(sickle PRIVATE ZLIB::ZLIB)
target_include_directories(sickle PRIVATE src)
target_compile_definitions(sickle PRIVATE VERSION=1.33)

target_compile_options(sickle PRIVATE
    $<$<AND:$<C_COMPILER_ID:MSVC>,$<CONFIG:Release>>:/O2>
    $<$<C_COMPILER_ID:MSVC>:/W3>
    $<$<NOT:$<C_COMPILER_ID:MSVC>>:-O3 -Wall -pedantic>
)

if(WIN32)
    set_target_properties(sickle
        PROPERTIES
        SUFFIX ".exe"
    )
endif()

install(TARGETS sickle DESTINATION tools/sickle)
