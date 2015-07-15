// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! GDI procedure declarations, constant definitions and macros
//83
pub const SRCCOPY: ::DWORD = 0x00CC0020;
pub const SRCPAINT: ::DWORD = 0x00EE0086;
pub const SRCAND: ::DWORD = 0x008800C6;
pub const SRCINVERT: ::DWORD = 0x00660046;
pub const SRCERASE: ::DWORD = 0x00440328;
pub const NOTSRCCOPY: ::DWORD = 0x00330008;
pub const NOTSRCERASE: ::DWORD = 0x001100A6;
pub const MERGECOPY: ::DWORD = 0x00C000CA;
pub const MERGEPAINT: ::DWORD = 0x00BB0226;
pub const PATCOPY: ::DWORD = 0x00F00021;
pub const PATPAINT: ::DWORD = 0x00FB0A09;
pub const PATINVERT: ::DWORD = 0x005A0049;
pub const DSTINVERT: ::DWORD = 0x00550009;
pub const BLACKNESS: ::DWORD = 0x00000042;
pub const WHITENESS: ::DWORD = 0x00FF0062;
//121
// fnCombineMode values for CombineRgn
pub const RGN_AND: ::c_int = 1;
pub const RGN_OR: ::c_int = 2;
pub const RGN_XOR: ::c_int = 3;
pub const RGN_DIFF: ::c_int = 4;
pub const RGN_COPY: ::c_int = 5;
pub const RGN_MIN: ::c_int = RGN_AND;
pub const RGN_MAX: ::c_int = RGN_COPY;
//572 (Win 7 SDK)
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BITMAP {
    pub bmType: ::LONG,
    pub bmWidth: ::LONG,
    pub bmHeight: ::LONG,
    pub bmWidthBytes: ::LONG,
    pub bmPlanes: ::WORD,
    pub bmBitsPixel: ::WORD,
    pub bmBits: ::LPVOID,
}
pub type PBITMAP = *mut BITMAP;
pub type NPBITMAP = *mut BITMAP;
pub type LPBITMAP = *mut BITMAP;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RGBQUAD {
    pub rgbBlue: ::BYTE,
    pub rgbGreen: ::BYTE,
    pub rgbRed: ::BYTE,
    pub rgbReserved: ::BYTE,
}
pub type LPRGBQUAD = *mut RGBQUAD;
pub const CS_ENABLE: ::DWORD = 0x00000001;
pub const CS_DISABLE: ::DWORD = 0x00000002;
pub const CS_DELETE_TRANSFORM: ::DWORD = 0x00000003;
pub const LCS_SIGNATURE: ::DWORD = 0x5053_4F43; // 'PSOC'
pub const LCS_sRGB: LCSCSTYPE = 0x7352_4742; // 'sRGB'
pub const LCS_WINDOWS_COLOR_SPACE: LCSCSTYPE = 0x5769_6E20; // 'Win '
pub type LCSCSTYPE = ::LONG;
pub const LCS_CALIBRATED_RGB: LCSCSTYPE = 0x00000000;
pub type LCSGAMUTMATCH = ::LONG;
pub const LCS_GM_BUSINESS: LCSGAMUTMATCH = 0x00000001;
pub const LCS_GM_GRAPHICS: LCSGAMUTMATCH = 0x00000002;
pub const LCS_GM_IMAGES: LCSGAMUTMATCH = 0x00000004;
pub const LCS_GM_ABS_COLORIMETRIC: LCSGAMUTMATCH = 0x00000008;
pub const CM_OUT_OF_GAMUT: ::BYTE = 255;
pub const CM_IN_GAMUT: ::BYTE = 0;
pub const ICM_ADDPROFILE: ::UINT = 1;
pub const ICM_DELETEPROFILE: ::UINT = 2;
pub const ICM_QUERYPROFILE: ::UINT = 3;
pub const ICM_SETDEFAULTPROFILE: ::UINT = 4;
pub const ICM_REGISTERICMATCHER: ::UINT = 5;
pub const ICM_UNREGISTERICMATCHER: ::UINT = 6;
pub const ICM_QUERYMATCH: ::UINT = 7;
pub type FXPT16DOT16 = ::c_long;
pub type LPFXPT16DOT16 = *mut ::c_long;
pub type FXPT2DOT30 = ::c_long;
pub type LPFXPT2DOT30 = *mut ::c_long;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CIEXYZ {
    pub ciexyzX: FXPT2DOT30,
    pub ciexyzY: FXPT2DOT30,
    pub ciexyzZ: FXPT2DOT30,
}
pub type LPCIEXYZ = *mut CIEXYZ;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CIEXYZTRIPLE {
    pub ciexyzRed: CIEXYZ,
    pub ciexyzGreen: CIEXYZ,
    pub ciexyzBlue: CIEXYZ,
}
pub type LPCIEXYZTRIPLE = *mut CIEXYZTRIPLE;
//716 (Win 7 SDK)
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BITMAPINFOHEADER {
    pub biSize: ::DWORD,
    pub biWidth: ::LONG,
    pub biHeight: ::LONG,
    pub biPlanes: ::WORD,
    pub biBitCount: ::WORD,
    pub biCompression: ::DWORD,
    pub biSizeImage: ::DWORD,
    pub biXPelsPerMeter: ::LONG,
    pub biYPelsPerMeter: ::LONG,
    pub biClrUsed: ::DWORD,
    pub biClrImportant: ::DWORD,
}
pub type LPBITMAPINFOHEADER = *mut BITMAPINFOHEADER;
pub type PBITMAPINFOHEADER = *mut BITMAPINFOHEADER;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BITMAPV5HEADER {
    pub bV5Size: ::DWORD,
    pub bV5Width: ::LONG,
    pub bV5Height: ::LONG,
    pub bV5Planes: ::WORD,
    pub bV5BitCount: ::WORD,
    pub bV5Compression: ::DWORD,
    pub bV5SizeImage: ::DWORD,
    pub bV5XPelsPerMeter: ::LONG,
    pub bV5YPelsPerMeter: ::LONG,
    pub bV5ClrUsed: ::DWORD,
    pub bV5ClrImportant: ::DWORD,
    pub bV5RedMask: ::DWORD,
    pub bV5GreenMask: ::DWORD,
    pub bV5BlueMask: ::DWORD,
    pub bV5AlphaMask: ::DWORD,
    pub bV5CSType: ::LONG, // LONG to match LOGCOLORSPACE
    pub bV5Endpoints: CIEXYZTRIPLE,
    pub bV5GammaRed: ::DWORD,
    pub bV5GammaGreen: ::DWORD,
    pub bV5GammaBlue: ::DWORD,
    pub bV5Intent: ::LONG, // LONG to match LOGCOLORSPACE
    pub bV5ProfileData: ::DWORD,
    pub bV5ProfileSize: ::DWORD,
    pub bV5Reserved: ::DWORD,
}
pub type LPBITMAPV5HEADER = *mut BITMAPV5HEADER;
pub type PBITMAPV5HEADER = *mut BITMAPV5HEADER;
pub const PROFILE_LINKED: ::LONG = 0x4C49_4E4B; // 'LINK'
pub const PROFILE_EMBEDDED: ::LONG = 0x4D42_4544; // 'MBED'
pub const BI_RGB: ::DWORD = 0;
pub const BI_RLE8: ::DWORD = 1;
pub const BI_RLE4: ::DWORD = 2;
pub const BI_BITFIELDS: ::DWORD = 3;
pub const BI_JPEG: ::DWORD = 4;
pub const BI_PNG: ::DWORD = 5;
#[repr(C)] #[derive(Debug)] #[allow(missing_copy_implementations)]
pub struct BITMAPINFO {
    pub bmiHeader: BITMAPINFOHEADER,
    pub bmiColors: [RGBQUAD; 0],
}
pub type LPBITMAPINFO = *mut BITMAPINFO;
pub type PBITMAPINFO = *mut BITMAPINFO;
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
//1906 (Win 7 SDK)
pub const DIB_RGB_COLORS: ::UINT = 0;
pub const DIB_PAL_COLORS: ::UINT = 1;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RGNDATAHEADER {
    pub dwSize: ::DWORD,
    pub iType: ::DWORD,
    pub nCount: ::DWORD,
    pub nRgnSize: ::DWORD,
    pub rcBound: ::RECT,
}
pub type PRGNDATAHEADER = *mut RGNDATAHEADER;
#[repr(C)] #[derive(Debug)] #[allow(missing_copy_implementations)]
pub struct RGNDATA {
    pub rdh: RGNDATAHEADER,
    pub Buffer: [::c_char; 0],
}
pub type PRGNDATA = *mut RGNDATA;
pub type NPRGNDATA = *mut RGNDATA;
pub type LPRGNDATA = *mut RGNDATA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PALETTEENTRY {
    pub peRed: ::BYTE,
    pub peGreen: ::BYTE,
    pub peBlue: ::BYTE,
    pub peFlags: ::BYTE
}
//2824 (Win 7 SDK)
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ABC {
    pub abcA: ::c_int,
    pub abcB: ::UINT,
    pub abcC: ::c_int,
}
pub type PABC = *mut ABC;
pub type NPABC = *mut ABC;
pub type LPABC = *mut ABC;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ABCFLOAT {
    pub abcfA: ::FLOAT,
    pub abcfB: ::FLOAT,
    pub abcfC: ::FLOAT,
}
pub type PABCFLOAT = *mut ABCFLOAT;
pub type NPABCFLOAT = *mut ABCFLOAT;
pub type LPABCFLOAT = *mut ABCFLOAT;

