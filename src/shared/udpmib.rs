// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Contains the public definitions and structures for the UDP-specific parts of MIB-II
// #include <winapifamily.h>
use core::mem;
use core::ptr;
use shared::basetsd::{DWORD64, DWORD_PTR};
use shared::in6addr::IN6_ADDR;
use shared::minwindef::DWORD;
use shared::ntdef::{INT, LARGE_INTEGER, UCHAR, ULONGLONG};
const ALIGN_SIZE: DWORD_PTR = 0x00000008; // from RTInfo.h
const ANY_SIZE: usize = 1;
pub const TCPIP_OWNING_MODULE_SIZE: usize = 16;
macro_rules! offset_of_table {
    ($type:ty) => (unsafe {
        &(*(ptr::null() as *const $type)).table[0] as *const _ as usize
    })
}
STRUCT!{struct MIB_UDPROW {
    dwLocalAddr: DWORD,
    dwLocalPort: DWORD,
}}
pub type PMIB_UDPROW = *mut MIB_UDPROW;
STRUCT!{struct MIB_UDPTABLE {
    dwNumEntries: DWORD,
    table: [MIB_UDPROW; ANY_SIZE],
}}
pub type PMIB_UDPTABLE = *mut MIB_UDPTABLE;
#[inline]
pub fn SIZEOF_UDPTABLE(num: usize) -> usize {
    offset_of_table!(MIB_UDPTABLE) + num * mem::size_of::<MIB_UDPROW>() + ALIGN_SIZE
}
STRUCT!{struct MIB_UDPROW_OWNER_PID {
    dwLocalAddr: DWORD,
    dwLocalPort: DWORD,
    dwOwningPid: DWORD,
}}
pub type PMIB_UDPROW_OWNER_PID = *mut MIB_UDPROW_OWNER_PID;
STRUCT!{struct MIB_UDPTABLE_OWNER_PID {
    dwNumEntries: DWORD,
    table: [MIB_UDPROW_OWNER_PID; ANY_SIZE],
}}
pub type PMIB_UDPTABLE_OWNER_PID = *mut MIB_UDPTABLE_OWNER_PID;
#[inline]
pub fn SIZEOF_UDPTABLE_OWNER_PID(num: usize) -> usize {
    offset_of_table!(MIB_UDPTABLE_OWNER_PID) + num * mem::size_of::<MIB_UDPROW_OWNER_PID>()
        + ALIGN_SIZE
}
STRUCT!{struct MIB_UDPROW_OWNER_MODULE_u_s {
    bitfield: INT,
}}
BITFIELD!{MIB_UDPROW_OWNER_MODULE_u_s bitfield: INT [
    SpecificPortBind set_SpecificPortBind[0..1],
]}
UNION!{union MIB_UDPROW_OWNER_MODULE_u {
    [i32; 1],
    s s_mut: MIB_UDPROW_OWNER_MODULE_u_s,
    dwFlags dwFlags_mut: INT,
}}
STRUCT!{struct MIB_UDPROW_OWNER_MODULE {
    dwLocalAddr: DWORD,
    dwLocalPort: DWORD,
    dwOwningPid: DWORD,
    liCreateTimestamp: LARGE_INTEGER,
    u: MIB_UDPROW_OWNER_MODULE_u,
    OwningModuleInfo: [ULONGLONG; TCPIP_OWNING_MODULE_SIZE],
}}
pub type PMIB_UDPROW_OWNER_MODULE = *mut MIB_UDPROW_OWNER_MODULE;
STRUCT!{struct MIB_UDPTABLE_OWNER_MODULE {
    dwNumEntries: DWORD,
    table: [MIB_UDPROW_OWNER_MODULE; ANY_SIZE],
}}
pub type PMIB_UDPTABLE_OWNER_MODULE = *mut MIB_UDPTABLE_OWNER_MODULE;
#[inline]
pub fn SIZEOF_UDPTABLE_OWNER_MODULE(num: usize) -> usize {
    offset_of_table!(MIB_UDPTABLE_OWNER_MODULE) + num * mem::size_of::<MIB_UDPROW_OWNER_MODULE>()
        + ALIGN_SIZE
}
STRUCT!{struct MIB_UDP6ROW {
    dwLocalAddr: IN6_ADDR,
    dwLocalScopeId: DWORD,
    dwLocalPort: DWORD,
}}
pub type PMIB_UDP6ROW = *mut MIB_UDP6ROW;
STRUCT!{struct MIB_UDP6TABLE {
    dwNumEntries: DWORD,
    table: [MIB_UDP6ROW; ANY_SIZE],
}}
pub type PMIB_UDP6TABLE = *mut MIB_UDP6TABLE;
#[inline]
pub fn SIZEOF_UDP6TABLE(num: usize) -> usize {
    offset_of_table!(MIB_UDP6TABLE) + num * mem::size_of::<MIB_UDP6ROW>() + ALIGN_SIZE
}
STRUCT!{struct MIB_UDP6ROW_OWNER_PID {
    ucLocalAddr: [UCHAR; 16],
    dwLocalScopeId: DWORD,
    dwLocalPort: DWORD,
    dwOwningPid: DWORD,
}}
pub type PMIB_UDP6ROW_OWNER_PID = *mut MIB_UDP6ROW_OWNER_PID;
STRUCT!{struct MIB_UDP6TABLE_OWNER_PID {
    dwNumEntries: DWORD,
    table: [MIB_UDP6ROW_OWNER_PID; ANY_SIZE],
}}
pub type PMIB_UDP6TABLE_OWNER_PID = *mut MIB_UDP6TABLE_OWNER_PID;
#[inline]
pub fn SIZEOF_UDP6TABLE_OWNER_PID(num: usize) -> usize {
    offset_of_table!(MIB_UDP6TABLE_OWNER_PID) + num * mem::size_of::<MIB_UDP6ROW_OWNER_PID>()
        + ALIGN_SIZE
}
STRUCT!{struct MIB_UDP6ROW_OWNER_MODULE_u_s {
    bitfield: INT,
}}
BITFIELD!{MIB_UDP6ROW_OWNER_MODULE_u_s bitfield: INT [
    SpecificPortBind set_SpecificPortBind[0..1],
]}
UNION!{union MIB_UDP6ROW_OWNER_MODULE_u {
    [i32; 1],
    s s_mut: INT,
    dwFlags dwFlags_mut: INT,
}}
STRUCT!{struct MIB_UDP6ROW_OWNER_MODULE {
    ucLocalAddr: [UCHAR; 16],
    dwLocalScopeId: DWORD,
    dwLocalPort: DWORD,
    dwOwningPid: DWORD,
    liCreateTimestamp: LARGE_INTEGER,
    u: MIB_UDP6ROW_OWNER_MODULE_u,
    OwningModuleInfo: [ULONGLONG; TCPIP_OWNING_MODULE_SIZE],
}}
pub type PMIB_UDP6ROW_OWNER_MODULE = *mut MIB_UDP6ROW_OWNER_MODULE;
STRUCT!{struct MIB_UDP6TABLE_OWNER_MODULE {
    dwNumEntries: DWORD,
    table: [MIB_UDP6ROW_OWNER_MODULE; ANY_SIZE],
}}
pub type PMIB_UDP6TABLE_OWNER_MODULE = *mut MIB_UDP6TABLE_OWNER_MODULE;
#[inline]
pub fn SIZEOF_UDP6TABLE_OWNER_MODULE(num: usize) -> usize {
    offset_of_table!(MIB_UDP6TABLE_OWNER_MODULE) + num * mem::size_of::<MIB_UDP6ROW_OWNER_MODULE>()
        + ALIGN_SIZE
}
STRUCT!{struct MIB_UDPSTATS {
    dwInDatagrams: DWORD,
    dwNoPorts: DWORD,
    dwInErrors: DWORD,
    dwOutDatagrams: DWORD,
    dwNumAddrs: DWORD,
}}
pub type PMIB_UDPSTATS = *mut MIB_UDPSTATS;
STRUCT!{struct MIB_UDPSTATS2 {
    dw64InDatagrams: DWORD64,
    dwNoPorts: DWORD,
    dwInErrors: DWORD,
    dw64OutDatagrams: DWORD64,
    dwNumAddrs: DWORD,
}}
pub type PMIB_UDPSTATS2 = *mut MIB_UDPSTATS2;
