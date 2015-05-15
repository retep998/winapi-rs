// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! Procedure declarations, constant definitions, and macros for the NLS component.
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DWM_BLURBEHIND {
    pub dwFlags: ::DWORD,
    pub fEnable: ::BOOL,
    pub hRgnBlur: ::HRGN,
    pub fTransitionOnMaximized: ::BOOL,
}
