// Copyright © 2016-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

// TODO:It is a minimal implementation.

use shared::minwindef::{ULONG, DWORD};
use shared::wtypesbase::{LPOLESTR, LPCOLESTR};
use shared::wtypes::{VARTYPE, CLIPFORMAT};
use shared::ntdef::HRESULT;
use shared::guiddef::CLSID;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::oaidl::{VARIANT, IErrorLog};

ENUM!{enum PROPBAG2_TYPE {
    PROPBAG2_TYPE_UNDEFINED = 0,
    PROPBAG2_TYPE_DATA = 1,
    PROPBAG2_TYPE_URL = 2,
    PROPBAG2_TYPE_OBJECT = 3,
    PROPBAG2_TYPE_STREAM = 4,
    PROPBAG2_TYPE_STORAGE = 5,
    PROPBAG2_TYPE_MONIKER = 6,
}}
STRUCT!{struct  PROPBAG2 {
    dwType: DWORD,
    vt: VARTYPE,
    cfType: CLIPFORMAT,
    dwHint: DWORD,
    pstrName: LPOLESTR,
    clsid: CLSID,
}}
RIDL!(#[uuid(0x22F55882, 0x280B, 0x11d0, 0xA8, 0xA9, 0x00, 0xA0, 0xC9, 0x0C, 0x20, 0x04)]
interface IPropertyBag2(IPropertyBag2Vtbl): IUnknown(IUnknownVtbl) {
    fn Read(
        cProperties: ULONG,
        pPropBag: *const PROPBAG2,
        pErrLog: *const IErrorLog,
        pvarValue: *mut VARIANT,
        phrError: *mut HRESULT
    ) -> HRESULT,
    fn Write(
        cProperties: ULONG,
        pPropBag: *const PROPBAG2,
        pvarValue: *const VARIANT
    ) -> HRESULT,
    fn CountProperties(
        pcProperties: *mut ULONG
    ) -> HRESULT,
    fn GetPropertyInfo(
        iProperty: ULONG,
        cProperties: ULONG,
        pPropBag: *mut PROPBAG2,
        pcProperties: *mut ULONG
    ) -> HRESULT,
    fn LoadObject(
        pstrName: LPCOLESTR,
        dwHint: DWORD,
        pUnkObject: *const IUnknown,
        pErrLog: *const IErrorLog
    ) -> HRESULT
});
pub type LPPROPERTYBAG2 = *mut IPropertyBag2;
