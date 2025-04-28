def rust_cxx_bridge(
        name: str,
        src: str,
        deps: list[str] = [],
):
    src_export = src.rsplit("/")[-1]

    native.export_file(
        name = "%s/header" % name,
        src = ":%s/generated[generated.h]" % name,
        out = src_export + ".h",
    )

    native.export_file(
        name = "%s/source" % name,
        src = ":%s/generated[generated.cc]" % name,
        out = src + ".cc",
    )

    native.genrule(
        name = "%s/generated" % name,
        srcs = [src],
        outs = {
            "generated.cc": ["lib.rs.cc"],
            "generated.h": ["lib.rs.h"],
        },
        cmd = "cxxbridge ${SRCS} -o ${OUT}/lib.rs.h -o ${OUT}/lib.rs.cc",
        type = "cxxbridge",
    )

    native.export_file(
        name = "%s/cargo/src" % name,
        src = ":%s/cargo[src-file]" % name,
        out = src + ".cc",
    )

    native.export_file(
        name = "%s/cargo/header" % name,
        src = ":%s/cargo[header]" % name,
        out = src_export + ".h",
    )

    native.cxx_library(
        name = name,
        exported_headers = [":%s/header" % name],
        include_directories = [":%s/header" % name],
        exported_linker_flags = [
            "-L$(location :bridge/cargo[archive-dir])",
            "-lcxx_handlegraph",
            # "$(location :bridgecargo[archive])",
        ],
    )

    native.genrule(
        name = "%s/cargo" % name,
        cmd = "cargo build --release --target-dir $OUT",
        outs = {
            "source": ["cxxbridge/cxx-handlegraph/src/lib.rs.cc"],
            "header": ["cxxbridge/cxx-handlegraph/src/lib.rs.h"],
            "archive-dir": ["release"],
            # "archive": ["release/libcxx-handlegraph.a"],
        }
    )

    # native.cxx_library(
    #     name = name,
    #     srcs = [":%s/source" % name],
    #     preferred_linkage = "static",
    #     exported_headers = [":%s/header" % name],
    #     include_directories = [":%s/header" % name] + include_directories,
    #     exported_linker_flags = [
    #         "-L{}".format(":%s/cargo/archive-dir" % name),
    #         "-lcxx-handlegraph",
    #     ],
    #     visibility = ["PUBLIC"],
    #     deps = deps + [":%s/cargo" % name],
    # )

    # native.cxx_library(
    #     name = name,
    #     srcs = [":%s/source" % name],
    #     preferred_linkage = "static",
    #     exported_deps = deps + [":%s/include" % name],
    # )
