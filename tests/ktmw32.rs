// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
#![feature(test)]
#![cfg(windows)]
extern crate ktmw32;
extern crate test;
use ktmw32::*;
use test::black_box as bb;
#[test]
fn functions() {
    bb(CommitTransaction);
    bb(CreateTransaction);
    bb(RollbackTransaction);
}
