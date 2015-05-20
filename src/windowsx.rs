// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! Macro APIs, window message crackers, and control APIs
//1233
pub fn GET_X_LPARAM(lp: ::LPARAM) -> ::c_int {
	::LOWORD(lp as ::DWORD) as i16 as ::c_int
}
pub fn GET_Y_LPARAM(lp: ::LPARAM) -> ::c_int {
	::HIWORD(lp as ::DWORD) as i16 as ::c_int
}
#[test]
fn test_get_x_lparam() {
	assert_eq!(GET_X_LPARAM(0x00001234), 0x1234);
	assert_eq!(GET_X_LPARAM(0x0000ffff), -1);
	assert_eq!(GET_X_LPARAM(::LPARAM::max_value()), -1);
	assert_eq!(GET_X_LPARAM(-1234 as i16 as u16 as ::LPARAM), -1234);
}
#[test]
fn test_get_y_lparam() {
	assert_eq!(GET_Y_LPARAM(0x12340000), 0x1234);
	assert_eq!(GET_Y_LPARAM(0xffff0000), -1);
	assert_eq!(GET_X_LPARAM(::LPARAM::max_value()), -1);
	assert_eq!(GET_Y_LPARAM((-1234 as i16 as u16 as ::LPARAM) << 16), -1234);
}