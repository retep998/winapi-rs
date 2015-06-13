// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! ApiSet Contract for api-ms-win-core-heap-l1
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HEAP_SUMMARY {
    pub cb: ::DWORD,
    pub cbAllocated: ::SIZE_T,
    pub cbCommitted: ::SIZE_T,
    pub cbReserved: ::SIZE_T,
    pub cbMaxReserve: ::SIZE_T,
}
pub type PHEAP_SUMMARY = *mut HEAP_SUMMARY;
pub type LPHEAP_SUMMARY = PHEAP_SUMMARY;
