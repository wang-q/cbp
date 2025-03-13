file(COPY "${CMAKE_CURRENT_LIST_DIR}/../../sources/bifrost.tar.gz" DESTINATION "${CURRENT_BUILDTREES_DIR}")

set(VCPKG_POLICY_EMPTY_INCLUDE_FOLDER enabled)

vcpkg_extract_source_archive(
    SOURCE_PATH
    ARCHIVE "${CURRENT_BUILDTREES_DIR}/bifrost.tar.gz"
)

# # Comment out all shared library related commands
# vcpkg_replace_string(
#     "${SOURCE_PATH}/src/CMakeLists.txt"
#     "add_library(bifrost_dynamic SHARED"
#     "# add_library(bifrost_dynamic SHARED"
# )

vcpkg_cmake_configure(
    SOURCE_PATH "${SOURCE_PATH}"
    OPTIONS
        -DMAX_KMER_SIZE=128
        "-DCMAKE_CXX_FLAGS=-Wno-unqualified-std-cast-call -std=c++11"
        "-DCMAKE_BUILD_TYPE=Release"
        "-DCMAKE_CXX_FLAGS_RELEASE=-O3"
        "-DBUILD_SHARED_LIBS=OFF"
)

vcpkg_cmake_install()

vcpkg_copy_tools(
    TOOL_NAMES Bifrost
    AUTO_CLEAN
)

file(REMOVE_RECURSE "${CURRENT_PACKAGES_DIR}/debug")

file(INSTALL "${SOURCE_PATH}/LICENSE" DESTINATION "${CURRENT_PACKAGES_DIR}/share/${PORT}" RENAME copyright)

configure_file("${CMAKE_CURRENT_LIST_DIR}/usage" "${CURRENT_PACKAGES_DIR}/share/${PORT}/usage" COPYONLY)
