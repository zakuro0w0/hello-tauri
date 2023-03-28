fn main() {
    // C++言語のコードをビルドする設定
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++20")
        // .warnings(true)
        // .flag("-Wall")
        // .flag("-Wextra")
        // .flag("-v")
        // .flag("-g")
        .file("src/ffi.cpp")
        .compile("ffi_cpp");
    tauri_build::build()
}
