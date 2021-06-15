fn main() {
    // for (key, value) in std::env::vars() {
    //     println!("cargo:warning={}={}", key, value);
    // }
    let imgui_headers = format!(
        "{}/imgui",
        std::env::var("DEP_IMGUI_THIRD_PARTY").expect("Missing headers for imgui")
    );
    cxx_build::bridge("src/lib.rs")
        .file("src/imgui-terminal-rs.cc")
        .file("third_party/imgui-terminal/src/Terminal.cc")
        .file("third_party/imgui-terminal/src/Terminal_unity.cc")
        .includes(["src", "third_party/imgui-terminal/include", imgui_headers.as_str()])
        .compile("imgui-terminal");
    println!("cargo:rustc-link-lib=cimgui");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/imgui-terminal-rs.cc");
    println!("cargo:rerun-if-changed=src/imgui-terminal-rs.h");
    println!("cargo:rerun-if-changed=third_party/imgui-terminal/src/Terminal.cc");
    println!("cargo:rerun-if-changed=third_party/imgui-terminal/src/Terminal_unity.cc");
}
