use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .include("umac")
        .file("umac/umac.c")
        .file("umac/umac128.c")
        .file("umac/rijndael.c")
        .compile("libumac");

    bindgen::builder()
        .derive_default(true)
        .generate_comments(true)
        .header("umac/umac.h")
        .clang_arg("-Iumac")
        .allowlist_file("umac/umac.h")
        .generate()
        .expect("Failed to generate binding")
        .write_to_file(PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("umac.rs"))
        .expect("Failed to write binding");
}
