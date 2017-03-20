// Copyright © 2016-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! GUID definition
use ctypes::{c_uchar, c_ulong, c_ushort};
STRUCT!{struct GUID {
    Data1: c_ulong,
    Data2: c_ushort,
    Data3: c_ushort,
    Data4: [c_uchar; 8],
}}
pub type LPGUID = *mut GUID;
pub type LPCGUID = *const GUID;
pub type IID = GUID;
pub type LPIID = *mut IID;
pub use self::IsEqualGUID as IsEqualIID;
pub type CLSID = GUID;
pub type LPCLSID = *mut CLSID;
pub use self::IsEqualGUID as IsEqualCLSID;
pub type FMTID = GUID;
pub type LPFMTID = *mut FMTID;
pub use self::IsEqualGUID as IsEqualFMTID;
pub type REFGUID = *const GUID;
pub type REFIID = *const IID;
pub type REFCLSID = *const IID;
pub type REFFMTID = *const IID;
#[inline]
pub fn IsEqualGUID(g1: &GUID, g2: &GUID) -> bool {
    (g1.Data1, g1.Data2, g1.Data3, g1.Data4) == (g2.Data1, g2.Data2, g2.Data3, g2.Data4)
}
