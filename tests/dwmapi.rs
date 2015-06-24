// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
#![cfg(windows)]
extern crate dwmapi;
use dwmapi::*;
#[inline(never)] fn bb<T>(_: T) {}
#[test]
fn functions() {
    bb(DwmEnableBlurBehindWindow);
}
