// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! Procedure declarations, constant definitions, and macros for the NLS component.
use shared::minwindef::{BOOL, HRGN, DWORD};

STRUCT!{struct DWM_BLURBEHIND {
    dwFlags: DWORD,
    fEnable: BOOL,
    hRgnBlur: HRGN,
    fTransitionOnMaximized: BOOL,
}}
