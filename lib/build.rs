#[cfg(feature = "cpp")]
extern crate cbindgen;

#[cfg(feature = "cpp")]
fn main() {
    cbindgen::Builder::new()
        .with_crate(".")
        .generate()
        .unwrap()
        .write_to_file("../cpp/core.h");
}

#[cfg(not(feature = "cpp"))]
fn main() {}
