// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms
//! ApiSet Contract for api-ms-win-core-processthreads-l1
use ctypes::{c_void};
use shared::basetsd::{ULONG_PTR};
use shared::minwindef::{DWORD, LPBYTE, WORD};
use um::winnt::{HANDLE, LPSTR, LPWSTR, PAPCFUNC};
STRUCT!{struct PROCESS_INFORMATION {
    hProcess: HANDLE,
    hThread: HANDLE,
    dwProcessId: DWORD,
    dwThreadId: DWORD,
}}
pub type PPROCESS_INFORMATION = *mut PROCESS_INFORMATION;
pub type LPPROCESS_INFORMATION = *mut PROCESS_INFORMATION;
STRUCT!{struct STARTUPINFOA {
    cb: DWORD,
    lpReserved: LPSTR,
    lpDesktop: LPSTR,
    lpTitle: LPSTR,
    dwX: DWORD,
    dwY: DWORD,
    dwXSize: DWORD,
    dwYSize: DWORD,
    dwXCountChars: DWORD,
    dwYCountChars: DWORD,
    dwFillAttribute: DWORD,
    dwFlags: DWORD,
    wShowWindow: WORD,
    cbReserved2: WORD,
    lpReserved2: LPBYTE,
    hStdInput: HANDLE,
    hStdOutput: HANDLE,
    hStdError: HANDLE,
}}
pub type LPSTARTUPINFOA = *mut STARTUPINFOA;
STRUCT!{struct STARTUPINFOW {
    cb: DWORD,
    lpReserved: LPWSTR,
    lpDesktop: LPWSTR,
    lpTitle: LPWSTR,
    dwX: DWORD,
    dwY: DWORD,
    dwXSize: DWORD,
    dwYSize: DWORD,
    dwXCountChars: DWORD,
    dwYCountChars: DWORD,
    dwFillAttribute: DWORD,
    dwFlags: DWORD,
    wShowWindow: WORD,
    cbReserved2: WORD,
    lpReserved2: LPBYTE,
    hStdInput: HANDLE,
    hStdOutput: HANDLE,
    hStdError: HANDLE,
}}
pub type LPSTARTUPINFOW = *mut STARTUPINFOW;
extern "system" {
    pub fn QueueUserAPC(
        pfnAPC: PAPCFUNC,
        hThread: HANDLE,
        dwData: ULONG_PTR,
    ) -> DWORD;
}
STRUCT!{struct PROC_THREAD_ATTRIBUTE_LIST {
    dummy: *mut c_void,
}}
pub type PPROC_THREAD_ATTRIBUTE_LIST = *mut PROC_THREAD_ATTRIBUTE_LIST;
pub type LPPROC_THREAD_ATTRIBUTE_LIST = *mut PROC_THREAD_ATTRIBUTE_LIST;
ENUM!{enum THREAD_INFORMATION_CLASS {
    ThreadMemoryPriority,
    ThreadAbsoluteCpuPriority,
    ThreadInformationClassMax,
}}
