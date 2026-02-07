cmake_minimum_required(VERSION 3.15)
project(minigraph C)

find_package(ZLIB REQUIRED)
find_package(Threads REQUIRED)

set(SRCS
    kalloc.c kthread.c algo.c sys.c gfa-base.c gfa-io.c gfa-aug.c gfa-bbl.c
    gfa-ed.c sketch.c misc.c bseq.c options.c shortk.c miniwfa.c
    index.c lchain.c gchain1.c galign.c gcmisc.c map-algo.c cal_cov.c
    format.c gmap.c ggsimple.c ggen.c asm-call.c
    main.c
)

add_executable(minigraph ${SRCS})

target_include_directories(minigraph PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

# Consistent with bwa/minigraph Makefile
target_compile_definitions(minigraph PRIVATE HAVE_PTHREAD)

target_link_libraries(minigraph PRIVATE ZLIB::ZLIB Threads::Threads)

if(NOT WIN32)
    target_link_libraries(minigraph PRIVATE m)
endif()

if(MSVC)
    target_compile_options(minigraph PRIVATE /W3 /O2)
else()
    target_compile_options(minigraph PRIVATE -g -Wall -Wc++-compat -std=c99 -O3)
    if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
        target_compile_options(minigraph PRIVATE -msse4)
    endif()
endif()

if(WIN32)
    set_target_properties(minigraph
        PROPERTIES
        SUFFIX ".exe"
    )
endif()

install(TARGETS minigraph DESTINATION bin)
