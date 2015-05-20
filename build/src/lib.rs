// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
pub fn link(name: &str, bundled: bool) {
    use std::env::var;
    let target = var("TARGET").unwrap();
    let target: Vec<_> = target.split('-').collect();
    if target[2] == "windows" {
        println!("cargo:rustc-link-lib=dylib={}", name);
        if bundled && target[3] == "gnu" {
            let dir = var("CARGO_MANIFEST_DIR").unwrap();
            println!("cargo:rustc-link-search=native={}/{}", dir, target[0]);
        }
    }
}
