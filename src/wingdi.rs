// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! GDI procedure declarations, constant definitions and macros
pub const DISPLAY_DEVICE_ATTACHED_TO_DESKTOP: ::DWORD = 0x00000001;
pub const DISPLAY_DEVICE_MULTI_DRIVER: ::DWORD = 0x00000002;
pub const DISPLAY_DEVICE_PRIMARY_DEVICE: ::DWORD = 0x00000004;
pub const DISPLAY_DEVICE_MIRRORING_DRIVER: ::DWORD = 0x00000008;
pub const DISPLAY_DEVICE_VGA_COMPATIBLE: ::DWORD = 0x00000010;
pub const DISPLAY_DEVICE_REMOVABLE: ::DWORD = 0x00000020;
pub const DISPLAY_DEVICE_ACC_DRIVER: ::DWORD = 0x00000040;
pub const DISPLAY_DEVICE_MODESPRUNED: ::DWORD = 0x08000000;
pub const DISPLAY_DEVICE_REMOTE: ::DWORD = 0x04000000;
pub const DISPLAY_DEVICE_DISCONNECT: ::DWORD = 0x02000000;
pub const DISPLAY_DEVICE_TS_COMPATIBLE: ::DWORD = 0x00200000;
pub const DISPLAY_DEVICE_UNSAFE_MODES_ON: ::DWORD = 0x00080000;
pub const DISPLAY_DEVICE_ACTIVE: ::DWORD = 0x00000001;
pub const DISPLAY_DEVICE_ATTACHED: ::DWORD = 0x00000002;
pub const DM_ORIENTATION: ::DWORD = 0x00000001;
pub const DM_PAPERSIZE: ::DWORD = 0x00000002;
pub const DM_PAPERLENGTH: ::DWORD = 0x00000004;
pub const DM_PAPERWIDTH: ::DWORD = 0x00000008;
pub const DM_SCALE: ::DWORD = 0x00000010;
pub const DM_POSITION: ::DWORD = 0x00000020;
pub const DM_NUP: ::DWORD = 0x00000040;
pub const DM_DISPLAYORIENTATION: ::DWORD = 0x00000080;
pub const DM_COPIES: ::DWORD = 0x00000100;
pub const DM_DEFAULTSOURCE: ::DWORD = 0x00000200;
pub const DM_PRINTQUALITY: ::DWORD = 0x00000400;
pub const DM_COLOR: ::DWORD = 0x00000800;
pub const DM_DUPLEX: ::DWORD = 0x00001000;
pub const DM_YRESOLUTION: ::DWORD = 0x00002000;
pub const DM_TTOPTION: ::DWORD = 0x00004000;
pub const DM_COLLATE: ::DWORD = 0x00008000;
pub const DM_FORMNAME: ::DWORD = 0x00010000;
pub const DM_LOGPIXELS: ::DWORD = 0x00020000;
pub const DM_BITSPERPEL: ::DWORD = 0x00040000;
pub const DM_PELSWIDTH: ::DWORD = 0x00080000;
pub const DM_PELSHEIGHT: ::DWORD = 0x00100000;
pub const DM_DISPLAYFLAGS: ::DWORD = 0x00200000;
pub const DM_DISPLAYFREQUENCY: ::DWORD = 0x00400000;
pub const DM_ICMMETHOD: ::DWORD = 0x00800000;
pub const DM_ICMINTENT: ::DWORD = 0x01000000;
pub const DM_MEDIATYPE: ::DWORD = 0x02000000;
pub const DM_DITHERTYPE: ::DWORD = 0x04000000;
pub const DM_PANNINGWIDTH: ::DWORD = 0x08000000;
pub const DM_PANNINGHEIGHT: ::DWORD = 0x10000000;
pub const DM_DISPLAYFIXEDOUTPUT: ::DWORD = 0x20000000;
pub const PFD_TYPE_RGBA: ::BYTE = 0;
pub const PFD_TYPE_COLORINDEX: ::BYTE = 1;
pub const PFD_MAIN_PLANE: ::BYTE = 0;
pub const PFD_OVERLAY_PLANE: ::BYTE = 1;
pub const PFD_UNDERLAY_PLANE: ::BYTE = 0xFF;
pub const PFD_DOUBLEBUFFER: ::DWORD = 0x00000001;
pub const PFD_STEREO: ::DWORD = 0x00000002;
pub const PFD_DRAW_TO_WINDOW: ::DWORD = 0x00000004;
pub const PFD_DRAW_TO_BITMAP: ::DWORD = 0x00000008;
pub const PFD_SUPPORT_GDI: ::DWORD = 0x00000010;
pub const PFD_SUPPORT_OPENGL: ::DWORD = 0x00000020;
pub const PFD_GENERIC_FORMAT: ::DWORD = 0x00000040;
pub const PFD_NEED_PALETTE: ::DWORD = 0x00000080;
pub const PFD_NEED_SYSTEM_PALETTE: ::DWORD = 0x00000100;
pub const PFD_SWAP_EXCHANGE: ::DWORD = 0x00000200;
pub const PFD_SWAP_COPY: ::DWORD = 0x00000400;
pub const PFD_SWAP_LAYER_BUFFERS: ::DWORD = 0x00000800;
pub const PFD_GENERIC_ACCELERATED: ::DWORD = 0x00001000;
pub const PFD_SUPPORT_DIRECTDRAW: ::DWORD = 0x00002000;
pub const PFD_DIRECT3D_ACCELERATED: ::DWORD = 0x00004000;
pub const PFD_SUPPORT_COMPOSITION: ::DWORD = 0x00008000;
pub const PFD_DEPTH_DONTCARE: ::DWORD = 0x20000000;
pub const PFD_DOUBLEBUFFER_DONTCARE: ::DWORD = 0x40000000;
pub const PFD_STEREO_DONTCARE: ::DWORD = 0x80000000;
pub const CCHFORMNAME: usize = 32;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DEVMODEA {
    pub dmDeviceName: [::CHAR; ::CCHDEVICENAME],
    pub dmSpecVersion: ::WORD,
    pub dmDriverVersion: ::WORD,
    pub dmSize: ::WORD,
    pub dmDriverExtra: ::WORD,
    pub dmFields: ::DWORD,
    pub union1: [u8; 16],
    pub dmColor: ::c_short,
    pub dmDuplex: ::c_short,
    pub dmYResolution: ::c_short,
    pub dmTTOption: ::c_short,
    pub dmCollate: ::c_short,
    pub dmFormName: [::CHAR; CCHFORMNAME],
    pub dmLogPixels: ::WORD,
    pub dmBitsPerPel: ::DWORD,
    pub dmPelsWidth: ::DWORD,
    pub dmPelsHeight: ::DWORD,
    pub dmDisplayFlags: ::DWORD,
    pub dmDisplayFrequency: ::DWORD,
    pub dmICMMethod: ::DWORD,
    pub dmICMIntent: ::DWORD,
    pub dmMediaType: ::DWORD,
    pub dmDitherType: ::DWORD,
    pub dmReserved1: ::DWORD,
    pub dmReserved2: ::DWORD,
    pub dmPanningWidth: ::DWORD,
    pub dmPanningHeight: ::DWORD,
}
pub type PDEVMODEA = *mut DEVMODEA;
pub type NPDEVMODEA = *mut DEVMODEA;
pub type LPDEVMODEA = *mut DEVMODEA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DEVMODEW {
    pub dmDeviceName: [::WCHAR; ::CCHDEVICENAME],
    pub dmSpecVersion: ::WORD,
    pub dmDriverVersion: ::WORD,
    pub dmSize: ::WORD,
    pub dmDriverExtra: ::WORD,
    pub dmFields: ::DWORD,
    pub union1: [u8; 16],
    pub dmColor: ::c_short,
    pub dmDuplex: ::c_short,
    pub dmYResolution: ::c_short,
    pub dmTTOption: ::c_short,
    pub dmCollate: ::c_short,
    pub dmFormName: [::WCHAR; CCHFORMNAME],
    pub dmLogPixels: ::WORD,
    pub dmBitsPerPel: ::DWORD,
    pub dmPelsWidth: ::DWORD,
    pub dmPelsHeight: ::DWORD,
    pub dmDisplayFlags: ::DWORD,
    pub dmDisplayFrequency: ::DWORD,
    pub dmICMMethod: ::DWORD,
    pub dmICMIntent: ::DWORD,
    pub dmMediaType: ::DWORD,
    pub dmDitherType: ::DWORD,
    pub dmReserved1: ::DWORD,
    pub dmReserved2: ::DWORD,
    pub dmPanningWidth: ::DWORD,
    pub dmPanningHeight: ::DWORD,
}
pub type PDEVMODEW = *mut DEVMODEW;
pub type NPDEVMODEW = *mut DEVMODEW;
pub type LPDEVMODEW = *mut DEVMODEW;
#[repr(C)] #[derive(Copy)]
pub struct DISPLAY_DEVICEW {
    pub cb: ::DWORD,
    pub DeviceName: [::WCHAR; 32],
    pub DeviceString: [::WCHAR; 128],
    pub StateFlags: ::DWORD,
    pub DeviceID: [::WCHAR; 128],
    pub DeviceKey: [::WCHAR; 128],
}
impl Clone for DISPLAY_DEVICEW { fn clone(&self) -> DISPLAY_DEVICEW { *self } }
pub type PDISPLAY_DEVICEW = *mut DISPLAY_DEVICEW;
pub type LPDISPLAY_DEVICEW = *mut DISPLAY_DEVICEW;
#[repr(C)] #[derive(Copy)]
pub struct DISPLAY_DEVICEA {
    pub cb: ::DWORD,
    pub DeviceName: [::CHAR; 32],
    pub DeviceString: [::CHAR; 128],
    pub StateFlags: ::DWORD,
    pub DeviceID: [::CHAR; 128],
    pub DeviceKey: [::CHAR; 128],
}
impl Clone for DISPLAY_DEVICEA { fn clone(&self) -> DISPLAY_DEVICEA { *self } }
pub type PDISPLAY_DEVICEA = *mut DISPLAY_DEVICEA;
pub type LPDISPLAY_DEVICEA = *mut DISPLAY_DEVICEA;
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PIXELFORMATDESCRIPTOR {
    pub nSize: ::WORD,
    pub nVersion: ::WORD,
    pub dwFlags: ::DWORD,
    pub iPixelType: ::BYTE,
    pub cColorBits: ::BYTE,
    pub cRedBits: ::BYTE,
    pub cRedShift: ::BYTE,
    pub cGreenBits: ::BYTE,
    pub cGreenShift: ::BYTE,
    pub cBlueBits: ::BYTE,
    pub cBlueShift: ::BYTE,
    pub cAlphaBits: ::BYTE,
    pub cAlphaShift: ::BYTE,
    pub cAccumBits: ::BYTE,
    pub cAccumRedBits: ::BYTE,
    pub cAccumGreenBits: ::BYTE,
    pub cAccumBlueBits: ::BYTE,
    pub cAccumAlphaBits: ::BYTE,
    pub cDepthBits: ::BYTE,
    pub cStencilBits: ::BYTE,
    pub cAuxBuffers: ::BYTE,
    pub iLayerType: ::BYTE,
    pub bReserved: ::BYTE,
    pub dwLayerMask: ::DWORD,
    pub dwVisibleMask: ::DWORD,
    pub dwDamageMask: ::DWORD,
}
pub type PPIXELFORMATDESCRIPTOR = *mut PIXELFORMATDESCRIPTOR;
pub type LPPIXELFORMATDESCRIPTOR = *mut PIXELFORMATDESCRIPTOR;
pub const R2_BLACK: ::c_int = 1;
pub const R2_NOTMERGEPEN: ::c_int = 2;
pub const R2_MASKNOTPEN: ::c_int = 3;
pub const R2_NOTCOPYPEN: ::c_int = 4;
pub const R2_MASKPENNOT: ::c_int = 5;
pub const R2_NOT: ::c_int = 6;
pub const R2_XORPEN: ::c_int = 7;
pub const R2_NOTMASKPEN: ::c_int = 8;
pub const R2_MASKPEN: ::c_int = 9;
pub const R2_NOTXORPEN: ::c_int = 10;
pub const R2_NOP: ::c_int = 11;
pub const R2_MERGENOTPEN: ::c_int = 12;
pub const R2_COPYPEN: ::c_int = 13;
pub const R2_MERGEPENNOT: ::c_int = 14;
pub const R2_MERGEPEN: ::c_int = 15;
pub const R2_WHITE: ::c_int = 16;
pub const R2_LAST: ::c_int = 16;
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
//1906
pub const DIB_RGB_COLORS: ::UINT = 0;
pub const DIB_PAL_COLORS: ::UINT = 1;
pub const CBM_INIT: ::DWORD = 4;
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
pub type PPALETTEENTRY = *mut PALETTEENTRY;
pub type LPPALETTEENTRY = *mut PALETTEENTRY;
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
pub struct TEXTMETRICA {
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
    pub tmFirstChar: ::BYTE,
    pub tmLastChar: ::BYTE,
    pub tmDefaultChar: ::BYTE,
    pub tmBreakChar: ::BYTE,
    pub tmItalic: ::BYTE,
    pub tmUnderlined: ::BYTE,
    pub tmStruckOut: ::BYTE,
    pub tmPitchAndFamily: ::BYTE,
    pub tmCharSet: ::BYTE,
}

