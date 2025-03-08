file(COPY "${CMAKE_CURRENT_LIST_DIR}/../../sources/clustalo.tar.gz" DESTINATION "${CURRENT_BUILDTREES_DIR}")

set(VCPKG_POLICY_EMPTY_INCLUDE_FOLDER enabled)

vcpkg_extract_source_archive(
    SOURCE_PATH
    ARCHIVE "${CURRENT_BUILDTREES_DIR}/clustalo.tar.gz"
)

vcpkg_configure_make(
    SOURCE_PATH "${SOURCE_PATH}"
    NO_ADDITIONAL_PATHS
    OPTIONS
        --disable-dependency-tracking
)

vcpkg_install_make(
    INSTALL_TARGET install
)

vcpkg_copy_tools(
    TOOL_NAMES clustalo
    AUTO_CLEAN
)

file(REMOVE_RECURSE "${CURRENT_PACKAGES_DIR}/debug")

file(INSTALL "${SOURCE_PATH}/COPYING" DESTINATION "${CURRENT_PACKAGES_DIR}/share/${PORT}" RENAME copyright)

configure_file("${CMAKE_CURRENT_LIST_DIR}/usage" "${CURRENT_PACKAGES_DIR}/share/${PORT}/usage" COPYONLY)
