// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! ApiSet Contract for api-ms-win-core-memory-l1-1-0
use shared::basetsd::{SIZE_T};
use shared::minwindef::{DWORD};
use um::winnt::{
    PVOID, SECTION_MAP_WRITE, SECTION_MAP_READ, SECTION_ALL_ACCESS, SECTION_MAP_EXECUTE_EXPLICIT,
};
pub const FILE_MAP_WRITE: DWORD = SECTION_MAP_WRITE;
pub const FILE_MAP_READ: DWORD = SECTION_MAP_READ;
pub const FILE_MAP_ALL_ACCESS: DWORD = SECTION_ALL_ACCESS;
pub const FILE_MAP_EXECUTE: DWORD = SECTION_MAP_EXECUTE_EXPLICIT;
pub const FILE_MAP_COPY: DWORD = 0x00000001;
pub const FILE_MAP_RESERVE: DWORD = 0x80000000;
ENUM!{enum MEMORY_RESOURCE_NOTIFICATION_TYPE {
    LowMemoryResourceNotification,
    HighMemoryResourceNotification,
}}
STRUCT!{struct WIN32_MEMORY_RANGE_ENTRY {
    VirtualAddress: PVOID,
    NumberOfBytes: SIZE_T,
}}
pub type PWIN32_MEMORY_RANGE_ENTRY = *mut WIN32_MEMORY_RANGE_ENTRY;
pub type PBAD_MEMORY_CALLBACK_ROUTINE = Option<unsafe extern "system" fn()>;
