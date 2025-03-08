cmake_minimum_required(VERSION 3.15)
project(bwa C)

find_package(ZLIB REQUIRED)

# Core library source files
set(LOBJS
    utils.c kthread.c kstring.c ksw.c bwt.c bntseq.c bwa.c bwamem.c
    bwamem_pair.c bwamem_extra.c malloc_wrap.c QSufSort.c bwt_gen.c
    rope.c rle.c is.c bwtindex.c
)

# Build static library
add_library(libbwa STATIC ${LOBJS})
set_target_properties(libbwa PROPERTIES OUTPUT_NAME bwa)
target_link_libraries(libbwa PRIVATE ZLIB::ZLIB)

# Additional source files
set(AOBJS
    bwashm.c bwase.c bwaseqio.c bwtgap.c bwtaln.c bamlite.c
    bwape.c kopen.c pemerge.c maxk.c
    bwtsw2_core.c bwtsw2_main.c bwtsw2_aux.c bwt_lite.c
    bwtsw2_chain.c fastmap.c bwtsw2_pair.c
)

# Compilation definitions
add_compile_definitions(HAVE_PTHREAD)
add_compile_definitions(USE_MALLOC_WRAPPERS)

# Build executable
add_executable(bwa main.c ${AOBJS})
target_link_libraries(bwa PRIVATE
    libbwa
    ZLIB::ZLIB
    m
    pthread
)

# Linux specific libraries
if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    target_link_libraries(bwa PRIVATE rt)
endif()

# Installation
install(TARGETS bwa
    RUNTIME DESTINATION bin
)
