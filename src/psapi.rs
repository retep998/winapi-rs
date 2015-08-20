// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
//! API Prototypes and Definitions for PSAPI.DLL
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MODULEINFO {
    pub lpBaseOfDll: ::LPVOID,
    pub SizeOfImage: ::DWORD,
    pub EntryPoint: ::LPVOID,
}
pub type LPMODULEINFO = *mut MODULEINFO;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PSAPI_WS_WATCH_INFORMATION {
    pub FaultingPc: ::LPVOID,
    pub FaultingVa: ::LPVOID,
}
pub type PPSAPI_WS_WATCH_INFORMATION = *mut PSAPI_WS_WATCH_INFORMATION;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PSAPI_WS_WATCH_INFORMATION_EX {
    pub BasicInfo: PSAPI_WS_WATCH_INFORMATION,
    pub FaultingThreadId: ::ULONG_PTR,
    pub Flags: ::ULONG_PTR,
}
pub type PPSAPI_WS_WATCH_INFORMATION_EX = *mut PSAPI_WS_WATCH_INFORMATION_EX;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PERFORMANCE_INFORMATION {
    pub cb: ::DWORD,
    pub CommitTotal: ::SIZE_T,
    pub CommitLimit: ::SIZE_T,
    pub CommitPeak: ::SIZE_T,
    pub PhysicalTotal: ::SIZE_T,
    pub PhysicalAvailable: ::SIZE_T,
    pub SystemCache: ::SIZE_T,
    pub KernelTotal: ::SIZE_T,
    pub KernelPaged: ::SIZE_T,
    pub KernelNonpaged: ::SIZE_T,
    pub PageSize: ::SIZE_T,
    pub HandleCount: ::DWORD,
    pub ProcessCount: ::DWORD,
    pub ThreadCount: ::DWORD,
}
pub type PPERFORMANCE_INFORMATION = *mut PERFORMANCE_INFORMATION;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ENUM_PAGE_FILE_INFORMATION {
    pub cb: ::DWORD,
    pub Reserved: ::DWORD,
    pub TotalSize: ::SIZE_T,
    pub TotalInUse: ::SIZE_T,
    pub PeakUsage: ::SIZE_T,
}
pub type PENUM_PAGE_FILE_INFORMATION = *mut ENUM_PAGE_FILE_INFORMATION;

pub type PENUM_PAGE_FILE_CALLBACKA = Option<unsafe extern "system" fn(
    pContext: ::LPVOID, pPageFileInfo: PENUM_PAGE_FILE_INFORMATION, lpFilename: ::LPCSTR,
) -> ::BOOL>;
pub type PENUM_PAGE_FILE_CALLBACKW = Option<unsafe extern "system" fn(
    pContext: ::LPVOID, pPageFileInfo: PENUM_PAGE_FILE_INFORMATION, lpFilename: ::LPCWSTR,
) -> ::BOOL>;
