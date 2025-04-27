def rust_cxx_bridge(
        name: str,
        src: str,
        deps: list[str] = [],
        include_directories: list[str] = [],
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
            # "include": ["${OUT}"]
        },
        cmd = "cxxbridge ${SRCS} -o ${OUT}/lib.rs.h -o ${OUT}/lib.rs.cc",
        type = "cxxbridge",
    )

    native.cxx_library(
        name = "%s/lib" % name,
        srcs = [":%s/source" % name],
        preferred_linkage = "static",
        exported_headers = [":%s/header" % name],
        include_directories = [":%s/header" % name] + include_directories,
        visibility = ["PUBLIC"],
        deps = deps,
    )

    # native.cxx_library(
    #     name = name,
    #     srcs = [":%s/source" % name],
    #     preferred_linkage = "static",
    #     exported_deps = deps + [":%s/include" % name],
    # )

    # native.cxx_library(
    #     name = "%s/include" % name,
    #     exported_headers = [":%s/header" % name],
    #     include_directories = [":%s/header" % name],
    # )