//3581
pub type LINEDDAPROC = Option<unsafe extern "system" fn(::c_int, ::c_int, ::LPARAM)>;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct XFORM {
    pub eM11: ::FLOAT,
    pub eM12: ::FLOAT,
    pub eM21: ::FLOAT,
    pub eM22: ::FLOAT,
    pub eDx: ::FLOAT,
    pub eDy: ::FLOAT,
}
pub type PXFORM = *mut XFORM;
pub type LPXFORM = *mut XFORM;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LOGBRUSH {
    pub lbStyle: ::UINT,
    pub lbColor: ::COLORREF,
    pub lbHatch: ::ULONG_PTR,
}
pub type PLOGBRUSH = *mut LOGBRUSH;

#[repr(C)] #[derive(Copy)]
pub struct LOGCOLORSPACEA {
    pub lcsSignature: ::DWORD,
    pub lcsVersion: ::DWORD,
    pub lcsSize: ::DWORD,
    pub lcsCSType: LCSCSTYPE,
    pub lcsIntent: LCSGAMUTMATCH,
    pub lcsEndpoints: CIEXYZTRIPLE,
    pub lcsGammaRed: ::DWORD,
    pub lcsGammaGreen: ::DWORD,
    pub lcsGammaBlue: ::DWORD,
    pub lcsFilename: [::CHAR; ::MAX_PATH],
}
impl Clone for LOGCOLORSPACEA { fn clone(&self) -> LOGCOLORSPACEA { *self } }
pub type LPLOGCOLORSPACEA = *mut LOGCOLORSPACEA;