pub type PTEXTMETRICA = *mut TEXTMETRICA;
pub type NPTEXTMETRICA = *mut TEXTMETRICA;
pub type LPTEXTMETRICA = *mut TEXTMETRICA;

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
pub type PTEXTMETRICW = *mut TEXTMETRICW;
pub type NPTEXTMETRICW = *mut TEXTMETRICW;
pub type LPTEXTMETRICW = *mut TEXTMETRICW;

pub const TA_NOUPDATECP: ::UINT = 0;
pub const TA_UPDATECP: ::UINT = 1;
pub const TA_LEFT: ::UINT = 0;
pub const TA_RIGHT: ::UINT = 2;
pub const TA_CENTER: ::UINT = 6;
pub const TA_TOP: ::UINT = 0;
pub const TA_BOTTOM: ::UINT = 8;
pub const TA_BASELINE: ::UINT = 24;
pub const TA_RTLREADING: ::UINT = 256;
pub const TA_MASK: ::UINT = TA_BASELINE + TA_CENTER + TA_UPDATECP + TA_RTLREADING;
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
pub const DC_BRUSH: ::c_int = 18;
pub const DC_PEN: ::c_int = 19;
pub const STOCK_LAST: ::c_int = 19;pub const PS_SOLID: ::c_int = 0;
pub const PS_DASH: ::c_int = 1;
pub const PS_DOT: ::c_int = 2;
pub const PS_DASHDOT: ::c_int = 3;
pub const PS_DASHDOTDOT: ::c_int = 4;
pub const PS_NULL: ::c_int = 5;
pub const PS_INSIDEFRAME: ::c_int = 6;
pub const PS_USERSTYLE: ::c_int = 7;
pub const PS_ALTERNATE: ::c_int = 8;
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
pub const CLIP_LH_ANGLES: ::DWORD = 1 << 4;
pub const CLIP_TT_ALWAYS: ::DWORD = 2 << 4;
pub const CLIP_DFA_DISABLE: ::DWORD = 4 << 4;
pub const CLIP_EMBEDDED: ::DWORD = 8 << 4;
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

