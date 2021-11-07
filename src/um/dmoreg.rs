// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::{c_ulong};
use shared::guiddef::{GUID, REFCLSID, REFGUID};
use shared::minwindef::{DWORD};
use um::mediaobj::{IEnumDMO};
use um::winnt::{HRESULT, LPCWSTR, WCHAR};
DEFINE_GUID!(DMOCATEGORY_AUDIO_DECODER,
    0x57f2db8b, 0xe6bb, 0x4513, 0x9d, 0x43, 0xdc, 0xd2, 0xa6, 0x59, 0x31, 0x25);
DEFINE_GUID!(DMOCATEGORY_AUDIO_ENCODER,
    0x33d9a761, 0x90c8, 0x11d0, 0xbd, 0x43, 0x00, 0xa0, 0xc9, 0x11, 0xce, 0x86);
DEFINE_GUID!(DMOCATEGORY_VIDEO_DECODER,
    0x4a69b442, 0x28be, 0x4991, 0x96, 0x9c, 0xb5, 0x00, 0xad, 0xf5, 0xd8, 0xa8);
DEFINE_GUID!(DMOCATEGORY_VIDEO_ENCODER,
    0x33d9a760, 0x90c8, 0x11d0, 0xbd, 0x43, 0x00, 0xa0, 0xc9, 0x11, 0xce, 0x86);
DEFINE_GUID!(DMOCATEGORY_AUDIO_EFFECT,
    0xf3602b3f, 0x0592, 0x48df, 0xa4, 0xcd, 0x67, 0x47, 0x21, 0xe7, 0xeb, 0xeb);
DEFINE_GUID!(DMOCATEGORY_VIDEO_EFFECT,
    0xd990ee14, 0x776c, 0x4723, 0xbe, 0x46, 0x3d, 0xa2, 0xf5, 0x6f, 0x10, 0xb9);
DEFINE_GUID!(DMOCATEGORY_AUDIO_CAPTURE_EFFECT,
    0xf665aaba, 0x3e09, 0x4920, 0xaa, 0x5f, 0x21, 0x98, 0x11, 0x14, 0x8f, 0x09);
DEFINE_GUID!(DMOCATEGORY_ACOUSTIC_ECHO_CANCEL,
    0xbf963d80, 0xc559, 0x11d0, 0x8a, 0x2b, 0x00, 0xa0, 0xc9, 0x25, 0x5a, 0xc1);
DEFINE_GUID!(DMOCATEGORY_AUDIO_NOISE_SUPPRESS,
    0xe07f903f, 0x62fd, 0x4e60, 0x8c, 0xdd, 0xde, 0xa7, 0x23, 0x66, 0x65, 0xb5);
DEFINE_GUID!(DMOCATEGORY_AGC,
    0xe88c9ba0, 0xc557, 0x11d0, 0x8a, 0x2b, 0x00, 0xa0, 0xc9, 0x25, 0x5a, 0xc1);
STRUCT!{struct DMO_PARTIAL_MEDIATYPE {
    type_: GUID,
    subtype: GUID,
}}
pub type PDMO_PARTIAL_MEDIATYPE = *mut DMO_PARTIAL_MEDIATYPE;
ENUM!{enum DMO_REGISTER_FLAGS {
    DMO_REGISTERF_IS_KEYED = 1,
}}
ENUM!{enum DMO_ENUM_FLAGS {
    DMO_ENUMF_INCLUDE_KEYED = 1,
}}
extern "system" {
    pub fn DMORegister(
        szName: LPCWSTR,
        clsidDMO: REFCLSID,
        guidCategory: REFGUID,
        dwFlags: DWORD,
        cInTypes: DWORD,
        pInTypes: *const DMO_PARTIAL_MEDIATYPE,
        cOutTypes: DWORD,
        pOutTypes: *const DMO_PARTIAL_MEDIATYPE,
    ) -> HRESULT;
    pub fn DMOUnregister(
        clsidDMO: REFCLSID,
        guidCategory: REFGUID,
    ) -> HRESULT;
    pub fn DMOEnum(
        guidCategory: REFGUID,
        dwFlags: DWORD,
        cInTypes: DWORD,
        pInTypes: *const DMO_PARTIAL_MEDIATYPE,
        cOutTypes: DWORD,
        pOutTypes: *const DMO_PARTIAL_MEDIATYPE,
        ppEnum: *mut *mut IEnumDMO,
    ) -> HRESULT;
    pub fn DMOGetTypes(
        clsidDMO: REFCLSID,
        ulInputTypesRequested: c_ulong,
        pulInputTypesSupplied: c_ulong,
        pInputTypes: *mut DMO_PARTIAL_MEDIATYPE,
        ulOutputTypesRequested: c_ulong,
        pulOutputTypesSupplied: c_ulong,
        pOutputTypes: *mut DMO_PARTIAL_MEDIATYPE,
    ) -> HRESULT;
    pub fn DMOGetName(
        clsidDMO: REFCLSID,
        szName: &mut [WCHAR; 80],
    ) -> HRESULT;
}
