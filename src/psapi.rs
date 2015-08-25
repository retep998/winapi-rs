// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
//! API Prototypes and Definitions for PSAPI.DLL
pub const LIST_MODULES_DEFAULT: ::DWORD = 0x0;
pub const LIST_MODULES_32BIT: ::DWORD = 0x01;
pub const LIST_MODULES_64BIT: ::DWORD = 0x02;
pub const LIST_MODULES_ALL: ::DWORD = LIST_MODULES_32BIT | LIST_MODULES_64BIT;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MODULEINFO {
    pub lpBaseOfDll: ::LPVOID,
    pub SizeOfImage: ::DWORD,
    pub EntryPoint: ::LPVOID,
}
pub type LPMODULEINFO = *mut MODULEINFO;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PSAPI_WORKING_SET_BLOCK {
    pub Flags: ::ULONG_PTR,
    pub BitFields: ::ULONG_PTR,
}
#[cfg(target_arch="x86")]
BITFIELD!(PSAPI_WORKING_SET_BLOCK BitFields: ::ULONG_PTR [
    Protection set_Protection[0..5],
    ShareCount set_ShareCount[5..8],
    Shared set_Shared[8..9],
    Reserved set_Reserved[9..12],
    VirtualPage set_VirtualPage[12..32],
]);
#[cfg(target_arch="x86_64")]
BITFIELD!(PSAPI_WORKING_SET_BLOCK BitFields: ::ULONG_PTR [
    Protection set_Protection[0..5],
    ShareCount set_ShareCount[5..8],
    Shared set_Shared[8..9],
    Reserved set_Reserved[9..12],
    VirtualPage set_VirtualPage[12..64],
]);
pub type PPSAPI_WORKING_SET_BLOCK = *mut PSAPI_WORKING_SET_BLOCK;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PSAPI_WORKING_SET_INFORMATION {
    pub NumberOfEntries: ::ULONG_PTR,
    pub WorkingSetInfo: [PSAPI_WORKING_SET_BLOCK; 1],
}
pub type PPSAPI_WORKING_SET_INFORMATION = *mut PSAPI_WORKING_SET_INFORMATION;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PSAPI_WORKING_SET_EX_BLOCK_Invalid {
    pub BitFields: ::ULONG_PTR,
}
#[cfg(target_arch="x86")]
BITFIELD!(PSAPI_WORKING_SET_EX_BLOCK_Invalid BitFields: ::ULONG_PTR [
    Valid set_Valid[0..1],
    Reserved0 set_Reserved0[1..15],
    Shared set_Shared[15..16],
    Reserved1 set_Reserved1[16..31],
    Bad set_Bad[31..32],
]);
#[cfg(target_arch="x86_64")]
BITFIELD!(PSAPI_WORKING_SET_EX_BLOCK_Invalid BitFields: ::ULONG_PTR [
    Valid set_Valid[0..1],
    Reserved0 set_Reserved0[1..15],
    Shared set_Shared[15..16],
    Reserved1 set_Reserved1[16..31],
    Bad set_Bad[31..32],
    ReservedUlong set_ReservedUlong[32..64],
]);
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PSAPI_WORKING_SET_EX_BLOCK {
    pub Flags: ::ULONG_PTR,
    pub BitFields: ::ULONG_PTR,
}
#[cfg(target_arch="x86")]
BITFIELD!(PSAPI_WORKING_SET_EX_BLOCK BitFields: ::ULONG_PTR [
    Valid set_Valid[0..1],
    ShareCount set_ShareCount[1..4],
    Win32Protection set_Win32Protection[4..15],
    Shared set_Shared[15..16],
    Node set_Node[16..22],
    Locked set_Locked[22..23],
    LargePage set_LargePage[23..24],
    Reserved set_Reserved[24..31],
    Bad set_Bad[31..32],
]);
#[cfg(target_arch="x86_64")]
BITFIELD!(PSAPI_WORKING_SET_EX_BLOCK BitFields: ::ULONG_PTR [
    Valid set_Valid[0..1],
    ShareCount set_ShareCount[1..4],
    Win32Protection set_Win32Protection[4..15],
    Shared set_Shared[15..16],
    Node set_Node[16..22],
    Locked set_Locked[22..23],
    LargePage set_LargePage[23..24],
    Reserved set_Reserved[24..31],
    Bad set_Bad[31..32],
    ReservedUlong set_ReservedUlong[32..64],
]);
UNION!(
    PSAPI_WORKING_SET_EX_BLOCK, BitFields, Invalid, Invalid_mut, PSAPI_WORKING_SET_EX_BLOCK_Invalid
);
pub type PPSAPI_WORKING_SET_EX_BLOCK = *mut PSAPI_WORKING_SET_EX_BLOCK;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PSAPI_WORKING_SET_EX_INFORMATION {
    pub VirtualAddress: ::PVOID,
    pub VirtualAttributes: PSAPI_WORKING_SET_EX_BLOCK,
}
pub type PPSAPI_WORKING_SET_EX_INFORMATION = *mut PSAPI_WORKING_SET_EX_INFORMATION;

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
pub struct PROCESS_MEMORY_COUNTERS {
    pub cb: ::DWORD,
    pub PageFaultCount: ::DWORD,
    pub PeakWorkingSetSize: ::SIZE_T,
    pub WorkingSetSize: ::SIZE_T,
    pub QuotaPeakPagedPoolUsage: ::SIZE_T,
    pub QuotaPagedPoolUsage: ::SIZE_T,
    pub QuotaPeakNonPagedPoolUsage: ::SIZE_T,
    pub QuotaNonPagedPoolUsage: ::SIZE_T,
    pub PagefileUsage: ::SIZE_T,
    pub PeakPagefileUsage: ::SIZE_T,
}
pub type PPROCESS_MEMORY_COUNTERS = *mut PROCESS_MEMORY_COUNTERS;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PROCESS_MEMORY_COUNTERS_EX {
    pub cb: ::DWORD,
    pub PageFaultCount: ::DWORD,
    pub PeakWorkingSetSize: ::SIZE_T,
    pub WorkingSetSize: ::SIZE_T,
    pub QuotaPeakPagedPoolUsage: ::SIZE_T,
    pub QuotaPagedPoolUsage: ::SIZE_T,
    pub QuotaPeakNonPagedPoolUsage: ::SIZE_T,
    pub QuotaNonPagedPoolUsage: ::SIZE_T,
    pub PagefileUsage: ::SIZE_T,
    pub PeakPagefileUsage: ::SIZE_T,
    pub PrivateUsage: ::SIZE_T,
}
pub type PPROCESS_MEMORY_COUNTERS_EX = *mut PROCESS_MEMORY_COUNTERS_EX;

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
