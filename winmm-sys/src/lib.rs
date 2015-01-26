// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to winmm.
#![no_std]
#![experimental]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn PlaySoundA(pszSound: LPCSTR, hmod: HMODULE, fdwSound: DWORD) -> BOOL;
    pub fn PlaySoundW(pszSound: LPCWSTR, hmod: HMODULE, fdwSound: DWORD) -> BOOL;
    pub fn sndPlaySoundA(pszSound: LPCSTR, fuSound: UINT) -> BOOL;
    pub fn sndPlaySoundW(pszSound: LPCWSTR, fuSound: UINT) -> BOOL;
}
