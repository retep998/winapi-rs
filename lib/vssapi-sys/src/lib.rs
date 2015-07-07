// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to vssapi.
#![cfg(windows)]
extern crate winapi;
use winapi::*;

// x86_64 will work once rust issue 23216 is fixed.
// Starting link_name with \x01 prevents the name from being mangled further.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
extern "system" {
    #[cfg(target_arch = "x86_64")]
    #[link_name="\x01?CreateVssBackupComponents@@YAJPEAPEAVIVssBackupComponents@@@Z"]
    pub fn CreateVssBackupComponents(ppBackup: *mut *mut IVssBackupComponents) -> HRESULT;
    
    #[cfg(target_arch = "x86")]
    #[link_name="\x01?CreateVssBackupComponents@@YGJPAPAVIVssBackupComponents@@@Z"]
    pub fn CreateVssBackupComponents(ppBackup: *mut *mut IVssBackupComponents) -> HRESULT;
}

