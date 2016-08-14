// Copyright © 2016 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
#![cfg(windows)]
extern crate winapi;
use std::mem::{size_of, align_of};
#[cfg(target_arch = "x86")] #[test]
fn shared_guiddef() {
    use winapi::shared::guiddef::*;
    assert_eq!(size_of::<GUID>(), 16);
    assert_eq!(align_of::<GUID>(), 4);
}
#[cfg(target_arch = "x86_64")] #[test]
fn shared_guiddef() {
    use winapi::shared::guiddef::*;
    assert_eq!(size_of::<GUID>(), 16);
    assert_eq!(align_of::<GUID>(), 4);
}
#[cfg(target_arch = "x86")] #[test]
fn shared_minwindef() {
    use winapi::shared::minwindef::*;
    assert_eq!(size_of::<FILETIME>(), 8);
    assert_eq!(align_of::<FILETIME>(), 4);
}
#[cfg(target_arch = "x86_64")] #[test]
fn shared_minwindef() {
    use winapi::shared::minwindef::*;
    assert_eq!(size_of::<FILETIME>(), 8);
    assert_eq!(align_of::<FILETIME>(), 4);
}
#[cfg(target_arch = "x86")] #[test]
fn shared_windef() {
    use winapi::shared::windef::*;
    assert_eq!(size_of::<RECT>(), 16);
    assert_eq!(align_of::<RECT>(), 4);
    assert_eq!(size_of::<RECTL>(), 16);
    assert_eq!(align_of::<RECTL>(), 4);
    assert_eq!(size_of::<POINT>(), 8);
    assert_eq!(align_of::<POINT>(), 4);
    assert_eq!(size_of::<POINTL>(), 8);
    assert_eq!(align_of::<POINTL>(), 4);
    assert_eq!(size_of::<SIZE>(), 8);
    assert_eq!(align_of::<SIZE>(), 4);
    assert_eq!(size_of::<POINTS>(), 4);
    assert_eq!(align_of::<POINTS>(), 2);
}
#[cfg(target_arch = "x86_64")] #[test]
fn shared_windef() {
    use winapi::shared::windef::*;
    assert_eq!(size_of::<RECT>(), 16);
    assert_eq!(align_of::<RECT>(), 4);
    assert_eq!(size_of::<RECTL>(), 16);
    assert_eq!(align_of::<RECTL>(), 4);
    assert_eq!(size_of::<POINT>(), 8);
    assert_eq!(align_of::<POINT>(), 4);
    assert_eq!(size_of::<POINTL>(), 8);
    assert_eq!(align_of::<POINTL>(), 4);
    assert_eq!(size_of::<SIZE>(), 8);
    assert_eq!(align_of::<SIZE>(), 4);
    assert_eq!(size_of::<POINTS>(), 4);
    assert_eq!(align_of::<POINTS>(), 2);
}