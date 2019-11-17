// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::{FALSE, HIBYTE, LOBYTE, WORD};
use shared::sdkddkver::{
    _WIN32_WINNT_VISTA, _WIN32_WINNT_WIN7, _WIN32_WINNT_WIN8, _WIN32_WINNT_WINBLUE,
    _WIN32_WINNT_WINTHRESHOLD, _WIN32_WINNT_WINXP
};
use um::winbase::VerifyVersionInfoW;
use um::winnt::{
    OSVERSIONINFOEXW, VER_EQUAL, VER_GREATER_EQUAL, VER_MAJORVERSION, VER_MINORVERSION,
    VER_NT_WORKSTATION, VER_PRODUCT_TYPE, VER_SERVICEPACKMAJOR, VerSetConditionMask
};
#[inline]
pub fn IsWnidowsVersionOrGreater(
    wMajorVersion: WORD,
    wMinorVersion: WORD,
    wServicePackVersion: WORD,
) -> bool {
    let mut osvi = OSVERSIONINFOEXW {
        dwOSVersionInfoSize: 0,
        dwMajorVersion: wMajorVersion as _,
        dwMinorVersion: wMinorVersion as _,
        dwBuildNumber: 0,
        dwPlatformId: 0,
        szCSDVersion: [0; 128],
        wServicePackMajor: wServicePackVersion,
        wServicePackMinor: 0,
        wSuiteMask: 0,
        wProductType: 0,
        wReserved: 0,
    };
    let dwlConditionMask = unsafe {
        VerSetConditionMask(
            VerSetConditionMask(
                VerSetConditionMask(0, VER_MAJORVERSION, VER_GREATER_EQUAL),
                VER_MINORVERSION,
                VER_GREATER_EQUAL,
            ),
            VER_SERVICEPACKMAJOR,
            VER_GREATER_EQUAL,
        )
    };
    FALSE != unsafe {
        VerifyVersionInfoW(
            &mut osvi,
            VER_MAJORVERSION | VER_MINORVERSION | VER_SERVICEPACKMAJOR,
            dwlConditionMask,
        )
    }
}
#[inline]
pub fn IsWindowsXPOrGreater() -> bool {
    IsWnidowsVersionOrGreater(
        HIBYTE(_WIN32_WINNT_WINXP) as _,
        LOBYTE(_WIN32_WINNT_WINXP) as _,
        0,
    )
}
#[inline]
pub fn IsWindowsXPSP1OrGreater() -> bool {
    IsWnidowsVersionOrGreater(
        HIBYTE(_WIN32_WINNT_WINXP) as _,
        LOBYTE(_WIN32_WINNT_WINXP) as _,
        1,
    )
}
#[inline]
pub fn IsWindowsXPSP2OrGreater() -> bool {
    IsWnidowsVersionOrGreater(
        HIBYTE(_WIN32_WINNT_WINXP) as _,
        LOBYTE(_WIN32_WINNT_WINXP) as _,
        2,
    )
}
#[inline]
pub fn IsWindowsXPSP3OrGreater() -> bool {
    IsWnidowsVersionOrGreater(
        HIBYTE(_WIN32_WINNT_WINXP) as _,
        LOBYTE(_WIN32_WINNT_WINXP) as _,
        3,
    )
}
#[inline]
pub fn IsWindowsVistaOrGreater() -> bool {
    IsWnidowsVersionOrGreater(
        HIBYTE(_WIN32_WINNT_VISTA) as _,
        LOBYTE(_WIN32_WINNT_VISTA) as _,
        0,
    )
}
#[inline]
pub fn IsWindowsVistaSP1OrGreater() -> bool {
    IsWnidowsVersionOrGreater(
        HIBYTE(_WIN32_WINNT_VISTA) as _,
        LOBYTE(_WIN32_WINNT_VISTA) as _,
        1,
    )
}
#[inline]
pub fn IsWindowsVistaSP2OrGreater() -> bool {
    IsWnidowsVersionOrGreater(
        HIBYTE(_WIN32_WINNT_VISTA) as _,
        LOBYTE(_WIN32_WINNT_VISTA) as _,
        2,
    )
}
#[inline]
pub fn IsWindows7OrGreater() -> bool {
    IsWnidowsVersionOrGreater(
        HIBYTE(_WIN32_WINNT_WIN7) as _,
        LOBYTE(_WIN32_WINNT_WIN7) as _,
        0,
    )
}
#[inline]
pub fn IsWindows7SP1OrGreater() -> bool {
    IsWnidowsVersionOrGreater(
        HIBYTE(_WIN32_WINNT_WIN7) as _,
        LOBYTE(_WIN32_WINNT_WIN7) as _,
        1,
    )
}
#[inline]
pub fn IsWindows8OrGreater() -> bool {
    IsWnidowsVersionOrGreater(
        HIBYTE(_WIN32_WINNT_WIN8) as _,
        LOBYTE(_WIN32_WINNT_WIN8) as _,
        0,
    )
}
#[inline]
pub fn IsWindows8Point1OrGreater() -> bool {
    IsWnidowsVersionOrGreater(
        HIBYTE(_WIN32_WINNT_WINBLUE) as _,
        LOBYTE(_WIN32_WINNT_WINBLUE) as _,
        0,
    )
}
#[inline]
pub fn IsWindowsThresholdOrGreater() -> bool {
    IsWnidowsVersionOrGreater(
        HIBYTE(_WIN32_WINNT_WINTHRESHOLD) as _,
        LOBYTE(_WIN32_WINNT_WINTHRESHOLD) as _,
        0,
    )
}
#[inline]
pub fn IsWindows10OrGreater() -> bool {
    IsWnidowsVersionOrGreater(
        HIBYTE(_WIN32_WINNT_WINTHRESHOLD) as _,
        LOBYTE(_WIN32_WINNT_WINTHRESHOLD) as _,
        0,
    )
}
#[inline]
pub fn IsWindowsServer() -> bool {
    let mut osvi = OSVERSIONINFOEXW {
        dwOSVersionInfoSize: 0,
        dwMajorVersion: 0,
        dwMinorVersion: 0,
        dwBuildNumber: 0,
        dwPlatformId: 0,
        szCSDVersion: [0; 128],
        wServicePackMajor: 0,
        wServicePackMinor: 0,
        wSuiteMask: 0,
        wProductType: VER_NT_WORKSTATION,
        wReserved: 0,
    };
    let dwlConditionMask = unsafe { VerSetConditionMask(0, VER_PRODUCT_TYPE, VER_EQUAL) };
    FALSE == unsafe { VerifyVersionInfoW(&mut osvi, VER_PRODUCT_TYPE, dwlConditionMask) }
}
