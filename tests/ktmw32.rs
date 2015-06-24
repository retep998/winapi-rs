// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
#![cfg(windows)]
extern crate ktmw32;
use ktmw32::*;
#[inline(never)] fn bb<T>(_: T) {}
#[test]
fn functions() {
    bb(CommitTransaction);
    bb(CreateTransaction);
    bb(RollbackTransaction);
}
