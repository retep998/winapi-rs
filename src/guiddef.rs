// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
use std::fmt;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct GUID {
    pub Data1: ::c_ulong,
    pub Data2: ::c_ushort,
    pub Data3: ::c_ushort,
    pub Data4: [::c_uchar; 8],
}
pub type LPGUID = *mut GUID;
pub type LPCGUID = *const GUID;
pub type IID = GUID;
pub type LPIID = *mut IID;
pub type CLSID = GUID;
pub type LPCLSID = *mut CLSID;
pub type FMTID = GUID;
pub type LPFMTID = *mut FMTID;
pub type REFGUID = *const GUID;
pub type REFIID = *const IID;
pub type REFCLSID = *const IID;
pub type REFFMTID = *const IID;

impl fmt::Display for GUID {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
            self.Data1, self.Data2, self.Data3,
            self.Data4 [0], self.Data4 [1], self.Data4 [2], self.Data4 [3],
            self.Data4 [4], self.Data4 [5], self.Data4 [6], self.Data4 [7])
    }
}
