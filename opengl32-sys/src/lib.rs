// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to opengl32.
#![no_std]
#![experimental]
extern crate winapi;
use winapi::*;
extern "system" {
  pub fn wglCopyContext(hglrcSrc: HGLRC, hglrcDst: HGLRC, mask: UINT) -> BOOL;
  pub fn wglCreateContext(hdc: HDC) -> HGLRC;
  pub fn wglCreateLayerContext(hdc: HDC, iLayerPlane: c_int) -> HGLRC;
  pub fn wglDeleteContext(hglrc: HGLRC) -> BOOL;
  //pub fn wglDescribeLayerPlane();
  pub fn wglGetCurrentContext() -> HGLRC;
  pub fn wglGetCurrentDC() -> HDC;
  // pub fn wglGetLayerPaletteEntries();
  pub fn wglGetProcAddress(lpszProc: LPCSTR) -> PROC;
  pub fn wglMakeCurrent(hdc: HDC, hglrc: HGLRC) -> BOOL;
  // pub fn wglRealizeLayerPalette();
  // pub fn wglSetLayerPaletteEntries();
  pub fn wglShareLists(hglrc1: HGLRC, hglrc2: HGLRC) -> BOOL;
  // pub fn wglSwapLayerBuffers();
  // pub fn wglUseFontBitmaps();
  // pub fn wglUseFontOutlines();
}