pub type COLOR16 = ::c_ushort;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TRIVERTEX {
    pub x: ::LONG,
    pub y: ::LONG,
    pub Red: COLOR16,
    pub Green: COLOR16,
    pub Blue: COLOR16,
    pub Alpha: COLOR16,
}

pub type PTRIVERTEX = *mut TRIVERTEX;
pub type LPTRIVERTEX = *mut TRIVERTEX;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct GRADIENT_RECT {
    pub UpperLeft: ::ULONG,
    pub LowerRight: ::ULONG,
}

pub type PGRADIENT_RECT = *mut GRADIENT_RECT;
pub type LPGRADIENT_RECT = *mut GRADIENT_RECT;

/* Object Definitions for EnumObjects() */
pub const OBJ_PEN: ::UINT = 1;
pub const OBJ_BRUSH: ::UINT = 2;
pub const OBJ_DC: ::UINT = 3;
pub const OBJ_METADC: ::UINT = 4;
pub const OBJ_PAL: ::UINT = 5;
pub const OBJ_FONT: ::UINT = 6;
pub const OBJ_BITMAP: ::UINT = 7;
pub const OBJ_REGION: ::UINT = 8;
pub const OBJ_METAFILE: ::UINT = 9;
pub const OBJ_MEMDC: ::UINT = 10;
pub const OBJ_EXTPEN: ::UINT = 11;
pub const OBJ_ENHMETADC: ::UINT = 12;
pub const OBJ_ENHMETAFILE: ::UINT = 13;
pub const OBJ_COLORSPACE: ::UINT = 14;
pub const GDI_OBJ_LAST: ::UINT = OBJ_COLORSPACE;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct COLORADJUSTMENT {
    pub caSize: ::WORD,
    pub caFlags: ::WORD,
    pub caIlluminantIndex: ::WORD,
    pub caRedGamma: ::WORD,
    pub caGreenGamma: ::WORD,
    pub caBlueGamma: ::WORD,
    pub caReferenceBlack: ::WORD,
    pub caReferenceWhite: ::WORD,
    pub caContrast: ::SHORT,
    pub caBrightness: ::SHORT,
    pub caColorfulness: ::SHORT,
    pub caRedGreenTint: ::SHORT,
}

