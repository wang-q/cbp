file(COPY "${CMAKE_CURRENT_LIST_DIR}/../../sources/diamond.tar.gz" DESTINATION "${CURRENT_BUILDTREES_DIR}")

set(VCPKG_POLICY_EMPTY_INCLUDE_FOLDER enabled)

vcpkg_extract_source_archive(
    SOURCE_PATH
    ARCHIVE "${CURRENT_BUILDTREES_DIR}/diamond.tar.gz"
)

vcpkg_cmake_configure(
    SOURCE_PATH "${SOURCE_PATH}"
    OPTIONS
)

vcpkg_cmake_install()

vcpkg_copy_tools(
    TOOL_NAMES diamond
    AUTO_CLEAN
)

file(REMOVE_RECURSE "${CURRENT_PACKAGES_DIR}/debug")

file(INSTALL "${SOURCE_PATH}/LICENSE" DESTINATION "${CURRENT_PACKAGES_DIR}/share/${PORT}" RENAME copyright)

configure_file("${CMAKE_CURRENT_LIST_DIR}/usage" "${CURRENT_PACKAGES_DIR}/share/${PORT}/usage" COPYONLY)
