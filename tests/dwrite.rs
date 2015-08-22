// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
#![cfg(windows)]
extern crate dwrite;
use dwrite::*;
#[inline(never)] fn bb<T>(_: T) {}
#[test]
fn functions() {
    bb(DWriteCreateFactory);
}
