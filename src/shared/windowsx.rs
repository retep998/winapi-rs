// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! Macro APIs, window message crackers, and control APIs
use ctypes::{c_int, c_short};
use shared::minwindef::{DWORD, LOWORD, HIWORD, LPARAM};

//1233
#[inline]
pub fn GET_X_LPARAM(lp: LPARAM) -> c_int {
    LOWORD(lp as DWORD) as c_short as c_int
}
#[inline]
pub fn GET_Y_LPARAM(lp: LPARAM) -> c_int {
    HIWORD(lp as DWORD) as c_short as c_int
}
