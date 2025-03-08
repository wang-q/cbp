cmake_minimum_required(VERSION 3.15)
project(consel C)

set(TARGETS
    makermt
    catpv
    catci
    catrep
    treeass
    consel
    randrep
    catmt
    seqmt
    makerep
    catass
)

set(SOURCES
    src/misc.c
    src/rand.c
    src/tree.c
    src/freadmat.c
    src/opt.c
    src/mt19937.c
)

foreach(TARGET ${TARGETS})
    add_executable(${TARGET} src/${TARGET}.c)

    target_compile_options(${TARGET} PRIVATE
        $<$<AND:$<C_COMPILER_ID:MSVC>,$<CONFIG:Release>>:/O2>
        $<$<C_COMPILER_ID:MSVC>:/W3>
        $<$<NOT:$<C_COMPILER_ID:MSVC>>:-fcommon -Wno-unused-result>
    )

    if(WIN32)
        set_target_properties(${TARGET}
            PROPERTIES
            SUFFIX ".exe"
        )
    endif()
endforeach()

# Add source files to specific targets
target_sources(makermt PRIVATE src/misc.c src/rand.c src/freadmat.c src/mt19937.c)
target_sources(catpv PRIVATE src/misc.c)
target_sources(treeass PRIVATE src/tree.c src/misc.c)
target_sources(consel PRIVATE src/misc.c src/rand.c src/opt.c src/mt19937.c)
target_sources(catmt PRIVATE src/misc.c src/rand.c src/mt19937.c)
target_sources(seqmt PRIVATE src/freadmat.c src/misc.c)
target_sources(makerep PRIVATE src/misc.c src/rand.c src/freadmat.c src/mt19937.c)
target_sources(catci PRIVATE src/misc.c)
target_sources(catrep PRIVATE src/misc.c)
target_sources(catass PRIVATE src/misc.c)
target_sources(randrep PRIVATE src/misc.c src/rand.c src/mt19937.c)

target_link_libraries(makermt PRIVATE m)
target_link_libraries(consel PRIVATE m)
target_link_libraries(catmt PRIVATE m)
target_link_libraries(makerep PRIVATE m)
target_link_libraries(randrep PRIVATE m)

install(TARGETS ${TARGETS}
    RUNTIME DESTINATION bin
)
