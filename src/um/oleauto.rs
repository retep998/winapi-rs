// Copyright © 2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of OleAuto.h

use shared::wtypes::{BSTR};
use shared::wtypesbase::{LPCOLESTR, OLECHAR};
use um::oaidl::{DISPID_UNKNOWN, ITypeLib};
use um::winnt::{HRESULT, LONG};

pub type DISPID = LONG;
pub type MEMBERID = DISPID;
extern "system" {
    pub fn SysAllocString(
        psz: *const OLECHAR,
    ) -> BSTR;
    pub fn SysFreeString(
        bstrString: BSTR,
    ) -> ();
}
pub const MEMBERID_NIL: MEMBERID = DISPID_UNKNOWN;

ENUM!{enum REGKIND {
    REGKIND_DEFAULT = 0,
    REGKIND_REGISTER,
    REGKIND_NONE,
}}
extern "system" {
    pub fn LoadTypeLibEx(
        szFile: LPCOLESTR,
        regKind: REGKIND,
        pptlib: *mut *mut ITypeLib,
    ) -> HRESULT;
}
