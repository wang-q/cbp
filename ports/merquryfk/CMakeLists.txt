cmake_minimum_required(VERSION 3.15)
project(merquryfk C)

find_package(ZLIB REQUIRED)

set(SOURCES
    libfastk.c
)

set(TARGETS
    HAPmaker
    CNplot
    ASMplot
    HAPplot
    MerquryFK
    KatComp
    KatGC
    PloidyPlot
)

foreach(TARGET ${TARGETS})
    if("${TARGET}" STREQUAL "PloidyPlot")
        add_executable(${TARGET} ${TARGET}.c ${SOURCES} matrix.c)
    elseif("${TARGET}" STREQUAL "CNplot")
        add_executable(${TARGET} ${TARGET}.c cn_plotter.c ${SOURCES})
    elseif("${TARGET}" STREQUAL "ASMplot")
        add_executable(${TARGET} ${TARGET}.c asm_plotter.c ${SOURCES})
    elseif("${TARGET}" STREQUAL "HAPplot")
        add_executable(${TARGET} ${TARGET}.c hap_plotter.c ${SOURCES})
    elseif("${TARGET}" STREQUAL "MerquryFK")
        add_executable(${TARGET} ${TARGET}.c cn_plotter.c asm_plotter.c hap_plotter.c ${SOURCES})
    else()
        add_executable(${TARGET} ${TARGET}.c ${SOURCES})
    endif()

    target_link_libraries(${TARGET} PRIVATE
        m
        Threads::Threads
        ZLIB::ZLIB
    )

    target_compile_options(${TARGET} PRIVATE
        -O3 -Wall -Wextra -Wno-unused-result -fno-strict-aliasing
    )
endforeach()

find_package(Threads REQUIRED)

install(
    TARGETS ${TARGETS}
    RUNTIME DESTINATION tools/merquryfk
)
