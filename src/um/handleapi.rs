// Copyright © 2016-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! handleapi include file
// Done as of 10.0.14393.0
use shared::minwindef::{BOOL, DWORD, LPDWORD, LPHANDLE};
use um::winnt::HANDLE;

pub const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE;
EXTERN!{stdcall fn CloseHandle(
    hObject: HANDLE
) -> BOOL}
EXTERN!{stdcall fn DuplicateHandle(
    hSourceProcessHandle: HANDLE,
    hSourceHandle: HANDLE,
    hTargetProcessHandle: HANDLE,
    lpTargetHandle: LPHANDLE,
    dwDesiredAccess: DWORD,
    bInheritHandle: BOOL,
    dwOptions: DWORD
) -> BOOL}
EXTERN!{stdcall fn CompareObjectHandles(
    hFirstObjectHandle: HANDLE,
    hSecondObjectHandle: HANDLE
) -> BOOL}
EXTERN!{stdcall fn GetHandleInformation(
    hObject: HANDLE,
    lpdwFlags: LPDWORD
) -> BOOL}
EXTERN!{stdcall fn SetHandleInformation(
    hObject: HANDLE,
    dwMask: DWORD,
    dwFlags: DWORD
) -> BOOL}
