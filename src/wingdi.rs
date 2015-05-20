// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! GDI procedure declarations, constant definitions and macros

//121
// fnCombineMode values for CombineRgn
pub const RGN_AND: ::c_int = 1;
pub const RGN_OR: ::c_int = 2;
pub const RGN_XOR: ::c_int = 3;
pub const RGN_DIFF: ::c_int = 4;
pub const RGN_COPY: ::c_int = 5;
pub const RGN_MIN: ::c_int = RGN_AND;
pub const RGN_MAX: ::c_int = RGN_COPY;


//1438
pub const LF_FACESIZE: usize = 32;
//1595
#[inline]
pub fn RGB (r: ::BYTE, g: ::BYTE, b: ::BYTE) -> ::COLORREF {
  r as ::COLORREF | ((g as ::COLORREF) << 8) | ((b as ::COLORREF) << 16)
}

// this type is weird because it's a hacky "unsized type"
#[repr(C)]
pub struct RGNDATA;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PALETTEENTRY {
    peRed: ::BYTE,
    peGreen: ::BYTE,
    peBlue: ::BYTE,
    peFlags: ::BYTE
}
//3581
pub type LINEDDAPROC = Option<unsafe extern "system" fn(::c_int, ::c_int, ::LPARAM)>;
