// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::guiddef::{GUID, REFGUID, REFIID};
use shared::minwindef::{BOOL, BYTE, DWORD, LPBOOL, LPDWORD, LPLONG, LPVOID, WORD};
use shared::windef::{HDC, HMONITOR, HWND, LPRECT, LPSIZE};
use shared::winerror::{
    CO_E_NOTINITIALIZED, E_FAIL, E_INVALIDARG, E_NOTIMPL, E_OUTOFMEMORY, HRESULT, MAKE_HRESULT,
    S_FALSE, S_OK
};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::wingdi::{LPPALETTEENTRY, LPRGNDATA};
use um::winnt::{HANDLE, LARGE_INTEGER, LONG, LPSTR, LPWSTR};
pub const _FACDD: HRESULT = 0x876;
#[inline]
pub fn MAKE_DDHRESULT(code: HRESULT) -> HRESULT {
    MAKE_HRESULT(1, _FACDD, code)
}
pub const FOURCC_DXT1: DWORD = MAKEFOURCC!(b'D', b'X', b'T', b'1');
pub const FOURCC_DXT2: DWORD = MAKEFOURCC!(b'D', b'X', b'T', b'2');
pub const FOURCC_DXT3: DWORD = MAKEFOURCC!(b'D', b'X', b'T', b'3');
pub const FOURCC_DXT4: DWORD = MAKEFOURCC!(b'D', b'X', b'T', b'4');
pub const FOURCC_DXT5: DWORD = MAKEFOURCC!(b'D', b'X', b'T', b'5');
DEFINE_GUID!{CLSID_DirectDraw,
    0xd7b70ee0, 0x4340, 0x11cf, 0xb0, 0x63, 0x00, 0x20, 0xaf, 0xc2, 0xcd, 0x35}
DEFINE_GUID!{CLSID_DirectDraw7,
    0x3c305196, 0x50db, 0x11d3, 0x9c, 0xfe, 0x00, 0xc0, 0x4f, 0xd9, 0x30, 0xc5}
DEFINE_GUID!{CLSID_DirectDrawClipper,
    0x593817a0, 0x7db3, 0x11cf, 0xa2, 0xde, 0x00, 0xaa, 0x00, 0xb9, 0x33, 0x56}
DEFINE_GUID!{IID_IDirectDraw,
    0x6c14db80, 0xa733, 0x11ce, 0xa5, 0x21, 0x00, 0x20, 0xaf, 0x0b, 0xe5, 0x60}
DEFINE_GUID!{IID_IDirectDraw2,
    0xb3a6f3e0, 0x2b43, 0x11cf, 0xa2, 0xde, 0x00, 0xaa, 0x00, 0xb9, 0x33, 0x56}
DEFINE_GUID!{IID_IDirectDraw4,
    0x9c59509a, 0x39bd, 0x11d1, 0x8c, 0x4a, 0x00, 0xc0, 0x4f, 0xd9, 0x30, 0xc5}
DEFINE_GUID!{IID_IDirectDraw7,
    0x15e65ec0, 0x3b9c, 0x11d2, 0xb9, 0x2f, 0x00, 0x60, 0x97, 0x97, 0xea, 0x5b}
DEFINE_GUID!{IID_IDirectDrawSurface,
    0x6c14db81, 0xa733, 0x11ce, 0xa5, 0x21, 0x00, 0x20, 0xaf, 0x0b, 0xe5, 0x60}
DEFINE_GUID!{IID_IDirectDrawSurface2,
    0x57805885, 0x6eec, 0x11cf, 0x94, 0x41, 0xa8, 0x23, 0x03, 0xc1, 0x0e, 0x27}
DEFINE_GUID!{IID_IDirectDrawSurface3,
    0xda044e00, 0x69b2, 0x11d0, 0xa1, 0xd5, 0x00, 0xaa, 0x00, 0xb8, 0xdf, 0xbb}
DEFINE_GUID!{IID_IDirectDrawSurface4,
    0x0b2b8630, 0xad35, 0x11d0, 0x8e, 0xa6, 0x00, 0x60, 0x97, 0x97, 0xea, 0x5b}
DEFINE_GUID!{IID_IDirectDrawSurface7,
    0x06675a80, 0x3b9b, 0x11d2, 0xb9, 0x2f, 0x00, 0x60, 0x97, 0x97, 0xea, 0x5b}
DEFINE_GUID!{IID_IDirectDrawPalette,
    0x6c14db84, 0xa733, 0x11ce, 0xa5, 0x21, 0x00, 0x20, 0xaf, 0x0b, 0xe5, 0x60}
DEFINE_GUID!{IID_IDirectDrawClipper,
    0x6c14db85, 0xa733, 0x11ce, 0xa5, 0x21, 0x00, 0x20, 0xaf, 0x0b, 0xe5, 0x60}
DEFINE_GUID!{IID_IDirectDrawColorControl,
    0x4b9f0ee0, 0x0d7e, 0x11d0, 0x9b, 0x06, 0x00, 0xa0, 0xc9, 0x03, 0xa3, 0xb8}
DEFINE_GUID!{IID_IDirectDrawGammaControl,
    0x69c11c3e, 0xb46b, 0x11d1, 0xad, 0x7a, 0x00, 0xc0, 0x4f, 0xc2, 0x9b, 0x4e}
