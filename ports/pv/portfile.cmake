file(COPY "${CMAKE_CURRENT_LIST_DIR}/../../sources/pv.tar.gz" DESTINATION "${CURRENT_BUILDTREES_DIR}")

set(VCPKG_POLICY_EMPTY_INCLUDE_FOLDER enabled)

vcpkg_extract_source_archive(
    SOURCE_PATH
    ARCHIVE "${CURRENT_BUILDTREES_DIR}/pv.tar.gz"
)

vcpkg_configure_make(
    SOURCE_PATH "${SOURCE_PATH}"
    NO_ADDITIONAL_PATHS
    OPTIONS
        --disable-dependency-tracking
        --disable-silent-rules
    CFLAGS=-Wno-implicit-function-declaration
)

vcpkg_install_make(
    INSTALL_TARGET install
)

vcpkg_copy_tools(
    TOOL_NAMES pv
    AUTO_CLEAN
)

file(REMOVE_RECURSE "${CURRENT_PACKAGES_DIR}/debug")

file(INSTALL "${SOURCE_PATH}/docs/COPYING" DESTINATION "${CURRENT_PACKAGES_DIR}/share/${PORT}" RENAME copyright)

configure_file("${CMAKE_CURRENT_LIST_DIR}/usage" "${CURRENT_PACKAGES_DIR}/share/${PORT}/usage" COPYONLY)
