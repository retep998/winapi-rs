// Copyright © 2015, Dmitry Roschin
// Licensed under the MIT License <LICENSE.md>
#![cfg(windows)]
extern crate dxgi_sys;
use dxgi_sys::*;
#[inline(never)] fn bb<T>(_: T) {}
#[test]
fn functions() {
    bb(CreateDXGIFactory);
    bb(CreateDXGIFactory1);
    bb(CreateDXGIFactory2);
    bb(DXGIGetDebugInterface1);
}
#[cfg(target_env = "msvc")]
#[test]
fn msvc_functions() {
    bb(CreateDXGIFactory);
    bb(CreateDXGIFactory1);
    bb(CreateDXGIFactory2);
    bb(DXGIGetDebugInterface1);
}