pub type PCOLORADJUSTMENT = *mut COLORADJUSTMENT;
pub type LPCOLORADJUSTMENT = *mut COLORADJUSTMENT;

pub type OLDFONTENUMPROCA = Option<unsafe extern "system" fn(
    *const LOGFONTA, *const ::VOID, ::DWORD, ::LPARAM
) -> ::c_int>;

pub type OLDFONTENUMPROCW = Option<unsafe extern "system" fn(
    *const LOGFONTW, *const ::VOID, ::DWORD, ::LPARAM
) -> ::c_int>;

pub type FONTENUMPROCA = OLDFONTENUMPROCA;
pub type FONTENUMPROCW = OLDFONTENUMPROCW;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct WCRANGE {
    pub wcLow: ::WCHAR,
    pub cGlyphs: ::USHORT,
}

pub type PWCRANGE = *mut WCRANGE;
pub type LPWCRANGE = *mut WCRANGE;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct GLYPHSET {
    pub cbThis: ::DWORD,
    pub flAccel: ::DWORD,
    pub cGlyphsSupported: ::DWORD,
    pub cRanges: ::DWORD,
    pub ranges: [WCRANGE;1],
}

pub type PGLYPHSET = *mut GLYPHSET;
pub type LPGLYPHSET = *mut GLYPHSET;

