file(COPY "${CMAKE_CURRENT_LIST_DIR}/../../sources/usalign.tar.gz" DESTINATION "${CURRENT_BUILDTREES_DIR}")

set(VCPKG_POLICY_EMPTY_INCLUDE_FOLDER enabled)

vcpkg_extract_source_archive(
    SOURCE_PATH
    ARCHIVE "${CURRENT_BUILDTREES_DIR}/usalign.tar.gz"
)

configure_file("${CMAKE_CURRENT_LIST_DIR}/CMakeLists.txt" "${SOURCE_PATH}/CMakeLists.txt" COPYONLY)

vcpkg_cmake_configure(
    SOURCE_PATH "${SOURCE_PATH}"
)

vcpkg_cmake_install()

file(REMOVE_RECURSE "${CURRENT_PACKAGES_DIR}/debug")

configure_file("${CMAKE_CURRENT_LIST_DIR}/usage" "${CURRENT_PACKAGES_DIR}/share/${PORT}/usage" COPYONLY)
