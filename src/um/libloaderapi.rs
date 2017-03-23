// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! ApiSet Contract for api-ms-win-core-libraryloader-l1
use shared::basetsd::LONG_PTR;
use shared::minwindef::{BOOL, DWORD, HMODULE, WORD};
use um::winnt::{LPCSTR, LPCWSTR, LPSTR, LPWSTR, PVOID};
pub type DLL_DIRECTORY_COOKIE = PVOID;
pub type PDLL_DIRECTORY_COOKIE = *mut PVOID;
pub type ENUMRESLANGPROCA = Option<unsafe extern "system" fn(
    hModule: HMODULE,
    lpType: LPCSTR,
    lpName: LPCSTR,
    wLanguage: WORD,
    lParam: LONG_PTR,
) -> BOOL>;
pub type ENUMRESLANGPROCW = Option<unsafe extern "system" fn(
    hModule: HMODULE,
    lpType: LPCWSTR,
    lpName: LPCWSTR,
    wLanguage: WORD,
    lParam: LONG_PTR,
) -> BOOL>;
pub type ENUMRESNAMEPROCA = Option<unsafe extern "system" fn(
    hModule: HMODULE,
    lpType: LPCSTR,
    lpName: LPSTR,
    lParam: LONG_PTR,
) -> BOOL>;
pub type ENUMRESNAMEPROCW = Option<unsafe extern "system" fn(
    hModule: HMODULE,
    lpType: LPCWSTR,
    lpName: LPWSTR,
    lParam: LONG_PTR,
) -> BOOL>;
pub type ENUMRESTYPEPROCA = Option<unsafe extern "system" fn(
    hModule: HMODULE,
    lpType: LPSTR,
    lParam: LONG_PTR,
) -> BOOL>;
pub type ENUMRESTYPEPROCW = Option<unsafe extern "system" fn(
    hModule: HMODULE,
    lpType: LPWSTR,
    lParam: LONG_PTR,
) -> BOOL>;
extern "system" {
    pub fn GetModuleHandleA(
        lpModuleName: LPCSTR,
    ) -> HMODULE;
    pub fn GetModuleHandleW(
        lpModuleName: LPCWSTR,
    ) -> HMODULE;
    pub fn GetModuleHandleExA(
        dwFlags: DWORD,
        lpModuleName: LPCSTR,
        phModule: *mut HMODULE,
    ) -> BOOL;
    pub fn GetModuleHandleExW(
        dwFlags: DWORD,
        lpModuleName: LPCWSTR,
        phModule: *mut HMODULE,
    ) -> BOOL;
}
