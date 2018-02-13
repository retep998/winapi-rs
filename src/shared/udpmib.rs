// Copyright © 2018 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

// #include <winapifamily.h>

use ctypes::*;
use shared::minwindef::*;
use shared::basetsd::*;
use shared::ntdef::*;
use shared::ws2def::*;
use shared::guiddef::GUID;
use um::minwinbase::{
    OVERLAPPED, LPOVERLAPPED, 
};

pub const TCPIP_OWNING_MODULE_SIZE: usize = 16;

const ANY_SIZE: usize = 1;


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


UNION!{union MIB_UDPROW_OWNER_MODULE_u {
    s: c_int,
    dwFlags: c_int,
}}
BITFIELD!(MIB_UDPROW_OWNER_MODULE_u s: c_int [
    SpecificPortBind set_SpecificPortBind[0..1],
])
STRUCT!{struct MIB_UDPROW_OWNER_MODULE {
    dwLocalAddr: DWORD,
    dwLocalPort: DWORD,
    dwOwningPid: DWORD,
    liCreateTimestamp: LARGE_INTEGER,
    u: MIB_UDPROW_OWNER_MODULE_u,
    OwningModuleInfo: [ULONGULONG; TCPIP_OWNING_MODULE_SIZE],
}}
pub type PMIB_UDPROW_OWNER_MODULE = *mut MIB_UDPROW_OWNER_MODULE;

STRUCT!{struct MIB_UDPTABLE_OWNER_MODULE {
    dwNumEntries: DWORD,
    table: [MIB_UDPROW_OWNER_MODULE; ANY_SIZE],
}}
pub type PMIB_UDPTABLE_OWNER_MODULE = *mut MIB_UDPTABLE_OWNER_MODULE;

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

STRUCT!{struct MIB_UDP6ROW_OWNER_PID {
    ucLocalAddr: [UCHAR; 16],
    dwLocalScopeId: DWORD,
    dwLocalPort: DWORD,
    dwOwningPid: DWORD,
}}
pub type PMIB_UDP6ROW_OWNER_PID = *mut MIB_UDP6ROW_OWNER_PID;

STRUCT!{struct MIB_UDP6TABLE_OWNER_PID {
    dwNumEntries: DWORD,
    // _Field_size_(dwNumEntries)
    table: [MIB_UDP6ROW_OWNER_PID; ANY_SIZE],
}}
pub type PMIB_UDP6TABLE_OWNER_PID = *mut MIB_UDP6TABLE_OWNER_PID;

UNION!{union MIB_UDP6ROW_OWNER_MODULE_u {
    s: c_int,
    dwFlags: c_int,
}}
BITFIELD!(MIB_UDP6ROW_OWNER_MODULE_u s: c_int [
    SpecificPortBind set_SpecificPortBind[0..1],
])
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

STRUCT!{struct MIB_UDPSTATS {
    dwInDatagrams: DWORD,
    dwNoPorts: DWORD,
    dwInErrors: DWORD,
    dwOutDatagrams: DWORD,
    dwNumAddrs: DWORD,
}}
pub type PMIB_UDPSTATS = *mut MIB_UDPSTATS;

// if (NTDDI_VERSION >= NTDDI_WIN10_RS3)
STRUCT!{struct MIB_UDPSTATS2 {
    dw64InDatagrams: DWORD64,
    dwNoPorts: DWORD,
    dwInErrors: DWORD,
    dw64OutDatagrams: DWORD64,
    dwNumAddrs: DWORD,
}}
pub type PMIB_UDPSTATS2 = *mut MIB_UDPSTATS2;

