// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Data Protection API Prototypes and Definitions
use shared::minwindef::DWORD;
use shared::windef::HWND;
use um::winnt::LPCWSTR;
pub const szFORCE_KEY_PROTECTION: &'static str = "ForceKeyProtection";
STRUCT!{struct CRYPTPROTECT_PROMPTSTRUCT {
    cbSize: DWORD,
    dwPromptFlags: DWORD,
    hwndApp: HWND,
    szPrompt: LPCWSTR,
}}
pub type PCRYPTPROTECT_PROMPTSTRUCT = *mut CRYPTPROTECT_PROMPTSTRUCT;
