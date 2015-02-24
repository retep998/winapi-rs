// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! ApiSet Contract for api-ms-win-core-file-l1
pub const CREATE_NEW: ::DWORD = 1;
pub const CREATE_ALWAYS: ::DWORD = 2;
pub const OPEN_EXISTING: ::DWORD = 3;
pub const OPEN_ALWAYS: ::DWORD = 4;
pub const TRUNCATE_EXISTING: ::DWORD = 5;
pub const INVALID_FILE_SIZE: ::DWORD = 0xFFFFFFFF;
pub const INVALID_SET_FILE_POINTER: ::DWORD = -1;
pub const INVALID_FILE_ATTRIBUTES: ::DWORD = -1;
#[repr(C)] #[derive(Clone, Copy, Debug, Default)]
pub struct WIN32_FILE_ATTRIBUTE_DATA {
    pub dwFileAttributes: ::DWORD,
    pub ftCreationTime: ::FILETIME,
    pub ftLastAccessTime: ::FILETIME,
    pub ftLastWriteTime: ::FILETIME,
    pub nFileSizeHigh: ::DWORD,
    pub nFileSizeLow: ::DWORD,
}
pub type LPWIN32_FILE_ATTRIBUTE_DATA = *mut WIN32_FILE_ATTRIBUTE_DATA;
#[repr(C)] #[derive(Clone, Copy, Debug, Default)]
pub struct BY_HANDLE_FILE_INFORMATION {
    pub dwFileAttributes: ::DWORD,
    pub ftCreationTime: ::FILETIME,
    pub ftLastAccessTime: ::FILETIME,
    pub ftLastWriteTime: ::FILETIME,
    pub dwVolumeSerialNumber: ::DWORD,
    pub nFileSizeHigh: ::DWORD,
    pub nFileSizeLow: ::DWORD,
    pub nNumberOfLinks: ::DWORD,
    pub nFileIndexHigh: ::DWORD,
    pub nFileIndexLow: ::DWORD,
}
pub type PBY_HANDLE_FILE_INFORMATION = *mut BY_HANDLE_FILE_INFORMATION;
pub type LPBY_HANDLE_FILE_INFORMATION = *mut BY_HANDLE_FILE_INFORMATION;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CREATEFILE2_EXTENDED_PARAMETERS {
    pub dwSize: ::DWORD,
    pub dwFileAttributes: ::DWORD,
    pub dwFileFlags: ::DWORD,
    pub dwSecurityQosFlags: ::DWORD,
    pub lpSecurityAttributes: ::LPSECURITY_ATTRIBUTES,
    pub hTemplateFile: ::HANDLE,
}
pub type PCREATEFILE2_EXTENDED_PARAMETERS = *mut CREATEFILE2_EXTENDED_PARAMETERS;
pub type LPCREATEFILE2_EXTENDED_PARAMETERS = *mut CREATEFILE2_EXTENDED_PARAMETERS;
