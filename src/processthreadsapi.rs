// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct STARTUPINFOA {
    pub cb: ::DWORD,
    pub lpReserved: ::LPSTR,
    pub lpDesktop: ::LPSTR,
    pub lpTitle: ::LPSTR,
    pub dwX: ::DWORD,
    pub dwY: ::DWORD,
    pub dwXSize: ::DWORD,
    pub dwYSize: ::DWORD,
    pub dwXCountChars: ::DWORD,
    pub dwYCountChars: ::DWORD,
    pub dwFillAttribute: ::DWORD,
    pub dwFlags: ::DWORD,
    pub wShowWindow: ::WORD,
    pub cbReserved2: ::WORD,
    pub lpReserved2: ::LPBYTE,
    pub hStdInput: ::HANDLE,
    pub hStdOutput: ::HANDLE,
    pub hStdError: ::HANDLE,
}
pub type LPSTARTUPINFOA = *mut STARTUPINFOA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct STARTUPINFOW {
    pub cb: ::DWORD,
    pub lpReserved: ::LPWSTR,
    pub lpDesktop: ::LPWSTR,
    pub lpTitle: ::LPWSTR,
    pub dwX: ::DWORD,
    pub dwY: ::DWORD,
    pub dwXSize: ::DWORD,
    pub dwYSize: ::DWORD,
    pub dwXCountChars: ::DWORD,
    pub dwYCountChars: ::DWORD,
    pub dwFillAttribute: ::DWORD,
    pub dwFlags: ::DWORD,
    pub wShowWindow: ::WORD,
    pub cbReserved2: ::WORD,
    pub lpReserved2: ::LPBYTE,
    pub hStdInput: ::HANDLE,
    pub hStdOutput: ::HANDLE,
    pub hStdError: ::HANDLE,
}
pub type LPSTARTUPINFOW = *mut STARTUPINFOW;
