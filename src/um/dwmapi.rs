// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Procedure declarations, constant definitions, and macros for the NLS component.
use shared::minwindef::{BOOL, DWORD, HRGN};
STRUCT!{struct DWM_BLURBEHIND {
    dwFlags: DWORD,
    fEnable: BOOL,
    hRgnBlur: HRGN,
    fTransitionOnMaximized: BOOL,
}}

pub const DWMWA_NCRENDERING_ENABLED: DWORD = 1;
pub const DWMWA_NCRENDERING_POLICY: DWORD = 2;
pub const DWMWA_TRANSITIONS_FORCEDISABLED: DWORD = 2;
pub const DWMWA_ALLOW_NCPAINT: DWORD = 4;
pub const DWMWA_CAPTION_BUTTON_BOUNDS: DWORD = 5;
pub const DWMWA_NONCLIENT_RTL_LAYOUT: DWORD = 6;
pub const DWMWA_FORCE_ICONIC_REPRESENTATION: DWORD = 7;
pub const DWMWA_FLIP3D_POLICY: DWORD = 8;
pub const DWMWA_EXTENDED_FRAME_BOUNDS: DWORD = 9;
pub const DWMWA_HAS_ICONIC_BITMAP: DWORD = 10;
pub const DWMWA_DISALLOW_PEEK: DWORD = 11;
pub const DWMWA_EXCLUDED_FROM_PEEK: DWORD = 12;
pub const DWMWA_CLOAK: DWORD = 13;
pub const DWMWA_CLOAKED: DWORD = 14;
pub const DWMWA_FREEZE_REPRESENTATION: DWORD = 15;
pub const DWMWA_LAST: DWORD = 16;