pub type ABORTPROC = Option<unsafe extern "system" fn(::HDC, ::c_int) -> ::BOOL>;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DOCINFOA {
    pub cbSize: ::c_int,
    pub lpszDocName: ::LPCSTR,
    pub lpszOutput: ::LPCSTR,
    pub lpszDatatype: ::LPCSTR,
    pub fwType: ::DWORD,
}

pub type LPDOCINFOA = *mut DOCINFOA;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct DOCINFOW {
    pub cbSize: ::c_int,
    pub lpszDocName: ::LPCWSTR,
    pub lpszOutput: ::LPCWSTR,
    pub lpszDatatype: ::LPCWSTR,
    pub fwType: ::DWORD,
}

pub type LPDOCINFOW = *mut DOCINFOW;

pub type ICMENUMPROCA = Option<unsafe extern "system" fn(::LPSTR, ::LPARAM) -> ::c_int>;
pub type ICMENUMPROCW = Option<unsafe extern "system" fn(::LPWSTR, ::LPARAM) -> ::c_int>;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct HANDLETABLE {
    pub objectHandle: [::HGDIOBJ; 1],
}

pub type LPHANDLETABLE = *mut HANDLETABLE;
pub type PHANDLETABLE = *mut HANDLETABLE;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct METARECORD {
    pub rdSize: ::DWORD,
    pub rdFunction: ::WORD,
    pub rdParm: [::WORD; 1],
}
pub type PMETARECORD = *mut METARECORD;
pub type LPMETARECORD = *mut METARECORD;

pub type MFENUMPROC = Option<unsafe extern "system" fn(
    hdc: ::HDC, lpht: *const ::HANDLETABLE, lpMR: *const ::METARECORD, nObj: ::c_int, param: ::LPARAM
) -> ::c_int>;

pub type GOBJENUMPROC = Option<unsafe extern "system" fn(::LPVOID, ::LPARAM) -> ::c_int>;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct GCP_RESULTSA {
    pub lStructSize: ::DWORD,
    pub lpOutString: ::LPSTR,
    pub lpOrder: *const ::UINT,
    pub lpDx: *const ::c_int,
    pub lpCaretPos: *const ::c_int,
    pub lpClass: ::LPSTR,
    pub lpGlyphs: ::LPWSTR,
    pub nGlyphs: ::UINT,
    pub nMaxFit: ::c_int,
}

pub type LPGCP_RESULTSA = *mut GCP_RESULTSA;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct GCP_RESULTSW {
    pub lStructSize: ::DWORD,
    pub lpOutString: ::LPWSTR,
    pub lpOrder: *const ::UINT,
    pub lpDx: *const ::c_int,
    pub lpCaretPos: *const ::c_int,
    pub lpClass: ::LPSTR,
    pub lpGlyphs: ::LPWSTR,
    pub nGlyphs: ::UINT,
    pub nMaxFit: ::c_int,
}

