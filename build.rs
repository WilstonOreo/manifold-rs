// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use cmake::Config;

fn main() {
    // Skip building the library when building documentation
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }

    let out_dir = std::env::var("OUT_DIR").unwrap();

    std::env::set_var("CMAKE_PREFIX_PATH", format!("{out_dir}/build/glm"));
    std::env::set_var("CMAKE_GENERATOR", "Ninja");
    std::env::set_var("CMAKE_BUILD_TYPE", "Release");

    let cxxflags = if cfg!(windows) { "/EHsc" } else { "" };

    let glm = Config::new("glm").cxxflag(cxxflags).build();
    println!("cargo:rustc-link-search=native={}", glm.display());

    Config::new("manifold")
        .cxxflag(cxxflags) //  MSVC flag to enable exception handling
        .define("CMAKE_BUILD_TYPE", "Release")
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
