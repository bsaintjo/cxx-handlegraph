fn main() {
    cxx_build::bridge("src/lib.rs")
        .include("libhandlegraph/src/include")
        .file("libhandlegraph/src/types.cpp")
        .std("c++14")
        .compile("cxx-handlegraph");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=libhandlegraph/src/types.cpp");
    println!("cargo:rerun-if-changed=libhandlegraph/src/util.cpp");
}
