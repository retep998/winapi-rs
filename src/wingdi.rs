// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! GDI procedure declarations, constant definitions and macros
//1438
pub const LF_FACESIZE: usize = 32;

// this type is weird because it's a hacky "unsized type"
#[repr(C)]
pub struct RGNDATA;

#[repr(C)]
#[derive(Copy)]
pub struct PALETTEENTRY {
    peRed: ::BYTE,
    peGreen: ::BYTE,
    peBlue: ::BYTE,
    peFlags: ::BYTE
}