pub type LPGCP_RESULTSW = *mut GCP_RESULTSW;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct FONTSIGNATURE {
    pub fsUsb: [::DWORD; 4],
    pub fsCsb: [::DWORD; 2],
}

pub type LPFONTSIGNATURE = *mut FONTSIGNATURE;
pub type PFONTSIGNATURE = *mut FONTSIGNATURE;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct POLYTEXTA {
    pub x: ::c_int,
    pub y: ::c_int,
    pub n: ::UINT,
    pub lpstr: ::LPCSTR,
    pub uiFlags: ::UINT,
    pub rcl: ::RECT,
    pub pdx: *const ::c_int,
}

pub type PPOLYTEXTA = *mut POLYTEXTA;
pub type NPPOLYTEXTA = *mut POLYTEXTA;
pub type LPPOLYTEXTA = *mut POLYTEXTA;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct POLYTEXTW {
    pub x: ::c_int,
    pub y: ::c_int,
    pub n: ::UINT,
    pub lpstr: ::LPCWSTR,
    pub uiFlags: ::UINT,
    pub rcl: ::RECT,
    pub pdx: *const ::c_int,
}

pub type PPOLYTEXTW = *mut POLYTEXTW;
pub type NPPOLYTEXTW = *mut POLYTEXTW;
pub type LPPOLYTEXTW = *mut POLYTEXTW;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct CHARSETINFO {
    pub ciCharset: ::UINT,
    pub ciACP: ::UINT,
    pub fs: ::FONTSIGNATURE,
}

pub type PCHARSETINFO = *mut CHARSETINFO;
pub type NPCHARSETINFO = *mut CHARSETINFO;
pub type LPCHARSETINFO = *mut CHARSETINFO;

pub const GRADIENT_FILL_RECT_H: ::ULONG = 0x00000000;
pub const GRADIENT_FILL_RECT_V: ::ULONG = 0x00000001;
pub const GRADIENT_FILL_TRIANGLE: ::ULONG = 0x00000002;
pub const GRADIENT_FILL_OP_FLAG: ::ULONG = 0x000000ff;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LAYERPLANEDESCRIPTOR {
    pub nSize: ::WORD,
    pub nVersion: ::WORD,
    pub dwFlags: ::DWORD,
    pub iPixelType: ::BYTE,
    pub cColorBits: ::BYTE,
    pub cRedBits: ::BYTE,
    pub cRedShift: ::BYTE,
    pub cGreenBits: ::BYTE,
    pub cGreenShift: ::BYTE,
    pub cBlueBits: ::BYTE,
    pub cBlueShift: ::BYTE,
    pub cAlphaBits: ::BYTE,
    pub cAlphaShift: ::BYTE,
    pub cAccumBits: ::BYTE,
    pub cAccumRedBits: ::BYTE,
    pub cAccumGreenBits: ::BYTE,
    pub cAccumBlueBits: ::BYTE,
    pub cAccumAlphaBits: ::BYTE,
    pub cDepthBits: ::BYTE,
    pub cStencilBits: ::BYTE,
    pub cAuxBuffers: ::BYTE,
    pub iLayerPlane: ::BYTE,
    pub bReserved: ::BYTE,
    pub crTransparent: ::COLORREF,
}

