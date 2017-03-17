// Copyright © 2016-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::{DWORD_PTR};
use shared::minwindef::{DWORD, HKEY};
use um::winnt::{LPSTR, LPWSTR, ACCESS_MASK};

pub type REGSAM = ACCESS_MASK;
STRUCT!{struct VALENTA {
    ve_valuename: LPSTR,
    ve_valuelen: DWORD,
    ve_valueptr: DWORD_PTR,
    ve_type: DWORD,
}}
pub type PVALENTA = *mut VALENTA;
STRUCT!{struct VALENTW {
    ve_valuename: LPWSTR,
    ve_valuelen: DWORD,
    ve_valueptr: DWORD_PTR,
    ve_type: DWORD,
}}
pub type PVALENTW = *mut VALENTW;
pub const HKEY_CLASSES_ROOT: HKEY = 0x80000000u32 as i32 as usize as HKEY;
pub const HKEY_CURRENT_USER: HKEY = 0x80000001u32 as i32 as usize as HKEY;
pub const HKEY_LOCAL_MACHINE: HKEY = 0x80000002u32 as i32 as usize as HKEY;
pub const HKEY_USERS: HKEY = 0x80000003u32 as i32 as usize as HKEY;
pub const HKEY_PERFORMANCE_DATA: HKEY = 0x80000004u32 as i32 as usize as HKEY;
pub const HKEY_PERFORMANCE_TEXT: HKEY = 0x80000050u32 as i32 as usize as HKEY;
pub const HKEY_PERFORMANCE_NLSTEXT: HKEY = 0x80000060u32 as i32 as usize as HKEY;
pub const HKEY_CURRENT_CONFIG: HKEY = 0x80000005u32 as i32 as usize as HKEY;
pub const HKEY_DYN_DATA: HKEY = 0x80000006u32 as i32 as usize as HKEY;
pub const HKEY_CURRENT_USER_LOCAL_SETTINGS: HKEY = 0x80000007u32 as i32 as usize as HKEY;
pub const REG_MUI_STRING_TRUNCATE: DWORD = 0x00000001;
pub const RRF_RT_REG_NONE: DWORD = 0x00000001;
pub const RRF_RT_REG_SZ: DWORD = 0x00000002;
pub const RRF_RT_REG_EXPAND_SZ: DWORD = 0x00000004;
pub const RRF_RT_REG_BINARY: DWORD = 0x00000008;
pub const RRF_RT_REG_DWORD: DWORD = 0x00000010;
pub const RRF_RT_REG_MULTI_SZ: DWORD = 0x00000020;
pub const RRF_RT_REG_QWORD: DWORD = 0x00000040;
pub const RRF_RT_DWORD: DWORD = RRF_RT_REG_BINARY | RRF_RT_REG_DWORD;
pub const RRF_RT_QWORD: DWORD = RRF_RT_REG_BINARY | RRF_RT_REG_QWORD;
pub const RRF_RT_ANY: DWORD = 0x0000ffff;
pub const RRF_NOEXPAND: DWORD = 0x10000000;
pub const RRF_ZEROONFAILURE: DWORD = 0x20000000;
