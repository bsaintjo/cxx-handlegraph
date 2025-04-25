# A list of available rules and their signatures can be found here: https://buck2.build/docs/prelude/globals/
load("//tools:rust_cxx_bridge.bzl", "rust_cxx_bridge")

genrule(
    name = "hello_world",
    out = "out.txt",
    cmd = "echo BUILT BY BUCK2> $OUT",
)

rust_cxx_bridge(
    name = "rust_handle_graph",
    src = "src/lib.rs",
)

cxx_binary(
    name = "glue_main",
    srcs = ["cpp/src/main.cpp"],
    link_style = "static",
    deps = [
        ":libhandlegraph",
        ":rust_handle_graph",
    ],
)

cxx_library(
    name = "libhandlegraph",
    srcs = glob(["libhandlegraph/src/*.c"]),
    exported_headers = glob(["libhandlegraph/src/include/handlegraph/*.h"]),
    visibility = ["PUBLIC"],
)