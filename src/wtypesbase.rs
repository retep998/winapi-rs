// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//114
pub type OLECHAR = ::WCHAR;
pub type LPOLESTR = *mut OLECHAR;
pub type LPCOLESTR = *const OLECHAR;
//147
pub type DOUBLE = ::c_double;
//281
pub type SCODE = ::LONG;
pub type PSCODE = *mut SCODE;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BLOB {
    pub cbSize: ::ULONG,
    pub pBlobData: *mut ::BYTE,
}
pub type LPBLOB = *mut BLOB;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct FLAGGED_WORD_BLOB {
    pub fFlags: ::ULONG,
    pub clSize: ::ULONG,
    pub asData: [::c_ushort; 1],
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BYTE_SIZEDARR {
    pub clSize: ::ULONG,
    pub pData: *mut ::BYTE,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct WORD_SIZEDARR {
    pub clSize: ::ULONG,
    pub pData: *mut ::c_ushort,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DWORD_SIZEDARR {
    pub clSize: ::ULONG,
    pub pData: *mut ::ULONG,
}
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HYPER_SIZEDARR {
    pub clSize: ::ULONG,
    pub pData: *mut i64,
}
