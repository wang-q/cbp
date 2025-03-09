cmake_minimum_required(VERSION 3.15)
project(dazzdb C)

set(SOURCES
    DB.c
    QV.c
)

set(TARGETS
    fasta2DB
    DB2fasta
    DBsplit
    DBdust
    Catrack
    DBshow
    DBstats
    DBrm
    DBmv
    DBcp
    simulator
    fasta2DAM
    DAM2fasta
    DBwipe
    DBtrim
    DB2ONE
    rangen
)

foreach(TARGET ${TARGETS})
    if("${TARGET}" STREQUAL "rangen")
        add_executable(${TARGET} ${TARGET}.c)
    elseif("${TARGET}" STREQUAL "DB2ONE")
        add_executable(${TARGET} ${TARGET}.c ${SOURCES} ONElib.c)
        target_link_libraries(${TARGET} PRIVATE m)
    elseif("${TARGET}" STREQUAL "DBcp")
        add_executable(${TARGET} DBmv.c ${SOURCES})
        target_link_libraries(${TARGET} PRIVATE m)
    elseif("${TARGET}" STREQUAL "DBmv")
        add_executable(${TARGET} DBmv.c ${SOURCES})
        target_compile_definitions(${TARGET} PRIVATE MOVE)
        target_link_libraries(${TARGET} PRIVATE m)
    else()
        add_executable(${TARGET} ${TARGET}.c ${SOURCES})
        target_link_libraries(${TARGET} PRIVATE m)
    endif()

    target_compile_options(${TARGET} PRIVATE
        -O3 -Wall -Wextra -Wno-unused-result -fno-strict-aliasing
    )
endforeach()

install(
    TARGETS ${TARGETS}
    RUNTIME DESTINATION tools/dazzdb
)
