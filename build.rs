use std::path::PathBuf;

fn main() -> miette::Result<()> {
    // let includes = [
    //     PathBuf::from("libhandlegraph/src/include"),
    //     // "libhandlegraph/src/include/handlegraph",
    //     // "libhandlegraph/src/include/handlegraph/util",
    //     // "libhandlegraph/src/include/handlegraph/types",
    // ];
    // let mut builder = autocxx_build::Builder::new("src/lib.rs", &includes)
    //     .build()?;
        
    // builder
    //     .file("libhandlegraph/src/types.cpp")
    //     .flag_if_supported("-std=c++14")
    //     .compile("cxx-handlegraph");
    // println!("cargo:rerun-if-changed=src/lib.rs");
    // Ok(())
    // cxx_build::bridge("src/lib.rs")
    //     .include("libhandlegraph/src/include")
    //     .file("libhandlegraph/src/types.cpp")
    //     .std("c++14")
    //     .compile("cxx-handlegraph");
    // println!("cargo:rerun-if-changed=src/lib.rs");
    // println!("cargo:rerun-if-changed=libhandlegraph/src/types.cpp");
    // println!("cargo:rerun-if-changed=libhandlegraph/src/util.cpp");
    Ok(())
}
