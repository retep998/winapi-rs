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
//
pub const DRIVERVERSION: ::c_int = 0;
pub const TECHNOLOGY: ::c_int = 2;
pub const HORZSIZE: ::c_int = 4;
pub const VERTSIZE: ::c_int = 6;
pub const HORZRES: ::c_int = 8;
pub const VERTRES: ::c_int = 10;
pub const BITSPIXEL: ::c_int = 12;
pub const PLANES: ::c_int = 14;
pub const NUMBRUSHES: ::c_int = 16;
pub const NUMPENS: ::c_int = 18;
pub const NUMMARKERS: ::c_int = 20;
pub const NUMFONTS: ::c_int = 22;
pub const NUMCOLORS: ::c_int = 24;
pub const PDEVICESIZE: ::c_int = 26;
pub const CURVECAPS: ::c_int = 28;
pub const LINECAPS: ::c_int = 30;
pub const POLYGONALCAPS: ::c_int = 32;
pub const TEXTCAPS: ::c_int = 34;
pub const CLIPCAPS: ::c_int = 36;
pub const RASTERCAPS: ::c_int = 38;
pub const ASPECTX: ::c_int = 40;
pub const ASPECTY: ::c_int = 42;
pub const ASPECTXY: ::c_int = 44;
pub const LOGPIXELSX: ::c_int = 88;
pub const LOGPIXELSY: ::c_int = 90;
pub const SIZEPALETTE: ::c_int = 104;
pub const NUMRESERVED: ::c_int = 106;
pub const COLORRES: ::c_int = 108;
pub const PHYSICALWIDTH: ::c_int = 110;
pub const PHYSICALHEIGHT: ::c_int = 111;
pub const PHYSICALOFFSETX: ::c_int = 112;
pub const PHYSICALOFFSETY: ::c_int = 113;
pub const SCALINGFACTORX: ::c_int = 114;
pub const SCALINGFACTORY: ::c_int = 115;
pub const VREFRESH: ::c_int = 116;
pub const DESKTOPVERTRES: ::c_int = 117;
pub const DESKTOPHORZRES: ::c_int = 118;
pub const BLTALIGNMENT: ::c_int = 119;
pub const SHADEBLENDCAPS: ::c_int = 120;
pub const COLORMGMTCAPS: ::c_int = 121;
//
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
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct BLENDFUNCTION {
    pub BlendOp: ::BYTE,
    pub BlendFlags: ::BYTE,
    pub SourceConstantAlpha: ::BYTE,
    pub AlphaFormat: ::BYTE,
}
pub type PBLENDFUNCTION = *mut BLENDFUNCTION;
pub const TMPF_FIXED_PITCH: ::BYTE = 0x01;
pub const TMPF_VECTOR: ::BYTE = 0x02;
pub const TMPF_DEVICE: ::BYTE = 0x08;
pub const TMPF_TRUETYPE: ::BYTE = 0x04;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TEXTMETRICW {
    pub tmHeight: ::LONG,
    pub tmAscent: ::LONG,
    pub tmDescent: ::LONG,
    pub tmInternalLeading: ::LONG,
    pub tmExternalLeading: ::LONG,
    pub tmAveCharWidth: ::LONG,
    pub tmMaxCharWidth: ::LONG,
    pub tmWeight: ::LONG,
    pub tmOverhang: ::LONG,
    pub tmDigitizedAspectX: ::LONG,
    pub tmDigitizedAspectY: ::LONG,
    pub tmFirstChar: ::WCHAR,
    pub tmLastChar: ::WCHAR,
    pub tmDefaultChar: ::WCHAR,
    pub tmBreakChar: ::WCHAR,
    pub tmItalic: ::BYTE,
    pub tmUnderlined: ::BYTE,
    pub tmStruckOut: ::BYTE,
    pub tmPitchAndFamily: ::BYTE,
    pub tmCharSet: ::BYTE,
}

pub const TA_NOUPDATECP: ::UINT = 0;
pub const TA_UPDATECP: ::UINT = 1;

