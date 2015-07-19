// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PROCESS_INFORMATION {
    pub hProcess: ::HANDLE,
    pub hThread: ::HANDLE,
    pub dwProcessId: ::DWORD,
    pub dwThreadId: ::DWORD,
}
pub type PPROCESS_INFORMATION = *mut PROCESS_INFORMATION;
pub type LPPROCESS_INFORMATION = *mut PROCESS_INFORMATION;

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

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PROC_THREAD_ATTRIBUTE_LIST {
    dummy: *mut ::c_void,
}
pub type PPROC_THREAD_ATTRIBUTE_LIST = *mut PROC_THREAD_ATTRIBUTE_LIST;
pub type LPPROC_THREAD_ATTRIBUTE_LIST = *mut PROC_THREAD_ATTRIBUTE_LIST;

#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum THREAD_INFORMATION_CLASS {
    ThreadMemoryPriority,
    ThreadAbsoluteCpuPriority,
    ThreadInformationClassMax,
}
