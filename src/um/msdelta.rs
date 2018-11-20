// Copyright © 2018 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Delta Compression Engine API.
use ctypes::__int64;
use shared::basetsd::SIZE_T;
use shared::minwindef::{BOOL, DWORD, FILETIME, LPCVOID, LPVOID, UCHAR};
use um::wincrypt::ALG_ID;
use um::winnt::{LPCSTR, LPCWSTR};
UNION!{union DELTA_INPUT_U {
    [u32; 1],
    lpcStart lpcStart_mut: LPCVOID,
    lpStart lpStart_mut: LPVOID,
}}
STRUCT!{struct DELTA_INPUT {
    uSize: SIZE_T,
    Editable: bool,
    u: DELTA_INPUT_U,
}}
pub type LPDELTA_INPUT = *mut DELTA_INPUT;
pub type LPCDELTA_INPUT = *const DELTA_INPUT;
STRUCT!{struct DELTA_OUTPUT {
    lpStart: LPVOID,
    uSize: SIZE_T,
}}
pub type LPDELTA_OUTPUT = *mut DELTA_OUTPUT;
pub type LPCDELTA_OUTPUT = *const DELTA_OUTPUT;
pub type DELTA_FILE_TYPE = __int64;
// TODO: Add DELTA_FILE_* macros.
pub type DELTA_FLAG_TYPE = __int64;
// TODO: Add DELTA_FLAG_* macros;
pub const DELTA_MAX_HASH_SIZE: usize = 32;
STRUCT!{struct DELTA_HASH {
    HashSize: DWORD,
    HashValue: [UCHAR; DELTA_MAX_HASH_SIZE],
}}
pub type LPDELTA_HASH = *mut DELTA_HASH;
pub type LPCDELTA_HASH = *const DELTA_HASH;
STRUCT!{struct DELTA_HEADER_INFO {
    FileTypeSet: DELTA_FILE_TYPE,
    FileType: DELTA_FILE_TYPE,
    Flags: DELTA_FLAG_TYPE,
    TargetSize: SIZE_T,
    TargetFileTime: FILETIME,
    TargetHashAlgId: ALG_ID,
    TargetHash: DELTA_HASH,
}}
pub type LPDELTA_HEADER_INFO = *mut DELTA_HEADER_INFO;
pub type LPCDELTA_HEADER_INFO = *const DELTA_HEADER_INFO;
extern "system" {
    pub fn ApplyDeltaA(
        ApplyFlags: DELTA_FLAG_TYPE,
        lpSourceName: LPCSTR,
        lpDeltaName: LPCSTR,
        lpTargetName: LPCSTR,
    ) -> BOOL;
    pub fn ApplyDeltaB(
        ApplyFlags: DELTA_FLAG_TYPE,
        Source: DELTA_INPUT,
        Delta: DELTA_INPUT,
        lpTarget: LPDELTA_OUTPUT,
    ) -> BOOL;
    pub fn ApplyDeltaProvidedB(
        ApplyFlags: DELTA_FLAG_TYPE,
        Source: DELTA_INPUT,
        Delta: DELTA_INPUT,
        lpTarget: LPVOID,
        uTargetSize: SIZE_T,
    ) -> BOOL;
    pub fn ApplyDeltaW(
        ApplyFlags: DELTA_FLAG_TYPE,
        lpSourceName: LPCWSTR,
        lpDeltaName: LPCWSTR,
        lpTargetName: LPCWSTR,
    ) -> BOOL;
    pub fn CreateDeltaA(
        FileTypeSet: DELTA_FILE_TYPE,
        SetFlags: DELTA_FLAG_TYPE,
        ResetFlags: DELTA_FLAG_TYPE,
        lpSourceName: LPCSTR,
        lpTargetName: LPCSTR,
        lpSourceOptionsName: LPCSTR,
        lpTargetOptionsName: LPCSTR,
        GlobalOptions: DELTA_INPUT,
        lpTargetFileTime: *const FILETIME,
        HashAlgId: ALG_ID,
        lpDeltaName: LPCSTR,
    ) -> BOOL;
    pub fn CreateDeltaB(
        FileTypeSet: DELTA_FILE_TYPE,
        SetFlags: DELTA_FLAG_TYPE,
        ResetFlags: DELTA_FLAG_TYPE,
        Source: DELTA_INPUT,
        Target: DELTA_INPUT,
        SourceOptions: DELTA_INPUT,
        TargetOptions: DELTA_INPUT,
        GlobalOptions: DELTA_INPUT ,
        lpTargetFileTime: *const FILETIME,
        HashAlgId: ALG_ID,
        lpDelta: LPDELTA_OUTPUT,
    ) -> BOOL;
    pub fn CreateDeltaW(
        FileTypeSet: DELTA_FILE_TYPE,
        SetFlags: DELTA_FLAG_TYPE,
        ResetFlags: DELTA_FLAG_TYPE,
        lpSourceName: LPCWSTR,
        lpTargetName: LPCWSTR,
        lpSourceOptionsName: LPCWSTR,
        lpTargetOptionsName: LPCWSTR,
        GlobalOptions: DELTA_INPUT,
        lpTargetFileTime: *const FILETIME,
        HashAlgId: ALG_ID,
        lpDeltaName: LPCWSTR,
    ) -> BOOL;
    pub fn DeltaFree(
        lpMemory: LPVOID,
    ) -> BOOL;
    pub fn DeltaNormalizeProvidedB(
        FileTypeSet: DELTA_FILE_TYPE,
        NormalizeFlags: DELTA_FLAG_TYPE,
        NormalizeOptions: DELTA_INPUT,
        lpSource: LPVOID,
        uSourceSize: SIZE_T,
    ) -> BOOL;
    pub fn GetDeltaInfoA(
        Delta: DELTA_INPUT,
        lpHeaderInfo: LPDELTA_HEADER_INFO,
    ) -> BOOL;
    pub fn GetDeltaInfoB(
        Delta: DELTA_INPUT,
        lpHeaderInfo: LPDELTA_HEADER_INFO,
    ) -> BOOL;
    pub fn GetDeltaInfoW(
        lpDeltaName: LPCWSTR,
        lpHeaderInfo: LPDELTA_HEADER_INFO,
    ) -> BOOL;
    pub fn GetDeltaSignatureA(
        FileTypeSet: DELTA_FILE_TYPE,
        HashAlgId: ALG_ID,
        lpSourceName: LPCSTR,
        lpHash: LPDELTA_HASH,
    ) -> BOOL;
    pub fn GetDeltaSignatureB(
        FileTypeSet: DELTA_FILE_TYPE,
        HashAlgId: ALG_ID,
        Source: DELTA_INPUT,
        lpHash: LPDELTA_HASH,
    ) -> BOOL;
    pub fn GetDeltaSignatureW(
        FileTypeSet: DELTA_FILE_TYPE,
        HashAlgId: ALG_ID,
        lpSourceName: LPCWSTR,
        lpHash: LPDELTA_HASH,
    ) -> BOOL;
}
