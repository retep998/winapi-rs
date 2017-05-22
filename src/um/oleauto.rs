// Copyright © 2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of OleAuto.h
use ctypes::c_uint;
use shared::minwindef::{UINT, USHORT, WORD};
use shared::wtypes::{BSTR, VARTYPE};
use shared::wtypesbase::{LPCOLESTR, OLECHAR};
use um::oaidl::{DISPID_UNKNOWN, ITypeLib, VARIANT, VARIANTARG};
use um::winnt::{HRESULT, INT, LCID, LONG, LPCSTR};
extern "system" {
    pub fn SysAllocString(
        psz: *const OLECHAR,
    ) -> BSTR;
    pub fn SysReAllocString(
        pbstr: *mut BSTR,
        psz: *const OLECHAR,
    ) -> INT;
    pub fn SysAllocStringLen(
        strIn: *const OLECHAR,
        ui: UINT,
    ) -> BSTR;
    pub fn SysReAllocStringLen(
        pbstr: *mut BSTR,
        psz: *const OLECHAR,
        len: c_uint,
    ) -> INT;
    pub fn SysFreeString(
        bstrString: BSTR,
    );
    pub fn SysStringLen(
        pbstr: BSTR,
    ) -> UINT;
    pub fn SysStringByteLen(
        bstr: BSTR,
    ) -> UINT;
    pub fn SysAllocStringByteLen(
        psz: LPCSTR,
        len: UINT,
    ) -> BSTR;
    pub fn VariantInit(
        pvarg: *mut VARIANTARG,
    );
    pub fn VariantClear(
        pvarg: *mut VARIANTARG,
    ) -> HRESULT;
    pub fn VariantCopy(
        pvargDest: *mut VARIANTARG,
        pvargSrc: *const VARIANTARG,
    ) -> HRESULT;
    pub fn VariantCopyInd(
        pvarDest: *mut VARIANT,
        pvargSrc: *const VARIANTARG,
    ) -> HRESULT;
    pub fn VariantChangeType(
        pvargDest: *mut VARIANTARG,
        pvarSrc: *const VARIANTARG,
        wFlags: USHORT,
        vt: VARTYPE,
    ) -> HRESULT;
    pub fn VariantChangeTypeEx(
        pvargDest: *mut VARIANTARG,
        pvarSrc: *const VARIANTARG,
        lcid: LCID,
        wFlags: USHORT,
        vt: VARTYPE,
    ) -> HRESULT;
}
pub type DISPID = LONG;
pub type MEMBERID = DISPID;
pub const MEMBERID_NIL: MEMBERID = DISPID_UNKNOWN;
pub const DISPATCH_METHOD: WORD = 0x1;
pub const DISPATCH_PROPERTYGET: WORD = 0x2;
pub const DISPATCH_PROPERTYPUT: WORD = 0x4;
pub const DISPATCH_PROPERTYPUTREF: WORD = 0x8;
ENUM!{enum REGKIND {
    REGKIND_DEFAULT = 0,
    REGKIND_REGISTER,
    REGKIND_NONE,
}}
extern "system" {
    pub fn LoadTypeLibEx(
        szFile: LPCOLESTR,
        regkind: REGKIND,
        pptlib: *mut *mut ITypeLib,
    ) -> HRESULT;
}