pub type PLAYERPLANEDESCRIPTOR = *mut LAYERPLANEDESCRIPTOR;
pub type LPLAYERPLANEDESCRIPTOR = *mut LAYERPLANEDESCRIPTOR;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ENHMETAHEADER {
    pub iType: ::DWORD,
    pub nSize: ::DWORD,
    pub rclBounds: ::RECTL,
    pub rclFrame: ::RECTL,
    pub dSignature: ::DWORD,
    pub nVersion: ::DWORD,
    pub nBytes: ::DWORD,
    pub nRecords: ::DWORD,
    pub nHandles: ::WORD,
    pub sReserved: ::WORD,
    pub nDescription: ::DWORD,
    pub offDescription: ::DWORD,
    pub nPalEntries: ::DWORD,
    pub szlDevice: ::SIZEL,
    pub szlMillimeters: ::SIZEL,
    pub cbPixelFormat: ::DWORD,
    pub offPixelFormat: ::DWORD,
    pub bOpenGL: ::DWORD,
    pub szlMicrometers: ::SIZEL,
}
pub type PENHMETAHEADER = *mut ENHMETAHEADER;
pub type LPENHMETAHEADER = *mut ENHMETAHEADER;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct FIXED {
    pub fract: ::WORD,
    pub value: ::c_short,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct MAT2 {
    pub eM11: FIXED,
    pub eM12: FIXED,
    pub eM21: FIXED,
    pub eM22: FIXED,
}
pub type LPMAT2 = *mut MAT2;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct GLYPHMETRICS {
    pub gmBlackBoxX: ::UINT,
    pub gmBlackBoxY: ::UINT,
    pub gmptGlyphOrigin: ::POINT,
    pub gmCellIncX: ::c_short,
    pub gmCellIncY: ::c_short,
}
pub type LPGLYPHMETRICS = *mut GLYPHMETRICS;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct KERNINGPAIR {
   pub wFirst: ::WORD,
   pub wSecond: ::WORD,
   pub iKernAmount: ::c_int,
}
pub type LPKERNINGPAIR = *mut KERNINGPAIR;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct PANOSE {
    pub bFamilyType: ::BYTE,
    pub bSerifStyle: ::BYTE,
    pub bWeight: ::BYTE,
    pub bProportion: ::BYTE,
    pub bContrast: ::BYTE,
    pub bStrokeVariation: ::BYTE,
    pub bArmStyle: ::BYTE,
    pub bLetterform: ::BYTE,
    pub bMidline: ::BYTE,
    pub bXHeight: ::BYTE,
}
pub type LPPANOSE = *mut PANOSE;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct OUTLINETEXTMETRICA {
    pub otmSize: ::UINT,
    pub otmTextMetrics: TEXTMETRICA,
    pub otmFiller: ::BYTE,
    pub otmPanoseNumber: ::PANOSE,
    pub otmfsSelection: ::UINT,
    pub otmfsType: ::UINT,
    pub otmsCharSlopeRise: ::c_int,
    pub otmsCharSlopeRun: ::c_int,
    pub otmItalicAngle: ::c_int,
    pub otmEMSquare: ::UINT,
    pub otmAscent: ::c_int,
    pub otmDescent: ::c_int,
    pub otmLineGap: ::UINT,
    pub otmsCapEmHeight: ::UINT,
    pub otmsXHeight: ::UINT,
    pub otmrcFontBox: ::RECT,
    pub otmMacAscent: ::c_int,
    pub otmMacDescent: ::c_int,
    pub otmMacLineGap: ::UINT,
    pub otmusMinimumPPEM: ::UINT,
    pub otmptSubscriptSize: ::POINT,
    pub otmptSubscriptOffset: ::POINT,
    pub otmptSuperscriptSize: ::POINT,
    pub otmptSuperscriptOffset: ::POINT,
    pub otmsStrikeoutSize: ::UINT,
    pub otmsStrikeoutPosition: ::c_int,
    pub otmsUnderscoreSize: ::c_int,
    pub otmsUnderscorePosition: ::c_int,
    pub otmpFamilyName: ::PSTR,
    pub otmpFaceName: ::PSTR,
    pub otmpStyleName: ::PSTR,
    pub otmpFullName: ::PSTR,
}
pub type POUTLINETEXTMETRICA = *mut OUTLINETEXTMETRICA;
pub type NPOUTLINETEXTMETRICA = *mut OUTLINETEXTMETRICA;
pub type LPOUTLINETEXTMETRICA = *mut OUTLINETEXTMETRICA;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct OUTLINETEXTMETRICW {
    pub otmSize: ::UINT,
    pub otmTextMetrics: TEXTMETRICW,
    pub otmFiller: ::BYTE,
    pub otmPanoseNumber: ::PANOSE,
    pub otmfsSelection: ::UINT,
    pub otmfsType: ::UINT,
    pub otmsCharSlopeRise: ::c_int,
    pub otmsCharSlopeRun: ::c_int,
    pub otmItalicAngle: ::c_int,
    pub otmEMSquare: ::UINT,
    pub otmAscent: ::c_int,
    pub otmDescent: ::c_int,
    pub otmLineGap: ::UINT,
    pub otmsCapEmHeight: ::UINT,
    pub otmsXHeight: ::UINT,
    pub otmrcFontBox: ::RECT,
    pub otmMacAscent: ::c_int,
    pub otmMacDescent: ::c_int,
    pub otmMacLineGap: ::UINT,
    pub otmusMinimumPPEM: ::UINT,
    pub otmptSubscriptSize: ::POINT,
    pub otmptSubscriptOffset: ::POINT,
    pub otmptSuperscriptSize: ::POINT,
    pub otmptSuperscriptOffset: ::POINT,
    pub otmsStrikeoutSize: ::UINT,
    pub otmsStrikeoutPosition: ::c_int,
    pub otmsUnderscoreSize: ::c_int,
    pub otmsUnderscorePosition: ::c_int,
    pub otmpFamilyName: ::PSTR,
    pub otmpFaceName: ::PSTR,
    pub otmpStyleName: ::PSTR,
    pub otmpFullName: ::PSTR,
}
pub type POUTLINETEXTMETRICW = *mut OUTLINETEXTMETRICW;
pub type NPOUTLINETEXTMETRICW = *mut OUTLINETEXTMETRICW;
pub type LPOUTLINETEXTMETRICW = *mut OUTLINETEXTMETRICW;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct RASTERIZER_STATUS {
    pub nSize: ::c_short,
    pub wFlags: ::c_short,
    pub nLanguageID: ::c_short,
}
pub type LPRASTERIZER_STATUS = *mut RASTERIZER_STATUS;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct ENHMETARECORD {
    pub iType: ::DWORD,
    pub nSize: ::DWORD,
    pub dParm: [::DWORD; 1],
}
pub type PENHMETARECORD = *mut ENHMETARECORD;
pub type LPENHMETARECORD = *mut ENHMETARECORD;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct METAFILEPICT {
    pub mm: ::LONG,
    pub xExt: ::LONG,
    pub yExt: ::LONG,
    pub hMF: ::HMETAFILE,
}
pub type LPMETAFILEPICT = *mut METAFILEPICT;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct POINTFLOAT {
    pub x: ::FLOAT,
    pub y: ::FLOAT,
}
pub type PPOINTFLOAT = *mut POINTFLOAT;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct GLYPHMETRICSFLOAT {
    pub gmfBlackBoxX: ::FLOAT,
    pub gmfBlackBoxY: ::FLOAT,
    pub gmfptGlyphOrigin: POINTFLOAT,
    pub gmfCellIncX: ::FLOAT,
    pub gmfCellIncY: ::FLOAT,
}
pub type PGLYPHMETRICSFLOAT = *mut GLYPHMETRICSFLOAT;
pub type LPGLYPHMETRICSFLOAT = *mut GLYPHMETRICSFLOAT;

pub const DT_PLOTTER: ::c_int = 0;
pub const DT_RASDISPLAY: ::c_int = 1;
pub const DT_RASPRINTER: ::c_int = 2;
pub const DT_RASCAMERA: ::c_int = 3;
pub const DT_CHARSTREAM: ::c_int = 4;
pub const DT_METAFILE: ::c_int = 5;
pub const DT_DISPFILE: ::c_int = 6;

pub const CLR_INVALID: ::COLORREF = 0xFFFFFFFF;

pub const ETO_OPAQUE: ::UINT = 0x0002;
pub const ETO_CLIPPED: ::UINT = 0x0004;
pub const ETO_GLYPH_INDEX: ::UINT = 0x0010;
pub const ETO_RTLREADING: ::UINT = 0x0080;
pub const ETO_NUMERICSLOCAL: ::UINT = 0x0400;
pub const ETO_NUMERICSLATIN: ::UINT = 0x0800;
pub const ETO_IGNORELANGUAGE: ::UINT = 0x1000;
pub const ETO_PDY: ::UINT = 0x2000;
pub const ETO_REVERSE_INDEX_MAP: ::UINT = 0x10000;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct EXTLOGPEN {
    pub elpPenStyle: ::DWORD, 
    pub elpWidth: ::DWORD, 
    pub elpBrushStyle: ::UINT, 
    pub elpColor: ::COLORREF, 
    pub elpHatch: ::ULONG_PTR, 
    pub elpNumEntries: ::DWORD, 
    pub elpStyleEntry: [::DWORD; 1],
}

pub type PEXTLOGPEN = *mut EXTLOGPEN;
pub type NPEXTLOGPEN = *mut EXTLOGPEN;
pub type LPEXTLOGPEN = *mut EXTLOGPEN;

pub type ENHMFENUMPROC = Option<unsafe extern "system" fn(
    hdc: ::HDC, lpht: HANDLETABLE, lpmr: *const ENHMETARECORD, nHandles: ::c_int, data: ::LPARAM
) -> ::c_int>;
