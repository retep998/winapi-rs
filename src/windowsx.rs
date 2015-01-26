// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! Macro APIs, window message crackers, and control APIs
//1233
pub fn GET_X_LPARAM(lp: ::LPARAM) -> ::c_int {
	::LOWORD(lp as ::DWORD) as ::c_int
}
pub fn GET_Y_LPARAM(lp: ::LPARAM) -> ::c_int {
	::HIWORD(lp as ::DWORD) as ::c_int
}
