// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! ApiSet Contract for api-ms-win-core-sysinfo-l1.

use shared::basetsd::DWORD_PTR;
use shared::minwindef::{DWORD, LPVOID, WORD};
use um::winnt::DWORDLONG;

STRUCT!{struct SYSTEM_INFO {
    wProcessorArchitecture: WORD,
    wReserved: WORD,
    dwPageSize: DWORD,
    lpMinimumApplicationAddress: LPVOID,
    lpMaximumApplicationAddress: LPVOID,
    dwActiveProcessorMask: DWORD_PTR,
    dwNumberOfProcessors: DWORD,
    dwProcessorType: DWORD,
    dwAllocationGranularity: DWORD,
    wProcessorLevel: WORD,
    wProcessorRevision: WORD,
}}
UNION!(SYSTEM_INFO, wProcessorArchitecture, dwOemId, dwOemId_mut, DWORD);
pub type LPSYSTEM_INFO = *mut SYSTEM_INFO;
STRUCT!{struct MEMORYSTATUSEX {
    dwLength: DWORD,
    dwMemoryLoad: DWORD,
    ullTotalPhys: DWORDLONG,
    ullAvailPhys: DWORDLONG,
    ullTotalPageFile: DWORDLONG,
    ullAvailPageFile: DWORDLONG,
    ullTotalVirtual: DWORDLONG,
    ullAvailVirtual: DWORDLONG,
    ullAvailExtendedVirtual: DWORDLONG,
}}
pub type LPMEMORYSTATUSEX = *mut MEMORYSTATUSEX;
ENUM!{enum COMPUTER_NAME_FORMAT {
    ComputerNameNetBIOS,
    ComputerNameDnsHostname,
    ComputerNameDnsDomain,
    ComputerNameDnsFullyQualified,
    ComputerNamePhysicalNetBIOS,
    ComputerNamePhysicalDnsHostname,
    ComputerNamePhysicalDnsDomain,
    ComputerNamePhysicalDnsFullyQualified,
    ComputerNameMax,
}}
