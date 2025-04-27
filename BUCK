# A list of available rules and their signatures can be found here: https://buck2.build/docs/prelude/globals/
load("//tools:rust_cxx_bridge.bzl", "rust_cxx_bridge")

genrule(
    name = "hello_world",
    out = "out.txt",
    cmd = "echo BUILT BY BUCK2> $OUT",
)

genrule(
    name = "cxx_build",
    cmd = "cargo build --release --target-dir $OUT",
    outs = {
        "src-file": ["cxxbridge/cxx-handlegraph/src/lib.rs.cc"],
        "src-dir": ["cxxbridge/cxx-handlegraph/src"],
        "header": ["cxxbridge/cxx-handlegraph/src/lib.rs.h"],
    }
)

rust_binary(
    name = "test_bin",
    srcs = [
        "src/main.rs",
    ],
    edition = "2021",
    visibility = ["PUBLIC"],
    deps = [
        ":libhandlegraph",
    ],
)

rust_cxx_bridge(
    name = "bridge",
    srcs = [
        "src/lib.rs",
    ],
)


cxx_library(
    name = "rust_handle_graph",
    srcs = [
        ":cxx_build[src-file]",
        "cpp/src/glue_hash_graph.cpp",
        "cpp/src/utils.cpp",
    ],
    include_directories = [
        ":cxx_build[src-dir]",
        "cpp/include",
        "cpp/include/rust_hash_graph",
        "libhandlegraph/src/include",
    ],
    headers = [
        ":cxx_build[header]",
    ],
    exported_headers = [
        "cpp/include/rust_hash_graph/glue_hash_graph.hpp",
        "cpp/include/rust_hash_graph/utils.hpp",
    ],
    header_namespace = "rust_handle_graph",
    visibility = ["PUBLIC"],
    deps = [
        ":cxx_build",
        ":libhandlegraph",
    ],
)

cxx_binary(
    name = "glue_main",
    srcs = ["cpp/src/test.cpp"],
    link_style = "static",
    deps = [
        ":libhandlegraph",
        ":rust_handle_graph",
    ],
)

cxx_library(
    name = "libhandlegraph",
    srcs = glob(["libhandlegraph/src/*.cpp"]),
    exported_headers = glob(["libhandlegraph/src/include/handlegraph/*.hpp"]) + glob(["libhandlegraph/src/include/handlegraph/algorithms/*.hpp"]),
    include_directories = [
        "libhandlegraph/src/include",
    #     "libhandlegraph/src/include/handlegraph",
    #     "libhandlegraph/src/include/handlegraph/algorithms",
    ],
    # header_namespace = "handlegraph",
    visibility = ["PUBLIC"],
)
