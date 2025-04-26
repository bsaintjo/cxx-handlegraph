# A list of available rules and their signatures can be found here: https://buck2.build/docs/prelude/globals/
load("//tools:rust_cxx_bridge.bzl", "rust_cxx_bridge")

genrule(
    name = "hello_world",
    out = "out.txt",
    cmd = "echo BUILT BY BUCK2> $OUT",
)

genrule(
    name = "cxx-build",
    out = "target",
    cmd = "cargo build --release --target-dir $OUT && ls $OUT",
)

rust_binary(
    name = "test_bin",
    srcs = [
        "src/main.rs",
    ],
    edition = "2021",
    visibility = ["PUBLIC"],
    deps = [
        ":bridge",
        ":libhandlegraph",
    ],
)


cxx_library(
    name = "rust_handle_graph",
    srcs = [
        "cpp/src/glue_hash_graph.cpp",
        "cpp/src/utils.cpp",
    ],
    include_directories = [
        "cpp/include",
        "cpp/include/rust_hash_graph",
    ],
    exported_headers = [
        "cpp/include/rust_hash_graph/glue_hash_graph.hpp",
        "cpp/include/rust_hash_graph/utils.hpp"
    ],
    header_namespace = "rust_handle_graph",
    visibility = ["PUBLIC"],
    deps = [
        ":bridge/include",
        ":bridge/header",
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
        ":bridge/include",
        ":bridge/header",
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
