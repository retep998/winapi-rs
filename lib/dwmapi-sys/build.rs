// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
use std::env::var;
fn main() {
    let target = var("TARGET").unwrap();
    let target: Vec<_> = target.split('-').collect();
    if target[2] == "windows" {
        println!("cargo:rustc-link-lib=dylib=dwmapi");
        if target[3] == "gnu" {
            let dir = var("CARGO_MANIFEST_DIR").unwrap();
            println!("cargo:rustc-link-search=native={}/{}", dir, target[0]);
        }
    }
}
