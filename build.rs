// Copyright © 2016 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
extern crate toml;

use std::env::var;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use toml::Value;

fn main() {
    let target = var("TARGET").unwrap();
    let target: Vec<_> = target.split('-').collect();
    if target.get(2) == Some(&"windows") {
        link_libs();
    }
}

fn link_libs() {
    for lib in collect_libs() {
        if let Ok(_) = var(&format!("CARGO_FEATURE_{}", lib)) {
            println!("cargo:rustc-link-lib=dylib={}", lib);
        }
    }
}

fn collect_libs() -> Vec<String> {
    let manifest_dir = PathBuf::from(var("CARGO_MANIFEST_DIR").unwrap());
    let name = manifest_dir.join("Cargo.toml");

    let mut text = String::new();
    File::open(&name)
        .and_then(|mut file| file.read_to_string(&mut text))
        .unwrap();

    let toml: Value = text.parse().unwrap();
    if let Value::Table(table) = toml {
        let features = table.get("features").unwrap();
        if let &Value::Array(ref libraries) = features.get("libraries").unwrap() {
            let mut libs = Vec::new();
            for value in libraries.into_iter() {
                if let &Value::String(ref string) = value {
                    libs.push(string.clone());
                }
            }
            libs
        } else {
            panic!("Expected array");
        }
    } else {
        panic!("Expected table");
    }
}
