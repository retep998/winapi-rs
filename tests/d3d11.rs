// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
#![cfg(windows)]
extern crate d3d11;
use d3d11::*;
#[inline(never)] fn bb<T>(_: T) {}
#[test]
fn functions() {
    bb(D3D11CreateDevice);
    bb(D3D11CreateDeviceAndSwapChain);
}
