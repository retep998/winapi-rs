// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
//! Data Protection API Prototypes and Definitions
pub const szFORCE_KEY_PROTECTION: &'static str = "ForceKeyProtection";
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CRYPTPROTECT_PROMPTSTRUCT {
    pub cbSize: ::DWORD,
    pub dwPromptFlags: ::DWORD,
    pub hwndApp: ::HWND,
    pub szPrompt: ::LPCWSTR,
}
pub type PCRYPTPROTECT_PROMPTSTRUCT = *mut CRYPTPROTECT_PROMPTSTRUCT;
