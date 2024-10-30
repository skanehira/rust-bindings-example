#[cfg(feature = "cpp")]
fn main() {
    cxx_build::bridge("src/cpp.rs") // cxxブリッジが定義されているRustファイルを指定
        .flag_if_supported("-std=c++11") // 必要ならC++のバージョンを指定
        .compile("core"); // 出力するライブラリの名前

    println!("cargo:rerun-if-changed=src/cpp.rs");
}

// NOTE: cpp以外のfeaturesを使う場合はbuild.rsファイルがあるとmain関数が必要なので、空実装しておく
#[cfg(not(feature = "cpp"))]
fn main() {}