pub const TA_LEFT: ::UINT = 0;
pub const TA_RIGHT: ::UINT = 2;
pub const TA_CENTER: ::UINT = 6;

pub const TA_TOP: ::UINT = 0;
pub const TA_BOTTOM: ::UINT = 8;
pub const TA_BASELINE: ::UINT = 24;

pub const TA_RTLREADING: ::UINT = 256;
pub const TA_MASK: ::UINT = (TA_BASELINE+TA_CENTER+TA_UPDATECP+TA_RTLREADING);

pub const WHITE_BRUSH: ::c_int = 0;
pub const LTGRAY_BRUSH: ::c_int = 1;
pub const GRAY_BRUSH: ::c_int = 2;
pub const DKGRAY_BRUSH: ::c_int = 3;
pub const BLACK_BRUSH: ::c_int = 4;
pub const NULL_BRUSH: ::c_int = 5;
pub const HOLLOW_BRUSH: ::c_int = 5;
pub const WHITE_PEN: ::c_int = 6;
pub const BLACK_PEN: ::c_int = 7;
pub const NULL_PEN: ::c_int = 8;
pub const OEM_FIXED_FONT: ::c_int = 10;
pub const ANSI_FIXED_FONT: ::c_int = 11;
pub const ANSI_VAR_FONT: ::c_int = 12;
pub const SYSTEM_FONT: ::c_int = 13;
pub const DEVICE_DEFAULT_FONT: ::c_int = 14;
pub const DEFAULT_PALETTE: ::c_int = 15;
pub const SYSTEM_FIXED_FONT: ::c_int = 16;
pub const DEFAULT_GUI_FONT: ::c_int = 17;

pub const TRANSPARENT: ::c_int = 1;
pub const OPAQUE: ::c_int = 2;
pub const BKMODE_LAST: ::c_int = 2;

pub const MM_TEXT: ::c_int = 1;
pub const MM_LOMETRIC: ::c_int = 2;
pub const MM_HIMETRIC: ::c_int = 3;
pub const MM_LOENGLISH: ::c_int = 4;
pub const MM_HIENGLISH: ::c_int = 5;
pub const MM_TWIPS: ::c_int = 6;
pub const MM_ISOTROPIC: ::c_int = 7;
pub const MM_ANISOTROPIC: ::c_int = 8;

pub const ALTERNATE: ::c_int = 1;
pub const WINDING: ::c_int = 2;
pub const POLYFILL_LAST: ::c_int = 2;

pub const OUT_DEFAULT_PRECIS: ::DWORD = 0;
pub const OUT_STRING_PRECIS: ::DWORD = 1;
pub const OUT_CHARACTER_PRECIS: ::DWORD = 2;
pub const OUT_STROKE_PRECIS: ::DWORD = 3;
pub const OUT_TT_PRECIS: ::DWORD = 4;
pub const OUT_DEVICE_PRECIS: ::DWORD = 5;
pub const OUT_RASTER_PRECIS: ::DWORD = 6;
pub const OUT_TT_ONLY_PRECIS: ::DWORD = 7;
pub const OUT_OUTLINE_PRECIS: ::DWORD = 8;
pub const OUT_SCREEN_OUTLINE_PRECIS: ::DWORD = 9;
pub const OUT_PS_ONLY_PRECIS: ::DWORD = 10;

pub const CLIP_DEFAULT_PRECIS: ::DWORD = 0;
pub const CLIP_CHARACTER_PRECIS: ::DWORD = 1;
pub const CLIP_STROKE_PRECIS: ::DWORD = 2;
pub const CLIP_MASK: ::DWORD = 0xf;
pub const CLIP_LH_ANGLES: ::DWORD = (1<<4);
pub const CLIP_TT_ALWAYS: ::DWORD = (2<<4);
pub const CLIP_DFA_DISABLE: ::DWORD = (4<<4);
pub const CLIP_EMBEDDED: ::DWORD = (8<<4);

