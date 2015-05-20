// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to winmm.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn PlaySoundA(pszSound: LPCSTR, hmod: HMODULE, fdwSound: DWORD) -> BOOL;
    pub fn PlaySoundW(pszSound: LPCWSTR, hmod: HMODULE, fdwSound: DWORD) -> BOOL;
    pub fn sndPlaySoundA(pszSound: LPCSTR, fuSound: UINT) -> BOOL;
    pub fn sndPlaySoundW(pszSound: LPCWSTR, fuSound: UINT) -> BOOL;
    pub fn timeBeginPeriod(uPeriod: UINT) -> MMRESULT;
    pub fn timeEndPeriod(uPeriod: UINT) -> MMRESULT;
}
