// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! ApiSet Contract for api-ms-win-core-heap-l1
use shared::basetsd::SIZE_T;
use shared::minwindef::DWORD;
STRUCT!{struct HEAP_SUMMARY {
    cb: DWORD,
    cbAllocated: SIZE_T,
    cbCommitted: SIZE_T,
    cbReserved: SIZE_T,
    cbMaxReserve: SIZE_T,
}}
pub type PHEAP_SUMMARY = *mut HEAP_SUMMARY;
pub type LPHEAP_SUMMARY = PHEAP_SUMMARY;