pub const DEFAULT_QUALITY: ::DWORD = 0;
pub const DRAFT_QUALITY: ::DWORD = 1;
pub const PROOF_QUALITY: ::DWORD = 2;
pub const NONANTIALIASED_QUALITY: ::DWORD = 3;
pub const ANTIALIASED_QUALITY: ::DWORD = 4;

pub const CLEARTYPE_QUALITY: ::DWORD = 5;
pub const CLEARTYPE_NATURAL_QUALITY: ::DWORD = 6;

pub const DEFAULT_PITCH: ::DWORD = 0;
pub const FIXED_PITCH: ::DWORD = 1;
pub const VARIABLE_PITCH: ::DWORD = 2;
pub const MONO_FONT: ::DWORD = 8;


pub const ANSI_CHARSET: ::DWORD = 0;
pub const DEFAULT_CHARSET: ::DWORD = 1;
pub const SYMBOL_CHARSET: ::DWORD = 2;
pub const SHIFTJIS_CHARSET: ::DWORD = 128;
pub const HANGEUL_CHARSET: ::DWORD = 129;
pub const HANGUL_CHARSET: ::DWORD = 129;
pub const GB2312_CHARSET: ::DWORD = 134;
pub const CHINESEBIG5_CHARSET: ::DWORD = 136;
pub const OEM_CHARSET: ::DWORD = 255;
pub const JOHAB_CHARSET: ::DWORD = 130;
pub const HEBREW_CHARSET: ::DWORD = 177;
pub const ARABIC_CHARSET: ::DWORD = 178;
pub const GREEK_CHARSET: ::DWORD = 161;
pub const TURKISH_CHARSET: ::DWORD = 162;
pub const VIETNAMESE_CHARSET: ::DWORD = 163;
pub const THAI_CHARSET: ::DWORD = 222;
pub const EASTEUROPE_CHARSET: ::DWORD = 238;
pub const RUSSIAN_CHARSET: ::DWORD = 204;

pub const MAC_CHARSET: ::DWORD = 77;
pub const BALTIC_CHARSET: ::DWORD = 186;

pub const FS_LATIN1: ::DWORD = 0x00000001;
pub const FS_LATIN2: ::DWORD = 0x00000002;
pub const FS_CYRILLIC: ::DWORD = 0x00000004;
pub const FS_GREEK: ::DWORD = 0x00000008;
pub const FS_TURKISH: ::DWORD = 0x00000010;
pub const FS_HEBREW: ::DWORD = 0x00000020;
pub const FS_ARABIC: ::DWORD = 0x00000040;
pub const FS_BALTIC: ::DWORD = 0x00000080;
pub const FS_VIETNAMESE: ::DWORD = 0x00000100;
pub const FS_THAI: ::DWORD = 0x00010000;
pub const FS_JISJAPAN: ::DWORD = 0x00020000;
pub const FS_CHINESESIMP: ::DWORD = 0x00040000;
pub const FS_WANSUNG: ::DWORD = 0x00080000;
pub const FS_CHINESETRAD: ::DWORD = 0x00100000;
pub const FS_JOHAB: ::DWORD = 0x00200000;
pub const FS_SYMBOL: ::DWORD = 0x80000000;


pub const FW_DONTCARE: ::c_int = 0;
pub const FW_THIN: ::c_int = 100;
pub const FW_EXTRALIGHT: ::c_int = 200;
pub const FW_LIGHT: ::c_int = 300;
pub const FW_NORMAL: ::c_int = 400;
pub const FW_MEDIUM: ::c_int = 500;
pub const FW_SEMIBOLD: ::c_int = 600;
pub const FW_BOLD: ::c_int = 700;
pub const FW_EXTRABOLD: ::c_int = 800;
pub const FW_HEAVY: ::c_int = 900;

pub const FW_ULTRALIGHT: ::c_int = FW_EXTRALIGHT;
pub const FW_REGULAR: ::c_int = FW_NORMAL;
pub const FW_DEMIBOLD: ::c_int = FW_SEMIBOLD;
pub const FW_ULTRABOLD: ::c_int = FW_EXTRABOLD;
pub const FW_BLACK: ::c_int = FW_HEAVY;
