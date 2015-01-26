// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to kernel32.
#![no_std]
#![experimental]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn AcquireSRWLockExclusive(SRWLock: PSRWLOCK);
    pub fn AcquireSRWLockShared(SRWLock: PSRWLOCK);
    pub fn ActivateActCtx(hActCtx: HANDLE, lpCookie: *mut ULONG_PTR) -> BOOL;
    pub fn AddAtomA(lpString: LPCSTR) -> ATOM;
    pub fn AddAtomW(lpString: LPCWSTR) -> ATOM;
    pub fn AddConsoleAliasA(Source: LPSTR, Target: LPSTR, ExeName: LPSTR) -> BOOL;
    pub fn AddConsoleAliasW(Source: LPWSTR, Target: LPWSTR, ExeName: LPWSTR) -> BOOL;
    pub fn AddDllDirectory(NewDirectory: PCWSTR) -> DLL_DIRECTORY_COOKIE;
    pub fn AddIntegrityLabelToBoundaryDescriptor(
        BoundaryDescriptor: *mut HANDLE, IntegrityLabel: PSID,
    ) -> BOOL;
    pub fn AddRefActCtx(hActCtx: HANDLE);
    pub fn AddResourceAttributeAce(
        pAcl: PACL, dwAceRevision: DWORD, AceFlags: DWORD, AccessMask: DWORD, pSid: PSID,
        pAttributeInfo: PCLAIM_SECURITY_ATTRIBUTES_INFORMATION, pReturnLength: PDWORD,
    ) -> BOOL;
    pub fn AddSIDToBoundaryDescriptor(BoundaryDescriptor: *mut HANDLE, RequiredSid: PSID) -> BOOL;
    pub fn AddScopedPolicyIDAce(
        pAcl: PACL, dwAceRevision: DWORD, AceFlags: DWORD, AccessMask: DWORD, pSid: PSID,
    ) -> BOOL;
    pub fn AddSecureMemoryCacheCallback(pfnCallBack: PSECURE_MEMORY_CACHE_CALLBACK) -> BOOL;
    pub fn AddVectoredContinueHandler(First: ULONG, Handler: PVECTORED_EXCEPTION_HANDLER) -> PVOID;
    pub fn CloseHandle(hObject: HANDLE) -> BOOL;
    pub fn CreateIoCompletionPort(
        FileHandle: HANDLE, ExistingCompletionPort: HANDLE, CompletionKey: ULONG_PTR,
        NumberOfConcurrentThreads: DWORD,
    ) -> HANDLE;
    pub fn GetConsoleMode(hConsoleHandle: HANDLE, lpMode: LPDWORD) -> BOOL;
    pub fn GetConsoleScreenBufferInfo(
        hConsoleOutput: HANDLE, lpConsoleScreenBufferInfo: PCONSOLE_SCREEN_BUFFER_INFO,
    ) -> BOOL;
    pub fn GetCurrentProcess() -> HANDLE;
    pub fn GetLastError() -> DWORD;
    pub fn GetModuleHandleA(lpModuleName: LPCSTR) -> HMODULE;
    pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;
    pub fn GetProcAddress(hModule: HMODULE, lpProcName: LPCSTR) -> FARPROC;
    pub fn GetProcessTimes(
        hProcess: HANDLE, lpCreationTime: LPFILETIME, lpExitTime: LPFILETIME,
        lpKernelTime: LPFILETIME, lpUserTime: LPFILETIME,
    ) -> BOOL;
    pub fn GetQueuedCompletionStatus(
        CompletionPort: HANDLE, lpNumberOfBytesTransferred: LPDWORD, lpCompletionKey: PULONG_PTR,
        lpOverlapped: *mut LPOVERLAPPED, dwMilliseconds: DWORD,
    ) -> BOOL;
    pub fn GetQueuedCompletionStatusEx(
        CompletionPort: HANDLE, lpCompletionPortEntries: LPOVERLAPPED_ENTRY, ulCount: ULONG,
        ulNumEntriesRemoved: PULONG, dwMilliseconds: DWORD, fAlertable: BOOL,
    ) -> BOOL;
    pub fn GetStdHandle(nStdHandle: DWORD) -> HANDLE;
    pub fn GetSystemRegistryQuota(pdwQuotaAllowed: PDWORD, pdwQuotaUsed: PDWORD) -> BOOL;
    pub fn K32GetProcessMemoryInfo(
        Process: HANDLE, ppsmemCounters: PPROCESS_MEMORY_COUNTERS, cb: DWORD,
    ) -> BOOL;
    pub fn LoadLibraryW(lpFileName: LPCWSTR) -> HMODULE;
    pub fn LocalFree(hMem: HLOCAL) -> HLOCAL;
    pub fn OpenProcess(dwDesiredAccess: DWORD, bInheritHandle: BOOL, dwProcessId: DWORD) -> HANDLE;
    pub fn PostQueuedCompletionStatus(
        CompletionPort: HANDLE, dwNumberOfBytesTransferred: DWORD, dwCompletionKey: ULONG_PTR,
        lpOverlapped: LPOVERLAPPED,
    ) -> BOOL;
    pub fn ReadConsoleW(
        hConsoleInput: HANDLE, lpBuffer: LPVOID, nNumberOfCharsToRead: DWORD,
        lpNumberOfCharsRead: LPDWORD, pInputControl: PCONSOLE_READCONSOLE_CONTROL,
    ) -> BOOL;
    pub fn ReadProcessMemory(
        hProcess: HANDLE, lpBaseAddress: LPCVOID, lpBuffer: LPVOID, nSize: SIZE_T,
        lpNumberOfBytesRead: *mut SIZE_T,
    ) -> BOOL;
    pub fn SetConsoleMode(hConsoleHandle: HANDLE, lpMode: DWORD) -> BOOL;
    pub fn SetConsoleTextAttribute(hConsoleOutput: HANDLE, wAttributes: WORD) -> BOOL;
    pub fn WinExec(lpCmdLine: LPCSTR, uCmdShow: UINT) -> UINT;
    pub fn WriteConsoleW(
        hConsoleOutput: HANDLE, lpBuffer: LPCVOID, nNumberOfCharsToWrite: DWORD,
        lpNumberOfCharsWritten: LPDWORD, lpReserved: LPVOID,
    ) -> BOOL;
    pub fn WriteProcessMemory(
        hProcess: HANDLE, lpBaseAddress: LPVOID, lpBuffer: LPCVOID, nSize: SIZE_T,
        lpNumberOfBytesWritten: *mut SIZE_T,
    ) -> BOOL;
}
