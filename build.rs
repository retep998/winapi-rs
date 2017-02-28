// Copyright © 2016 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use std::env::var;

const LIBS: &'static [&'static str] = &[
    "avrt",
    "advapi32",
    "bcrypt",
    "comctl32",
    "comdlg32",
    "credui",
    "crypt32",
    "d2d1",
    "d3d11",
    "d3d12",
    "d3d9",
    "d3dcompiler",
    "dbghelp",
    "dsound",
    "dwmapi",
    "dwrite",
    "dxgi",
    "dxguid",
    "gdi32",
    "hid",
    "httpapi",
    "kernel32",
    "ktmw32",
    "mpr",
    "ncrypt",
    "netapi32",
    "ntdll",
    "odbc32",
    "ole32",
    "oleaut32",
    "opengl32",
    "pdh",
    "psapi",
    "runtimeobject",
    "sapi",
    "secur32",
    "setupapi",
    "shcore",
    "shell32",
    "user32",
    "userenv",
    "usp10",
    "uuid",
    "version",
    "vssapi",
    "winhttp",
    "wininet",
    "winmm",
    "winscard",
    "winspool",
    "winusb",
    "ws2_32",
    "xinput",
];

fn main() {
    let target = var("TARGET").unwrap();
    let target: Vec<_> = target.split('-').collect();
    if target.get(2) == Some(&"windows") {
        link_libs();
    }
}

fn link_libs() {
    for lib in LIBS {
        if let Ok(_) = var(&format!("CARGO_FEATURE_{}", lib)) {
            println!("cargo:rustc-link-lib=dylib={}", lib);
        }
    }
}
