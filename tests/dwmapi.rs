// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
#![feature(test)]
#![cfg(windows)]
extern crate dwmapi;
extern crate test;
use dwmapi::*;
use test::black_box as bb;
#[test]
fn functions() {
    bb(DwmEnableBlurBehindWindow);
}
