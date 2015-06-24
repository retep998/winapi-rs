// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
#![cfg(windows)]
extern crate kernel32;
use kernel32::*;
#[inline(never)] fn bb<T>(_: T) {}
#[test]
fn functions() {
    bb(GetStartupInfoA);
    bb(GetStartupInfoW);
}
