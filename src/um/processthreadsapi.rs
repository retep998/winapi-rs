// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms
//! ApiSet Contract for api-ms-win-core-processthreads-l1
use ctypes::{c_int, c_void};
use shared::basetsd::{SIZE_T, ULONG_PTR};
use shared::minwindef::{BOOL, DWORD, LPBYTE, LPDWORD, LPFILETIME, LPVOID, PBOOL, UINT, WORD};
use um::minwinbase::{LPSECURITY_ATTRIBUTES, LPTHREAD_START_ROUTINE};
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
    pub fn GetProcessTimes(
        hProcess: HANDLE,
        lpCreationTime: LPFILETIME,
        lpExitTime: LPFILETIME,
        lpKernelTime: LPFILETIME,
        lpUserTime: LPFILETIME,
    ) -> BOOL;
    pub fn GetCurrentProcess() -> HANDLE;
    pub fn GetCurrentProcessId() -> DWORD;
    pub fn ExitProcess(
        uExitCode: UINT,
    );
    pub fn TerminateProcess(
        hProcess: HANDLE,
        uExitCode: UINT,
    ) -> BOOL;
    pub fn GetExitCodeProcess(
        hProcess: HANDLE,
        lpExitCode: LPDWORD,
    ) -> BOOL;
    pub fn SwitchToThread() -> BOOL;
    pub fn CreateThread(
        lpThreadAttributes: LPSECURITY_ATTRIBUTES,
        dwStackSize: SIZE_T,
        lpStartAddress: LPTHREAD_START_ROUTINE,
        lpParameter: LPVOID,
        dwCreationFlags: DWORD,
        lpThreadId: LPDWORD,
    ) -> HANDLE;
    pub fn CreateRemoteThread(
        hProcess: HANDLE,
        lpThreadAttributes: LPSECURITY_ATTRIBUTES,
        dwStackSize: SIZE_T,
        lpStartAddress: LPTHREAD_START_ROUTINE,
        lpParameter: LPVOID,
        dwCreationFlags: DWORD,
        lpThreadId: LPDWORD,
    ) -> HANDLE;
    pub fn GetCurrentThread() -> HANDLE;
    pub fn GetCurrentThreadId() -> DWORD;
    pub fn OpenThread(
        dwDesiredAccess: DWORD,
        bInheritHandle: BOOL,
        dwThreadId: DWORD,
    ) -> HANDLE;
    pub fn SetThreadPriority(
        hThread: HANDLE,
        nPriority: c_int,
    ) -> BOOL;
    pub fn SetThreadPriorityBoost(
        hThread: HANDLE,
        bDisablePriorityBoost: BOOL,
    ) -> BOOL;
    pub fn GetThreadPriorityBoost(
        hThread: HANDLE,
        pDisablePriorityBoost: PBOOL,
    ) -> BOOL;
    pub fn GetThreadPriority(
        hThread: HANDLE,
    ) -> c_int;
    pub fn ExitThread(
        dwExitCode: DWORD,
    );
    pub fn TerminateThread(
        hThread: HANDLE,
        dwExitCode: DWORD,
    ) -> BOOL;
    pub fn GetExitCodeThread(
        hThread: HANDLE,
        lpExitCode: LPDWORD,
    ) -> BOOL;
    pub fn SuspendThread(
        hThread: HANDLE,
    ) -> DWORD;
    pub fn ResumeThread(
        hThread: HANDLE,
    ) -> DWORD;
}
pub const TLS_OUT_OF_INDEXES: DWORD = 0xFFFFFFFF;
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
