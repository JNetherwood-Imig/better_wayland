use std::path::PathBuf;

fn main() {
    let src_dir = PathBuf::from("../../../src");
    let include_dir = PathBuf::from("../../../include");

    cc::Build::new()
        .include(include_dir)
        .file(src_dir.join("display.c"))
        .std("c23")
        .compile("denali");
}
