# winapi-rs [![Build status](https://ci.appveyor.com/api/projects/status/i47oonf5e7qm5utq/branch/master?svg=true)](https://ci.appveyor.com/project/retep998/winapi-rs/branch/master) [![Build Status](https://travis-ci.org/retep998/winapi-rs.svg?branch=master)](https://travis-ci.org/retep998/winapi-rs) [![Gitter](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/retep998/winapi-rs) [![Crates.io](https://img.shields.io/crates/v/winapi.svg)](https://crates.io/crates/winapi) ![Lines of Code](https://tokei.rs/b1/github/retep998/winapi-rs) ![100% unsafe](https://img.shields.io/badge/unsafe-100%25-blue.svg) #

[Documentation](https://retep998.github.io/doc/winapi/)

Official IRC channel: #winapi on [Mozilla IRC](https://wiki.mozilla.org/IRC)

This crate provides raw FFI bindings to all of Windows API. They are gathered by hand using the Windows 10 SDK from Microsoft. I aim to replace all existing Windows FFI in other crates with this crate through the "[Embrace, extend, and extinguish](http://en.wikipedia.org/wiki/Embrace,_extend_and_extinguish)" technique.

If this crate is missing something you need, feel free to create an issue, open a pull request, or contact me via [other means](http://www.rustaceans.org/retep998).

This crate depends on Rust 1.6 or newer on Windows. On other platforms this crate is a no-op and should compile with Rust 1.2 or newer.

This branch is for winapi 0.3 which is a work in progress rewrite and as such is not suitable for production use. Please use winapi 0.2 in the meantime.

## Example ##

Cargo.toml:
```toml
[target.'cfg(windows)'.dependencies]
winapi = "0.3"
```
main.rs:
```Rust
#[cfg(windows)] extern crate winapi;
use std::io::Error;

#[cfg(windows)]
fn print_message(msg: &str) -> Result<i32, Error> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    use winapi::um::winuser::{MB_OK, MessageBoxW};
    let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
    let ret = unsafe {
        MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK)
    };
    if ret == 0 { Err(Error::last_os_error()) }
    else { Ok(ret) }
}
#[cfg(not(windows))]
fn print_message(msg: &str) -> Result<(), Error> {
    println!("{}", msg);
    Ok(())
}
fn main() {
    print_message("Hello, world!").unwrap();
}
```
