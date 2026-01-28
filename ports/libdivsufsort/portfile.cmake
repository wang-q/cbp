file(COPY "${CMAKE_CURRENT_LIST_DIR}/../../sources/libdivsufsort.tar.gz" DESTINATION "${CURRENT_BUILDTREES_DIR}")

set(VCPKG_POLICY_EMPTY_INCLUDE_FOLDER enabled)

vcpkg_extract_source_archive(
    SOURCE_PATH
    ARCHIVE "${CURRENT_BUILDTREES_DIR}/libdivsufsort.tar.gz"
)

vcpkg_cmake_configure(
    SOURCE_PATH "${SOURCE_PATH}"
    OPTIONS
        "-DBUILD_EXAMPLES=ON"
        "-DBUILD_DIVSUFSORT64=ON"
        "-DBUILD_SHARED_LIBS=OFF"
        "-DCMAKE_BUILD_TYPE=Release"
        "-DCMAKE_C_FLAGS_RELEASE=-O3"
)

vcpkg_cmake_install()

file(REMOVE_RECURSE "${CURRENT_PACKAGES_DIR}/debug")

file(INSTALL "${SOURCE_PATH}/COPYING" DESTINATION "${CURRENT_PACKAGES_DIR}/share/${PORT}" RENAME copyright)

configure_file("${CMAKE_CURRENT_LIST_DIR}/usage" "${CURRENT_PACKAGES_DIR}/share/${PORT}/usage" COPYONLY)
