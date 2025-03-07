cmake_minimum_required(VERSION 3.15)
project(pigz C)

find_package(ZLIB REQUIRED)

set(PIGZ_SOURCES
    pigz.c
    yarn.c
    try.c
)

set(ZOPFLI_SOURCES
    zopfli/src/zopfli/deflate.c
    zopfli/src/zopfli/blocksplitter.c
    zopfli/src/zopfli/tree.c
    zopfli/src/zopfli/lz77.c
    zopfli/src/zopfli/cache.c
    zopfli/src/zopfli/hash.c
    zopfli/src/zopfli/util.c
    zopfli/src/zopfli/squeeze.c
    zopfli/src/zopfli/katajainen.c
    zopfli/src/zopfli/symbols.c
)

add_executable(pigz ${PIGZ_SOURCES} ${ZOPFLI_SOURCES})

target_include_directories(pigz PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}
    ${CMAKE_CURRENT_SOURCE_DIR}/zopfli/src
)
target_link_libraries(pigz PRIVATE ZLIB::ZLIB)
target_link_libraries(pigz PRIVATE m pthread)
target_compile_options(pigz PRIVATE
    -O3
    -Wall
    -Wextra
    -Wno-unknown-pragmas
    -Wcast-qual
)

if (WIN32)
    set_target_properties(pigz PROPERTIES SUFFIX ".bin")
endif()

# if(WIN32)
#     install(
#         TARGETS pigz
#         DESTINATION tools/pigz
#         RENAME pigz.exe
#     )
# else()
#     install(TARGETS pigz DESTINATION tools/pigz)
# endif()
install(TARGETS pigz)
