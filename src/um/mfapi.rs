// Copyright © 2018 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of mfapi.h
use shared::basetsd::UINT32;
use shared::d3d9types::{
    D3DFMT_A16B16G16R16F, D3DFMT_A2B10G10R10, D3DFMT_A8R8G8B8, D3DFMT_D16, D3DFMT_L16, D3DFMT_L8,
    D3DFMT_P8, D3DFMT_R5G6B5, D3DFMT_R8G8B8, D3DFMT_X1R5G5B5, D3DFMT_X8R8G8B8,
};
use shared::minwindef::{DWORD, ULONG};
use um::mfobjects::{IMFAttributes, IMFSample};
use um::winnt::HRESULT;
pub const MF_SDK_VERSION: ULONG = 0x0002;
pub const MF_API_VERSION: ULONG = 0x0070;
pub const MF_VERSION: ULONG = (MF_SDK_VERSION << 16 | MF_API_VERSION);
pub const MFSTARTUP_NOSOCKET: DWORD = 0x1;
pub const MFSTARTUP_LITE: DWORD = MFSTARTUP_NOSOCKET;
pub const MFSTARTUP_FULL: DWORD = 0;
extern "system" {
    pub fn MFStartup(
        Version: ULONG,
        dwFlags: DWORD,
    ) -> HRESULT;
    pub fn MFShutdown() -> HRESULT;
    pub fn MFCreateAttributes(
        ppMFAttributes: *mut *mut IMFAttributes,
        cInitialSize: UINT32,
    ) -> HRESULT;
}
extern "system" {
    pub fn MFCreateSample(
        ppIMFSample: *mut *mut IMFSample
    ) -> HRESULT;
}
macro_rules! DEFINE_MEDIATYPE_GUID {
    ( $name:ident, FCC($ch0:expr, $ch1:expr, $ch2:expr, $ch3:expr) ) => {
        DEFINE_MEDIATYPE_GUID!{$name,
            ($ch3 as DWORD) << 24 | ($ch2 as DWORD) << 16 | ($ch1 as DWORD) << 8 | ($ch0 as DWORD)
        }
    };
    ( $name:ident, $format:expr ) => {
        DEFINE_GUID!{$name, $format, 0x0000, 0x0010, 0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71}
    };
}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_Base, 0x00000000}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_RGB32, D3DFMT_X8R8G8B8}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_ARGB32, D3DFMT_A8R8G8B8}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_RGB24, D3DFMT_R8G8B8}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_RGB555, D3DFMT_X1R5G5B5}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_RGB565, D3DFMT_R5G6B5}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_RGB8, D3DFMT_P8}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_L8, D3DFMT_L8}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_L16, D3DFMT_L16}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_D16, D3DFMT_D16}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_AI44, FCC('A','I','4','4')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_AYUV, FCC('A','Y','U','V')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_YUY2, FCC('Y','U','Y','2')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_YVYU, FCC('Y','V','Y','U')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_YVU9, FCC('Y','V','U','9')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_UYVY, FCC('U','Y','V','Y')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_NV11, FCC('N','V','1','1')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_NV12, FCC('N','V','1','2')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_YV12, FCC('Y','V','1','2')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_I420, FCC('I','4','2','0')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_IYUV, FCC('I','Y','U','V')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_Y210, FCC('Y','2','1','0')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_Y216, FCC('Y','2','1','6')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_Y410, FCC('Y','4','1','0')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_Y416, FCC('Y','4','1','6')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_Y41P, FCC('Y','4','1','P')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_Y41T, FCC('Y','4','1','T')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_Y42T, FCC('Y','4','2','T')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_P210, FCC('P','2','1','0')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_P216, FCC('P','2','1','6')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_P010, FCC('P','0','1','0')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_P016, FCC('P','0','1','6')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_v210, FCC('v','2','1','0')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_v216, FCC('v','2','1','6')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_v410, FCC('v','4','1','0')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_MP43, FCC('M','P','4','3')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_MP4S, FCC('M','P','4','S')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_M4S2, FCC('M','4','S','2')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_MP4V, FCC('M','P','4','V')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_WMV1, FCC('W','M','V','1')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_WMV2, FCC('W','M','V','2')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_WMV3, FCC('W','M','V','3')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_WVC1, FCC('W','V','C','1')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_MSS1, FCC('M','S','S','1')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_MSS2, FCC('M','S','S','2')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_MPG1, FCC('M','P','G','1')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_DVSL, FCC('d','v','s','l')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_DVSD, FCC('d','v','s','d')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_DVHD, FCC('d','v','h','d')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_DV25, FCC('d','v','2','5')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_DV50, FCC('d','v','5','0')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_DVH1, FCC('d','v','h','1')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_DVC, FCC('d','v','c',' ')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_H264, FCC('H','2','6','4')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_H265, FCC('H','2','6','5')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_MJPG, FCC('M','J','P','G')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_420O, FCC('4','2','0','O')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_HEVC, FCC('H','E','V','C')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_HEVC_ES, FCC('H','E','V','S')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_VP80, FCC('V','P','8','0')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_VP90, FCC('V','P','9','0')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_ORAW, FCC('O','R','A','W')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_H263, FCC('H','2','6','3')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_A2R10G10B10, D3DFMT_A2B10G10R10}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_A16B16G16R16F, D3DFMT_A16B16G16R16F}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_VP10, FCC('V','P','1','0')}
DEFINE_MEDIATYPE_GUID!{MFVideoFormat_AV1, FCC('A','V','0','1')}
DEFINE_GUID!{MF_MT_MAJOR_TYPE,
    0x48eba18e, 0xf8c9, 0x4687, 0xbf, 0x11, 0x0a, 0x74, 0xc9, 0xf9, 0x6a, 0x8f}
