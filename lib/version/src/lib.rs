// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to version.
#![no_std]
#![cfg(windows)]
extern crate winapi;
use winapi::*;

extern "system" {
    pub fn GetFileVersionInfoSizeA(
        lptstrFilename: LPCSTR,
        lpdwHandle: *mut DWORD,
    ) -> DWORD;

    pub fn GetFileVersionInfoSizeW(
        lptstrFilename: LPCWSTR,
        lpdwHandle: *mut DWORD,
    ) -> DWORD;

    pub fn GetFileVersionInfoA(
        lptstrFilename: LPCSTR,
        dwHandle: DWORD,
        dwLen: DWORD,
        lpData: *mut c_void,
    ) -> BOOL;

    pub fn GetFileVersionInfoW(
        lptstrFilename: LPCWSTR,
        dwHandle: DWORD,
        dwLen: DWORD,
        lpData: *mut c_void,
    ) -> BOOL;

    pub fn VerQueryValueA(
        pBlock: LPCVOID,
        lpSubBlock: LPCSTR,
        lplpBuffer: &mut LPVOID,
        puLen: PUINT,
    ) -> BOOL;

    pub fn VerQueryValueW(
        pBlock: LPCVOID,
        lpSubBlock: LPCWSTR,
        lplpBuffer: &mut LPVOID,
        puLen: PUINT,
    ) -> BOOL;

}
