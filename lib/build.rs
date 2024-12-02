#[cfg(any(feature = "cpp", feature = "go"))]
extern crate cbindgen;

#[cfg(feature = "cpp")]
fn main() {
    cbindgen::Builder::new()
        .with_crate(".")
        .generate()
        .unwrap()
        .write_to_file("../cpp/core.h");
}

#[cfg(feature = "go")]
fn main() {
    cbindgen::Builder::new()
        .with_crate(".")
        .with_language(cbindgen::Language::C)
        .generate()
        .unwrap()
        .write_to_file("../go/core.h");
}

#[cfg(any(feature = "python", feature = "wasm"))]
fn main() {}
