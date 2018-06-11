// Copyright © 2018 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! theming API
use ctypes::c_int;
use shared::minwindef::BOOL;
use shared::windef::HWND;
use um::winnt::{HANDLE, LONG};
pub type HTHEME = HANDLE;
extern "system" {
    pub fn BeginPanningFeedback(hwnd: HWND) -> BOOL;
    pub fn UpdatePanningFeedback(
        hwnd: HWND,
        lTotalOverpanOffsetX: LONG,
        lTotalOverpanOffsetY: LONG,
        fInInertia: BOOL,
    ) -> BOOL;
    pub fn EndPanningFeedback(hwnd: HWND, fAnimateBack: BOOL) -> BOOL;
}
ENUM!{enum TA_PROPERTY {
    TAP_FLAGS = 0,
    TAP_TRANSFORMCOUNT = 1,
    TAP_STAGGERDELAY = 2,
    TAP_STAGGERDELAYCAP = 3,
    TAP_STAGGERDELAYFACTOR = 4,
    TAP_ZORDER = 5,
}}
ENUM!{enum TA_PROPERTY_FLAG {
    TAPF_NONE = 0x0,
    TAPF_HASSTAGGER = 0x1,
    TAPF_ISRTLAWARE = 0x2,
    TAPF_ALLOWCOLLECTION = 0x4,
    TAPF_HASBACKGROUND = 0x8,
    TAPF_HASPERSPECTIVE = 0x10,
}}
STRUCT!{struct MARGINS {
    cxLeftWidth: c_int,
    cxRightWidth: c_int,
    cyTopHeight: c_int,
    cyBottomHeight: c_int,
}}
pub type PMARGINS = *mut MARGINS;
