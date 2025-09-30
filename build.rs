// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use cmake::Config;

fn main() {
    use std::env;
    use std::fs;

    // Skip building the library when building documentation
    if env::var("DOCS_RS").is_ok() {
        return;
    }

    // Fix for https://github.com/bevyengine/bevy/issues/1110
    let target = env::var("TARGET").unwrap_or_default();
    let bevy_enabled = env::var("CARGO_FEATURE_BEVY_EXAMPLE").is_ok();

    if target == "x86_64-pc-windows-msvc" && bevy_enabled {
        let out_dir =
            std::path::PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join(".cargo");

        fs::create_dir_all(&out_dir).expect("Failed to create .cargo dir");

        let config_toml = out_dir.join("config.toml");

        fs::write(
            config_toml,
            r#"
[target.x86_64-pc-windows-msvc]
linker = "rust-lld.exe"
rustflags = ["-Zshare-generics=off"]
"#,
        )
        .expect("Failed to write config.toml");
    }

    let out_dir = std::env::var("OUT_DIR").unwrap();

    env::set_var("CMAKE_PREFIX_PATH", format!("{out_dir}/build/glm"));
    env::set_var("CMAKE_GENERATOR", "Ninja");
    env::set_var("CMAKE_BUILD_TYPE", "Release");

    let cxxflags = if cfg!(windows) { "/EHsc" } else { "" };

    let glm = Config::new("glm").cxxflag(cxxflags).build();
    println!("cargo:rustc-link-search=native={}", glm.display());

    Config::new("manifold")
        .cxxflag(cxxflags) //  MSVC flag to enable exception handling
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("CMAKE_INSTALL_LIBDIR", "lib")
        .define("MANIFOLD_CROSS_SECTION", "ON")
        .define("MANIFOLD_TEST", "OFF")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("MANIFOLD_CBIND", "OFF")
        .define("MANIFOLD_EXCEPTIONS", "OFF")
        .build();

    cxx_build::bridge("src/lib.rs")
        .std("c++17")
        .file("src/manifold_rs.cpp")
        .include("./src")
        .include("./manifold/src/manifold/include")
        .include("./manifold/src/utilities/include")
        .include(format!("{out_dir}/include"))
        .compile("manifold_rs");

    println!("cargo:rustc-link-search={out_dir}/lib");
    println!("cargo:rustc-link-lib=static=manifold");

    println!("cargo:rustc-link-search={out_dir}/build/_deps/clipper2-build");
    println!("cargo:rustc-link-lib=static=Clipper2");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/manifold_rs.h");
    println!("cargo:rerun-if-changed=src/manifold_rs.cpp");
}
