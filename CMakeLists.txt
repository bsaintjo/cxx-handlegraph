cmake_minimum_required(VERSION 3.22)
project(
        GlueHandleGraph
        VERSION 0.1.0
        LANGUAGES CXX
        DESCRIPTION "C++/Rust bridge for HandleGraph"
)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(CARGO_MANIFEST ${CMAKE_SOURCE_DIR}/Cargo.toml)
set(CARGO_TARGET_DIR ${CMAKE_SOURCE_DIR}/target)

set(GLUE_HANDLE_GRAPH_SOURCE_FILE ${CMAKE_SOURCE_DIR}/src/lib.rs)
set(GLUE_HANDLE_GRAPH_BRIDGE_CPP ${CARGO_TARGET_DIR}/cxxbridge/cxx-handlegraph/src/lib.rs.cc)
set(GLUE_HANDLE_GRAPH_LIB ${CARGO_TARGET_DIR}/debug/${CMAKE_STATIC_LIBRARY_PREFIX}cxx_handlegraph${CMAKE_STATIC_LIBRARY_SUFFIX})

# Add a custom command that builds the rust crate and generates C++ bridge code
add_custom_command(
        OUTPUT ${GLUE_HANDLE_GRAPH_BRIDGE_CPP} ${GLUE_HANDLE_GRAPH_LIB}
        COMMAND cargo build --manifest-path ${CARGO_MANIFEST}
        DEPENDS ${GLUE_HANDLE_GRAPH_SOURCE_FILE}
        USES_TERMINAL
        COMMENT "Running cargo..."
)
add_subdirectory("${CMAKE_SOURCE_DIR}/libhandlegraph")

# Add executable
add_executable(
        glue_main
        cpp/src/main.cpp
        cpp/src/glue_hash_graph.cpp
        cpp/src/utils.cpp
        ${GLUE_HANDLE_GRAPH_BRIDGE_CPP}
)


add_library(
        glue_hash_graph
        STATIC
        cpp/src/glue_hash_graph.cpp
        cpp/include/rust_hash_graph/glue_hash_graph.hpp
)

target_include_directories(
    glue_main
    PRIVATE
    cpp/include/
    ${CARGO_TARGET_DIR}/cxxbridge/cxx-handlegraph/src/
    ${CARGO_TARGET_DIR}/cxxbridge
)

target_include_directories(
    glue_hash_graph
    PUBLIC
    cpp/include/
)

target_include_directories(
    glue_hash_graph
    PRIVATE
    ${CARGO_TARGET_DIR}/cxxbridge/cxx-handlegraph/src/
    ${CARGO_TARGET_DIR}/cxxbridge
)

target_link_libraries(glue_main PRIVATE ${GLUE_HANDLE_GRAPH_LIB} handlegraph_shared)
target_link_libraries(glue_hash_graph PRIVATE ${GLUE_HANDLE_GRAPH_LIB} handlegraph_shared)

set_target_properties(
        glue_main
        PROPERTIES ADDITIONAL_CLEAN_FILES ${CARGO_TARGET_DIR}
)