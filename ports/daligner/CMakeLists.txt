cmake_minimum_required(VERSION 3.15)
project(daligner C)

set(SOURCES
    DB.c
    QV.c
)

set(TARGETS
    daligner
    HPC.daligner
    LAsort
    LAmerge
    LAsplit
    LAcat
    LAshow
    LA2ONE
    LAcheck
    ONE2LA
)

foreach(TARGET ${TARGETS})
    if("${TARGET}" STREQUAL "daligner")
        add_executable(${TARGET} ${TARGET}.c filter.c lsd.sort.c align.c ${SOURCES})
        target_link_libraries(${TARGET} PRIVATE m Threads::Threads)
    elseif("${TARGET}" STREQUAL "LA2ONE" OR "${TARGET}" STREQUAL "ONE2LA")
        add_executable(${TARGET} ${TARGET}.c align.c ${SOURCES} ONElib.c)
        target_link_libraries(${TARGET} PRIVATE m)
    elseif("${TARGET}" STREQUAL "LAshow" OR "${TARGET}" STREQUAL "LAcheck")
        add_executable(${TARGET} ${TARGET}.c align.c ${SOURCES})
        target_link_libraries(${TARGET} PRIVATE m)
    else()
        add_executable(${TARGET} ${TARGET}.c ${SOURCES})
        target_link_libraries(${TARGET} PRIVATE m)
    endif()

    target_compile_options(${TARGET} PRIVATE
        -O3 -Wall -Wextra -Wno-unused-result -fno-strict-aliasing
    )
endforeach()

find_package(Threads REQUIRED)

install(
    TARGETS ${TARGETS}
    RUNTIME DESTINATION tools/daligner
)
