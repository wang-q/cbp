cmake_minimum_required(VERSION 3.15)
project(multiz C)

find_package(ZLIB REQUIRED)

set(SOURCES
    util.c
    multi_util.c
    maf.c
    mz_scores.c
    mz_yama.c
    mz_preyama.c
    multiz.c
)

add_executable(multiz ${SOURCES})

target_link_libraries(multiz PRIVATE ZLIB::ZLIB)
target_include_directories(multiz PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})

target_compile_options(multiz PRIVATE
    $<$<C_COMPILER_ID:MSVC>:/W3 /O2>
    $<$<NOT:$<C_COMPILER_ID:MSVC>>:-Wall -O3 -fcommon>
)

if(WIN32)
    set_target_properties(multiz
        PROPERTIES
        SUFFIX ".exe"
    )
endif()

install(TARGETS multiz
    RUNTIME DESTINATION bin
)
