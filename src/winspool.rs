// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
//! Winspool header file
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PRINTER_DEFAULTSA {
    pub pDataType: ::LPSTR,
    pub pDevMode: ::LPDEVMODEA,
    pub DesiredAccess: ::ACCESS_MASK,
}
pub type PPRINTER_DEFAULTSA = *mut PRINTER_DEFAULTSA;
pub type LPPRINTER_DEFAULTSA = *mut PRINTER_DEFAULTSA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PRINTER_DEFAULTSW {
    pub pDataType: ::LPWSTR,
    pub pDevMode: ::LPDEVMODEW,
    pub DesiredAccess: ::ACCESS_MASK,
}
pub type PPRINTER_DEFAULTSW = *mut PRINTER_DEFAULTSW;
pub type LPPRINTER_DEFAULTSW = *mut PRINTER_DEFAULTSW;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PRINTER_OPTIONSA {
    pub cbSize: ::UINT,
    pub dwFlags: ::DWORD,
}
pub type PPRINTER_OPTIONSA = *mut PRINTER_OPTIONSA;
pub type LPPRINTER_OPTIONSA = *mut PRINTER_OPTIONSA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PRINTER_OPTIONSW {
    pub cbSize: ::UINT,
    pub dwFlags: ::DWORD,
}
pub type PPRINTER_OPTIONSW = *mut PRINTER_OPTIONSW;
pub type LPPRINTER_OPTIONSW = *mut PRINTER_OPTIONSW;
