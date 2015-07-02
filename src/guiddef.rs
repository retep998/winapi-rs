// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
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
