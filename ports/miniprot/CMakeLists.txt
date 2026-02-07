cmake_minimum_required(VERSION 3.15)
project(miniprot C)

find_package(ZLIB REQUIRED)
find_package(Threads REQUIRED)

set(SRCS
    kalloc.c kthread.c nasw-sse.c nasw-tab.c sys.c misc.c options.c ntseq.c sketch.c
    index.c bseq.c chain.c hit.c format.c map.c align.c
    main.c
)

add_executable(miniprot ${SRCS})

target_include_directories(miniprot PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

# Consistent with lh3 tools (bwa/minigraph)
target_compile_definitions(miniprot PRIVATE HAVE_PTHREAD)

target_link_libraries(miniprot PRIVATE ZLIB::ZLIB Threads::Threads)

if(NOT WIN32)
    target_link_libraries(miniprot PRIVATE m)
endif()

if(MSVC)
    target_compile_options(miniprot PRIVATE /W3 /O2)
else()
    target_compile_options(miniprot PRIVATE -std=c99 -g -Wall -O3)
endif()

if(WIN32)
    set_target_properties(miniprot
        PROPERTIES
        SUFFIX ".exe"
    )
endif()

install(TARGETS miniprot DESTINATION bin)
