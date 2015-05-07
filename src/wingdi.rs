// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! GDI procedure declarations, constant definitions and macros
//1438
pub const LF_FACESIZE: usize = 32;

#[repr(C)] #[derive(Copy, Clone)]
pub struct LOGFONTA {
    pub lfHeight: ::LONG,
    pub lfWidth: ::LONG,
    pub lfEscapement: ::LONG,
    pub lfOrientation: ::LONG,
    pub lfWeight: ::LONG,
    pub lfItalic: ::BYTE,
    pub lfUnderline: ::BYTE,
    pub lfStrikeOut: ::BYTE,
    pub lfCharSet: ::BYTE,
    pub lfOutPrecision: ::BYTE,
    pub lfClipPrecision: ::BYTE,
    pub lfQuality: ::BYTE,
    pub lfPitchAndFamily: ::BYTE,
    pub lfFaceName: [::CHAR; LF_FACESIZE],
}
pub type LPLOGFONTA = *mut LOGFONTA;

#[repr(C)] #[derive(Copy, Clone)]
pub struct LOGFONTW {
    pub lfHeight: ::LONG,
    pub lfWidth: ::LONG,
    pub lfEscapement: ::LONG,
    pub lfOrientation: ::LONG,
    pub lfWeight: ::LONG,
    pub lfItalic: ::BYTE,
    pub lfUnderline: ::BYTE,
    pub lfStrikeOut: ::BYTE,
    pub lfCharSet: ::BYTE,
    pub lfOutPrecision: ::BYTE,
    pub lfClipPrecision: ::BYTE,
    pub lfQuality: ::BYTE,
    pub lfPitchAndFamily: ::BYTE,
    pub lfFaceName: [::WCHAR; LF_FACESIZE],
}
pub type LPLOGFONTW = *mut LOGFONTW;

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
