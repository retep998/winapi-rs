// Copyright © 2016 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use std::env::var;
const LIBS: &'static [&'static str] = &[
    "avrt",
    "kernel32",
    "ncrypt",
    "ole32",
    "setupapi",
    "wininet",
];
fn main() {
    let target = var("TARGET").unwrap();
    let target: Vec<_> = target.split('-').collect();
    if target.get(2) == Some(&"windows") {
        link_stuff();
    }
}
fn link_stuff() {
    for lib in LIBS {
        if let Ok(_) = var(&format!("CARGO_FEATURE_{}", lib)) {
            println!("cargo:rustc-link-lib=dylib={}", lib);
        }
    }
}
