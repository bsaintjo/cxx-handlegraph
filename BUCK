# A list of available rules and their signatures can be found here: https://buck2.build/docs/prelude/globals/
load("//tools:rust_cxx_bridge.bzl", "rust_cxx_bridge")

genrule(
    name = "hello_world",
    out = "out.txt",
    cmd = "echo BUILT BY BUCK2> $OUT",
)

rust_cxx_bridge(
    name = "bridge",
    src = "src/lib.rs",
    deps = [":handlegraph"],
)

cxx_library(
    name = "rust_hash_graph",
    srcs = [
        "cpp/src/glue_hash_graph.cpp",
        "cpp/src/utils.cpp",
    ],
    public_include_directories = ["cpp/include"],
    exported_headers = [
        "cpp/include/rust_hash_graph/glue_hash_graph.hpp",
        "cpp/include/rust_hash_graph/utils.hpp",
    ],
    link_style = "static",
    visibility = ["PUBLIC"],
    exported_deps = [
        ":handlegraph",
        ":bridge",
    ],
)

cxx_binary(
    name = "glue_main",
    srcs = ["cpp/src/main.cpp"],
    link_style = "static",
    deps = [":rust_hash_graph"],
)

cxx_binary(
    name = "test",
    srcs = ["cpp/src/test.cpp"],
    link_style = "static",
    deps = [
        ":handlegraph",
        ":rust_hash_graph",
        ":bridge"
    ],
)


cxx_library(
    name = "handlegraph",
    srcs = glob(["libhandlegraph/src/*.cpp"]),
    exported_headers = glob(["libhandlegraph/src/include/handlegraph/**/*.hpp"]),
    public_include_directories = ["libhandlegraph/src/include"],
    visibility = ["PUBLIC"],
)
