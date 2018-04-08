// Copyright © 2018 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::{DWORD, INT, UINT};
use um::winnt::DWORDLONG;
pub type ARGB = DWORD;
pub type ARGB64 = DWORDLONG;
pub const ALPHA_SHIFT: DWORD = 24;
pub const RED_SHIFT: DWORD = 16;
pub const GREEN_SHIFT: DWORD = 8;
pub const BLUE_SHIFT: DWORD = 0;
pub const ALPHA_MASK: ARGB = (0xff << ALPHA_SHIFT);
pub type PixelFormat = INT;
pub const PixelFormatIndexed: PixelFormat = 0x00010000;
pub const PixelFormatGDI: PixelFormat = 0x00020000;
pub const PixelFormatAlpha: PixelFormat = 0x00040000;
pub const PixelFormatPAlpha: PixelFormat = 0x00080000;
pub const PixelFormatExtended: PixelFormat = 0x00100000;
pub const PixelFormatCanonical: PixelFormat = 0x00200000;
pub const PixelFormatUndefined: PixelFormat = 0;
pub const PixelFormatDontCare: PixelFormat = 0;
pub const PixelFormat1bppIndexed: PixelFormat = (1 | ( 1 << 8) | PixelFormatIndexed | PixelFormatGDI);
pub const PixelFormat4bppIndexed: PixelFormat = (2 | ( 4 << 8) | PixelFormatIndexed | PixelFormatGDI);
pub const PixelFormat8bppIndexed: PixelFormat = (3 | ( 8 << 8) | PixelFormatIndexed | PixelFormatGDI);
pub const PixelFormat16bppGrayScale: PixelFormat = (4 | (16 << 8) | PixelFormatExtended);
pub const PixelFormat16bppRGB555: PixelFormat = (5 | (16 << 8) | PixelFormatGDI);
pub const PixelFormat16bppRGB565: PixelFormat = (6 | (16 << 8) | PixelFormatGDI);
pub const PixelFormat16bppARGB1555: PixelFormat = (7 | (16 << 8) | PixelFormatAlpha | PixelFormatGDI);
pub const PixelFormat24bppRGB: PixelFormat = (8 | (24 << 8) | PixelFormatGDI);
pub const PixelFormat32bppRGB: PixelFormat = (9 | (32 << 8) | PixelFormatGDI);
pub const PixelFormat32bppARGB: PixelFormat = (10 | (32 << 8) | PixelFormatAlpha | PixelFormatGDI | PixelFormatCanonical);
pub const PixelFormat32bppPARGB: PixelFormat = (11 | (32 << 8) | PixelFormatAlpha | PixelFormatPAlpha | PixelFormatGDI);
pub const PixelFormat48bppRGB: PixelFormat = (12 | (48 << 8) | PixelFormatExtended);
pub const PixelFormat64bppARGB: PixelFormat = (13 | (64 << 8) | PixelFormatAlpha  | PixelFormatCanonical | PixelFormatExtended);
pub const PixelFormat64bppPARGB: PixelFormat = (14 | (64 << 8) | PixelFormatAlpha  | PixelFormatPAlpha | PixelFormatExtended);
pub const PixelFormat32bppCMYK: PixelFormat = (15 | (32 << 8));
pub const PixelFormatMax: PixelFormat = 16;
#[inline]
pub fn GetPixelFormatSize(pixfmt: PixelFormat) -> UINT {
    (pixfmt as UINT >> 8) & 0xff
}
#[inline]
pub fn IsIndexedPixelFormat(pixfmt: PixelFormat) -> bool {
    (pixfmt & PixelFormatIndexed) != 0
}
#[inline]
pub fn IsAlphaPixelFormat(pixfmt: PixelFormat) -> bool {
    (pixfmt & PixelFormatAlpha) != 0
}
#[inline]
pub fn IsExtendedPixelFormat(pixfmt: PixelFormat) -> bool {
    (pixfmt & PixelFormatExtended) != 0
}
#[inline]
pub fn IsCanonicalPixelFormat(pixfmt: PixelFormat) -> bool {
    (pixfmt & PixelFormatCanonical) != 0
}
ENUM!{enum PaletteType {
    PaletteTypeCustom = 0,
    PaletteTypeOptimal = 1,
    PaletteTypeFixedBW = 2,
    PaletteTypeFixedHalftone8 = 3,
    PaletteTypeFixedHalftone27 = 4,
    PaletteTypeFixedHalftone64 = 5,
    PaletteTypeFixedHalftone125 = 6,
    PaletteTypeFixedHalftone216 = 7,
    PaletteTypeFixedHalftone252 = 8,
    PaletteTypeFixedHalftone256 = 9,
}}
ENUM!{enum DitherType {
    DitherTypeNone = 0,
    DitherTypeSolid = 1,
    DitherTypeOrdered4x4 = 2,
    DitherTypeOrdered8x8 = 3,
    DitherTypeOrdered16x16 = 4,
    DitherTypeSpiral4x4 = 5,
    DitherTypeSpiral8x8 = 6,
    DitherTypeDualSpiral4x4 = 7,
    DitherTypeDualSpiral8x8 = 8,
    DitherTypeErrorDiffusion = 9,
    DitherTypeMax = 10,
}}
ENUM!{enum PaletteFlags {
    PaletteFlagsHasAlpha = 0x0001,
    PaletteFlagsGrayScale = 0x0002,
    PaletteFlagsHalftone = 0x0004,
}}
STRUCT!{struct ColorPalette {
    Flags: UINT,
    Count: UINT,
    Entries: [ARGB; 1],
}}
