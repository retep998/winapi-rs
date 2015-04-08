// The MIT License (MIT)
//
// Copyright (c) 2015 Johan Johansson, Peter Atashian
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

//! FFI bindings to dxgi.

#![cfg(windows)]

#[macro_use]
extern crate winapi;

use winapi::{ REFIID, HRESULT, UINT, c_void };
pub use constants::*;
pub use enumerations::*;
pub use structures::*;
pub use interfaces::*;

mod macros;
pub mod structures;
pub mod enumerations;
pub mod constants;
pub mod interfaces;

#[link(name = "dxgi")]
extern "system" {
    pub fn CreateDXGIFactory(riid: REFIID, ppFactory: *mut *mut c_void) -> HRESULT;
    pub fn CreateDXGIFactory1(riid: REFIID, ppFactory: *mut *mut c_void) -> HRESULT;
    pub fn CreateDXGIFactory2(Flags: UINT, riid: REFIID, ppFactory: *mut *mut c_void) -> HRESULT;
    pub fn DXGIGetDebugInterface(riid: REFIID, ppDebug: *mut *mut c_void) -> HRESULT;
    pub fn DXGIGetDebugInterface1(Flags: UINT, riid: REFIID, pDebug: *mut *mut c_void) -> HRESULT;
}