DEFINE_GUID!{MF_MT_SUBTYPE,
    0xf7e34c9a, 0x42e8, 0x4714, 0xb7, 0x4b, 0xcb, 0x29, 0xd7, 0x2c, 0x35, 0xe5}
DEFINE_GUID!{MF_MT_ALL_SAMPLES_INDEPENDENT,
    0xc9173739, 0x5e56, 0x461c, 0xb7, 0x13, 0x46, 0xfb, 0x99, 0x5c, 0xb9, 0x5f}
DEFINE_GUID!{MF_MT_FIXED_SIZE_SAMPLES,
    0xb8ebefaf, 0xb718, 0x4e04, 0xb0, 0xa9, 0x11, 0x67, 0x75, 0xe3, 0x32, 0x1b}
DEFINE_GUID!{MF_MT_COMPRESSED,
    0x3afd0cee, 0x18f2, 0x4ba5, 0xa1, 0x10, 0x8b, 0xea, 0x50, 0x2e, 0x1f, 0x92}
DEFINE_GUID!{MF_MT_SAMPLE_SIZE,
    0xdad3ab78, 0x1990, 0x408b, 0xbc, 0xe2, 0xeb, 0xa6, 0x73, 0xda, 0xcc, 0x10}
DEFINE_GUID!{MF_MT_WRAPPED_TYPE,
    0x4d3f7b23, 0xd02f, 0x4e6c, 0x9b, 0xee, 0xe4, 0xbf, 0x2c, 0x6c, 0x69, 0x5d}
DEFINE_GUID!{MF_MT_FRAME_SIZE,
    0x1652c33d, 0xd6b2, 0x4012, 0xb8, 0x34, 0x72, 0x03, 0x08, 0x49, 0xa3, 0x7d}
DEFINE_GUID!{MF_MT_FRAME_RATE,
    0xc459a2e8, 0x3d2c, 0x4e44, 0xb1, 0x32, 0xfe, 0xe5, 0x15, 0x6c, 0x7b, 0xb0}
DEFINE_GUID!{MF_MT_PIXEL_ASPECT_RATIO,
    0xc6376a1e, 0x8d0a, 0x4027, 0xbe, 0x45, 0x6d, 0x9a, 0x0a, 0xd3, 0x9b, 0xb6}
DEFINE_GUID!{MF_MT_DRM_FLAGS,
    0x8772f323, 0x355a, 0x4cc7, 0xbb, 0x78, 0x6d, 0x61, 0xa0, 0x48, 0xae, 0x82}
DEFINE_GUID!{MF_MT_AM_FORMAT_TYPE,
    0x73d1072d, 0x1870, 0x4174, 0xa0, 0x63, 0x29, 0xff, 0x4f, 0xf6, 0xc1, 0x1e}
