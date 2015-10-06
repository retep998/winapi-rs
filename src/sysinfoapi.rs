// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
//! ApiSet Contract for api-ms-win-core-sysinfo-l1.
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SYSTEM_INFO {
    pub wProcessorArchitecture: ::WORD,
    pub wReserved: ::WORD,
    pub dwPageSize: ::DWORD,
    pub lpMinimumApplicationAddress: ::LPVOID,
    pub lpMaximumApplicationAddress: ::LPVOID,
    pub dwActiveProcessorMask: ::DWORD_PTR,
    pub dwNumberOfProcessors: ::DWORD,
    pub dwProcessorType: ::DWORD,
    pub dwAllocationGranularity: ::DWORD,
    pub wProcessorLevel: ::WORD,
    pub wProcessorRevision: ::WORD,
}
UNION!(SYSTEM_INFO, wProcessorArchitecture, dwOemId, dwOemId_mut, ::DWORD);
pub type LPSYSTEM_INFO = *mut SYSTEM_INFO;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MEMORYSTATUSEX {
    pub dwLength: ::DWORD,
    pub dwMemoryLoad: ::DWORD,
    pub ullTotalPhys: ::DWORDLONG,
    pub ullAvailPhys: ::DWORDLONG,
    pub ullTotalPageFile: ::DWORDLONG,
    pub ullAvailPageFile: ::DWORDLONG,
    pub ullTotalVirtual: ::DWORDLONG,
    pub ullAvailVirtual: ::DWORDLONG,
    pub ullAvailExtendedVirtual: ::DWORDLONG,
}
pub type LPMEMORYSTATUSEX = *mut MEMORYSTATUSEX;
#[repr(i32)] #[derive(Clone, Copy, Debug)]
pub enum COMPUTER_NAME_FORMAT {
    ComputerNameNetBIOS,
    ComputerNameDnsHostname,
    ComputerNameDnsDomain,
    ComputerNameDnsFullyQualified,
    ComputerNamePhysicalNetBIOS,
    ComputerNamePhysicalDnsHostname,
    ComputerNamePhysicalDnsDomain,
    ComputerNamePhysicalDnsFullyQualified,
    ComputerNameMax,
}
pub type INIT_ONCE = ::RTL_RUN_ONCE;
pub type PINIT_ONCE = ::PRTL_RUN_ONCE;
pub type LPINIT_ONCE = ::PRTL_RUN_ONCE;
pub type CONDITION_VARIABLE = ::RTL_CONDITION_VARIABLE;
pub type PCONDITION_VARIABLE = *mut CONDITION_VARIABLE;
