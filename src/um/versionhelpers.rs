// Copyright © 2018 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Version Helper functions.
use shared::minwindef::{DWORD, FALSE, HIBYTE, LOBYTE, WORD};
use shared::sdkddkver::{
    _WIN32_WINNT_VISTA, _WIN32_WINNT_WIN7, _WIN32_WINNT_WIN8, _WIN32_WINNT_WINBLUE,
    _WIN32_WINNT_WINTHRESHOLD, _WIN32_WINNT_WINXP
};
use um::sysinfoapi::VerSetConditionMask;
use um::winbase::VerifyVersionInfoW;
use um::winnt::{
    DWORDLONG, OSVERSIONINFOEXW, VER_EQUAL, VER_GREATER_EQUAL, VER_MAJORVERSION, VER_MINORVERSION,
    VER_NT_WORKSTATION, VER_PRODUCT_TYPE, VER_SERVICEPACKMAJOR
};
#[inline]
pub fn IsWindowsVersionOrGreater(
    MajorVersion: WORD, MinorVersion: WORD, ServicePackMajor: WORD) -> bool {
    use core::mem::{size_of, zeroed};
    unsafe {
        let condition_mask: DWORDLONG =
            VerSetConditionMask(
                VerSetConditionMask(
                    VerSetConditionMask(
                        0 as DWORDLONG, VER_MAJORVERSION, VER_GREATER_EQUAL),
                        VER_MINORVERSION, VER_GREATER_EQUAL),
                        VER_SERVICEPACKMAJOR, VER_GREATER_EQUAL);
        let mut osvi: OSVERSIONINFOEXW = zeroed();
        osvi.dwOSVersionInfoSize = size_of::<OSVERSIONINFOEXW>() as DWORD;
        osvi.dwMajorVersion = MajorVersion as DWORD;
        osvi.dwMinorVersion = MinorVersion as DWORD;
        osvi.wServicePackMajor = ServicePackMajor;
        VerifyVersionInfoW(
            &mut osvi,
            VER_MAJORVERSION | VER_MINORVERSION | VER_SERVICEPACKMAJOR,
            condition_mask) != FALSE
    }
}
#[inline]
pub fn IsWindowsXPOrGreater() -> bool {
    IsWindowsVersionOrGreater(
        HIBYTE(_WIN32_WINNT_WINXP) as WORD,
        LOBYTE(_WIN32_WINNT_WINXP) as WORD,
        0 as WORD)
}
#[inline]
pub fn IsWindowsXPSP1OrGreater() -> bool {
    IsWindowsVersionOrGreater(
        HIBYTE(_WIN32_WINNT_WINXP) as WORD,
        LOBYTE(_WIN32_WINNT_WINXP) as WORD,
        1 as WORD)
}
#[inline]
pub fn IsWindowsXPSP2OrGreater() -> bool {
    IsWindowsVersionOrGreater(
        HIBYTE(_WIN32_WINNT_WINXP) as WORD,
        LOBYTE(_WIN32_WINNT_WINXP) as WORD,
        2 as WORD)
}
#[inline]
pub fn IsWindowsXPSP3OrGreater() -> bool {
    IsWindowsVersionOrGreater(
        HIBYTE(_WIN32_WINNT_WINXP) as WORD,
        LOBYTE(_WIN32_WINNT_WINXP) as WORD,
        3 as WORD)
}
#[inline]
pub fn IsWindowsVistaOrGreater() -> bool {
    IsWindowsVersionOrGreater(
        HIBYTE(_WIN32_WINNT_VISTA) as WORD,
        LOBYTE(_WIN32_WINNT_VISTA) as WORD,
        0 as WORD)
}
#[inline]
pub fn IsWindowsVistaSP1OrGreater() -> bool {
    IsWindowsVersionOrGreater(
        HIBYTE(_WIN32_WINNT_VISTA) as WORD,
        LOBYTE(_WIN32_WINNT_VISTA) as WORD,
        1 as WORD)
}
#[inline]
pub fn IsWindowsVistaSP2OrGreater() -> bool {
    IsWindowsVersionOrGreater(
        HIBYTE(_WIN32_WINNT_VISTA) as WORD,
        LOBYTE(_WIN32_WINNT_VISTA) as WORD,
        2 as WORD)
}
#[inline]
pub fn IsWindows7OrGreater() -> bool {
    IsWindowsVersionOrGreater(
        HIBYTE(_WIN32_WINNT_WIN7) as WORD,
        LOBYTE(_WIN32_WINNT_WIN7) as WORD,
        0 as WORD)
}
#[inline]
pub fn IsWindows7SP1OrGreater() -> bool {
    IsWindowsVersionOrGreater(
        HIBYTE(_WIN32_WINNT_WIN7) as WORD,
        LOBYTE(_WIN32_WINNT_WIN7) as WORD,
        1 as WORD)
}
#[inline]
pub fn IsWindows8OrGreater() -> bool {
    IsWindowsVersionOrGreater(
        HIBYTE(_WIN32_WINNT_WIN8) as WORD,
        LOBYTE(_WIN32_WINNT_WIN8) as WORD,
        0 as WORD)
}
#[inline]
pub fn IsWindows8Point1OrGreater() -> bool {
    IsWindowsVersionOrGreater(
        HIBYTE(_WIN32_WINNT_WINBLUE) as WORD,
        LOBYTE(_WIN32_WINNT_WINBLUE) as WORD,
        0 as WORD)
}
#[inline]
pub fn IsWindowsThresholdOrGreater() -> bool {
    IsWindowsVersionOrGreater(
        HIBYTE(_WIN32_WINNT_WINTHRESHOLD) as WORD,
        LOBYTE(_WIN32_WINNT_WINTHRESHOLD) as WORD,
        0 as WORD)
}
#[inline]
pub fn IsWindows10OrGreater() -> bool {
    IsWindowsVersionOrGreater(
        HIBYTE(_WIN32_WINNT_WINTHRESHOLD) as WORD,
        LOBYTE(_WIN32_WINNT_WINTHRESHOLD) as WORD,
        0 as WORD)
}
#[inline]
pub fn IsWindowsServer() -> bool {
    use core::mem::{size_of, zeroed};
    unsafe {
        let condition_mask = VerSetConditionMask(0 as DWORDLONG, VER_PRODUCT_TYPE, VER_EQUAL);
        let mut osvi: OSVERSIONINFOEXW = zeroed();
        osvi.dwOSVersionInfoSize = size_of::<OSVERSIONINFOEXW>() as DWORD;
        osvi.wProductType = VER_NT_WORKSTATION;
        VerifyVersionInfoW(&mut osvi, VER_PRODUCT_TYPE, condition_mask) == FALSE
    }
}
