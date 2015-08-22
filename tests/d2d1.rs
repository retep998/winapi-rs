// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
#![cfg(windows)]
extern crate d2d1;
use d2d1::*;
#[inline(never)] fn bb<T>(_: T) {}
#[test]
fn functions() {
    //bb(D2D1ConvertColorSpace);
    //bb(D2D1CreateDevice);
    //bb(D2D1CreateDeviceContext);
    bb(D2D1CreateFactory);
    bb(D2D1InvertMatrix);
    bb(D2D1IsMatrixInvertible);
    bb(D2D1MakeRotateMatrix);
    bb(D2D1MakeSkewMatrix);
}
#[cfg(target_env = "msvc")]
#[test]
fn msvc_functions() {
    bb(D2D1ComputeMaximumScaleFactor);
    bb(D2D1SinCos);
    bb(D2D1Tan);
    bb(D2D1Vec3Length);
}
