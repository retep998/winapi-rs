// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
#![feature(test)]
#![cfg(windows)]
extern crate kernel32;
extern crate test;
use kernel32::*;
use test::black_box as bb;
#[test]
fn functions() {
    bb(GetStartupInfoA);
    bb(GetStartupInfoW);
}
