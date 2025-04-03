cmake_minimum_required(VERSION 3.15)
project(usalign CXX)

set(TARGETS
    qTMclust
    USalign
    TMalign
    TMscore
    MMalign
    se
    pdb2xyz
    xyz_sfetch
    pdb2fasta
    pdb2ss
    NWalign
    HwRMSD
    cif2pdb
    pdbAtomName
    addChainID
)

foreach(TARGET ${TARGETS})
    add_executable(${TARGET} ${TARGET}.cpp)
    target_compile_options(${TARGET} PRIVATE
        -O3
        -ffast-math
        -fno-exceptions        # Disable exception handling
        -fno-rtti             # Disable runtime type information
        -fno-unwind-tables    # Disable stack unwinding tables
        -fno-asynchronous-unwind-tables  # Disable async stack unwinding tables
    )
endforeach()

install(
    TARGETS ${TARGETS}
    RUNTIME DESTINATION tools/usalign
)
