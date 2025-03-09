cmake_minimum_required(VERSION 3.15)
project(fastga C)

find_package(ZLIB REQUIRED)
find_package(Threads REQUIRED)

set(CORE_SOURCES
    gene_core.c
)

set(GDB_SOURCES
    GDB.c
    ${CORE_SOURCES}
)

set(TARGETS
    FAtoGDB
    GDBtoFA
    GDBstat
    GDBshow
    GIXmake
    GIXshow
    GIXrm
    GIXmv
    GIXcp
    FastGA
    ALNshow
    ALNtoPAF
    ALNtoPSL
    ALNreset
    ALNplot
    ALNchain
    PAFtoALN
    PAFtoPSL
    ONEview
)

foreach(TARGET ${TARGETS})
    if("${TARGET}" STREQUAL "GIXshow")
        add_executable(${TARGET} ${TARGET}.c libfastk.c ${CORE_SOURCES})
        target_link_libraries(${TARGET} PRIVATE Threads::Threads)
    elseif("${TARGET}" STREQUAL "GIXrm")
        add_executable(${TARGET} ${TARGET}.c ${CORE_SOURCES})
    elseif("${TARGET}" STREQUAL "GIXmv")
        add_executable(${TARGET} GIXxfer.c ${GDB_SOURCES} ONElib.c)
        target_compile_definitions(${TARGET} PRIVATE MOVE)
    elseif("${TARGET}" STREQUAL "GIXcp")
        add_executable(${TARGET} GIXxfer.c ${GDB_SOURCES} ONElib.c)
    elseif("${TARGET}" STREQUAL "FastGA")
        add_executable(${TARGET} ${TARGET}.c RSDsort.c libfastk.c align.c ${GDB_SOURCES} alncode.c ONElib.c)
        target_link_libraries(${TARGET} PRIVATE Threads::Threads)
    elseif("${TARGET}" STREQUAL "ALNshow")
        add_executable(${TARGET} ${TARGET}.c align.c ${GDB_SOURCES} alncode.c select.c hash.c ONElib.c)
        target_link_libraries(${TARGET} PRIVATE Threads::Threads)
    elseif("${TARGET}" STREQUAL "ALNtoPAF")
        add_executable(${TARGET} ${TARGET}.c align.c ${GDB_SOURCES} alncode.c ONElib.c)
        target_link_libraries(${TARGET} PRIVATE Threads::Threads)
    elseif("${TARGET}" STREQUAL "ALNtoPSL")
        add_executable(${TARGET} ${TARGET}.c align.c ${GDB_SOURCES} alncode.c ONElib.c)
        target_link_libraries(${TARGET} PRIVATE Threads::Threads)
    elseif("${TARGET}" STREQUAL "ALNreset")
        add_executable(${TARGET} ${TARGET}.c ${GDB_SOURCES} alncode.c ONElib.c)
        target_link_libraries(${TARGET} PRIVATE Threads::Threads)
    elseif("${TARGET}" STREQUAL "ALNplot")
        add_executable(${TARGET} ${TARGET}.c ${GDB_SOURCES} alncode.c select.c hash.c ONElib.c)
        target_link_libraries(${TARGET} PRIVATE Threads::Threads)
    elseif("${TARGET}" STREQUAL "ALNchain")
        add_executable(${TARGET} ${TARGET}.c ${GDB_SOURCES} alncode.c ONElib.c)
        target_link_libraries(${TARGET} PRIVATE Threads::Threads)
    elseif("${TARGET}" STREQUAL "PAFtoALN")
        add_executable(${TARGET} ${TARGET}.c ${GDB_SOURCES} alncode.c hash.c ONElib.c)
        target_link_libraries(${TARGET} PRIVATE Threads::Threads)
    elseif("${TARGET}" STREQUAL "GDBshow")
        add_executable(${TARGET} ${TARGET}.c ${GDB_SOURCES} select.c hash.c ONElib.c)
        target_link_libraries(${TARGET} PRIVATE Threads::Threads)
    elseif("${TARGET}" STREQUAL "GIXmake")
        add_executable(${TARGET} ${TARGET}.c MSDsort.c libfastk.c ONElib.c ${GDB_SOURCES})
        target_compile_definitions(${TARGET} PRIVATE LCPs)
        target_link_libraries(${TARGET} PRIVATE Threads::Threads)
    elseif("${TARGET}" STREQUAL "PAFtoPSL")
        add_executable(${TARGET} ${TARGET}.c ${CORE_SOURCES})
        target_link_libraries(${TARGET} PRIVATE Threads::Threads)
    elseif("${TARGET}" STREQUAL "ONEview")
        add_executable(${TARGET} ${TARGET}.c ONElib.c)
    else()
        add_executable(${TARGET} ${TARGET}.c ${GDB_SOURCES} ONElib.c)
        target_link_libraries(${TARGET} PRIVATE Threads::Threads)
    endif()

    target_link_libraries(${TARGET} PRIVATE
        m
        ZLIB::ZLIB
    )

    target_compile_options(${TARGET} PRIVATE
        -O3 -Wall -Wextra -Wno-unused-result -fno-strict-aliasing
    )
endforeach()

install(
    TARGETS ${TARGETS}
    RUNTIME DESTINATION tools/fastga
)
