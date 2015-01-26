// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to gdi32.
#![no_std]
#![experimental]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn BitBlt(
        hdc: HDC, x: c_int, y: c_int, cx: c_int, cy: c_int, hdcSrc: HDC, x1: c_int, y1: c_int,
        rop: DWORD,
    ) -> BOOL;
    pub fn ChoosePixelFormat(hdc: HDC, ppfd: *const PIXELFORMATDESCRIPTOR) -> c_int;
    pub fn CreateCompatibleBitmap(hdc: HDC, cx: c_int, cy: c_int) -> HBITMAP;
    pub fn CreateCompatibleDC(hdc: HDC) -> HDC;
    pub fn CreateFontA(
        cHeight: c_int, cWidth: c_int, cEscapement: c_int, cOrientation: c_int, cWeight: c_int,
        bItalic: DWORD, bUnderline: DWORD, bStrikeOut: DWORD, iCharSet: DWORD,
        iOutPrecision: DWORD, iClipPrecision: DWORD, iQuality: DWORD, iPitchAndFamily: DWORD,
        pszFaceName: LPCSTR,
    ) -> HFONT;
    pub fn CreateFontW(
        cHeight: c_int, cWidth: c_int, cEscapement: c_int, cOrientation: c_int, cWeight: c_int,
        bItalic: DWORD, bUnderline: DWORD, bStrikeOut: DWORD, iCharSet: DWORD,
        iOutPrecision: DWORD, iClipPrecision: DWORD, iQuality: DWORD, iPitchAndFamily: DWORD,
        pszFaceName: LPCWSTR,
    ) -> HFONT;
    pub fn DeleteDC(hdc: HDC) -> BOOL;
    pub fn DeleteObject(ho: HGDIOBJ) -> BOOL;
    pub fn DescribePixelFormat(
        hdc: HDC, iPixelFormat: c_int, nBytes: UINT, ppfd: LPPIXELFORMATDESCRIPTOR,
    ) -> c_int;
    pub fn FillRect(hDC: HDC, lprc: *const RECT, hbr: HBRUSH) -> c_int;
    pub fn Rectangle(hdc: HDC, left: c_int, top: c_int, right: c_int, bottom: c_int) -> BOOL;
    pub fn SelectObject(hdc: HDC, h: HGDIOBJ) -> HGDIOBJ;
    pub fn SetBkColor(hdc: HDC, color: COLORREF) -> COLORREF;
    pub fn SetPixelFormat(
        hdc: HDC, iPixelFormat: c_int, ppfd: *const PIXELFORMATDESCRIPTOR,
    ) -> BOOL;
    pub fn SetTextColor(hdc: HDC, color: COLORREF) -> COLORREF;
    pub fn SwapBuffers(hdc: HDC) -> BOOL;
    pub fn TextOutA(hdc: HDC, x: c_int, y: c_int, lpString: LPCSTR, c: c_int) -> BOOL;
    pub fn TextOutW(hdc: HDC, x: c_int, y: c_int, lpString: LPCWSTR, c: c_int) -> BOOL;
}
