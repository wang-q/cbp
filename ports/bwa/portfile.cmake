file(COPY "${CMAKE_CURRENT_LIST_DIR}/../../sources/bwa.tar.gz" DESTINATION "${CURRENT_BUILDTREES_DIR}")

vcpkg_extract_source_archive(
    SOURCE_PATH
    ARCHIVE "${CURRENT_BUILDTREES_DIR}/bwa.tar.gz"
)

# Copy CMakeLists.txt to source directory
configure_file("${CMAKE_CURRENT_LIST_DIR}/CMakeLists.txt" "${SOURCE_PATH}/CMakeLists.txt" COPYONLY)

vcpkg_cmake_configure(
    SOURCE_PATH "${SOURCE_PATH}"
    OPTIONS
        "-DCMAKE_C_FLAGS=-O2"
        "-DCMAKE_EXE_LINKER_FLAGS=-static"
    OPTIONS_RELEASE
        "-DCMAKE_BUILD_TYPE=Release"
)

vcpkg_cmake_install()
vcpkg_copy_pdbs()
vcpkg_copy_tools(TOOL_NAMES bwa AUTO_CLEAN)

file(REMOVE_RECURSE "${CURRENT_PACKAGES_DIR}/debug")

file(INSTALL "${SOURCE_PATH}/COPYING" DESTINATION "${CURRENT_PACKAGES_DIR}/share/${PORT}" RENAME copyright)

configure_file("${CMAKE_CURRENT_LIST_DIR}/usage" "${CURRENT_PACKAGES_DIR}/share/${PORT}/usage" COPYONLY)
