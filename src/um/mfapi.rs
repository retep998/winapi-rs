// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of mfapi.h
use shared::basetsd::UINT32;
use shared::minwindef::{DWORD, ULONG};
use um::mfobjects::{IMFAttributes, IMFSample};
use um::winnt::HRESULT;
pub const MF_SDK_VERSION: ULONG = 0x0002;
pub const MF_API_VERSION: ULONG = 0x0070;
pub const MF_VERSION: ULONG = (MF_SDK_VERSION << 16 | MF_API_VERSION);
pub const MFSTARTUP_NOSOCKET: DWORD = 0x1;
pub const MFSTARTUP_LITE: DWORD = MFSTARTUP_NOSOCKET;
pub const MFSTARTUP_FULL: DWORD = 0;
extern "system" {
    pub fn MFStartup(
        Version: ULONG,
        dwFlags: DWORD,
    ) -> HRESULT;
    pub fn MFShutdown() -> HRESULT;
    pub fn MFCreateAttributes(
        ppMFAttributes: *mut *mut IMFAttributes,
        cInitialSize: UINT32,
    ) -> HRESULT;
}
extern "system" {
    pub fn MFCreateSample(
        ppIMFSample: *mut *mut IMFSample
    ) -> HRESULT;
}