pub type LPDIRECTDRAW = *mut IDirectDraw;
pub type LPDIRECTDRAW2 = *mut IDirectDraw2;
pub type LPDIRECTDRAW4 = *mut IDirectDraw4;
pub type LPDIRECTDRAW7 = *mut IDirectDraw7;
pub type LPDIRECTDRAWSURFACE = *mut IDirectDrawSurface;
pub type LPDIRECTDRAWSURFACE2 = *mut IDirectDrawSurface2;
pub type LPDIRECTDRAWSURFACE3 = *mut IDirectDrawSurface3;
pub type LPDIRECTDRAWSURFACE4 = *mut IDirectDrawSurface4;
pub type LPDIRECTDRAWSURFACE7 = *mut IDirectDrawSurface7;
pub type LPDIRECTDRAWPALETTE = *mut IDirectDrawPalette;
pub type LPDIRECTDRAWCLIPPER = *mut IDirectDrawClipper;
pub type LPDIRECTDRAWCOLORCONTROL = *mut IDirectDrawColorControl;
pub type LPDIRECTDRAWGAMMACONTROL = *mut IDirectDrawGammaControl;
// pub type LPDDFXROP = *mut DDFXROP;
pub type LPDDSURFACEDESC = *mut DDSURFACEDESC;
pub type LPDDSURFACEDESC2 = *mut DDSURFACEDESC2;
pub type LPDDCOLORCONTROL = *mut DDCOLORCONTROL;
FN!{stdcall LPDDENUMCALLBACKA(
    *mut GUID,
    LPSTR,
    LPSTR,
    LPVOID,
) -> BOOL}
FN!{stdcall LPDDENUMCALLBACKW(
    *mut GUID,
    LPWSTR,
    LPWSTR,
    LPVOID,
) -> BOOL}
extern "system" {
    pub fn DirectDrawEnumerateW(
        lpCallback: LPDDENUMCALLBACKW,
        lpContext: LPVOID,
    ) -> HRESULT;
    pub fn DirectDrawEnumerateA(
        lpCallback: LPDDENUMCALLBACKA,
        lpContext: LPVOID,
    ) -> HRESULT;
}
FN!{stdcall LPDDENUMCALLBACKEXA(
    *mut GUID,
    LPSTR,
    LPSTR,
    LPVOID,
    HMONITOR,
) -> bool}
FN!{stdcall LPDDENUMCALLBACKEXW(
    *mut GUID,
    LPWSTR,
    LPWSTR,
    LPVOID,
    HMONITOR,
) -> bool}
extern "system" {
    pub fn DirectDrawEnumerateExW(
        lpCallback: LPDDENUMCALLBACKEXW,
        lpContext: LPVOID,
        dwFlags: DWORD,
    ) -> HRESULT;
    pub fn DirectDrawEnumerateExA(
        lpCallback: LPDDENUMCALLBACKEXA,
        lpContext: LPVOID,
        dwFlags: DWORD,
    ) -> HRESULT;
}
FN!{stdcall LPDIRECTDRAWENUMERATEEXA(
    lpCallback: LPDDENUMCALLBACKEXA,
    plContext: LPVOID,
    dwFlags: DWORD,
) -> HRESULT}
FN!{stdcall LPDIRECTDRAWENUMERATEEXW(
    lpCallback: LPDDENUMCALLBACKEXW,
    plContext: LPVOID,
    dwFlags: DWORD,
) -> HRESULT}
extern "system" {
    pub fn DirectDrawCreate(
        lpGUID: *mut GUID,
        lplpDD: *mut LPDIRECTDRAW,
        pUnkOuter: *mut IUnknown,
    ) -> HRESULT;
    pub fn DirectDrawCreateEx(
        lpGuid: *mut GUID,
        lplpDD: *mut LPVOID,
        iid: REFIID,
        pUnkOuter: *mut IUnknown,
    ) -> HRESULT;
    pub fn DirectDrawCreateClipper(
        dwFlags: DWORD,
        lplpDDClipper: *mut LPDIRECTDRAWCLIPPER,
        pUnkOuter: *mut IUnknown,
    ) -> HRESULT;
}
pub const DDENUM_ATTACHEDSECONDARYDEVICES: DWORD = 0x00000001;
pub const DDENUM_DETACHEDSECONDARYDEVICES: DWORD = 0x00000002;
pub const DDENUM_NONDISPLAYDEVICES: DWORD = 0x00000004;
pub const REGSTR_KEY_DDHW_DESCRIPTION: &'static str = "Description";
pub const REGSTR_KEY_DDHW_DRIVERNAME: &'static str = "DriverName";
pub const REGSTR_PATH_DDHW: &'static str = "Hardware\\DirectDrawDrivers";
pub const DDCREATE_HARDWAREONLY: DWORD = 0x00000001;
pub const DDCREATE_EMULATIONONLY: DWORD = 0x00000002;
FN!{stdcall LPDDENUMMODESCALLBACK(
    LPDDSURFACEDESC,
    LPVOID,
) -> HRESULT}
FN!{stdcall LPDDENUMMODESCALLBACK2(
    LPDDSURFACEDESC2,
    LPVOID,
) -> HRESULT}
FN!{stdcall LPDDENUMSURFACESCALLBACK(
    LPDIRECTDRAWSURFACE,
    LPDDSURFACEDESC,
    LPVOID,
) -> HRESULT}
FN!{stdcall LPDDENUMSURFACESCALLBACK2(
    LPDIRECTDRAWSURFACE4,
    LPDDSURFACEDESC2,
    LPVOID,
) -> HRESULT}
FN!{stdcall LPDDENUMSURFACESCALLBACK7(
    LPDIRECTDRAWSURFACE7,
    LPDDSURFACEDESC2,
    LPVOID,
) -> HRESULT}
STRUCT!{struct DDARGB {
    blue: BYTE,
    green: BYTE,
    red: BYTE,
    alpha: BYTE,
}}
pub type LPDDARGB = *mut DDARGB;
STRUCT!{struct DDRGBA {
    red: BYTE,
    green: BYTE,
    blue: BYTE,
    alpha: BYTE,
}}
pub type LPDDRGBA = *mut DDRGBA;
STRUCT!{struct DDCOLORKEY {
    dwColorSpaceLowValue: DWORD,
    dwColorSpaceHighValue: DWORD,
}}
pub type LPDDCOLORKEY = *mut DDCOLORKEY;
UNION!{union DDBLTFX_u1 {
    [usize; 1],
    dwZDestConst dwZDestConst_mut: DWORD,
    lpDDSZBufferDest lpDDSZBufferDest_mut: LPDIRECTDRAWSURFACE,
}}
UNION!{union DDBLTFX_u2 {
    [usize; 1],
    dwZSrcConst dwZSrcConst_mut: DWORD,
    lpDDSZBufferSrc lpDDSZBufferSrc_mut: LPDIRECTDRAWSURFACE,
}}
UNION!{union DDBLTFX_u3 {
    [usize; 1],
    dwAlphaDestConst dwAlphaDestConst_mut: DWORD,
    lpDDSAlphaDest lpDDSAlphaDest_mut: LPDIRECTDRAWSURFACE,
}}
UNION!{union DDBLTFX_u4 {
    [usize; 1],
    dwAlphaSrcConst dwAlphaSrcConst_mut: DWORD,
    lpDDSAlphaSrc lpDDSAlphaSrc_mut: LPDIRECTDRAWSURFACE,
}}
UNION!{union DDBLTFX_u5 {
    [usize; 1],
    dwFillColor dwFillColor_mut: DWORD,
    dwFillDepth dwFillDepth_mut: DWORD,
    dwFillPixel dwFillPixel_mut: DWORD,
    lpDDSPattern lpDDSPattern_mut: LPDIRECTDRAWSURFACE,
}}
STRUCT!{struct DDBLTFX {
    dwSize: DWORD,
    dwDDFX: DWORD,
    dwROP: DWORD,
    dwDDROP: DWORD,
    dwRotationAngle: DWORD,
    dwZBufferOpCode: DWORD,
    dwZBufferLow: DWORD,
    dwZBufferHigh: DWORD,
    dwZBufferBaseDest: DWORD,
    dwZDestConstBitDepth: DWORD,
    u1: DDBLTFX_u1,
    dwZSrcConstBitDepth: DWORD,
    u2: DDBLTFX_u2,
    dwAlphaEdgeBlendBitDepth: DWORD,
    dwAlphaEdgeBlend: DWORD,
    dwReserved: DWORD,
    dwAlphaDestConstBitDepth: DWORD,
    u3: DDBLTFX_u3,
    dwAlphaSrcConstBitDepth: DWORD,
    u4: DDBLTFX_u4,
    u5: DDBLTFX_u5,
    ddckDestColorkey: DDCOLORKEY,
    ddckSrcColorkey: DDCOLORKEY,
}}
pub type LPDDBLTFX = *mut DDBLTFX;
STRUCT!{struct DDSCAPS {
    dwCaps: DWORD,
}}
pub type LPDDSCAPS = *mut DDSCAPS;
STRUCT!{struct DDOSCAPS {
    dwCaps: DWORD,
}}
pub type LPDDOSCAPS = *mut DDOSCAPS;
UNION!{union DDSCAPSEX_u1 {
    [u32; 1],
    dwCaps4 dwCaps4_mut: DWORD,
    dwVolumeDepth dwVolumeDepth_mut: DWORD,
}}
STRUCT!{struct DDSCAPSEX {
    dwCaps2: DWORD,
    dwCaps3: DWORD,
    u1: DDSCAPSEX_u1,
}}
pub type LPDDSCAPSEX = *mut DDSCAPSEX;
UNION!{union DDSCAPS2_u1 {
    [u32; 1],
    dwCaps4 dwCaps4_mut: DWORD,
    dwVolumeDepth dwVolumeDepth_mut: DWORD,
}}
STRUCT!{struct DDSCAPS2 {
    dwCaps: DWORD,
    dwCaps2: DWORD,
    dwCaps3: DWORD,
    u1: DDSCAPS2_u1,
}}
pub type LPDDSCAPS2 = *mut DDSCAPS2;
pub const DD_ROP_SPACE: usize = 256 / 32;
STRUCT!{struct DDCAPS_DX1 {
    dwSize: DWORD,
    dwCaps: DWORD,
    dwCaps2: DWORD,
    dwCKeyCaps: DWORD,
    dwFXCaps: DWORD,
    dwFXAlphaCaps: DWORD,
    dwPalCaps: DWORD,
    dwSVCaps: DWORD,
    dwAlphaBltConstBitDepths: DWORD,
    dwAlphaBltPixelBitDepths: DWORD,
    dwAlphaBltSurfaceBitDepths: DWORD,
    dwAlphaOverlayConstBitDepths: DWORD,
    dwAlphaOverlayPixelBitDepths: DWORD,
    dwAlphaOverlaySurfaceBitDepths: DWORD,
    dwZBufferBitDepths: DWORD,
    dwVidMemTotal: DWORD,
    dwVidMemFree: DWORD,
    dwMaxVisibleOverlays: DWORD,
    dwCurrVisibleOverlays: DWORD,
    dwNumFourCCCodes: DWORD,
    dwAlignBoundarySrc: DWORD,
    dwAlignSizeSrc: DWORD,
    dwAlignBoundaryDest: DWORD,
    dwAlignSizeDest: DWORD,
    dwAlignStrideAlign: DWORD,
    dwRops: [DWORD; DD_ROP_SPACE],
    ddsCaps: DDSCAPS,
    dwMinOverlayStretch: DWORD,
    dwMaxOverlayStretch: DWORD,
    dwMinLiveVideoStretch: DWORD,
    dwMaxLiveVideoStretch: DWORD,
    dwMinHwCodecStretch: DWORD,
    dwMaxHwCodecStretch: DWORD,
    dwReserved1: DWORD,
    dwReserved2: DWORD,
    dwReserved3: DWORD,
}}
pub type LPDDCAPS_DX1 = *mut DDCAPS_DX1;
STRUCT!{struct DDCAPS_DX3 {
    dwSize: DWORD,
    dwCaps: DWORD,
    dwCaps2: DWORD,
    dwCKeyCaps: DWORD,
    dwFXCaps: DWORD,
    dwFXAlphaCaps: DWORD,
    dwPalCaps: DWORD,
    dwSVCaps: DWORD,
    dwAlphaBltConstBitDepths: DWORD,
    dwAlphaBltPixelBitDepths: DWORD,
    dwAlphaBltSurfaceBitDepths: DWORD,
    dwAlphaOverlayConstBitDepths: DWORD,
    dwAlphaOverlayPixelBitDepths: DWORD,
    dwAlphaOverlaySurfaceBitDepths: DWORD,
    dwZBufferBitDepths: DWORD,
    dwVidMemTotal: DWORD,
    dwVidMemFree: DWORD,
    dwMaxVisibleOverlays: DWORD,
    dwCurrVisibleOverlays: DWORD,
    dwNumFourCCCodes: DWORD,
    dwAlignBoundarySrc: DWORD,
    dwAlignSizeSrc: DWORD,
    dwAlignBoundaryDest: DWORD,
    dwAlignSizeDest: DWORD,
    dwAlignStrideAlign: DWORD,
    dwRops: [DWORD; DD_ROP_SPACE],
    ddsCaps: DDSCAPS,
    dwMinOverlayStretch: DWORD,
    dwMaxOverlayStretch: DWORD,
    dwMinLiveVideoStretch: DWORD,
    dwMaxLiveVideoStretch: DWORD,
    dwMinHwCodecStretch: DWORD,
    dwMaxHwCodecStretch: DWORD,
    dwReserved1: DWORD,
    dwReserved2: DWORD,
    dwReserved3: DWORD,
    dwSVBCaps: DWORD,
    dwSVBCKeyCaps: DWORD,
    dwSVBFXCaps: DWORD,
    dwSVBRops: [DWORD; DD_ROP_SPACE],
    dwVSBCaps: DWORD,
    dwVSBCKeyCaps: DWORD,
    dwVSBFXCaps: DWORD,
    dwVSBRops: [DWORD; DD_ROP_SPACE],
    dwSSBCaps: DWORD,
    dwSSBCKeyCaps: DWORD,
    dwSSBFXCaps: DWORD,
    dwSSBRops: [DWORD; DD_ROP_SPACE],
    dwReserved4: DWORD,
    dwReserved5: DWORD,
    dwReserved6: DWORD,
}}
pub type LPDDCAPS_DX3 = *mut DDCAPS_DX3;
STRUCT!{struct DDCAPS_DX5 {
    dwSize: DWORD,
    dwCaps: DWORD,
    dwCaps2: DWORD,
    dwCKeyCaps: DWORD,
    dwFXCaps: DWORD,
    dwFXAlphaCaps: DWORD,
    dwPalCaps: DWORD,
    dwSVCaps: DWORD,
    dwAlphaBltConstBitDepths: DWORD,
    dwAlphaBltPixelBitDepths: DWORD,
    dwAlphaBltSurfaceBitDepths: DWORD,
    dwAlphaOverlayConstBitDepths: DWORD,
    dwAlphaOverlayPixelBitDepths: DWORD,
    dwAlphaOverlaySurfaceBitDepths: DWORD,
    dwZBufferBitDepths: DWORD,
    dwVidMemTotal: DWORD,
    dwVidMemFree: DWORD,
    dwMaxVisibleOverlays: DWORD,
    dwCurrVisibleOverlays: DWORD,
    dwNumFourCCCodes: DWORD,
    dwAlignBoundarySrc: DWORD,
    dwAlignSizeSrc: DWORD,
    dwAlignBoundaryDest: DWORD,
    dwAlignSizeDest: DWORD,
    dwAlignStrideAlign: DWORD,
    dwRops: [DWORD; DD_ROP_SPACE],
    ddsCaps: DDSCAPS,
    dwMinOverlayStretch: DWORD,
    dwMaxOverlayStretch: DWORD,
    dwMinLiveVideoStretch: DWORD,
    dwMaxLiveVideoStretch: DWORD,
    dwMinHwCodecStretch: DWORD,
    dwMaxHwCodecStretch: DWORD,
    dwReserved1: DWORD,
    dwReserved2: DWORD,
    dwReserved3: DWORD,
    dwSVBCaps: DWORD,
    dwSVBCKeyCaps: DWORD,
    dwSVBFXCaps: DWORD,
    dwSVBRops: [DWORD; DD_ROP_SPACE],
    dwVSBCaps: DWORD,
    dwVSBCKeyCaps: DWORD,
    dwVSBFXCaps: DWORD,
    dwVSBRops: [DWORD; DD_ROP_SPACE],
    dwSSBCaps: DWORD,
    dwSSBCKeyCaps: DWORD,
    dwSSBFXCaps: DWORD,
    dwSSBRops: [DWORD; DD_ROP_SPACE],
    dwMaxVideoPorts: DWORD,
    dwCurrVideoPorts: DWORD,
    dwSVBCaps2: DWORD,
    dwNLVBCaps: DWORD,
    dwNLVBCaps2: DWORD,
    dwNLVBCKeyCaps: DWORD,
    dwNLVBFXCaps: DWORD,
    dwNLVBRops: [DWORD; DD_ROP_SPACE],
}}
pub type LPDDCAPS_DX5 = *mut DDCAPS_DX5;
STRUCT!{struct DDCAPS_DX6 {
    dwSize: DWORD,
    dwCaps: DWORD,
    dwCaps2: DWORD,
    dwCKeyCaps: DWORD,
    dwFXCaps: DWORD,
    dwFXAlphaCaps: DWORD,
    dwPalCaps: DWORD,
    dwSVCaps: DWORD,
    dwAlphaBltConstBitDepths: DWORD,
    dwAlphaBltPixelBitDepths: DWORD,
    dwAlphaBltSurfaceBitDepths: DWORD,
    dwAlphaOverlayConstBitDepths: DWORD,
    dwAlphaOverlayPixelBitDepths: DWORD,
    dwAlphaOverlaySurfaceBitDepths: DWORD,
    dwZBufferBitDepths: DWORD,
    dwVidMemTotal: DWORD,
    dwVidMemFree: DWORD,
    dwMaxVisibleOverlays: DWORD,
    dwCurrVisibleOverlays: DWORD,
    dwNumFourCCCodes: DWORD,
    dwAlignBoundarySrc: DWORD,
    dwAlignSizeSrc: DWORD,
    dwAlignBoundaryDest: DWORD,
    dwAlignSizeDest: DWORD,
    dwAlignStrideAlign: DWORD,
    dwRops: [DWORD; DD_ROP_SPACE],
    ddsOldCaps: DDSCAPS,
    dwMinOverlayStretch: DWORD,
    dwMaxOverlayStretch: DWORD,
    dwMinLiveVideoStretch: DWORD,
    dwMaxLiveVideoStretch: DWORD,
    dwMinHwCodecStretch: DWORD,
    dwMaxHwCodecStretch: DWORD,
    dwReserved1: DWORD,
    dwReserved2: DWORD,
    dwReserved3: DWORD,
    dwSVBCaps: DWORD,
    dwSVBCKeyCaps: DWORD,
    dwSVBFXCaps: DWORD,
    dwSVBRops: [DWORD; DD_ROP_SPACE],
    dwVSBCaps: DWORD,
    dwVSBCKeyCaps: DWORD,
    dwVSBFXCaps: DWORD,
    dwVSBRops: [DWORD; DD_ROP_SPACE],
    dwSSBCaps: DWORD,
    dwSSBCKeyCaps: DWORD,
    dwSSBFXCaps: DWORD,
    dwSSBRops: [DWORD; DD_ROP_SPACE],
    dwMaxVideoPorts: DWORD,
    dwCurrVideoPorts: DWORD,
    dwSVBCaps2: DWORD,
    dwNLVBCaps: DWORD,
    dwNLVBCaps2: DWORD,
    dwNLVBCKeyCaps: DWORD,
    dwNLVBFXCaps: DWORD,
    dwNLVBRops: [DWORD; DD_ROP_SPACE],
    ddsCaps: DDSCAPS2,
}}
pub type LPDDCAPS_DX6 = *mut DDCAPS_DX6;
STRUCT!{struct DDCAPS_DX7 {
    dwSize: DWORD,
    dwCaps: DWORD,
    dwCaps2: DWORD,
    dwCKeyCaps: DWORD,
    dwFXCaps: DWORD,
    dwFXAlphaCaps: DWORD,
    dwPalCaps: DWORD,
    dwSVCaps: DWORD,
    dwAlphaBltConstBitDepths: DWORD,
    dwAlphaBltPixelBitDepths: DWORD,
    dwAlphaBltSurfaceBitDepths: DWORD,
    dwAlphaOverlayConstBitDepths: DWORD,
    dwAlphaOverlayPixelBitDepths: DWORD,
    dwAlphaOverlaySurfaceBitDepths: DWORD,
    dwZBufferBitDepths: DWORD,
    dwVidMemTotal: DWORD,
    dwVidMemFree: DWORD,
    dwMaxVisibleOverlays: DWORD,
    dwCurrVisibleOverlays: DWORD,
    dwNumFourCCCodes: DWORD,
    dwAlignBoundarySrc: DWORD,
    dwAlignSizeSrc: DWORD,
    dwAlignBoundaryDest: DWORD,
    dwAlignSizeDest: DWORD,
    dwAlignStrideAlign: DWORD,
    dwRops: [DWORD; DD_ROP_SPACE],
    ddsOldCaps: DDSCAPS,
    dwMinOverlayStretch: DWORD,
    dwMaxOverlayStretch: DWORD,
    dwMinLiveVideoStretch: DWORD,
    dwMaxLiveVideoStretch: DWORD,
    dwMinHwCodecStretch: DWORD,
    dwMaxHwCodecStretch: DWORD,
    dwReserved1: DWORD,
    dwReserved2: DWORD,
    dwReserved3: DWORD,
    dwSVBCaps: DWORD,
    dwSVBCKeyCaps: DWORD,
    dwSVBFXCaps: DWORD,
    dwSVBRops: [DWORD; DD_ROP_SPACE],
    dwVSBCaps: DWORD,
    dwVSBCKeyCaps: DWORD,
    dwVSBFXCaps: DWORD,
    dwVSBRops: [DWORD; DD_ROP_SPACE],
    dwSSBCaps: DWORD,
    dwSSBCKeyCaps: DWORD,
    dwSSBFXCaps: DWORD,
    dwSSBRops: [DWORD; DD_ROP_SPACE],
    dwMaxVideoPorts: DWORD,
    dwCurrVideoPorts: DWORD,
    dwSVBCaps2: DWORD,
    dwNLVBCaps: DWORD,
    dwNLVBCaps2: DWORD,
    dwNLVBCKeyCaps: DWORD,
    dwNLVBFXCaps: DWORD,
    dwNLVBRops: [DWORD; DD_ROP_SPACE],
    ddsCaps: DDSCAPS2,
}}
pub type LPDDCAPS_DX7 = *mut DDCAPS_DX7;
pub type DDCAPS = DDCAPS_DX7;
pub type LPDDCAPS = *mut DDCAPS;
UNION!{union DDPIXELFORMAT_u1 {
    [u32; 1],
    dwRGBBitCount dwRGBBitCount_mut: DWORD,
    dwYUVBitCount dwYUVBitCount_mut: DWORD,
    dwZBufferBitDepth dwZBufferBitDepth_mut: DWORD,
    dwAlphaBitDepth dwAlphaBitDepth_mut: DWORD,
    dwLuminanceBitCount dwLuminanceBitCount_mut: DWORD,
    dwBumpBitCount dwBumpBitCount_mut: DWORD,
    dwPrivateFormatBitCount dwPrivateFormatBitCount_mut: DWORD,
}}
UNION!{union DDPIXELFORMAT_u2 {
    [u32; 1],
    dwRBitMask dwRBitMask_mut: DWORD,
    dwYBitMask dwYBitMask_mut: DWORD,
    dwStencilBitDepth dwStencilBitDepth_mut: DWORD,
    dwLuminanceBitMask dwLuminanceBitMask_mut: DWORD,
    dwBumpDuBitMask dwBumpDuBitMask_mut: DWORD,
    dwOperations dwOperations_mut: DWORD,
}}
STRUCT!{struct DDPIXELFORMAT_u3_MultiSampleCaps {
    wFlipMSTypes: WORD,
    wBltMSTypes: WORD,
}}
UNION!{union DDPIXELFORMAT_u3 {
    [u32; 1],
    dwGBitMask dwGBitMask_mut: DWORD,
    dwUBitMask dwUBitMask_mut: DWORD,
    dwZBitMask dwZBitMask_mut: DWORD,
    dwBumpDvBitMask dwBumpDvBitMask_mut: DWORD,
    MultiSampleCaps MultiSampleCaps_mut: DDPIXELFORMAT_u3_MultiSampleCaps,
}}
UNION!{union DDPIXELFORMAT_u4 {
    [u32; 1],
    dwBBitMask dwBBitMask_mut: DWORD,
    dwVBitMask dwVBitMask_mut: DWORD,
    dwStencilBitMask dwStencilBitMask_mut: DWORD,
    dwBumpLuminanceBitMask dwBumpLuminanceBitMask_mut: DWORD,
}}
UNION!{union DDPIXELFORMAT_u5 {
    [u32; 1],
    dwRGBAlphaBitMask dwRGBAlphaBitMask_mut: DWORD,
    dwYUVAlphaBitMask dwYUVAlphaBitMask_mut: DWORD,
    dwLuminanceAlphaBitMask dwLuminanceAlphaBitMask_mut: DWORD,
    dwRGBZBitMask dwRGBZBitMask_mut: DWORD,
    dwYUVZBitMask dwYUVZBitMask_mut: DWORD,
}}
STRUCT!{struct DDPIXELFORMAT {
    dwSize: DWORD,
    dwFlags: DWORD,
    dwFourCC: DWORD,
    u1: DDPIXELFORMAT_u1,
    u2: DDPIXELFORMAT_u2,
    u3: DDPIXELFORMAT_u3,
    u4: DDPIXELFORMAT_u4,
    u5: DDPIXELFORMAT_u5,
}}
pub type LPDDPIXELFORMAT = *mut DDPIXELFORMAT;
UNION!{union DDOVERLAYFX_u1 {
    [usize; 1],
    dwAlphaDestConst dwAlphaDestConst_mut: DWORD,
    lpDDSAlphaDest lpDDSAlphaDest_mut: LPDIRECTDRAWSURFACE,
}}
UNION!{union DDOVERLAYFX_u2 {
    [usize; 1],
    dwAlphaSrcConst dwAlphaSrcConst_mut: DWORD,
    lpDDSAlphaSrc lpDDSAlphaSrc_mut: LPDIRECTDRAWSURFACE,
}}
STRUCT!{struct DDOVERLAYFX {
    dwSize: DWORD,
    dwAlphaEdgeBlendBitDepth: DWORD,
    dwAlphaEdgeBlend: DWORD,
    dwReserved: DWORD,
    dwAlphaDestConstBitDepth: DWORD,
    u1: DDOVERLAYFX_u1,
    dwAlphaSrcConstBitDepth: DWORD,
    u2: DDOVERLAYFX_u2,
    dckDestColorkey: DDCOLORKEY,
    dckSrcColorkey: DDCOLORKEY,
    dwDDFX: DWORD,
    dwFlags: DWORD,
}}
pub type LPDDOVERLAYFX = *mut DDOVERLAYFX;
STRUCT!{struct DDBLTBATCH {
    lprDest: LPRECT,
    lpDDSSrc: LPDIRECTDRAWSURFACE,
    lprSrc: LPRECT,
    dwFlags: DWORD,
    lpDDBltFx: LPDDBLTFX,
}}
pub type LPDDBLTBATCH = *mut DDBLTBATCH;
STRUCT!{struct DDGAMMARAMP {
    red: [WORD; 256],
    green: [WORD; 256],
    blue: [WORD; 256],
}}
pub type LPDDGAMMARAMP = *mut DDGAMMARAMP;
pub const MAX_DDDEVICEID_STRING: usize = 512;
STRUCT!{struct DDDEVICEIDENTIFIER {
    szDriver: [char; MAX_DDDEVICEID_STRING],
    szDescription: [char; MAX_DDDEVICEID_STRING],
    liDriverVersion: LARGE_INTEGER,
    dwVendorId: DWORD,
    dwDeviceId: DWORD,
    dwSubSysId: DWORD,
    dwRevision: DWORD,
    guidDeviceIdentifier: GUID,
}}
pub type LPDDDEVICEIDENTIFIER = *mut DDDEVICEIDENTIFIER;
STRUCT!{struct DDDEVICEIDENTIFIER2 {
    szDriver: [char; MAX_DDDEVICEID_STRING],
    szDescription: [char; MAX_DDDEVICEID_STRING],
    liDriverVersion: LARGE_INTEGER,
    dwVendorId: DWORD,
    dwDeviceId: DWORD,
    dwSubSysId: DWORD,
    dwRevision: DWORD,
    guidDeviceIdentifier: GUID,
    dwWHQLLevel: DWORD,
}}
pub type LPDDDEVICEIDENTIFIER2 = *mut DDDEVICEIDENTIFIER2;
pub const DDGDI_GETHOSTIDENTIFIER: DWORD = 0x00000001;
#[inline]
pub fn GET_WHQL_YEAR(dwWHQLLevel: u32) -> u32 {
    dwWHQLLevel / 0x10000
}
#[inline]
pub fn GET_WHQL_MONTH(dwWHQLLevel: u32) -> u32 {
    (dwWHQLLevel / 0x100) & 0x00ff
}
#[inline]
pub fn GET_WHQL_DAY(dwWHQLLevel: u32) -> u32 {
    dwWHQLLevel & 0xff
}
FN!{stdcall LPCLIPPERCALLBACK(
    lpDDClipper: LPDIRECTDRAWCLIPPER,
    hWnd: HWND,
    code: DWORD,
    lpContext: LPVOID,
) -> DWORD}
FN!{stdcall LPSURFACESTREAMINGCALLBACK(
    DWORD,
) -> DWORD}
RIDL!{#[uuid(0x6c14db80, 0xa733, 0x11ce, 0xa5, 0x21, 0x00, 0x20, 0xaf, 0x0b, 0xe5, 0x60)]
interface IDirectDraw(IDirectDrawVtbl): IUnknown(IUnknownVtbl) {
    fn Compact() -> HRESULT,
    fn CreateClipper(
        dwFlags: DWORD,
        lplpDDClipper: *mut LPDIRECTDRAWCLIPPER,
        pUnkOuter: *mut IUnknown,
    ) -> HRESULT,
    fn CreatePalette(
        dwFlags: DWORD,
        lpDDColorArray: LPPALETTEENTRY,
        lplpDDPalette: *mut LPDIRECTDRAWPALETTE,
        pUnkOuter: *mut IUnknown,
    ) -> HRESULT,
    fn CreateSurface(
        lpDDSurfaceDesc: LPDDSURFACEDESC,
        lplpDDSurface: *mut LPDIRECTDRAWSURFACE,
        pUnkOuter: *mut IUnknown,
    ) -> HRESULT,
    fn DuplicateSurface(
        lpDDSurface: LPDIRECTDRAWSURFACE,
        lplpDupDDSurface: *mut LPDIRECTDRAWSURFACE,
    ) -> HRESULT,
    fn EnumDisplayModes(
        dwFlags: DWORD,
        lpDDSurfaceDesc: LPDDSURFACEDESC,
        lpContext: LPVOID,
        lpEnumModesCallback: LPDDENUMMODESCALLBACK,
    ) -> HRESULT,
    fn EnumSurfaces(
        dwFlags: DWORD,
        lpDDSD: LPDDSURFACEDESC,
        lpContext: LPVOID,
        lpEnumSurfacesCallback: LPDDENUMSURFACESCALLBACK,
    ) -> HRESULT,
    fn FlipToGDISurface() -> HRESULT,
    fn GetCaps(
        lpDDDriverCaps: LPDDCAPS,
        lpDDHELCaps: LPDDCAPS,
    ) -> HRESULT,
    fn GetDisplayMode(
        lpDDSurfaceDesc: LPDDSURFACEDESC,
    ) -> HRESULT,
    fn GetFourCCCodes(
        lpNumCodes: LPDWORD,
        lpCodes: LPDWORD,
    ) -> HRESULT,
    fn GetGDISurface(
        lplpGDIDDSSurface: *mut LPDIRECTDRAWSURFACE,
    ) -> HRESULT,
    fn GetMonitorFrequency(
        lpdwFrequency: LPDWORD,
    ) -> HRESULT,
    fn GetScanLine(
        lpdwScanLine: LPDWORD,
    ) -> HRESULT,
    fn GetVerticalBlankStatus(
        lpbIsInVB: LPBOOL,
    ) -> HRESULT,
    fn Initialize(
        lpGUID: *mut GUID,
    ) -> HRESULT,
    fn RestoreDisplayMode() -> HRESULT,
    fn SetCooperativeLevel(
        hWnd: HWND,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn SetDisplayMode(
        dwWidth: DWORD,
        dwHeight: DWORD,
        dwBPP: DWORD,
    ) -> HRESULT,
    fn WaitForVerticalBlank(
        dwFlags: DWORD,
        hEvent: HANDLE,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xb3a6f3e0, 0x2b43, 0x11cf, 0xa2, 0xde, 0x00, 0xaa, 0x00, 0xb9, 0x33, 0x56)]
interface IDirectDraw2(IDirectDraw2Vtbl): IUnknown(IUnknownVtbl) {
    fn Compact() -> HRESULT,
    fn CreateClipper(
        dwFlags: DWORD,
        lplpDDClipper: *mut LPDIRECTDRAWCLIPPER,
        pUnkOuter: *mut IUnknown,
    ) -> HRESULT,
    fn CreatePalette(
        dwFlags: DWORD,
        lpDDColorArray: LPPALETTEENTRY,
        lplpDDPalette: *mut LPDIRECTDRAWPALETTE,
        pUnkOuter: *mut IUnknown,
    ) -> HRESULT,
    fn CreateSurface(
        lpDDSurfaceDesc: LPDDSURFACEDESC,
        lplpDDSurface: *mut LPDIRECTDRAWSURFACE,
        pUnkOuter: *mut IUnknown,
    ) -> HRESULT,
    fn DuplicateSurface(
        lpDDSurface: LPDIRECTDRAWSURFACE,
        lplpDupDDSurface: *mut LPDIRECTDRAWSURFACE,
    ) -> HRESULT,
    fn EnumDisplayModes(
        dwFlags: DWORD,
        lpDDSurfaceDesc: LPDDSURFACEDESC,
        lpContext: LPVOID,
        lpEnumModesCallback: LPDDENUMMODESCALLBACK,
    ) -> HRESULT,
    fn EnumSurfaces(
        dwFlags: DWORD,
        lpDDSD: LPDDSURFACEDESC,
        lpContext: LPVOID,
        lpEnumSurfacesCallback: LPDDENUMSURFACESCALLBACK,
    ) -> HRESULT,
    fn FlipToGDISurface() -> HRESULT,
    fn GetCaps(
        lpDDDriverCaps: LPDDCAPS,
        lpDDHELCaps: LPDDCAPS,
    ) -> HRESULT,
    fn GetDisplayMode(
        lpDDSurfaceDesc: LPDDSURFACEDESC,
    ) -> HRESULT,
    fn GetFourCCCodes(
        lpNumCodes: LPDWORD,
        lpCodes: LPDWORD,
    ) -> HRESULT,
    fn GetGDISurface(
        lplpGDIDDSSurface: *mut LPDIRECTDRAWSURFACE,
    ) -> HRESULT,
    fn GetMonitorFrequency(
        lpdwFrequency: LPDWORD,
    ) -> HRESULT,
    fn GetScanLine(
        lpdwScanLine: LPDWORD,
    ) -> HRESULT,
    fn GetVerticalBlankStatus(
        lpbIsInVB: LPBOOL,
    ) -> HRESULT,
    fn Initialize(
        lpGUID: *mut GUID,
    ) -> HRESULT,
    fn RestoreDisplayMode() -> HRESULT,
    fn SetCooperativeLevel(
        hWnd: HWND,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn SetDisplayMode(
        dwWidth: DWORD,
        dwHeight: DWORD,
        dwBPP: DWORD,
        dwRefreshRate: DWORD,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn WaitForVerticalBlank(
        dwFlags: DWORD,
        hEvent: HANDLE,
    ) -> HRESULT,
    fn GetAvailableVidMem(
        lpDDSCaps: LPDDSCAPS,
        lpdwTotal: LPDWORD,
        lpdwFree: LPDWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x9c59509a, 0x39bd, 0x11d1, 0x8c, 0x4a, 0x00, 0xc0, 0x4f, 0xd9, 0x30, 0xc5)]
interface IDirectDraw4(IDirectDraw4Vtbl): IUnknown(IUnknownVtbl) {
    fn Compact() -> HRESULT,
    fn CreateClipper(
        dwFlags: DWORD,
        lplpDDClipper: *mut LPDIRECTDRAWCLIPPER,
        pUnkOuter: *mut IUnknown,
    ) -> HRESULT,
    fn CreatePalette(
        dwFlags: DWORD,
        lpDDColorArray: LPPALETTEENTRY,
        lplpDDPalette: *mut LPDIRECTDRAWPALETTE,
        pUnkOuter: *mut IUnknown,
    ) -> HRESULT,
    fn CreateSurface(
        lpDDSurfaceDesc2: LPDDSURFACEDESC2,
        lplpDDSurface: *mut LPDIRECTDRAWSURFACE4,
        pUnkOuter: *mut IUnknown,
    ) -> HRESULT,
    fn DuplicateSurface(
        lpDDSurface: LPDIRECTDRAWSURFACE4,
        lplpDupDDSurface: *mut LPDIRECTDRAWSURFACE4,
    ) -> HRESULT,
    fn EnumDisplayModes(
        dwFlags: DWORD,
        lpDDSurfaceDesc2: LPDDSURFACEDESC2,
        lpContext: LPVOID,
        lpEnumModesCallback: LPDDENUMMODESCALLBACK2,
    ) -> HRESULT,
    fn EnumSurfaces(
        dwFlags: DWORD,
        lpDDSD2: LPDDSURFACEDESC2,
        lpContext: LPVOID,
        lpEnumSurfacesCallback: LPDDENUMSURFACESCALLBACK2,
    ) -> HRESULT,
    fn FlipToGDISurface() -> HRESULT,
    fn GetCaps(
        lpDDDriverCaps: LPDDCAPS,
        lpDDHELCaps: LPDDCAPS,
    ) -> HRESULT,
    fn GetDisplayMode(
        lpDDSurfaceDesc2: LPDDSURFACEDESC2,
    ) -> HRESULT,
    fn GetFourCCCodes(
        lpNumCodes: LPDWORD,
        lpCodes: LPDWORD,
    ) -> HRESULT,
    fn GetGDISurface(
        lplpGDIDDSSurface: *mut LPDIRECTDRAWSURFACE4,
    ) -> HRESULT,
    fn GetMonitorFrequency(
        lpdwFrequency: LPDWORD,
    ) -> HRESULT,
    fn GetScanLine(
        lpdwScanLine: LPDWORD,
    ) -> HRESULT,
    fn GetVerticalBlankStatus(
        lpbIsInVB: LPBOOL,
    ) -> HRESULT,
    fn Initialize(
        lpGUID: *mut GUID,
    ) -> HRESULT,
    fn RestoreDisplayMode() -> HRESULT,
    fn SetCooperativeLevel(
        hWnd: HWND,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn SetDisplayMode(
        dwWidth: DWORD,
        dwHeight: DWORD,
        dwBPP: DWORD,
        dwRefreshRate: DWORD,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn WaitForVerticalBlank(
        dwFlags: DWORD,
        hEvent: HANDLE,
    ) -> HRESULT,
    fn GetAvailableVidMem(
        lpDDSCaps2: LPDDSCAPS2,
        lpdwTotal: LPDWORD,
        lpdwFree: LPDWORD,
    ) -> HRESULT,
    fn GetSurfaceFromDC(
        hdc: HDC,
        lpDDS: *mut LPDIRECTDRAWSURFACE4,
    ) -> HRESULT,
    fn RestoreAllSurfaces() -> HRESULT,
    fn TestCooperativeLevel() -> HRESULT,
    fn GetDeviceIdentifier(
        lpdddi: LPDDDEVICEIDENTIFIER,
        dwFlags: DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x15e65ec0, 0x3b9c, 0x11d2, 0xb9, 0x2f, 0x00, 0x60, 0x97, 0x97, 0xea, 0x5b)]
interface IDirectDraw7(IDirectDraw7Vtbl): IUnknown(IUnknownVtbl) {
    fn Compact() -> HRESULT,
    fn CreateClipper(
        dwFlags: DWORD,
        lplpDDClipper: *mut LPDIRECTDRAWCLIPPER,
        pUnkOuter: *mut IUnknown,
    ) -> HRESULT,
    fn CreatePalette(
        dwFlags: DWORD,
        lpDDColorArray: LPPALETTEENTRY,
        lplpDDPalette: *mut LPDIRECTDRAWPALETTE,
        pUnkOuter: *mut IUnknown,
    ) -> HRESULT,
    fn CreateSurface(
        lpDDSurfaceDesc2: LPDDSURFACEDESC2,
        lplpDDSurface: *mut LPDIRECTDRAWSURFACE7,
        pUnkOuter: *mut IUnknown,
    ) -> HRESULT,
    fn DuplicateSurface(
        lpDDSurface: LPDIRECTDRAWSURFACE7,
        lplpDupDDSurface: *mut LPDIRECTDRAWSURFACE7,
    ) -> HRESULT,
    fn EnumDisplayModes(
        dwFlags: DWORD,
        lpDDSurfaceDesc2: LPDDSURFACEDESC2,
        lpContext: LPVOID,
        lpEnumModesCallback: LPDDENUMMODESCALLBACK2,
    ) -> HRESULT,
    fn EnumSurfaces(
        dwFlags: DWORD,
        lpDDSD2: LPDDSURFACEDESC2,
        lpContext: LPVOID,
        lpEnumSurfacesCallback: LPDDENUMSURFACESCALLBACK7,
    ) -> HRESULT,
    fn FlipToGDISurface() -> HRESULT,
    fn GetCaps(
        lpDDDriverCaps: LPDDCAPS,
        lpDDHELCaps: LPDDCAPS,
    ) -> HRESULT,
    fn GetDisplayMode(
        lpDDSurfaceDesc2: LPDDSURFACEDESC2,
    ) -> HRESULT,
    fn GetFourCCCodes(
        lpNumCodes: LPDWORD,
        lpCodes: LPDWORD,
    ) -> HRESULT,
    fn GetGDISurface(
        lplpGDIDDSSurface: *mut LPDIRECTDRAWSURFACE7,
    ) -> HRESULT,
    fn GetMonitorFrequency(
        lpdwFrequency: LPDWORD,
    ) -> HRESULT,
    fn GetScanLine(
        lpdwScanLine: LPDWORD,
    ) -> HRESULT,
    fn GetVerticalBlankStatus(
        lpbIsInVB: LPBOOL,
    ) -> HRESULT,
    fn Initialize(
        lpGUID: *mut GUID,
    ) -> HRESULT,
    fn RestoreDisplayMode() -> HRESULT,
    fn SetCooperativeLevel(
        hWnd: HWND,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn SetDisplayMode(
        dwWidth: DWORD,
        dwHeight: DWORD,
        dwBPP: DWORD,
        dwRefreshRate: DWORD,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn WaitForVerticalBlank(
        dwFlags: DWORD,
        hEvent: HANDLE,
    ) -> HRESULT,
    fn GetAvailableVidMem(
        lpDDSCaps2: LPDDSCAPS2,
        lpdwTotal: LPDWORD,
        lpdwFree: LPDWORD,
    ) -> HRESULT,
    fn GetSurfaceFromDC(
        hdc: HDC,
        lpDDS: *mut LPDIRECTDRAWSURFACE7,
    ) -> HRESULT,
    fn RestoreAllSurfaces() -> HRESULT,
    fn TestCooperativeLevel() -> HRESULT,
    fn GetDeviceIdentifier(
        lpdddi: LPDDDEVICEIDENTIFIER2,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn StartModeTest(
        lpModesToTest: LPSIZE,
        dwNumEntries: DWORD,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn EvaluateMode(
        dwFlags: DWORD,
        pSecondsUntilTimeout: *mut DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x6c14db84, 0xa733, 0x11ce, 0xa5, 0x21, 0x00, 0x20, 0xaf, 0x0b, 0xe5, 0x60)]
interface IDirectDrawPalette(IDirectDrawPaletteVtbl): IUnknown(IUnknownVtbl) {
    fn GetCaps(
        lpdwCaps: LPDWORD,
    ) -> HRESULT,
    fn GetEntries(
        dwFlags: DWORD,
        dwBase: DWORD,
        dwNumEntries: DWORD,
        lpEntries: LPPALETTEENTRY,
    ) -> HRESULT,
    fn Initialize(
        lpDD: LPDIRECTDRAW,
        dwFlags: DWORD,
        lpDDColorTable: LPPALETTEENTRY,
    ) -> HRESULT,
    fn SetEntries(
        dwFlags: DWORD,
        dwStartingEntry: DWORD,
        dwCount: DWORD,
        lpEntries: LPPALETTEENTRY,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x6c14db85, 0xa733, 0x11ce, 0xa5, 0x21, 0x00, 0x20, 0xaf, 0x0b, 0xe5, 0x60)]
interface IDirectDrawClipper(IDirectDrawClipperVtbl): IUnknown(IUnknownVtbl) {
    fn GetClipList(
        lpRect: LPRECT,
        lpClipList: LPRGNDATA,
        lpdwSize: LPDWORD,
    ) -> HRESULT,
    fn GetHWnd(
        lphWnd: *mut HWND,
    ) -> HRESULT,
    fn Initialize(
        lpDD: LPDIRECTDRAW,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn IsClipListChanged(
        lpbChanged: *mut BOOL,
    ) -> HRESULT,
    fn SetClipList(
        lpClipList: LPRGNDATA,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn SetHWnd(
        dwFlags: DWORD,
        hWnd: HWND,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x6c14db81, 0xa733, 0x11ce, 0xa5, 0x21, 0x00, 0x20, 0xaf, 0x0b, 0xe5, 0x60)]
interface IDirectDrawSurface(IDirectDrawSurfaceVtbl): IUnknown(IUnknownVtbl) {
    fn AddAttachedSurface(
        lpDDSurface: LPDIRECTDRAWSURFACE,
    ) -> HRESULT,
    fn AddOverlayDirtyRect(
        lpRect: LPRECT,
    ) -> HRESULT,
    fn Blt(
        lpDestRect: LPRECT,
        lpDDSrcSurface: LPDIRECTDRAWSURFACE,
        lpSrcRect: LPRECT,
        dwFlags: DWORD,
        lpDDBltFx: LPDDBLTFX,
    ) -> HRESULT,
    fn BltBatch(
        lpDDBltBatch: LPDDBLTBATCH,
        dwCount: DWORD,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn BltFast(
        dwX: DWORD,
        dwY: DWORD,
        lpDDSrcSurface: LPDIRECTDRAWSURFACE,
        lpSrcRect: LPRECT,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn DeleteAttachedSurface(
        dwFlags: DWORD,
        lpDDSAttachedSurface: LPDIRECTDRAWSURFACE,
    ) -> HRESULT,
    fn EnumAttachedSurfaces(
        lpContext: LPVOID,
        lpEnumSurfacesCallback: LPDDENUMSURFACESCALLBACK,
    ) -> HRESULT,
    fn EnumOverlayZOrders(
        dwFlags: DWORD,
        lpContext: LPVOID,
        lpfnCallback: LPDDENUMSURFACESCALLBACK,
    ) -> HRESULT,
    fn Flip(
        lpDDSurfaceTargetOverride: LPDIRECTDRAWSURFACE,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetAttachedSurface(
        lpDDSCaps: LPDDSCAPS,
        lplpDDAttachedSurface: *mut LPDIRECTDRAWSURFACE,
    ) -> HRESULT,
    fn GetBltStatus(
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetCaps(
        lpDDSCaps: LPDDSCAPS,
    ) -> HRESULT,
    fn GetClipper(
        lplpDDClipper: *mut LPDIRECTDRAWCLIPPER,
    ) -> HRESULT,
    fn GetColorKey(
        dwFlags: DWORD,
        lpDDColorKey: LPDDCOLORKEY,
    ) -> HRESULT,
    fn GetDC(
        lphDC: *mut HDC,
    ) -> HRESULT,
    fn GetFlipStatus(
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetOverlayPosition(
        lplX: LPLONG,
        lplY: LPLONG,
    ) -> HRESULT,
    fn GetPalette(
        lplpDDPalette: *mut LPDIRECTDRAWPALETTE,
    ) -> HRESULT,
    fn GetPixelFormat(
        lpDDPixelFormat: LPDDPIXELFORMAT,
    ) -> HRESULT,
    fn GetSurfaceDesc(
        lpDDSurfaceDesc: LPDDSURFACEDESC,
    ) -> HRESULT,
    fn Initialize(
        lpDD: LPDIRECTDRAW,
        lpDDSurfaceDesc: LPDDSURFACEDESC,
    ) -> HRESULT,
    fn IsLost() -> HRESULT,
    fn Lock(
        lpDestRect: LPRECT,
        lpDDSurfaceDesc: LPDDSURFACEDESC,
        dwFlags: DWORD,
        hEvent: HANDLE,
    ) -> HRESULT,
    fn ReleaseDC(
        hDC: HDC,
    ) -> HRESULT,
    fn Restore() -> HRESULT,
    fn SetClipper(
        lpDDClipper: LPDIRECTDRAWCLIPPER,
    ) -> HRESULT,
    fn SetColorKey(
        dwFlags: DWORD,
        lpDDColorKey: LPDDCOLORKEY,
    ) -> HRESULT,
    fn SetOverlayPosition(
        lX: LONG,
        lY: LONG,
    ) -> HRESULT,
    fn SetPalette(
        lpDDPalette: LPDIRECTDRAWPALETTE,
    ) -> HRESULT,
    fn Unlock(
        lpRect: LPVOID,
    ) -> HRESULT,
    fn UpdateOverlay(
        lpSrcRect: LPRECT,
        lpDDDestSurface: LPDIRECTDRAWSURFACE,
        lpDestRect: LPRECT,
        dwFlags: DWORD,
        lpDDOverlayFx: LPDDOVERLAYFX,
    ) -> HRESULT,
    fn UpdateOverlayDisplay(
        dwFlags: DWORD,
    ) -> HRESULT,
    fn UpdateOverlayZOrder(
        dwFlags: DWORD,
        lpDDSReference: LPDIRECTDRAWSURFACE,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x57805885, 0x6eec, 0x11cf, 0x94, 0x41, 0xa8, 0x23, 0x03, 0xc1, 0x0e, 0x27)]
interface IDirectDrawSurface2(IDirectDrawSurface2Vtbl): IUnknown(IUnknownVtbl) {
    fn AddAttachedSurface(
        lpDDSurface: LPDIRECTDRAWSURFACE2,
    ) -> HRESULT,
    fn AddOverlayDirtyRect(
        lpRect: LPRECT,
    ) -> HRESULT,
    fn Blt(
        lpDestRect: LPRECT,
        lpDDSrcSurface: LPDIRECTDRAWSURFACE2,
        lpSrcRect: LPRECT,
        dwFlags: DWORD,
        lpDDBltFx: LPDDBLTFX,
    ) -> HRESULT,
    fn BltBatch(
        lpDDBltBatch: LPDDBLTBATCH,
        dwCount: DWORD,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn BltFast(
        dwX: DWORD,
        dwY: DWORD,
        lpDDSrcSurface: LPDIRECTDRAWSURFACE2,
        lpSrcRect: LPRECT,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn DeleteAttachedSurface(
        dwFlags: DWORD,
        lpDDSAttachedSurface: LPDIRECTDRAWSURFACE2,
    ) -> HRESULT,
    fn EnumAttachedSurfaces(
        lpContext: LPVOID,
        lpEnumSurfacesCallback: LPDDENUMSURFACESCALLBACK,
    ) -> HRESULT,
    fn EnumOverlayZOrders(
        dwFlags: DWORD,
        lpContext: LPVOID,
        lpfnCallback: LPDDENUMSURFACESCALLBACK,
    ) -> HRESULT,
    fn Flip(
        lpDDSurfaceTargetOverride: LPDIRECTDRAWSURFACE2,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetAttachedSurface(
        lpDDSCaps: LPDDSCAPS,
        lplpDDAttachedSurface: *mut LPDIRECTDRAWSURFACE2,
    ) -> HRESULT,
    fn GetBltStatus(
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetCaps(
        lpDDSCaps: LPDDSCAPS,
    ) -> HRESULT,
    fn GetClipper(
        lplpDDClipper: *mut LPDIRECTDRAWCLIPPER,
    ) -> HRESULT,
    fn GetColorKey(
        dwFlags: DWORD,
        lpDDColorKey: LPDDCOLORKEY,
    ) -> HRESULT,
    fn GetDC(
        lphDC: *mut HDC,
    ) -> HRESULT,
    fn GetFlipStatus(
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetOverlayPosition(
        lplX: LPLONG,
        lplY: LPLONG,
    ) -> HRESULT,
    fn GetPalette(
        lplpDDPalette: *mut LPDIRECTDRAWPALETTE,
    ) -> HRESULT,
    fn GetPixelFormat(
        lpDDPixelFormat: LPDDPIXELFORMAT,
    ) -> HRESULT,
    fn GetSurfaceDesc(
        lpDDSurfaceDesc: LPDDSURFACEDESC,
    ) -> HRESULT,
    fn Initialize(
        lpDD: LPDIRECTDRAW,
        lpDDSurfaceDesc: LPDDSURFACEDESC,
    ) -> HRESULT,
    fn IsLost() -> HRESULT,
    fn Lock(
        lpDestRect: LPRECT,
        lpDDSurfaceDesc: LPDDSURFACEDESC,
        dwFlags: DWORD,
        hEvent: HANDLE,
    ) -> HRESULT,
    fn ReleaseDC(
        hDC: HDC,
    ) -> HRESULT,
    fn Restore() -> HRESULT,
    fn SetClipper(
        lpDDClipper: LPDIRECTDRAWCLIPPER,
    ) -> HRESULT,
    fn SetColorKey(
        dwFlags: DWORD,
        lpDDColorKey: LPDDCOLORKEY,
    ) -> HRESULT,
    fn SetOverlayPosition(
        lX: LONG,
        lY: LONG,
    ) -> HRESULT,
    fn SetPalette(
        lpDDPalette: LPDIRECTDRAWPALETTE,
    ) -> HRESULT,
    fn Unlock(
        lpRect: LPVOID,
    ) -> HRESULT,
    fn UpdateOverlay(
        lpSrcRect: LPRECT,
        lpDDDestSurface: LPDIRECTDRAWSURFACE2,
        lpDestRect: LPRECT,
        dwFlags: DWORD,
        lpDDOverlayFx: LPDDOVERLAYFX,
    ) -> HRESULT,
    fn UpdateOverlayDisplay(
        dwFlags: DWORD,
    ) -> HRESULT,
    fn UpdateOverlayZOrder(
        dwFlags: DWORD,
        lpDDSReference: LPDIRECTDRAWSURFACE2,
    ) -> HRESULT,
    fn GetDDInterface(
        lplpDD: *mut LPVOID,
    ) -> HRESULT,
    fn PageLock(
        dwFlags: DWORD,
    ) -> HRESULT,
    fn PageUnlock(
        dwFlags: DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xda044e00, 0x69b2, 0x11d0, 0xa1, 0xd5, 0x00, 0xaa, 0x00, 0xb8, 0xdf, 0xbb)]
interface IDirectDrawSurface3(IDirectDrawSurface3Vtbl): IUnknown(IUnknownVtbl) {
    fn AddAttachedSurface(
        lpDDSurface: LPDIRECTDRAWSURFACE3,
    ) -> HRESULT,
    fn AddOverlayDirtyRect(
        lpRect: LPRECT,
    ) -> HRESULT,
    fn Blt(
        lpDestRect: LPRECT,
        lpDDSrcSurface: LPDIRECTDRAWSURFACE3,
        lpSrcRect: LPRECT,
        dwFlags: DWORD,
        lpDDBltFx: LPDDBLTFX,
    ) -> HRESULT,
    fn BltBatch(
        lpDDBltBatch: LPDDBLTBATCH,
        dwCount: DWORD,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn BltFast(
        dwX: DWORD,
        dwY: DWORD,
        lpDDSrcSurface: LPDIRECTDRAWSURFACE3,
        lpSrcRect: LPRECT,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn DeleteAttachedSurface(
        dwFlags: DWORD,
        lpDDSAttachedSurface: LPDIRECTDRAWSURFACE3,
    ) -> HRESULT,
    fn EnumAttachedSurfaces(
        lpContext: LPVOID,
        lpEnumSurfacesCallback: LPDDENUMSURFACESCALLBACK,
    ) -> HRESULT,
    fn EnumOverlayZOrders(
        dwFlags: DWORD,
        lpContext: LPVOID,
        lpfnCallback: LPDDENUMSURFACESCALLBACK,
    ) -> HRESULT,
    fn Flip(
        lpDDSurfaceTargetOverride: LPDIRECTDRAWSURFACE3,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetAttachedSurface(
        lpDDSCaps: LPDDSCAPS,
        lplpDDAttachedSurface: *mut LPDIRECTDRAWSURFACE3,
    ) -> HRESULT,
    fn GetBltStatus(
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetCaps(
        lpDDSCaps: LPDDSCAPS,
    ) -> HRESULT,
    fn GetClipper(
        lplpDDClipper: *mut LPDIRECTDRAWCLIPPER,
    ) -> HRESULT,
    fn GetColorKey(
        dwFlags: DWORD,
        lpDDColorKey: LPDDCOLORKEY,
    ) -> HRESULT,
    fn GetDC(
        lphDC: *mut HDC,
    ) -> HRESULT,
    fn GetFlipStatus(
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetOverlayPosition(
        lplX: LPLONG,
        lplY: LPLONG,
    ) -> HRESULT,
    fn GetPalette(
        lplpDDPalette: *mut LPDIRECTDRAWPALETTE,
    ) -> HRESULT,
    fn GetPixelFormat(
        lpDDPixelFormat: LPDDPIXELFORMAT,
    ) -> HRESULT,
    fn GetSurfaceDesc(
        lpDDSurfaceDesc: LPDDSURFACEDESC,
    ) -> HRESULT,
    fn Initialize(
        lpDD: LPDIRECTDRAW,
        lpDDSurfaceDesc: LPDDSURFACEDESC,
    ) -> HRESULT,
    fn IsLost() -> HRESULT,
    fn Lock(
        lpDestRect: LPRECT,
        lpDDSurfaceDesc: LPDDSURFACEDESC,
        dwFlags: DWORD,
        hEvent: HANDLE,
    ) -> HRESULT,
    fn ReleaseDC(
        hDC: HDC,
    ) -> HRESULT,
    fn Restore() -> HRESULT,
    fn SetClipper(
        lpDDClipper: LPDIRECTDRAWCLIPPER,
    ) -> HRESULT,
    fn SetColorKey(
        dwFlags: DWORD,
        lpDDColorKey: LPDDCOLORKEY,
    ) -> HRESULT,
    fn SetOverlayPosition(
        lX: LONG,
        lY: LONG,
    ) -> HRESULT,
    fn SetPalette(
        lpDDPalette: LPDIRECTDRAWPALETTE,
    ) -> HRESULT,
    fn Unlock(
        lpRect: LPVOID,
    ) -> HRESULT,
    fn UpdateOverlay(
        lpSrcRect: LPRECT,
        lpDDDestSurface: LPDIRECTDRAWSURFACE3,
        lpDestRect: LPRECT,
        dwFlags: DWORD,
        lpDDOverlayFx: LPDDOVERLAYFX,
    ) -> HRESULT,
    fn UpdateOverlayDisplay(
        dwFlags: DWORD,
    ) -> HRESULT,
    fn UpdateOverlayZOrder(
        dwFlags: DWORD,
        lpDDSReference: LPDIRECTDRAWSURFACE3,
    ) -> HRESULT,
    fn GetDDInterface(
        lplpDD: *mut LPVOID,
    ) -> HRESULT,
    fn PageLock(
        dwFlags: DWORD,
    ) -> HRESULT,
    fn PageUnlock(
        dwFlags: DWORD,
    ) -> HRESULT,
    fn SetSurfaceDesc(
        lpDDsd2: LPDDSURFACEDESC,
        dwFlags: DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x0b2b8630, 0xad35, 0x11d0, 0x8e, 0xa6, 0x00, 0x60, 0x97, 0x97, 0xea, 0x5b)]
interface IDirectDrawSurface4(IDirectDrawSurface4Vtbl): IUnknown(IUnknownVtbl) {
    fn AddAttachedSurface(
        lpDDSurface: LPDIRECTDRAWSURFACE4,
    ) -> HRESULT,
    fn AddOverlayDirtyRect(
        lpRect: LPRECT,
    ) -> HRESULT,
    fn Blt(
        lpDestRect: LPRECT,
        lpDDSrcSurface: LPDIRECTDRAWSURFACE4,
        lpSrcRect: LPRECT,
        dwFlags: DWORD,
        lpDDBltFx: LPDDBLTFX,
    ) -> HRESULT,
    fn BltBatch(
        lpDDBltBatch: LPDDBLTBATCH,
        dwCount: DWORD,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn BltFast(
        dwX: DWORD,
        dwY: DWORD,
        lpDDSrcSurface: LPDIRECTDRAWSURFACE4,
        lpSrcRect: LPRECT,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn DeleteAttachedSurface(
        dwFlags: DWORD,
        lpDDSAttachedSurface: LPDIRECTDRAWSURFACE4,
    ) -> HRESULT,
    fn EnumAttachedSurfaces(
        lpContext: LPVOID,
        lpEnumSurfacesCallback: LPDDENUMSURFACESCALLBACK2,
    ) -> HRESULT,
    fn EnumOverlayZOrders(
        dwFlags: DWORD,
        lpContext: LPVOID,
        lpfnCallback: LPDDENUMSURFACESCALLBACK2,
    ) -> HRESULT,
    fn Flip(
        lpDDSurfaceTargetOverride: LPDIRECTDRAWSURFACE4,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetAttachedSurface(
        lpDDSCaps: LPDDSCAPS2,
        lplpDDAttachedSurface: *mut LPDIRECTDRAWSURFACE4,
    ) -> HRESULT,
    fn GetBltStatus(
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetCaps(
        lpDDSCaps: LPDDSCAPS2,
    ) -> HRESULT,
    fn GetClipper(
        lplpDDClipper: *mut LPDIRECTDRAWCLIPPER,
    ) -> HRESULT,
    fn GetColorKey(
        dwFlags: DWORD,
        lpDDColorKey: LPDDCOLORKEY,
    ) -> HRESULT,
    fn GetDC(
        lphDC: *mut HDC,
    ) -> HRESULT,
    fn GetFlipStatus(
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetOverlayPosition(
        lplX: LPLONG,
        lplY: LPLONG,
    ) -> HRESULT,
    fn GetPalette(
        lplpDDPalette: *mut LPDIRECTDRAWPALETTE,
    ) -> HRESULT,
    fn GetPixelFormat(
        lpDDPixelFormat: LPDDPIXELFORMAT,
    ) -> HRESULT,
    fn GetSurfaceDesc(
        lpDDSurfaceDesc: LPDDSURFACEDESC2,
    ) -> HRESULT,
    fn Initialize(
        lpDD: LPDIRECTDRAW,
        lpDDSurfaceDesc: LPDDSURFACEDESC2,
    ) -> HRESULT,
    fn IsLost() -> HRESULT,
    fn Lock(
        lpDestRect: LPRECT,
        lpDDSurfaceDesc: LPDDSURFACEDESC2,
        dwFlags: DWORD,
        hEvent: HANDLE,
    ) -> HRESULT,
    fn ReleaseDC(
        hDC: HDC,
    ) -> HRESULT,
    fn Restore() -> HRESULT,
    fn SetClipper(
        lpDDClipper: LPDIRECTDRAWCLIPPER,
    ) -> HRESULT,
    fn SetColorKey(
        dwFlags: DWORD,
        lpDDColorKey: LPDDCOLORKEY,
    ) -> HRESULT,
    fn SetOverlayPosition(
        lX: LONG,
        lY: LONG,
    ) -> HRESULT,
    fn SetPalette(
        lpDDPalette: LPDIRECTDRAWPALETTE,
    ) -> HRESULT,
    fn Unlock(
        lpRect: LPVOID,
    ) -> HRESULT,
    fn UpdateOverlay(
        lpSrcRect: LPRECT,
        lpDDDestSurface: LPDIRECTDRAWSURFACE4,
        lpDestRect: LPRECT,
        dwFlags: DWORD,
        lpDDOverlayFx: LPDDOVERLAYFX,
    ) -> HRESULT,
    fn UpdateOverlayDisplay(
        dwFlags: DWORD,
    ) -> HRESULT,
    fn UpdateOverlayZOrder(
        dwFlags: DWORD,
        lpDDSReference: LPDIRECTDRAWSURFACE4,
    ) -> HRESULT,
    fn GetDDInterface(
        lplpDD: *mut LPVOID,
    ) -> HRESULT,
    fn PageLock(
        dwFlags: DWORD,
    ) -> HRESULT,
    fn PageUnlock(
        dwFlags: DWORD,
    ) -> HRESULT,
    fn SetSurfaceDesc(
        lpDDsd2: LPDDSURFACEDESC2,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn SetPrivateData(
        guidTag: REFGUID,
        lpData: LPVOID,
        cbSize: DWORD,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetPrivateData(
        guidTag: REFGUID,
        lpBuffer: LPVOID,
        lpcbBufferSize: LPDWORD,
    ) -> HRESULT,
    fn FreePrivateData(
        guidTag: REFGUID,
    ) -> HRESULT,
    fn GetUniquenessValue(
        lpValue: LPDWORD,
    ) -> HRESULT,
    fn ChangeUniquenessValue() -> HRESULT,
}}
RIDL!{#[uuid(0x06675a80, 0x3b9b, 0x11d2, 0xb9, 0x2f, 0x00, 0x60, 0x97, 0x97, 0xea, 0x5b)]
interface IDirectDrawSurface7(IDirectDrawSurface7Vtbl): IUnknown(IUnknownVtbl) {
    fn AddAttachedSurface(
        lpDDSurface: LPDIRECTDRAWSURFACE7,
    ) -> HRESULT,
    fn AddOverlayDirtyRect(
        lpRect: LPRECT,
    ) -> HRESULT,
    fn Blt(
        lpDestRect: LPRECT,
        lpDDSrcSurface: LPDIRECTDRAWSURFACE7,
        lpSrcRect: LPRECT,
        dwFlags: DWORD,
        lpDDBltFx: LPDDBLTFX,
    ) -> HRESULT,
    fn BltBatch(
        lpDDBltBatch: LPDDBLTBATCH,
        dwCount: DWORD,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn BltFast(
        dwX: DWORD,
        dwY: DWORD,
        lpDDSrcSurface: LPDIRECTDRAWSURFACE7,
        lpSrcRect: LPRECT,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn DeleteAttachedSurface(
        dwFlags: DWORD,
        lpDDSAttachedSurface: LPDIRECTDRAWSURFACE7,
    ) -> HRESULT,
    fn EnumAttachedSurfaces(
        lpContext: LPVOID,
        lpEnumSurfacesCallback: LPDDENUMSURFACESCALLBACK7,
    ) -> HRESULT,
    fn EnumOverlayZOrders(
        dwFlags: DWORD,
        lpContext: LPVOID,
        lpfnCallback: LPDDENUMSURFACESCALLBACK7,
    ) -> HRESULT,
    fn Flip(
        lpDDSurfaceTargetOverride: LPDIRECTDRAWSURFACE7,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetAttachedSurface(
        lpDDSCaps: LPDDSCAPS2,
        lplpDDAttachedSurface: *mut LPDIRECTDRAWSURFACE7,
    ) -> HRESULT,
    fn GetBltStatus(
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetCaps(
        lpDDSCaps: LPDDSCAPS2,
    ) -> HRESULT,
    fn GetClipper(
        lplpDDClipper: *mut LPDIRECTDRAWCLIPPER,
    ) -> HRESULT,
    fn GetColorKey(
        dwFlags: DWORD,
        lpDDColorKey: LPDDCOLORKEY,
    ) -> HRESULT,
    fn GetDC(
        lphDC: *mut HDC,
    ) -> HRESULT,
    fn GetFlipStatus(
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetOverlayPosition(
        lplX: LPLONG,
        lplY: LPLONG,
    ) -> HRESULT,
    fn GetPalette(
        lplpDDPalette: *mut LPDIRECTDRAWPALETTE,
    ) -> HRESULT,
    fn GetPixelFormat(
        lpDDPixelFormat: LPDDPIXELFORMAT,
    ) -> HRESULT,
    fn GetSurfaceDesc(
        lpDDSurfaceDesc: LPDDSURFACEDESC2,
    ) -> HRESULT,
    fn Initialize(
        lpDD: LPDIRECTDRAW,
        lpDDSurfaceDesc: LPDDSURFACEDESC2,
    ) -> HRESULT,
    fn IsLost() -> HRESULT,
    fn Lock(
        lpDestRect: LPRECT,
        lpDDSurfaceDesc: LPDDSURFACEDESC2,
        dwFlags: DWORD,
        hEvent: HANDLE,
    ) -> HRESULT,
    fn ReleaseDC(
        hDC: HDC,
    ) -> HRESULT,
    fn Restore() -> HRESULT,
    fn SetClipper(
        lpDDClipper: LPDIRECTDRAWCLIPPER,
    ) -> HRESULT,
    fn SetColorKey(
        dwFlags: DWORD,
        lpDDColorKey: LPDDCOLORKEY,
    ) -> HRESULT,
    fn SetOverlayPosition(
        lX: LONG,
        lY: LONG,
    ) -> HRESULT,
    fn SetPalette(
        lpDDPalette: LPDIRECTDRAWPALETTE,
    ) -> HRESULT,
    fn Unlock(
        lpRect: LPVOID,
    ) -> HRESULT,
    fn UpdateOverlay(
        lpSrcRect: LPRECT,
        lpDDDestSurface: LPDIRECTDRAWSURFACE7,
        lpDestRect: LPRECT,
        dwFlags: DWORD,
        lpDDOverlayFx: LPDDOVERLAYFX,
    ) -> HRESULT,
    fn UpdateOverlayDisplay(
        dwFlags: DWORD,
    ) -> HRESULT,
    fn UpdateOverlayZOrder(
        dwFlags: DWORD,
        lpDDSReference: LPDIRECTDRAWSURFACE7,
    ) -> HRESULT,
    fn GetDDInterface(
        lplpDD: *mut LPVOID,
    ) -> HRESULT,
    fn PageLock(
        dwFlags: DWORD,
    ) -> HRESULT,
    fn PageUnlock(
        dwFlags: DWORD,
    ) -> HRESULT,
    fn SetSurfaceDesc(
        lpDDsd2: LPDDSURFACEDESC2,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn SetPrivateData(
        guidTag: REFGUID,
        lpData: LPVOID,
        cbSize: DWORD,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetPrivateData(
        guidTag: REFGUID,
        lpBuffer: LPVOID,
        lpcbBufferSize: LPDWORD,
    ) -> HRESULT,
    fn FreePrivateData(
        guidTag: REFGUID,
    ) -> HRESULT,
    fn GetUniquenessValue(
        lpValue: LPDWORD,
    ) -> HRESULT,
    fn ChangeUniquenessValue() -> HRESULT,
    fn SetPriority(
        dwPriority: DWORD,
    ) -> HRESULT,
    fn GetPriority(
        lpdwPriority: LPDWORD,
    ) -> HRESULT,
    fn SetLOD(
        dwMaxLOD: DWORD,
    ) -> HRESULT,
    fn GetLOD(
        lpdwMaxLOD: LPDWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x4b9f0ee0, 0x0d7e, 0x11d0, 0x9b, 0x06, 0x00, 0xa0, 0xc9, 0x03, 0xa3, 0xb8)]
interface IDirectDrawColorControl(IDirectDrawColorControlVtbl): IUnknown(IUnknownVtbl) {
    fn GetColorControls(
        lpColorControl: LPDDCOLORCONTROL,
    ) -> HRESULT,
    fn SetColorControls(
        lpColorControl: LPDDCOLORCONTROL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x69c11c3e, 0xb46b, 0x11d1, 0xad, 0x7a, 0x00, 0xc0, 0x4f, 0xc2, 0x9b, 0x4e)]
interface IDirectDrawGammaControl(IDirectDrawGammaControlVtbl): IUnknown(IUnknownVtbl) {
    fn GetGammaRamp(
        dwFlags: DWORD,
        lpRampData: LPDDGAMMARAMP,
    ) -> HRESULT,
    fn SetGammaRamp(
        dwFlags: DWORD,
        lpRampData: LPDDGAMMARAMP,
    ) -> HRESULT,
}}
UNION!{union DDSURFACEDESC_u1 {
    [u32; 1],
    lPitch lPitch_mut: LONG,
    dwLinearSize dwLinearSize_mut: DWORD,
}}
UNION!{union DDSURFACEDESC_u2 {
    [u32; 1],
    dwMipMapCount dwMipMapCount_mut: DWORD,
    dwZBufferBitDepth dwZBufferBitDepth_mut: DWORD,
    dwRefreshRate dwRefreshRate_mut: DWORD,
}}
STRUCT!{struct DDSURFACEDESC {
    dwSize: DWORD,
    dwFlags: DWORD,
    dwHeight: DWORD,
    dwWidth: DWORD,
    u1: DDSURFACEDESC_u1,
    dwBackBufferCount: DWORD,
    u2: DDSURFACEDESC_u2,
    dwAlphaBitDepth: DWORD,
    dwReserved: DWORD,
    lpSurface: LPVOID,
    ddckCKDestOverlay: DDCOLORKEY,
    ddckCKDestBlt: DDCOLORKEY,
    ddckCKSrcOverlay: DDCOLORKEY,
    ddckCKSrcBlt: DDCOLORKEY,
    ddpfPixelFormat: DDPIXELFORMAT,
    ddsCaps: DDSCAPS,
}}
UNION!{union DDSURFACEDESC2_u1 {
    [u32; 1],
    lPitch lPitch_mut: LONG,
    dwLinearSize dwLinearSize_mut: DWORD,
}}
UNION!{union DDSURFACEDESC2_u5 {
    [u32; 1],
    dwBackBufferCount dwBackBufferCount_mut: DWORD,
    dwDepth dwDepth_mut: DWORD,
}}
UNION!{union DDSURFACEDESC2_u2 {
    [u32; 1],
    dwMipMapCount dwMipMapCount_mut: DWORD,
    dwRefreshRate dwRefreshRate_mut: DWORD,
    dwSrcVBHandle dwSrcVBHandle_mut: DWORD,
}}
UNION!{union DDSURFACEDESC2_u3 {
    [u32; 2],
    ddckCKDestOverlay ddckCKDestOverlay_mut: DDCOLORKEY,
    dwEmptyFaceColor dwEmptyFaceColor_mut: DWORD,
}}
UNION!{union DDSURFACEDESC2_u4 {
    [u32; 8],
    ddpfPixelFormat ddpfPixelFormat_mut: DDPIXELFORMAT,
    dwFVF dwFVF_mut: DWORD,
}}
STRUCT!{struct DDSURFACEDESC2 {
    dwSize: DWORD,
    dwFlags: DWORD,
    dwHeight: DWORD,
    dwWidth: DWORD,
    u1: DDSURFACEDESC2_u1,
    u5: DDSURFACEDESC2_u5,
    u2: DDSURFACEDESC2_u2,
    dwAlphaBitDepth: DWORD,
    dwReserved: DWORD,
    lpSurface: LPVOID,
    u3: DDSURFACEDESC2_u3,
    ddckCKDestBlt: DDCOLORKEY,
    ddckCKSrcOverlay: DDCOLORKEY,
    ddckCKSrcBlt: DDCOLORKEY,
    u4: DDSURFACEDESC2_u4,
    ddsCaps: DDSCAPS2,
    dwTextureStage: DWORD,
}}
pub const DDSD_CAPS: DWORD = 0x00000001;
pub const DDSD_HEIGHT: DWORD = 0x00000002;
pub const DDSD_WIDTH: DWORD = 0x00000004;
pub const DDSD_PITCH: DWORD = 0x00000008;
pub const DDSD_BACKBUFFERCOUNT: DWORD = 0x00000020;
pub const DDSD_ZBUFFERBITDEPTH: DWORD = 0x00000040;
pub const DDSD_ALPHABITDEPTH: DWORD = 0x00000080;
pub const DDSD_LPSURFACE: DWORD = 0x00000800;
pub const DDSD_PIXELFORMAT: DWORD = 0x00001000;
pub const DDSD_CKDESTOVERLAY: DWORD = 0x00002000;
pub const DDSD_CKDESTBLT: DWORD = 0x00004000;
pub const DDSD_CKSRCOVERLAY: DWORD = 0x00008000;
pub const DDSD_CKSRCBLT: DWORD = 0x00010000;
pub const DDSD_MIPMAPCOUNT: DWORD = 0x00020000;
pub const DDSD_REFRESHRATE: DWORD = 0x00040000;
pub const DDSD_LINEARSIZE: DWORD = 0x00080000;
pub const DDSD_TEXTURESTAGE: DWORD = 0x00100000;
pub const DDSD_FVF: DWORD = 0x00200000;
pub const DDSD_SRCVBHANDLE: DWORD = 0x00400000;
pub const DDSD_DEPTH: DWORD = 0x00800000;
pub const DDSD_ALL: DWORD = 0x00fff9ee;
STRUCT!{struct DDOPTSURFACEDESC {
    dwSize: DWORD,
    dwFlags: DWORD,
    ddSCaps: DDSCAPS2,
    ddOSCaps: DDOSCAPS,
    guid: GUID,
    dwCompressionRatio: DWORD,
}}
pub const DDOSD_GUID: DWORD = 0x00000001;
pub const DDOSD_COMPRESSION_RATIO: DWORD = 0x00000002;
pub const DDOSD_SCAPS: DWORD = 0x00000004;
pub const DDOSD_OSCAPS: DWORD = 0x00000008;
pub const DDOSD_ALL: DWORD = 0x0000000f;
pub const DDOSDCAPS_OPTCOMPRESSED: DWORD = 0x00000001;
pub const DDOSDCAPS_OPTREORDERED: DWORD = 0x00000002;
pub const DDOSDCAPS_MONOLITHICMIPMAP: DWORD = 0x00000004;
pub const DDOSDCAPS_VALIDSCAPS: DWORD = 0x30004800;
pub const DDOSDCAPS_VALIDOSCAPS: DWORD = 0x00000007;
STRUCT!{struct DDCOLORCONTROL {
    dwSize: DWORD,
    dwFlags: DWORD,
    lBrightness: LONG,
    lContrast: LONG,
    lHue: LONG,
    lSaturation: LONG,
    lSharpness: LONG,
    lGamma: LONG,
    lColorEnable: LONG,
    dwReserved1: DWORD,
}}
pub const DDCOLOR_BRIGHTNESS: DWORD = 0x00000001;
pub const DDCOLOR_CONTRAST: DWORD = 0x00000002;
pub const DDCOLOR_HUE: DWORD = 0x00000004;
pub const DDCOLOR_SATURATION: DWORD = 0x00000008;
pub const DDCOLOR_SHARPNESS: DWORD = 0x00000010;
pub const DDCOLOR_GAMMA: DWORD = 0x00000020;
pub const DDCOLOR_COLORENABLE: DWORD = 0x00000040;
pub const DDSCAPS_RESERVED1: DWORD = 0x00000001;
pub const DDSCAPS_ALPHA: DWORD = 0x00000002;
pub const DDSCAPS_BACKBUFFER: DWORD = 0x00000004;
pub const DDSCAPS_COMPLEX: DWORD = 0x00000008;
pub const DDSCAPS_FLIP: DWORD = 0x00000010;
pub const DDSCAPS_FRONTBUFFER: DWORD = 0x00000020;
pub const DDSCAPS_OFFSCREENPLAIN: DWORD = 0x00000040;
pub const DDSCAPS_OVERLAY: DWORD = 0x00000080;
pub const DDSCAPS_PALETTE: DWORD = 0x00000100;
pub const DDSCAPS_PRIMARYSURFACE: DWORD = 0x00000200;
pub const DDSCAPS_RESERVED3: DWORD = 0x00000400;
pub const DDSCAPS_PRIMARYSURFACELEFT: DWORD = 0x00000000;
pub const DDSCAPS_SYSTEMMEMORY: DWORD = 0x00000800;
pub const DDSCAPS_TEXTURE: DWORD = 0x00001000;
pub const DDSCAPS_3DDEVICE: DWORD = 0x00002000;
pub const DDSCAPS_VIDEOMEMORY: DWORD = 0x00004000;
pub const DDSCAPS_VISIBLE: DWORD = 0x00008000;
pub const DDSCAPS_WRITEONLY: DWORD = 0x00010000;
pub const DDSCAPS_ZBUFFER: DWORD = 0x00020000;
pub const DDSCAPS_OWNDC: DWORD = 0x00040000;
pub const DDSCAPS_LIVEVIDEO: DWORD = 0x00080000;
pub const DDSCAPS_HWCODEC: DWORD = 0x00100000;
pub const DDSCAPS_MODEX: DWORD = 0x00200000;
pub const DDSCAPS_MIPMAP: DWORD = 0x00400000;
pub const DDSCAPS_RESERVED2: DWORD = 0x00800000;
pub const DDSCAPS_ALLOCONLOAD: DWORD = 0x04000000;
pub const DDSCAPS_VIDEOPORT: DWORD = 0x08000000;
pub const DDSCAPS_LOCALVIDMEM: DWORD = 0x10000000;
pub const DDSCAPS_NONLOCALVIDMEM: DWORD = 0x20000000;
pub const DDSCAPS_STANDARDVGAMODE: DWORD = 0x40000000;
pub const DDSCAPS_OPTIMIZED: DWORD = 0x80000000;
pub const DDSCAPS2_RESERVED4: DWORD = 0x00000002;
pub const DDSCAPS2_HARDWAREDEINTERLACE: DWORD = 0x00000000;
pub const DDSCAPS2_HINTDYNAMIC: DWORD = 0x00000004;
pub const DDSCAPS2_HINTSTATIC: DWORD = 0x00000008;
pub const DDSCAPS2_TEXTUREMANAGE: DWORD = 0x00000010;
pub const DDSCAPS2_RESERVED1: DWORD = 0x00000020;
pub const DDSCAPS2_RESERVED2: DWORD = 0x00000040;
pub const DDSCAPS2_OPAQUE: DWORD = 0x00000080;
pub const DDSCAPS2_HINTANTIALIASING: DWORD = 0x00000100;
pub const DDSCAPS2_CUBEMAP: DWORD = 0x00000200;
pub const DDSCAPS2_CUBEMAP_POSITIVEX: DWORD = 0x00000400;
pub const DDSCAPS2_CUBEMAP_NEGATIVEX: DWORD = 0x00000800;
pub const DDSCAPS2_CUBEMAP_POSITIVEY: DWORD = 0x00001000;
pub const DDSCAPS2_CUBEMAP_NEGATIVEY: DWORD = 0x00002000;
pub const DDSCAPS2_CUBEMAP_POSITIVEZ: DWORD = 0x00004000;
pub const DDSCAPS2_CUBEMAP_NEGATIVEZ: DWORD = 0x00008000;
pub const DDSCAPS2_CUBEMAP_ALLFACES: DWORD = DDSCAPS2_CUBEMAP_POSITIVEX
    | DDSCAPS2_CUBEMAP_NEGATIVEX | DDSCAPS2_CUBEMAP_POSITIVEY | DDSCAPS2_CUBEMAP_NEGATIVEY
    | DDSCAPS2_CUBEMAP_POSITIVEZ | DDSCAPS2_CUBEMAP_NEGATIVEZ;
pub const DDSCAPS2_MIPMAPSUBLEVEL: DWORD = 0x00010000;
pub const DDSCAPS2_D3DTEXTUREMANAGE: DWORD = 0x00020000;
pub const DDSCAPS2_DONOTPERSIST: DWORD = 0x00040000;
pub const DDSCAPS2_STEREOSURFACELEFT: DWORD = 0x00080000;
pub const DDSCAPS2_VOLUME: DWORD = 0x00200000;
pub const DDSCAPS2_NOTUSERLOCKABLE: DWORD = 0x00400000;
pub const DDSCAPS2_POINTS: DWORD = 0x00800000;
pub const DDSCAPS2_RTPATCHES: DWORD = 0x01000000;
pub const DDSCAPS2_NPATCHES: DWORD = 0x02000000;
pub const DDSCAPS2_RESERVED3: DWORD = 0x04000000;
pub const DDSCAPS2_DISCARDBACKBUFFER: DWORD = 0x10000000;
pub const DDSCAPS2_ENABLEALPHACHANNEL: DWORD = 0x20000000;
pub const DDSCAPS2_EXTENDEDFORMATPRIMARY: DWORD = 0x40000000;
pub const DDSCAPS2_ADDITIONALPRIMARY: DWORD = 0x80000000;
pub const DDSCAPS3_MULTISAMPLE_MASK: DWORD = 0x0000001F;
pub const DDSCAPS3_MULTISAMPLE_QUALITY_MASK: DWORD = 0x000000E0;
pub const DDSCAPS3_MULTISAMPLE_QUALITY_SHIFT: DWORD = 5;
pub const DDSCAPS3_RESERVED1: DWORD = 0x00000100;
pub const DDSCAPS3_RESERVED2: DWORD = 0x00000200;
pub const DDSCAPS3_LIGHTWEIGHTMIPMAP: DWORD = 0x00000400;
pub const DDSCAPS3_AUTOGENMIPMAP: DWORD = 0x00000800;
pub const DDSCAPS3_DMAP: DWORD = 0x00001000;
pub const DDSCAPS3_CREATESHAREDRESOURCE: DWORD = 0x00002000;
pub const DDSCAPS3_READONLYRESOURCE: DWORD = 0x00004000;
pub const DDSCAPS3_OPENSHAREDRESOURCE: DWORD = 0x00008000;
pub const DDCAPS_3D: DWORD = 0x00000001;
pub const DDCAPS_ALIGNBOUNDARYDEST: DWORD = 0x00000002;
pub const DDCAPS_ALIGNSIZEDEST: DWORD = 0x00000004;
pub const DDCAPS_ALIGNBOUNDARYSRC: DWORD = 0x00000008;
pub const DDCAPS_ALIGNSIZESRC: DWORD = 0x00000010;
pub const DDCAPS_ALIGNSTRIDE: DWORD = 0x00000020;
pub const DDCAPS_BLT: DWORD = 0x00000040;
pub const DDCAPS_BLTQUEUE: DWORD = 0x00000080;
pub const DDCAPS_BLTFOURCC: DWORD = 0x00000100;
pub const DDCAPS_BLTSTRETCH: DWORD = 0x00000200;
pub const DDCAPS_GDI: DWORD = 0x00000400;
pub const DDCAPS_OVERLAY: DWORD = 0x00000800;
pub const DDCAPS_OVERLAYCANTCLIP: DWORD = 0x00001000;
pub const DDCAPS_OVERLAYFOURCC: DWORD = 0x00002000;
pub const DDCAPS_OVERLAYSTRETCH: DWORD = 0x00004000;
pub const DDCAPS_PALETTE: DWORD = 0x00008000;
pub const DDCAPS_PALETTEVSYNC: DWORD = 0x00010000;
pub const DDCAPS_READSCANLINE: DWORD = 0x00020000;
pub const DDCAPS_RESERVED1: DWORD = 0x00040000;
pub const DDCAPS_VBI: DWORD = 0x00080000;
pub const DDCAPS_ZBLTS: DWORD = 0x00100000;
pub const DDCAPS_ZOVERLAYS: DWORD = 0x00200000;
pub const DDCAPS_COLORKEY: DWORD = 0x00400000;
pub const DDCAPS_ALPHA: DWORD = 0x00800000;
pub const DDCAPS_COLORKEYHWASSIST: DWORD = 0x01000000;
pub const DDCAPS_NOHARDWARE: DWORD = 0x02000000;
pub const DDCAPS_BLTCOLORFILL: DWORD = 0x04000000;
pub const DDCAPS_BANKSWITCHED: DWORD = 0x08000000;
pub const DDCAPS_BLTDEPTHFILL: DWORD = 0x10000000;
pub const DDCAPS_CANCLIP: DWORD = 0x20000000;
pub const DDCAPS_CANCLIPSTRETCHED: DWORD = 0x40000000;
pub const DDCAPS_CANBLTSYSMEM: DWORD = 0x80000000;
pub const DDCAPS2_CERTIFIED: DWORD = 0x00000001;
pub const DDCAPS2_NO2DDURING3DSCENE: DWORD = 0x00000002;
pub const DDCAPS2_VIDEOPORT: DWORD = 0x00000004;
pub const DDCAPS2_AUTOFLIPOVERLAY: DWORD = 0x00000008;
pub const DDCAPS2_CANBOBINTERLEAVED: DWORD = 0x00000010;
pub const DDCAPS2_CANBOBNONINTERLEAVED: DWORD = 0x00000020;
pub const DDCAPS2_COLORCONTROLOVERLAY: DWORD = 0x00000040;
pub const DDCAPS2_COLORCONTROLPRIMARY: DWORD = 0x00000080;
pub const DDCAPS2_CANDROPZ16BIT: DWORD = 0x00000100;
pub const DDCAPS2_NONLOCALVIDMEM: DWORD = 0x00000200;
pub const DDCAPS2_NONLOCALVIDMEMCAPS: DWORD = 0x00000400;
pub const DDCAPS2_NOPAGELOCKREQUIRED: DWORD = 0x00000800;
pub const DDCAPS2_WIDESURFACES: DWORD = 0x00001000;
pub const DDCAPS2_CANFLIPODDEVEN: DWORD = 0x00002000;
pub const DDCAPS2_CANBOBHARDWARE: DWORD = 0x00004000;
pub const DDCAPS2_COPYFOURCC: DWORD = 0x00008000;
pub const DDCAPS2_PRIMARYGAMMA: DWORD = 0x00020000;
pub const DDCAPS2_CANRENDERWINDOWED: DWORD = 0x00080000;
pub const DDCAPS2_CANCALIBRATEGAMMA: DWORD = 0x00100000;
pub const DDCAPS2_FLIPINTERVAL: DWORD = 0x00200000;
pub const DDCAPS2_FLIPNOVSYNC: DWORD = 0x00400000;
pub const DDCAPS2_CANMANAGETEXTURE: DWORD = 0x00800000;
pub const DDCAPS2_TEXMANINNONLOCALVIDMEM: DWORD = 0x01000000;
pub const DDCAPS2_STEREO: DWORD = 0x02000000;
pub const DDCAPS2_SYSTONONLOCAL_AS_SYSTOLOCAL: DWORD = 0x04000000;
pub const DDCAPS2_RESERVED1: DWORD = 0x08000000;
pub const DDCAPS2_CANMANAGERESOURCE: DWORD = 0x10000000;
pub const DDCAPS2_DYNAMICTEXTURES: DWORD = 0x20000000;
pub const DDCAPS2_CANAUTOGENMIPMAP: DWORD = 0x40000000;
pub const DDCAPS2_CANSHARERESOURCE: DWORD = 0x80000000;
pub const DDFXALPHACAPS_BLTALPHAEDGEBLEND: DWORD = 0x00000001;
pub const DDFXALPHACAPS_BLTALPHAPIXELS: DWORD = 0x00000002;
pub const DDFXALPHACAPS_BLTALPHAPIXELSNEG: DWORD = 0x00000004;
pub const DDFXALPHACAPS_BLTALPHASURFACES: DWORD = 0x00000008;
pub const DDFXALPHACAPS_BLTALPHASURFACESNEG: DWORD = 0x00000010;
pub const DDFXALPHACAPS_OVERLAYALPHAEDGEBLEND: DWORD = 0x00000020;
pub const DDFXALPHACAPS_OVERLAYALPHAPIXELS: DWORD = 0x00000040;
pub const DDFXALPHACAPS_OVERLAYALPHAPIXELSNEG: DWORD = 0x00000080;
pub const DDFXALPHACAPS_OVERLAYALPHASURFACES: DWORD = 0x00000100;
pub const DDFXALPHACAPS_OVERLAYALPHASURFACESNEG: DWORD = 0x00000200;
pub const DDFXCAPS_BLTARITHSTRETCHY: DWORD = 0x00000020;
pub const DDFXCAPS_BLTARITHSTRETCHYN: DWORD = 0x00000010;
pub const DDFXCAPS_BLTMIRRORLEFTRIGHT: DWORD = 0x00000040;
pub const DDFXCAPS_BLTMIRRORUPDOWN: DWORD = 0x00000080;
pub const DDFXCAPS_BLTROTATION: DWORD = 0x00000100;
pub const DDFXCAPS_BLTROTATION90: DWORD = 0x00000200;
pub const DDFXCAPS_BLTSHRINKX: DWORD = 0x00000400;
pub const DDFXCAPS_BLTSHRINKXN: DWORD = 0x00000800;
pub const DDFXCAPS_BLTSHRINKY: DWORD = 0x00001000;
pub const DDFXCAPS_BLTSHRINKYN: DWORD = 0x00002000;
pub const DDFXCAPS_BLTSTRETCHX: DWORD = 0x00004000;
pub const DDFXCAPS_BLTSTRETCHXN: DWORD = 0x00008000;
pub const DDFXCAPS_BLTSTRETCHY: DWORD = 0x00010000;
pub const DDFXCAPS_BLTSTRETCHYN: DWORD = 0x00020000;
pub const DDFXCAPS_OVERLAYARITHSTRETCHY: DWORD = 0x00040000;
pub const DDFXCAPS_OVERLAYARITHSTRETCHYN: DWORD = 0x00000008;
pub const DDFXCAPS_OVERLAYSHRINKX: DWORD = 0x00080000;
pub const DDFXCAPS_OVERLAYSHRINKXN: DWORD = 0x00100000;
pub const DDFXCAPS_OVERLAYSHRINKY: DWORD = 0x00200000;
pub const DDFXCAPS_OVERLAYSHRINKYN: DWORD = 0x00400000;
pub const DDFXCAPS_OVERLAYSTRETCHX: DWORD = 0x00800000;
pub const DDFXCAPS_OVERLAYSTRETCHXN: DWORD = 0x01000000;
pub const DDFXCAPS_OVERLAYSTRETCHY: DWORD = 0x02000000;
pub const DDFXCAPS_OVERLAYSTRETCHYN: DWORD = 0x04000000;
pub const DDFXCAPS_OVERLAYMIRRORLEFTRIGHT: DWORD = 0x08000000;
pub const DDFXCAPS_OVERLAYMIRRORUPDOWN: DWORD = 0x10000000;
pub const DDFXCAPS_OVERLAYDEINTERLACE: DWORD = 0x20000000;
pub const DDFXCAPS_BLTALPHA: DWORD = 0x00000001;
pub const DDFXCAPS_BLTFILTER: DWORD = DDFXCAPS_BLTARITHSTRETCHY;
pub const DDFXCAPS_OVERLAYALPHA: DWORD = 0x00000004;
pub const DDFXCAPS_OVERLAYFILTER: DWORD = DDFXCAPS_OVERLAYARITHSTRETCHY;
pub const DDSVCAPS_RESERVED1: DWORD = 0x00000001;
pub const DDSVCAPS_RESERVED2: DWORD = 0x00000002;
pub const DDSVCAPS_RESERVED3: DWORD = 0x00000004;
pub const DDSVCAPS_RESERVED4: DWORD = 0x00000008;
pub const DDSVCAPS_STEREOSEQUENTIAL: DWORD = 0x00000010;
pub const DDPCAPS_4BIT: DWORD = 0x00000001;
pub const DDPCAPS_8BITENTRIES: DWORD = 0x00000002;
pub const DDPCAPS_8BIT: DWORD = 0x00000004;
pub const DDPCAPS_INITIALIZE: DWORD = 0x00000000;
pub const DDPCAPS_PRIMARYSURFACE: DWORD = 0x00000010;
pub const DDPCAPS_PRIMARYSURFACELEFT: DWORD = 0x00000020;
pub const DDPCAPS_ALLOW256: DWORD = 0x00000040;
pub const DDPCAPS_VSYNC: DWORD = 0x00000080;
pub const DDPCAPS_1BIT: DWORD = 0x00000100;
pub const DDPCAPS_2BIT: DWORD = 0x00000200;
pub const DDPCAPS_ALPHA: DWORD = 0x00000400;
pub const DDSPD_IUNKNOWNPOINTER: DWORD = 0x00000001;
pub const DDSPD_VOLATILE: DWORD = 0x00000002;
pub const DDBD_1: DWORD = 0x00004000;
pub const DDBD_2: DWORD = 0x00002000;
pub const DDBD_4: DWORD = 0x00001000;
pub const DDBD_8: DWORD = 0x00000800;
pub const DDBD_16: DWORD = 0x00000400;
pub const DDBD_24: DWORD = 0x00000200;
pub const DDBD_32: DWORD = 0x00000100;
pub const DDCKEY_COLORSPACE: DWORD = 0x00000001;
pub const DDCKEY_DESTBLT: DWORD = 0x00000002;
pub const DDCKEY_DESTOVERLAY: DWORD = 0x00000004;
pub const DDCKEY_SRCBLT: DWORD = 0x00000008;
pub const DDCKEY_SRCOVERLAY: DWORD = 0x00000010;
pub const DDCKEYCAPS_DESTBLT: DWORD = 0x00000001;
pub const DDCKEYCAPS_DESTBLTCLRSPACE: DWORD = 0x00000002;
pub const DDCKEYCAPS_DESTBLTCLRSPACEYUV: DWORD = 0x00000004;
pub const DDCKEYCAPS_DESTBLTYUV: DWORD = 0x00000008;
pub const DDCKEYCAPS_DESTOVERLAY: DWORD = 0x00000010;
pub const DDCKEYCAPS_DESTOVERLAYCLRSPACE: DWORD = 0x00000020;
pub const DDCKEYCAPS_DESTOVERLAYCLRSPACEYUV: DWORD = 0x00000040;
pub const DDCKEYCAPS_DESTOVERLAYONEACTIVE: DWORD = 0x00000080;
pub const DDCKEYCAPS_DESTOVERLAYYUV: DWORD = 0x00000100;
pub const DDCKEYCAPS_SRCBLT: DWORD = 0x00000200;
pub const DDCKEYCAPS_SRCBLTCLRSPACE: DWORD = 0x00000400;
pub const DDCKEYCAPS_SRCBLTCLRSPACEYUV: DWORD = 0x00000800;
pub const DDCKEYCAPS_SRCBLTYUV: DWORD = 0x00001000;
pub const DDCKEYCAPS_SRCOVERLAY: DWORD = 0x00002000;
pub const DDCKEYCAPS_SRCOVERLAYCLRSPACE: DWORD = 0x00004000;
pub const DDCKEYCAPS_SRCOVERLAYCLRSPACEYUV: DWORD = 0x00008000;
pub const DDCKEYCAPS_SRCOVERLAYONEACTIVE: DWORD = 0x00010000;
pub const DDCKEYCAPS_SRCOVERLAYYUV: DWORD = 0x00020000;
pub const DDCKEYCAPS_NOCOSTOVERLAY: DWORD = 0x00040000;
pub const DDPF_ALPHAPIXELS: DWORD = 0x00000001;
pub const DDPF_ALPHA: DWORD = 0x00000002;
pub const DDPF_FOURCC: DWORD = 0x00000004;
pub const DDPF_PALETTEINDEXED4: DWORD = 0x00000008;
pub const DDPF_PALETTEINDEXEDTO8: DWORD = 0x00000010;
pub const DDPF_PALETTEINDEXED8: DWORD = 0x00000020;
pub const DDPF_RGB: DWORD = 0x00000040;
pub const DDPF_COMPRESSED: DWORD = 0x00000080;
pub const DDPF_RGBTOYUV: DWORD = 0x00000100;
pub const DDPF_YUV: DWORD = 0x00000200;
pub const DDPF_ZBUFFER: DWORD = 0x00000400;
pub const DDPF_PALETTEINDEXED1: DWORD = 0x00000800;
pub const DDPF_PALETTEINDEXED2: DWORD = 0x00001000;
pub const DDPF_ZPIXELS: DWORD = 0x00002000;
pub const DDPF_STENCILBUFFER: DWORD = 0x00004000;
pub const DDPF_ALPHAPREMULT: DWORD = 0x00008000;
pub const DDPF_LUMINANCE: DWORD = 0x00020000;
pub const DDPF_BUMPLUMINANCE: DWORD = 0x00040000;
pub const DDPF_BUMPDUDV: DWORD = 0x00080000;
pub const DDENUMSURFACES_ALL: DWORD = 0x00000001;
pub const DDENUMSURFACES_MATCH: DWORD = 0x00000002;
pub const DDENUMSURFACES_NOMATCH: DWORD = 0x00000004;
pub const DDENUMSURFACES_CANBECREATED: DWORD = 0x00000008;
pub const DDENUMSURFACES_DOESEXIST: DWORD = 0x00000010;
pub const DDSDM_STANDARDVGAMODE: DWORD = 0x00000001;
pub const DDEDM_REFRESHRATES: DWORD = 0x00000001;
pub const DDEDM_STANDARDVGAMODES: DWORD = 0x00000002;
pub const DDSCL_FULLSCREEN: DWORD = 0x00000001;
pub const DDSCL_ALLOWREBOOT: DWORD = 0x00000002;
pub const DDSCL_NOWINDOWCHANGES: DWORD = 0x00000004;
pub const DDSCL_NORMAL: DWORD = 0x00000008;
pub const DDSCL_EXCLUSIVE: DWORD = 0x00000010;
pub const DDSCL_ALLOWMODEX: DWORD = 0x00000040;
pub const DDSCL_SETFOCUSWINDOW: DWORD = 0x00000080;
pub const DDSCL_SETDEVICEWINDOW: DWORD = 0x00000100;
pub const DDSCL_CREATEDEVICEWINDOW: DWORD = 0x00000200;
pub const DDSCL_MULTITHREADED: DWORD = 0x00000400;
pub const DDSCL_FPUSETUP: DWORD = 0x00000800;
pub const DDSCL_FPUPRESERVE: DWORD = 0x00001000;
pub const DDBLT_ALPHADEST: DWORD = 0x00000001;
pub const DDBLT_ALPHADESTCONSTOVERRIDE: DWORD = 0x00000002;
pub const DDBLT_ALPHADESTNEG: DWORD = 0x00000004;
pub const DDBLT_ALPHADESTSURFACEOVERRIDE: DWORD = 0x00000008;
pub const DDBLT_ALPHAEDGEBLEND: DWORD = 0x00000010;
pub const DDBLT_ALPHASRC: DWORD = 0x00000020;
pub const DDBLT_ALPHASRCCONSTOVERRIDE: DWORD = 0x00000040;
pub const DDBLT_ALPHASRCNEG: DWORD = 0x00000080;
pub const DDBLT_ALPHASRCSURFACEOVERRIDE: DWORD = 0x00000100;
pub const DDBLT_ASYNC: DWORD = 0x00000200;
pub const DDBLT_COLORFILL: DWORD = 0x00000400;
pub const DDBLT_DDFX: DWORD = 0x00000800;
pub const DDBLT_DDROPS: DWORD = 0x00001000;
pub const DDBLT_KEYDEST: DWORD = 0x00002000;
pub const DDBLT_KEYDESTOVERRIDE: DWORD = 0x00004000;
pub const DDBLT_KEYSRC: DWORD = 0x00008000;
pub const DDBLT_KEYSRCOVERRIDE: DWORD = 0x00010000;
pub const DDBLT_ROP: DWORD = 0x00020000;
pub const DDBLT_ROTATIONANGLE: DWORD = 0x00040000;
pub const DDBLT_ZBUFFER: DWORD = 0x00080000;
pub const DDBLT_ZBUFFERDESTCONSTOVERRIDE: DWORD = 0x00100000;
pub const DDBLT_ZBUFFERDESTOVERRIDE: DWORD = 0x00200000;
pub const DDBLT_ZBUFFERSRCCONSTOVERRIDE: DWORD = 0x00400000;
pub const DDBLT_ZBUFFERSRCOVERRIDE: DWORD = 0x00800000;
pub const DDBLT_WAIT: DWORD = 0x01000000;
pub const DDBLT_DEPTHFILL: DWORD = 0x02000000;
pub const DDBLT_DONOTWAIT: DWORD = 0x08000000;
pub const DDBLT_PRESENTATION: DWORD = 0x10000000;
pub const DDBLT_LAST_PRESENTATION: DWORD = 0x20000000;
pub const DDBLT_EXTENDED_FLAGS: DWORD = 0x40000000;
pub const DDBLT_EXTENDED_LINEAR_CONTENT: DWORD = 0x00000004;
pub const DDBLTFAST_NOCOLORKEY: DWORD = 0x00000000;
pub const DDBLTFAST_SRCCOLORKEY: DWORD = 0x00000001;
pub const DDBLTFAST_DESTCOLORKEY: DWORD = 0x00000002;
pub const DDBLTFAST_WAIT: DWORD = 0x00000010;
pub const DDBLTFAST_DONOTWAIT: DWORD = 0x00000020;
pub const DDFLIP_WAIT: DWORD = 0x00000001;
pub const DDFLIP_EVEN: DWORD = 0x00000002;
pub const DDFLIP_ODD: DWORD = 0x00000004;
pub const DDFLIP_NOVSYNC: DWORD = 0x00000008;
pub const DDFLIP_INTERVAL2: DWORD = 0x02000000;
pub const DDFLIP_INTERVAL3: DWORD = 0x03000000;
pub const DDFLIP_INTERVAL4: DWORD = 0x04000000;
pub const DDFLIP_STEREO: DWORD = 0x00000010;
pub const DDFLIP_DONOTWAIT: DWORD = 0x00000020;
pub const DDOVER_ALPHADEST: DWORD = 0x00000001;
pub const DDOVER_ALPHADESTCONSTOVERRIDE: DWORD = 0x00000002;
pub const DDOVER_ALPHADESTNEG: DWORD = 0x00000004;
pub const DDOVER_ALPHADESTSURFACEOVERRIDE: DWORD = 0x00000008;
pub const DDOVER_ALPHAEDGEBLEND: DWORD = 0x00000010;
pub const DDOVER_ALPHASRC: DWORD = 0x00000020;
pub const DDOVER_ALPHASRCCONSTOVERRIDE: DWORD = 0x00000040;
pub const DDOVER_ALPHASRCNEG: DWORD = 0x00000080;
pub const DDOVER_ALPHASRCSURFACEOVERRIDE: DWORD = 0x00000100;
pub const DDOVER_HIDE: DWORD = 0x00000200;
pub const DDOVER_KEYDEST: DWORD = 0x00000400;
pub const DDOVER_KEYDESTOVERRIDE: DWORD = 0x00000800;
pub const DDOVER_KEYSRC: DWORD = 0x00001000;
pub const DDOVER_KEYSRCOVERRIDE: DWORD = 0x00002000;
pub const DDOVER_SHOW: DWORD = 0x00004000;
pub const DDOVER_ADDDIRTYRECT: DWORD = 0x00008000;
pub const DDOVER_REFRESHDIRTYRECTS: DWORD = 0x00010000;
pub const DDOVER_REFRESHALL: DWORD = 0x00020000;
pub const DDOVER_DDFX: DWORD = 0x00080000;
pub const DDOVER_AUTOFLIP: DWORD = 0x00100000;
pub const DDOVER_BOB: DWORD = 0x00200000;
pub const DDOVER_OVERRIDEBOBWEAVE: DWORD = 0x00400000;
pub const DDOVER_INTERLEAVED: DWORD = 0x00800000;
pub const DDOVER_BOBHARDWARE: DWORD = 0x01000000;
pub const DDOVER_ARGBSCALEFACTORS: DWORD = 0x02000000;
pub const DDOVER_DEGRADEARGBSCALING: DWORD = 0x04000000;
pub const DDSETSURFACEDESC_RECREATEDC: DWORD = 0x00000000;
pub const DDSETSURFACEDESC_PRESERVEDC: DWORD = 0x00000001;
pub const DDLOCK_SURFACEMEMORYPTR: DWORD = 0x00000000;
pub const DDLOCK_WAIT: DWORD = 0x00000001;
pub const DDLOCK_EVENT: DWORD = 0x00000002;
pub const DDLOCK_READONLY: DWORD = 0x00000010;
pub const DDLOCK_WRITEONLY: DWORD = 0x00000020;
pub const DDLOCK_NOSYSLOCK: DWORD = 0x00000800;
pub const DDLOCK_NOOVERWRITE: DWORD = 0x00001000;
pub const DDLOCK_DISCARDCONTENTS: DWORD = 0x00002000;
pub const DDLOCK_OKTOSWAP: DWORD = 0x00002000;
pub const DDLOCK_DONOTWAIT: DWORD = 0x00004000;
pub const DDLOCK_HASVOLUMETEXTUREBOXRECT: DWORD = 0x00008000;
pub const DDLOCK_NODIRTYUPDATE: DWORD = 0x00010000;
pub const DDBLTFX_ARITHSTRETCHY: DWORD = 0x00000001;
pub const DDBLTFX_MIRRORLEFTRIGHT: DWORD = 0x00000002;
pub const DDBLTFX_MIRRORUPDOWN: DWORD = 0x00000004;
pub const DDBLTFX_NOTEARING: DWORD = 0x00000008;
pub const DDBLTFX_ROTATE180: DWORD = 0x00000010;
pub const DDBLTFX_ROTATE270: DWORD = 0x00000020;
pub const DDBLTFX_ROTATE90: DWORD = 0x00000040;
pub const DDBLTFX_ZBUFFERRANGE: DWORD = 0x00000080;
pub const DDBLTFX_ZBUFFERBASEDEST: DWORD = 0x00000100;
pub const DDOVERFX_ARITHSTRETCHY: DWORD = 0x00000001;
pub const DDOVERFX_MIRRORLEFTRIGHT: DWORD = 0x00000002;
pub const DDOVERFX_MIRRORUPDOWN: DWORD = 0x00000004;
pub const DDOVERFX_DEINTERLACE: DWORD = 0x00000008;
pub const DDWAITVB_BLOCKBEGIN: DWORD = 0x00000001;
pub const DDWAITVB_BLOCKBEGINEVENT: DWORD = 0x00000002;
pub const DDWAITVB_BLOCKEND: DWORD = 0x00000004;
pub const DDGFS_CANFLIP: DWORD = 0x00000001;
pub const DDGFS_ISFLIPDONE: DWORD = 0x00000002;
pub const DDGBS_CANBLT: DWORD = 0x00000001;
pub const DDGBS_ISBLTDONE: DWORD = 0x00000002;
pub const DDENUMOVERLAYZ_BACKTOFRONT: DWORD = 0x00000000;
pub const DDENUMOVERLAYZ_FRONTTOBACK: DWORD = 0x00000001;
pub const DDOVERZ_SENDTOFRONT: DWORD = 0x00000000;
pub const DDOVERZ_SENDTOBACK: DWORD = 0x00000001;
pub const DDOVERZ_MOVEFORWARD: DWORD = 0x00000002;
pub const DDOVERZ_MOVEBACKWARD: DWORD = 0x00000003;
pub const DDOVERZ_INSERTINFRONTOF: DWORD = 0x00000004;
pub const DDOVERZ_INSERTINBACKOF: DWORD = 0x00000005;
pub const DDSGR_CALIBRATE: DWORD = 0x00000001;
pub const DDSMT_ISTESTREQUIRED: DWORD = 0x00000001;
pub const DDEM_MODEPASSED: DWORD = 0x00000001;
pub const DDEM_MODEFAILED: DWORD = 0x00000002;
pub const DD_OK: HRESULT = S_OK;
pub const DD_FALSE: HRESULT = S_FALSE;
pub const DDENUMRET_CANCEL: DWORD = 0;
pub const DDENUMRET_OK: DWORD = 1;
pub const DDERR_ALREADYINITIALIZED: HRESULT = MAKE_DDHRESULT!(5);
pub const DDERR_CANNOTATTACHSURFACE: HRESULT = MAKE_DDHRESULT!(10);
pub const DDERR_CANNOTDETACHSURFACE: HRESULT = MAKE_DDHRESULT!(20);
pub const DDERR_CURRENTLYNOTAVAIL: HRESULT = MAKE_DDHRESULT!(40);
pub const DDERR_EXCEPTION: HRESULT = MAKE_DDHRESULT!(55);
pub const DDERR_GENERIC: HRESULT = E_FAIL;
pub const DDERR_HEIGHTALIGN: HRESULT = MAKE_DDHRESULT!(90);
pub const DDERR_INCOMPATIBLEPRIMARY: HRESULT = MAKE_DDHRESULT!(95);
pub const DDERR_INVALIDCAPS: HRESULT = MAKE_DDHRESULT!(100);
pub const DDERR_INVALIDCLIPLIST: HRESULT = MAKE_DDHRESULT!(110);
pub const DDERR_INVALIDMODE: HRESULT = MAKE_DDHRESULT!(120);
pub const DDERR_INVALIDOBJECT: HRESULT = MAKE_DDHRESULT!(130);
pub const DDERR_INVALIDPARAMS: HRESULT = E_INVALIDARG;
pub const DDERR_INVALIDPIXELFORMAT: HRESULT = MAKE_DDHRESULT!(145);
pub const DDERR_INVALIDRECT: HRESULT = MAKE_DDHRESULT!(150);
pub const DDERR_LOCKEDSURFACES: HRESULT = MAKE_DDHRESULT!(160);
pub const DDERR_NO3D: HRESULT = MAKE_DDHRESULT!(170);
pub const DDERR_NOALPHAHW: HRESULT = MAKE_DDHRESULT!(180);
pub const DDERR_NOSTEREOHARDWARE: HRESULT = MAKE_DDHRESULT!(181);
pub const DDERR_NOSURFACELEFT: HRESULT = MAKE_DDHRESULT!(182);
pub const DDERR_NOCLIPLIST: HRESULT = MAKE_DDHRESULT!(205);
pub const DDERR_NOCOLORCONVHW: HRESULT = MAKE_DDHRESULT!(210);
pub const DDERR_NOCOOPERATIVELEVELSET: HRESULT = MAKE_DDHRESULT!(212);
pub const DDERR_NOCOLORKEY: HRESULT = MAKE_DDHRESULT!(215);
pub const DDERR_NOCOLORKEYHW: HRESULT = MAKE_DDHRESULT!(220);
pub const DDERR_NODIRECTDRAWSUPPORT: HRESULT = MAKE_DDHRESULT!(222);
pub const DDERR_NOEXCLUSIVEMODE: HRESULT = MAKE_DDHRESULT!(225);
pub const DDERR_NOFLIPHW: HRESULT = MAKE_DDHRESULT!(230);
pub const DDERR_NOGDI: HRESULT = MAKE_DDHRESULT!(240);
pub const DDERR_NOMIRRORHW: HRESULT = MAKE_DDHRESULT!(250);
pub const DDERR_NOTFOUND: HRESULT = MAKE_DDHRESULT!(255);
pub const DDERR_NOOVERLAYHW: HRESULT = MAKE_DDHRESULT!(260);
pub const DDERR_OVERLAPPINGRECTS: HRESULT = MAKE_DDHRESULT!(270);
pub const DDERR_NORASTEROPHW: HRESULT = MAKE_DDHRESULT!(280);
pub const DDERR_NOROTATIONHW: HRESULT = MAKE_DDHRESULT!(290);
pub const DDERR_NOSTRETCHHW: HRESULT = MAKE_DDHRESULT!(310);
pub const DDERR_NOT4BITCOLOR: HRESULT = MAKE_DDHRESULT!(316);
pub const DDERR_NOT4BITCOLORINDEX: HRESULT = MAKE_DDHRESULT!(317);
pub const DDERR_NOT8BITCOLOR: HRESULT = MAKE_DDHRESULT!(320);
pub const DDERR_NOTEXTUREHW: HRESULT = MAKE_DDHRESULT!(330);
pub const DDERR_NOVSYNCHW: HRESULT = MAKE_DDHRESULT!(335);
pub const DDERR_NOZBUFFERHW: HRESULT = MAKE_DDHRESULT!(340);
pub const DDERR_NOZOVERLAYHW: HRESULT = MAKE_DDHRESULT!(350);
pub const DDERR_OUTOFCAPS: HRESULT = MAKE_DDHRESULT!(360);
pub const DDERR_OUTOFMEMORY: HRESULT = E_OUTOFMEMORY;
pub const DDERR_OUTOFVIDEOMEMORY: HRESULT = MAKE_DDHRESULT!(380);
pub const DDERR_OVERLAYCANTCLIP: HRESULT = MAKE_DDHRESULT!(382);
pub const DDERR_OVERLAYCOLORKEYONLYONEACTIVE: HRESULT = MAKE_DDHRESULT!(384);
pub const DDERR_PALETTEBUSY: HRESULT = MAKE_DDHRESULT!(387);
pub const DDERR_COLORKEYNOTSET: HRESULT = MAKE_DDHRESULT!(400);
pub const DDERR_SURFACEALREADYATTACHED: HRESULT = MAKE_DDHRESULT!(410);
pub const DDERR_SURFACEALREADYDEPENDENT: HRESULT = MAKE_DDHRESULT!(420);
pub const DDERR_SURFACEBUSY: HRESULT = MAKE_DDHRESULT!(430);
pub const DDERR_CANTLOCKSURFACE: HRESULT = MAKE_DDHRESULT!(435);
pub const DDERR_SURFACEISOBSCURED: HRESULT = MAKE_DDHRESULT!(440);
pub const DDERR_SURFACELOST: HRESULT = MAKE_DDHRESULT!(450);
pub const DDERR_SURFACENOTATTACHED: HRESULT = MAKE_DDHRESULT!(460);
pub const DDERR_TOOBIGHEIGHT: HRESULT = MAKE_DDHRESULT!(470);
pub const DDERR_TOOBIGSIZE: HRESULT = MAKE_DDHRESULT!(480);
pub const DDERR_TOOBIGWIDTH: HRESULT = MAKE_DDHRESULT!(490);
pub const DDERR_UNSUPPORTED: HRESULT = E_NOTIMPL;
pub const DDERR_UNSUPPORTEDFORMAT: HRESULT = MAKE_DDHRESULT!(510);
pub const DDERR_UNSUPPORTEDMASK: HRESULT = MAKE_DDHRESULT!(520);
pub const DDERR_INVALIDSTREAM: HRESULT = MAKE_DDHRESULT!(521);
pub const DDERR_VERTICALBLANKINPROGRESS: HRESULT = MAKE_DDHRESULT!(537);
pub const DDERR_WASSTILLDRAWING: HRESULT = MAKE_DDHRESULT!(540);
pub const DDERR_DDSCAPSCOMPLEXREQUIRED: HRESULT = MAKE_DDHRESULT!(542);
pub const DDERR_XALIGN: HRESULT = MAKE_DDHRESULT!(560);
pub const DDERR_INVALIDDIRECTDRAWGUID: HRESULT = MAKE_DDHRESULT!(561);
pub const DDERR_DIRECTDRAWALREADYCREATED: HRESULT = MAKE_DDHRESULT!(562);
pub const DDERR_NODIRECTDRAWHW: HRESULT = MAKE_DDHRESULT!(563);
pub const DDERR_PRIMARYSURFACEALREADYEXISTS: HRESULT = MAKE_DDHRESULT!(564);
pub const DDERR_NOEMULATION: HRESULT = MAKE_DDHRESULT!(565);
pub const DDERR_REGIONTOOSMALL: HRESULT = MAKE_DDHRESULT!(566);
pub const DDERR_CLIPPERISUSINGHWND: HRESULT = MAKE_DDHRESULT!(567);
pub const DDERR_NOCLIPPERATTACHED: HRESULT = MAKE_DDHRESULT!(568);
pub const DDERR_NOHWND: HRESULT = MAKE_DDHRESULT!(569);
pub const DDERR_HWNDSUBCLASSED: HRESULT = MAKE_DDHRESULT!(570);
pub const DDERR_HWNDALREADYSET: HRESULT = MAKE_DDHRESULT!(571);
pub const DDERR_NOPALETTEATTACHED: HRESULT = MAKE_DDHRESULT!(572);
pub const DDERR_NOPALETTEHW: HRESULT = MAKE_DDHRESULT!(573);
pub const DDERR_BLTFASTCANTCLIP: HRESULT = MAKE_DDHRESULT!(574);
pub const DDERR_NOBLTHW: HRESULT = MAKE_DDHRESULT!(575);
pub const DDERR_NODDROPSHW: HRESULT = MAKE_DDHRESULT!(576);
pub const DDERR_OVERLAYNOTVISIBLE: HRESULT = MAKE_DDHRESULT!(577);
pub const DDERR_NOOVERLAYDEST: HRESULT = MAKE_DDHRESULT!(578);
pub const DDERR_INVALIDPOSITION: HRESULT = MAKE_DDHRESULT!(579);
pub const DDERR_NOTAOVERLAYSURFACE: HRESULT = MAKE_DDHRESULT!(580);
pub const DDERR_EXCLUSIVEMODEALREADYSET: HRESULT = MAKE_DDHRESULT!(581);
pub const DDERR_NOTFLIPPABLE: HRESULT = MAKE_DDHRESULT!(582);
pub const DDERR_CANTDUPLICATE: HRESULT = MAKE_DDHRESULT!(583);
pub const DDERR_NOTLOCKED: HRESULT = MAKE_DDHRESULT!(584);
pub const DDERR_CANTCREATEDC: HRESULT = MAKE_DDHRESULT!(585);
pub const DDERR_NODC: HRESULT = MAKE_DDHRESULT!(586);
pub const DDERR_WRONGMODE: HRESULT = MAKE_DDHRESULT!(587);
pub const DDERR_IMPLICITLYCREATED: HRESULT = MAKE_DDHRESULT!(588);
pub const DDERR_NOTPALETTIZED: HRESULT = MAKE_DDHRESULT!(589);
pub const DDERR_UNSUPPORTEDMODE: HRESULT = MAKE_DDHRESULT!(590);
pub const DDERR_NOMIPMAPHW: HRESULT = MAKE_DDHRESULT!(591);
pub const DDERR_INVALIDSURFACETYPE: HRESULT = MAKE_DDHRESULT!(592);
pub const DDERR_NOOPTIMIZEHW: HRESULT = MAKE_DDHRESULT!(600);
pub const DDERR_NOTLOADED: HRESULT = MAKE_DDHRESULT!(601);
pub const DDERR_NOFOCUSWINDOW: HRESULT = MAKE_DDHRESULT!(602);
pub const DDERR_NOTONMIPMAPSUBLEVEL: HRESULT = MAKE_DDHRESULT!(603);
pub const DDERR_DCALREADYCREATED: HRESULT = MAKE_DDHRESULT!(620);
pub const DDERR_NONONLOCALVIDMEM: HRESULT = MAKE_DDHRESULT!(630);
pub const DDERR_CANTPAGELOCK: HRESULT = MAKE_DDHRESULT!(640);
pub const DDERR_CANTPAGEUNLOCK: HRESULT = MAKE_DDHRESULT!(660);
pub const DDERR_NOTPAGELOCKED: HRESULT = MAKE_DDHRESULT!(680);
pub const DDERR_MOREDATA: HRESULT = MAKE_DDHRESULT!(690);
pub const DDERR_EXPIRED: HRESULT = MAKE_DDHRESULT!(691);
pub const DDERR_TESTFINISHED: HRESULT = MAKE_DDHRESULT!(692);
pub const DDERR_NEWMODE: HRESULT = MAKE_DDHRESULT!(693);
pub const DDERR_D3DNOTINITIALIZED: HRESULT = MAKE_DDHRESULT!(694);
pub const DDERR_VIDEONOTACTIVE: HRESULT = MAKE_DDHRESULT!(695);
pub const DDERR_NOMONITORINFORMATION: HRESULT = MAKE_DDHRESULT!(696);
pub const DDERR_NODRIVERSUPPORT: HRESULT = MAKE_DDHRESULT!(697);
pub const DDERR_DEVICEDOESNTOWNSURFACE: HRESULT = MAKE_DDHRESULT!(699);
pub const DDERR_NOTINITIALIZED: HRESULT = CO_E_NOTINITIALIZED;