#[repr(C)] #[derive(Copy)]
pub struct LOGCOLORSPACEW {
    pub lcsSignature: ::DWORD,
    pub lcsVersion: ::DWORD,
    pub lcsSize: ::DWORD,
    pub lcsCSType: LCSCSTYPE,
    pub lcsIntent: LCSGAMUTMATCH,
    pub lcsEndpoints: CIEXYZTRIPLE,
    pub lcsGammaRed: ::DWORD,
    pub lcsGammaGreen: ::DWORD,
    pub lcsGammaBlue: ::DWORD,
    pub lcsFilename: [::WCHAR; ::MAX_PATH],
}
impl Clone for LOGCOLORSPACEW { fn clone(&self) -> LOGCOLORSPACEW { *self } }
pub type LPLOGCOLORSPACEW = *mut LOGCOLORSPACEW;

pub const LF_FULLFACESIZE: usize = 64;

#[repr(C)] #[derive(Copy)]
pub struct ENUMLOGFONTEXA {
    pub elfLogFont: LOGFONTA,
    pub elfFullName: [::BYTE; LF_FULLFACESIZE],
    pub elfStyle: [::BYTE; LF_FACESIZE],
    pub elfScript: [::BYTE; LF_FACESIZE],
}
impl Clone for ENUMLOGFONTEXA { fn clone(&self) -> ENUMLOGFONTEXA { *self } }
pub type LPENUMLOGFONTEXA = *mut ENUMLOGFONTEXA;

#[repr(C)] #[derive(Copy)]
pub struct ENUMLOGFONTEXW {
    pub elfLogFont: LOGFONTW,
    pub elfFullName: [::WCHAR; LF_FULLFACESIZE],
    pub elfStyle: [::WCHAR; LF_FACESIZE],
    pub elfScript: [::WCHAR; LF_FACESIZE],
}
impl Clone for ENUMLOGFONTEXW { fn clone(&self) -> ENUMLOGFONTEXW { *self } }
pub type LPENUMLOGFONTEXW = *mut ENUMLOGFONTEXW;

pub const MM_MAX_NUMAXES: usize = 16;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DESIGNVECTOR {
    pub dvReserved: ::DWORD,
    pub dvNumAxes: ::DWORD,
    pub dvValues: [::LONG; MM_MAX_NUMAXES],
}
pub type PDESIGNVECTOR = *mut DESIGNVECTOR;
pub type LPDESIGNVECTOR = *mut DESIGNVECTOR;

#[repr(C)] #[derive(Clone, Copy)]
pub struct ENUMLOGFONTEXDVA {
    pub elfEnumLogfontEx: ENUMLOGFONTEXA,
    pub elfDesignVector: DESIGNVECTOR,
}
pub type PENUMLOGFONTEXDVA = *mut ENUMLOGFONTEXDVA;
pub type LPENUMLOGFONTEXDVA = *mut ENUMLOGFONTEXDVA;

#[repr(C)] #[derive(Clone, Copy)]
pub struct ENUMLOGFONTEXDVW {
    pub elfEnumLogfontEx: ENUMLOGFONTEXW,
    pub elfDesignVector: DESIGNVECTOR,
}
pub type PENUMLOGFONTEXDVW = *mut ENUMLOGFONTEXDVW;
pub type LPENUMLOGFONTEXDVW = *mut ENUMLOGFONTEXDVW;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LOGPALETTE {
    pub palVersion: ::WORD,
    pub palNumEntries: ::WORD,
    pub palPalEntry: [PALETTEENTRY; 1],
}
pub type PLOGPALETTE = *mut LOGPALETTE;
pub type NPLOGPALETTE = *mut LOGPALETTE;
pub type LPLOGPALETTE = *mut LOGPALETTE;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LOGPEN {
    pub lopnStyle: ::UINT,
    pub lopnWidth: ::POINT,
    pub lopnColor: ::COLORREF
}
pub type PLOGPEN = *mut LOGPEN;
pub type NPLOGPEN = *mut LOGPEN;
pub type LPLOGPEN = *mut LOGPEN;
