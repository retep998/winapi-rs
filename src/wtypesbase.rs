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
