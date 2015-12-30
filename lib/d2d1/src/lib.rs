// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to d2d1.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn D2D1ComputeMaximumScaleFactor(matrix: *const D2D1_MATRIX_3X2_F) -> FLOAT;
    //pub fn D2D1ConvertColorSpace(
    //    sourceColorSpace: D2D1_COLOR_SPACE, destinationColorSpace: D2D1_COLOR_SPACE,
    //    color: *mut D2D1_COLOR_F
    //);
    //pub fn D2D1CreateDevice(
    //    dxgiDevice: *mut IDXGIDevice, creationProperties: *const D2D1_CREATION_PROPERTIES,
    //    d2dDevice: *mut *mut ID2D1Device
    //) -> HRESULT;
    //pub fn D2D1CreateDeviceContext(
    //    dxgiSurface: *mut IDXGISurface, creationProperties: *const D2D1_CREATION_PROPERTIES,
    //    d2dDeviceContext: *mut *mut ID2D1DeviceContext
    //) -> HRESULT;
    pub fn D2D1CreateFactory(
        factoryType: D2D1_FACTORY_TYPE, riid: REFIID, pFactoryOptions: *const D2D1_FACTORY_OPTIONS,
        ppIFactory: *mut *mut c_void
    ) -> HRESULT;
    pub fn D2D1InvertMatrix(matrix: *mut D2D1_MATRIX_3X2_F) -> BOOL;
    pub fn D2D1IsMatrixInvertible(matrix: *const D2D1_MATRIX_3X2_F) -> BOOL;
    pub fn D2D1MakeRotateMatrix(
        angle: FLOAT, center: D2D1_POINT_2F, matrix: *mut D2D1_MATRIX_3X2_F
    );
    pub fn D2D1MakeSkewMatrix(
        angleX: FLOAT, angleY: FLOAT, center: D2D1_POINT_2F, matrix: *mut D2D1_MATRIX_3X2_F
    );
    pub fn D2D1SinCos(angle: FLOAT, s: *mut FLOAT, c: *mut FLOAT);
    pub fn D2D1Tan(angle: FLOAT) -> FLOAT;
    pub fn D2D1Vec3Length(x: FLOAT, y: FLOAT, z: FLOAT) -> FLOAT;
}
