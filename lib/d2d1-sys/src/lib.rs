// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to d2d1.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm"))]
extern "system" {
    // pub fn D2D1ComputeMaximumScaleFactor();
    // pub fn D2D1ConvertColorSpace();
    // pub fn D2D1CreateDevice();
    // pub fn D2D1CreateDeviceContext();
    // pub fn D2D1CreateFactory();
    // pub fn D2D1InvertMatrix();
    // pub fn D2D1IsMatrixInvertible();
    // pub fn D2D1MakeRotateMatrix();
    // pub fn D2D1MakeSkewMatrix();
    // pub fn D2D1SinCos();
    // pub fn D2D1Tan();
    // pub fn D2D1Vec3Length();
}
