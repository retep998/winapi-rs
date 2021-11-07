// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! this ALWAYS GENERATED file contains the definitions for the interfaces
use ctypes::c_schar;
use shared::guiddef::GUID;
use shared::minwindef::{BYTE, UINT, USHORT};
use um::winnt::{LPCWSTR, LPWSTR};
STRUCT!{#[repr(packed)] struct SHITEMID {
    cb: USHORT,
    abID: [BYTE; 1],
}}
pub type LPSHITEMID = *mut SHITEMID;
pub type LPCSHITEMID = *const SHITEMID;
STRUCT!{#[repr(packed)] struct ITEMIDLIST {
    mkid: SHITEMID,
}}
pub type ITEMIDLIST_RELATIVE = ITEMIDLIST;
pub type ITEMID_CHILD = ITEMIDLIST;
pub type ITEMIDLIST_ABSOLUTE = ITEMIDLIST;
pub type LPITEMIDLIST = *mut ITEMIDLIST;
pub type LPCITEMIDLIST = *const ITEMIDLIST;
pub type PIDLIST_ABSOLUTE = *mut ITEMIDLIST_ABSOLUTE;
pub type PCIDLIST_ABSOLUTE = *const ITEMIDLIST_ABSOLUTE;
pub type PCUIDLIST_ABSOLUTE = *const ITEMIDLIST_ABSOLUTE;
pub type PIDLIST_RELATIVE = *mut ITEMIDLIST_RELATIVE;
pub type PCIDLIST_RELATIVE = *const ITEMIDLIST_RELATIVE;
pub type PUIDLIST_RELATIVE = *mut ITEMIDLIST_RELATIVE;
pub type PCUIDLIST_RELATIVE = *const ITEMIDLIST_RELATIVE;
pub type PITEMID_CHILD = *mut ITEMID_CHILD;
pub type PCITEMID_CHILD = *const ITEMID_CHILD;
pub type PUITEMID_CHILD = *mut ITEMID_CHILD;
pub type PCUITEMID_CHILD = *const ITEMID_CHILD;
pub type PCUITEMID_CHILD_ARRAY = *const PCUITEMID_CHILD;
pub type PCUIDLIST_RELATIVE_ARRAY = *const PCUIDLIST_RELATIVE;
pub type PCIDLIST_ABSOLUTE_ARRAY = *const PCIDLIST_ABSOLUTE;
pub type PCUIDLIST_ABSOLUTE_ARRAY = *const PCUIDLIST_ABSOLUTE;
STRUCT!{struct COMDLG_FILTERSPEC {
    pszName: LPCWSTR,
    pszSpec: LPCWSTR,
}}
pub type KNOWNFOLDERID = GUID;
pub type REFKNOWNFOLDERID = *const KNOWNFOLDERID;
ENUM!{enum STRRET_TYPE {
    STRRET_WSTR     = 0,
    STRRET_OFFSET   = 0x1,
    STRRET_CSTR     = 0x2,
}}
UNION!{union STRRET_u {
    [c_schar; 260],
    pOleStr pOleStrMut: LPWSTR,
    uOffset uOffsetMut: UINT,
    cStr cStrMut: [c_schar; 260],
}}
STRUCT!{struct STRRET {
    uType: UINT,
    u: STRRET_u,
}}
