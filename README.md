# winapi-rs [![Build status](https://ci.appveyor.com/api/projects/status/i47oonf5e7qm5utq/branch/master?svg=true)](https://ci.appveyor.com/project/retep998/winapi-rs/branch/master) [![Build Status](https://travis-ci.org/retep998/winapi-rs.svg?branch=master)](https://travis-ci.org/retep998/winapi-rs) [![Gitter](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/retep998/winapi-rs) [![Crates.io](https://img.shields.io/crates/v/winapi.svg)](https://crates.io/crates/winapi) #

[Documentation](https://retep998.github.io/doc/winapi/)

Official IRC channel: #winapi on [Mozilla IRC](https://wiki.mozilla.org/IRC)

This crate provides raw FFI bindings to all of Windows API. They are gathered by hand using the Windows 10 SDK from Microsoft. I aim to replace all existing Windows FFI in other crates with this crate through the "[Embrace, extend, and extinguish](http://en.wikipedia.org/wiki/Embrace,_extend_and_extinguish)" technique.

If this crate is missing something you need, feel free to create an issue, open a pull request, or contact me via [other means](http://www.rustaceans.org/retep998).

This crate depends on Rust 1.6 on Windows. On other platforms this crate is a no-op and should compile with Rust 1.0.

This branch is for winapi 0.3 which is a work in progress rewrite and as such is not suitable for production use. Please use winapi 0.2 in the meantime.

## Example ##

Cargo.toml:
```toml
[dependencies]
winapi = "0.2"
user32-sys = "0.2"
```
main.rs:
```Rust
extern crate winapi;
extern crate user32;
use std::ffi::OsStr;
use std::io::Error;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;

fn main() {
    let msg = "Hello, world!";
    let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
    let ret = unsafe {
        user32::MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), winapi::MB_OK)
    };
    if ret == 0 {
        println!("Failed: {:?}", Error::last_os_error());
    }
}
```
