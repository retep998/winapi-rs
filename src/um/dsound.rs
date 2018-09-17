// Copyright © 2015-2018 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! DSound procedure declarations, constant definitions and macros
use ctypes::{c_float};
use shared::basetsd::{DWORD_PTR};
use shared::d3d9types::{D3DVECTOR};
use shared::guiddef::{GUID, LPCGUID, LPGUID, REFGUID};
use shared::minwindef::{BOOL, BYTE, DWORD, FLOAT, LPDWORD, LPLONG, LPVOID};
use shared::windef::HWND;
use shared::winerror::{
    CLASS_E_NOAGGREGATION, E_ACCESSDENIED, E_FAIL, E_INVALIDARG, E_NOINTERFACE, E_NOTIMPL,
    E_OUTOFMEMORY, S_OK
};
use um::cguid::{GUID_NULL};
use um::mmsystem::{LPCWAVEFORMATEX, LPWAVEFORMATEX};
use um::unknwnbase::{IUnknown, IUnknownVtbl, LPUNKNOWN};
use um::winnt::{HANDLE, HRESULT, LONG, LPCSTR, LPCWSTR};
pub const DIRECTSOUND_VERSION: DWORD = 0x0900;
pub type D3DVALUE = c_float;
pub type LPD3DVALUE = *mut D3DVALUE;
pub type D3DCOLOR = DWORD;
pub type LPD3DCOLOR = *mut D3DCOLOR;
pub const _FACDS: HRESULT = 0x878;
DEFINE_GUID!{CLSID_DirectSound,
    0x47d4d946, 0x62e8, 0x11cf, 0x93, 0xbc, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
DEFINE_GUID!{CLSID_DirectSound8,
    0x3901cc3f, 0x84b5, 0x4fa4, 0xba, 0x35, 0xaa, 0x81, 0x72, 0xb8, 0xa0, 0x9b}
DEFINE_GUID!{CLSID_DirectSoundCapture,
    0xb0210780, 0x89cd, 0x11d0, 0xaf, 0x08, 0x00, 0xa0, 0xc9, 0x25, 0xcd, 0x16}
DEFINE_GUID!{CLSID_DirectSoundCapture8,
    0xe4bcac13, 0x7f99, 0x4908, 0x9a, 0x8e, 0x74, 0xe3, 0xbf, 0x24, 0xb6, 0xe1}
DEFINE_GUID!{CLSID_DirectSoundFullDuplex,
    0xfea4300c, 0x7959, 0x4147, 0xb2, 0x6a, 0x23, 0x77, 0xb9, 0xe7, 0xa9, 0x1d}
DEFINE_GUID!{DSDEVID_DefaultPlayback,
    0xdef00000, 0x9c6d, 0x47ed, 0xaa, 0xf1, 0x4d, 0xda, 0x8f, 0x2b, 0x5c, 0x03}
DEFINE_GUID!{DSDEVID_DefaultCapture,
    0xdef00001, 0x9c6d, 0x47ed, 0xaa, 0xf1, 0x4d, 0xda, 0x8f, 0x2b, 0x5c, 0x03}
DEFINE_GUID!{DSDEVID_DefaultVoicePlayback,
    0xdef00002, 0x9c6d, 0x47ed, 0xaa, 0xf1, 0x4d, 0xda, 0x8f, 0x2b, 0x5c, 0x03}
DEFINE_GUID!{DSDEVID_DefaultVoiceCapture,
    0xdef00003, 0x9c6d, 0x47ed, 0xaa, 0xf1, 0x4d, 0xda, 0x8f, 0x2b, 0x5c, 0x03}
pub type IDirectSoundCapture8 = IDirectSoundCapture;
pub type IDirectSound3DListener8 = IDirectSound3DListener;
pub type IDirectSound3DBuffer8 = IDirectSound3DBuffer;
pub type IDirectSoundNotify8 = IDirectSoundNotify;
pub type IDirectSoundFXGargle8 = IDirectSoundFXGargle;
pub type IDirectSoundFXChorus8 = IDirectSoundFXChorus;
pub type IDirectSoundFXFlanger8 = IDirectSoundFXFlanger;
pub type IDirectSoundFXEcho8 = IDirectSoundFXEcho;
pub type IDirectSoundFXDistortion8 = IDirectSoundFXDistortion;
pub type IDirectSoundFXCompressor8 = IDirectSoundFXCompressor;
pub type IDirectSoundFXParamEq8 = IDirectSoundFXParamEq;
pub type IDirectSoundFXWavesReverb8 = IDirectSoundFXWavesReverb;
pub type IDirectSoundFXI3DL2Reverb8 = IDirectSoundFXI3DL2Reverb;
pub type IDirectSoundCaptureFXAec8 = IDirectSoundCaptureFXAec;
pub type IDirectSoundCaptureFXNoiseSuppress8 = IDirectSoundCaptureFXNoiseSuppress;
pub type IDirectSoundFullDuplex8 = IDirectSoundFullDuplex;
pub type LPDIRECTSOUND = *mut IDirectSound;
pub type LPDIRECTSOUNDBUFFER = *mut IDirectSoundBuffer;
pub type LPDIRECTSOUND3DLISTENER = *mut IDirectSound3DListener;
pub type LPDIRECTSOUND3DBUFFER = *mut IDirectSound3DBuffer;
pub type LPDIRECTSOUNDCAPTURE = *mut IDirectSoundCapture;
pub type LPDIRECTSOUNDCAPTUREBUFFER = *mut IDirectSoundCaptureBuffer;
pub type LPDIRECTSOUNDNOTIFY = *mut IDirectSoundNotify;
pub type LPDIRECTSOUNDFXGARGLE = *mut IDirectSoundFXGargle;
pub type LPDIRECTSOUNDFXCHORUS = *mut IDirectSoundFXChorus;
pub type LPDIRECTSOUNDFXFLANGER = *mut IDirectSoundFXFlanger;
pub type LPDIRECTSOUNDFXECHO = *mut IDirectSoundFXEcho;
pub type LPDIRECTSOUNDFXDISTORTION = *mut IDirectSoundFXDistortion;
pub type LPDIRECTSOUNDFXCOMPRESSOR = *mut IDirectSoundFXCompressor;
pub type LPDIRECTSOUNDFXPARAMEQ = *mut IDirectSoundFXParamEq;
pub type LPDIRECTSOUNDFXWAVESREVERB = *mut IDirectSoundFXWavesReverb;
pub type LPDIRECTSOUNDFXI3DL2REVERB = *mut IDirectSoundFXI3DL2Reverb;
pub type LPDIRECTSOUNDCAPTUREFXAEC = *mut IDirectSoundCaptureFXAec;
pub type LPDIRECTSOUNDCAPTUREFXNOISESUPPRESS = *mut IDirectSoundCaptureFXNoiseSuppress;
pub type LPDIRECTSOUNDFULLDUPLEX = *mut IDirectSoundFullDuplex;
pub type LPDIRECTSOUND8 = *mut IDirectSound8;
pub type LPDIRECTSOUNDBUFFER8 = *mut IDirectSoundBuffer8;
pub type LPDIRECTSOUND3DLISTENER8 = *mut IDirectSound3DListener8;
pub type LPDIRECTSOUND3DBUFFER8 = *mut IDirectSound3DBuffer8;
pub type LPDIRECTSOUNDCAPTURE8 = *mut IDirectSoundCapture8;
pub type LPDIRECTSOUNDCAPTUREBUFFER8 = *mut IDirectSoundCaptureBuffer8;
pub type LPDIRECTSOUNDNOTIFY8 = *mut IDirectSoundNotify8;
pub type LPDIRECTSOUNDFXGARGLE8 = *mut IDirectSoundFXGargle8;
pub type LPDIRECTSOUNDFXCHORUS8 = *mut IDirectSoundFXChorus8;
pub type LPDIRECTSOUNDFXFLANGER8 = *mut IDirectSoundFXFlanger8;
pub type LPDIRECTSOUNDFXECHO8 = *mut IDirectSoundFXEcho8;
pub type LPDIRECTSOUNDFXDISTORTION8 = *mut IDirectSoundFXDistortion8;
pub type LPDIRECTSOUNDFXCOMPRESSOR8 = *mut IDirectSoundFXCompressor8;
pub type LPDIRECTSOUNDFXPARAMEQ8 = *mut IDirectSoundFXParamEq8;
pub type LPDIRECTSOUNDFXWAVESREVERB8 = *mut IDirectSoundFXWavesReverb8;
pub type LPDIRECTSOUNDFXI3DL2REVERB8 = *mut IDirectSoundFXI3DL2Reverb8;
pub type LPDIRECTSOUNDCAPTUREFXAEC8 = *mut IDirectSoundCaptureFXAec8;
pub type LPDIRECTSOUNDCAPTUREFXNOISESUPPRESS8 = *mut IDirectSoundCaptureFXNoiseSuppress8;
pub type LPDIRECTSOUNDFULLDUPLEX8 = *mut IDirectSoundFullDuplex8;
pub const IID_IDirectSoundCapture8: GUID = IID_IDirectSoundCapture;
pub const IID_IDirectSound3DListener8: GUID = IID_IDirectSound3DListener;
pub const IID_IDirectSound3DBuffer8: GUID = IID_IDirectSound3DBuffer;
pub const IID_IDirectSoundNotify8: GUID = IID_IDirectSoundNotify;
pub const IID_IDirectSoundFXGargle8: GUID = IID_IDirectSoundFXGargle;
pub const IID_IDirectSoundFXChorus8: GUID = IID_IDirectSoundFXChorus;
pub const IID_IDirectSoundFXFlanger8: GUID = IID_IDirectSoundFXFlanger;
pub const IID_IDirectSoundFXEcho8: GUID = IID_IDirectSoundFXEcho;
pub const IID_IDirectSoundFXDistortion8: GUID = IID_IDirectSoundFXDistortion;
pub const IID_IDirectSoundFXCompressor8: GUID = IID_IDirectSoundFXCompressor;
pub const IID_IDirectSoundFXParamEq8: GUID = IID_IDirectSoundFXParamEq;
pub const IID_IDirectSoundFXWavesReverb8: GUID = IID_IDirectSoundFXWavesReverb;
pub const IID_IDirectSoundFXI3DL2Reverb8: GUID = IID_IDirectSoundFXI3DL2Reverb;
pub const IID_IDirectSoundCaptureFXAec8: GUID = IID_IDirectSoundCaptureFXAec;
pub const IID_IDirectSoundCaptureFXNoiseSuppress8: GUID = IID_IDirectSoundCaptureFXNoiseSuppress;
pub const IID_IDirectSoundFullDuplex8: GUID = IID_IDirectSoundFullDuplex;
pub type LPLPDIRECTSOUND = *mut LPDIRECTSOUND;
pub type LPLPDIRECTSOUNDBUFFER = *mut LPDIRECTSOUNDBUFFER;
pub type LPLPDIRECTSOUND3DLISTENER = *mut LPDIRECTSOUND3DLISTENER;
pub type LPLPDIRECTSOUND3DBUFFER = *mut LPDIRECTSOUND3DBUFFER;
pub type LPLPDIRECTSOUNDCAPTURE = *mut LPDIRECTSOUNDCAPTURE;
pub type LPLPDIRECTSOUNDCAPTUREBUFFER = *mut LPDIRECTSOUNDCAPTUREBUFFER;
pub type LPLPDIRECTSOUNDNOTIFY = *mut LPDIRECTSOUNDNOTIFY;
pub type LPLPDIRECTSOUND8 = *mut LPDIRECTSOUND8;
pub type LPLPDIRECTSOUNDBUFFER8 = *mut LPDIRECTSOUNDBUFFER8;
pub type LPLPDIRECTSOUNDCAPTURE8 = *mut LPDIRECTSOUNDCAPTURE8;
pub type LPLPDIRECTSOUNDCAPTUREBUFFER8 = *mut LPDIRECTSOUNDCAPTUREBUFFER8;
STRUCT!{struct DSCAPS {
    dwSize: DWORD,
    dwFlags: DWORD,
    dwMinSecondarySampleRate: DWORD,
    dwMaxSecondarySampleRate: DWORD,
    dwPrimaryBuffers: DWORD,
    dwMaxHwMixingAllBuffers: DWORD,
    dwMaxHwMixingStaticBuffers: DWORD,
    dwMaxHwMixingStreamingBuffers: DWORD,
    dwFreeHwMixingAllBuffers: DWORD,
    dwFreeHwMixingStaticBuffers: DWORD,
    dwFreeHwMixingStreamingBuffers: DWORD,
    dwMaxHw3DAllBuffers: DWORD,
    dwMaxHw3DStaticBuffers: DWORD,
    dwMaxHw3DStreamingBuffers: DWORD,
    dwFreeHw3DAllBuffers: DWORD,
    dwFreeHw3DStaticBuffers: DWORD,
    dwFreeHw3DStreamingBuffers: DWORD,
    dwTotalHwMemBytes: DWORD,
    dwFreeHwMemBytes: DWORD,
    dwMaxContigFreeHwMemBytes: DWORD,
    dwUnlockTransferRateHwBuffers: DWORD,
    dwPlayCpuOverheadSwBuffers: DWORD,
    dwReserved1: DWORD,
    dwReserved2: DWORD,
}}
pub type LPDSCAPS = *mut DSCAPS;
pub type LPCDSCAPS = *const DSCAPS;
STRUCT!{struct DSBCAPS {
    dwSize: DWORD,
    dwFlags: DWORD,
    dwBufferBytes: DWORD,
    dwUnlockTransferRate: DWORD,
    dwPlayCpuOverhead: DWORD,
}}
pub type LPDSBCAPS = *mut DSBCAPS;
pub type LPCDSBCAPS = *const DSBCAPS;
STRUCT!{struct DSEFFECTDESC {
    dwSize: DWORD,
    dwFlags: DWORD,
    guidDSFXClass: GUID,
    dwReserved1: DWORD_PTR,
    dwReserved2: DWORD_PTR,
}}
pub type LPDSEFFECTDESC = *mut DSEFFECTDESC;
pub type LPCDSEFFECTDESC = *const DSEFFECTDESC;
pub const DSFX_LOCHARDWARE: DWORD = 0x00000001;
pub const DSFX_LOCSOFTWARE: DWORD = 0x00000002;
pub const DSFXR_PRESENT: DWORD = 0;
pub const DSFXR_LOCHARDWARE: DWORD = 1;
pub const DSFXR_LOCSOFTWARE: DWORD = 2;
pub const DSFXR_UNALLOCATED: DWORD = 3;
pub const DSFXR_FAILED: DWORD = 4;
pub const DSFXR_UNKNOWN: DWORD = 5;
pub const DSFXR_SENDLOOP: DWORD = 6;
STRUCT!{struct DSCEFFECTDESC {
    dwSize: DWORD,
    dwFlags: DWORD,
    guidDSCFXClass: GUID,
    guidDSCFXInstance: GUID,
    dwReserved1: DWORD,
    dwReserved2: DWORD,
}}
pub type LPDSCEFFECTDESC = *mut DSCEFFECTDESC;
pub type LPCDSCEFFECTDESC = *const DSCEFFECTDESC;
pub const DSCFX_LOCHARDWARE: DWORD = 0x00000001;
pub const DSCFX_LOCSOFTWARE: DWORD = 0x00000002;
pub const DSCFXR_LOCHARDWARE: DWORD = 0x00000010;
pub const DSCFXR_LOCSOFTWARE: DWORD = 0x00000020;
STRUCT!{struct DSBUFFERDESC {
    dwSize: DWORD,
    dwFlags: DWORD,
    dwBufferBytes: DWORD,
    dwReserved: DWORD,
    lpwfxFormat: LPWAVEFORMATEX,
    guid3DAlgorithm: GUID,
}}
pub type LPDSBUFFERDESC = *mut DSBUFFERDESC;
pub type LPCDSBUFFERDESC = *const DSBUFFERDESC;
STRUCT!{struct DSBUFFERDESC1 {
    dwSize: DWORD,
    dwFlags: DWORD,
    dwBufferBytes: DWORD,
    dwReserved: DWORD,
    lpwfxFormat: LPWAVEFORMATEX,
}}
pub type LPDSBUFFERDESC1 = *mut DSBUFFERDESC1;
pub type LPCDSBUFFERDESC1 = *const DSBUFFERDESC1;
STRUCT!{struct DS3DBUFFER {
    dwSize: DWORD,
    vPosition: D3DVECTOR,
    vVelocity: D3DVECTOR,
    dwInsideConeAngle: DWORD,
    dwOutsideConeAngle: DWORD,
    vConeOrientation: D3DVECTOR,
    lConeOutsideVolume: LONG,
    flMinDistance: D3DVALUE,
    flMaxDistance: D3DVALUE,
    dwMode: DWORD,
}}
pub type LPDS3DBUFFER = *mut DS3DBUFFER;
pub type LPCDS3DBUFFER = *const DS3DBUFFER;
STRUCT!{struct DS3DLISTENER {
    dwSize: DWORD,
    vPosition: D3DVECTOR,
    vVelocity: D3DVECTOR,
    vOrientFront: D3DVECTOR,
    vOrientTop: D3DVECTOR,
    flDistanceFactor: D3DVALUE,
    flRolloffFactor: D3DVALUE,
    flDopplerFactor: D3DVALUE,
}}
pub type LPDS3DLISTENER = *mut DS3DLISTENER;
pub type LPCDS3DLISTENER = *const DS3DLISTENER;
STRUCT!{struct DSCCAPS {
    dwSize: DWORD,
    dwFlags: DWORD,
    dwFormats: DWORD,
    dwChannels: DWORD,
}}
pub type LPDSCCAPS = *mut DSCCAPS;
pub type LPCDSCCAPS = *const DSCCAPS;
STRUCT!{struct DSCBUFFERDESC1 {
    dwSize: DWORD,
    dwFlags: DWORD,
    dwBufferBytes: DWORD,
    dwReserved: DWORD,
    lpwfxFormat: LPWAVEFORMATEX,
}}
pub type LPDSCBUFFERDESC1 = *mut DSCBUFFERDESC1;
STRUCT!{struct DSCBUFFERDESC {
    dwSize: DWORD,
    dwFlags: DWORD,
    dwBufferBytes: DWORD,
    dwReserved: DWORD,
    lpwfxFormat: LPWAVEFORMATEX,
    dwFXCount: DWORD,
    lpDSCFXDesc: LPDSCEFFECTDESC,
}}
pub type LPDSCBUFFERDESC = *mut DSCBUFFERDESC;
pub type LPCDSCBUFFERDESC = *const DSCBUFFERDESC;
STRUCT!{struct DSCBCAPS {
    dwSize: DWORD,
    dwFlags: DWORD,
    dwBufferBytes: DWORD,
    dwReserved: DWORD,
}}
pub type LPDSCBCAPS = *mut DSCBCAPS;
pub type LPCDSCBCAPS = *const DSCBCAPS;
STRUCT!{struct DSBPOSITIONNOTIFY {
    dwOffset: DWORD,
    hEventNotify: HANDLE,
}}
pub type LPDSBPOSITIONNOTIFY = *mut DSBPOSITIONNOTIFY;
pub type LPCDSBPOSITIONNOTIFY = *const DSBPOSITIONNOTIFY;
FN!{stdcall LPDSENUMCALLBACKA(
    lpGuid: LPGUID,
    lpcstrDescription: LPCSTR,
    lpcstrModule: LPCSTR,
    lpContext: LPVOID,
) -> BOOL}
FN!{stdcall LPDSENUMCALLBACKW(
    lpGuid: LPGUID,
    lpcstrDescription: LPCWSTR,
    lpcstrModule: LPCWSTR,
    lpContext: LPVOID,
) -> BOOL}
extern "system" {
    pub fn DirectSoundCreate(
        pcGuidDevice: LPCGUID,
        ppDS: *mut LPDIRECTSOUND,
        pUnkOuter: LPUNKNOWN,
    ) -> HRESULT;
    pub fn DirectSoundEnumerateA(
        pDSEnumCallback: LPDSENUMCALLBACKA,
        pContext: LPVOID,
    ) -> HRESULT;
    pub fn DirectSoundEnumerateW(
        pDSEnumCallback: LPDSENUMCALLBACKW,
        pContext: LPVOID,
    ) -> HRESULT;
    pub fn DirectSoundCaptureCreate(
        pcGuidDevice: LPCGUID,
        ppDSC: *mut LPDIRECTSOUNDCAPTURE,
        pUnkOuter: LPUNKNOWN,
    ) -> HRESULT;
    pub fn DirectSoundCaptureEnumerateA(
        pDSEnumCallback: LPDSENUMCALLBACKA,
        pContext: LPVOID,
    ) -> HRESULT;
    pub fn DirectSoundCaptureEnumerateW(
        pDSEnumCallback: LPDSENUMCALLBACKW,
        pContext: LPVOID,
    ) -> HRESULT;
    pub fn DirectSoundCreate8(
        pcGuidDevice: LPCGUID,
        ppDS8: *mut LPDIRECTSOUND8,
        pUnkOuter: LPUNKNOWN,
    ) -> HRESULT;
    pub fn DirectSoundCaptureCreate8(
        pcGuidDevice: LPCGUID,
        ppDSC8: *mut LPDIRECTSOUNDCAPTURE8,
        pUnkOuter: LPUNKNOWN,
    ) -> HRESULT;
    pub fn DirectSoundFullDuplexCreate(
        pcGuidCaptureDevice: LPCGUID,
        pcGuidRenderDevice: LPCGUID,
        pcDSCBufferDesc: LPCDSCBUFFERDESC,
        pcDSBufferDesc: LPCDSBUFFERDESC,
        hWnd: HWND,
        dwLevel: DWORD,
        ppDSFD: *mut LPDIRECTSOUNDFULLDUPLEX,
        ppDSCBuffer8: *mut LPDIRECTSOUNDCAPTUREBUFFER8,
        ppDSBuffer8: *mut LPDIRECTSOUNDBUFFER8,
        pUnkOuter: LPUNKNOWN,
    ) -> HRESULT;
    pub fn GetDeviceID(
        pGuidSrc: LPCGUID,
        pGuidDest: LPGUID,
    ) -> HRESULT;
}
#[inline]
pub unsafe fn DirectSoundFullDuplexCreate8(
    pcGuidCaptureDevice: LPCGUID,
    pcGuidRenderDevice: LPCGUID,
    pcDSCBufferDesc: LPCDSCBUFFERDESC,
    pcDSBufferDesc: LPCDSBUFFERDESC,
    hWnd: HWND,
    dwLevel: DWORD,
    ppDSFD: *mut LPDIRECTSOUNDFULLDUPLEX,
    ppDSCBuffer8: *mut LPDIRECTSOUNDCAPTUREBUFFER8,
    ppDSBuffer8: *mut LPDIRECTSOUNDBUFFER8,
    pUnkOuter: LPUNKNOWN,
) -> HRESULT {
    DirectSoundFullDuplexCreate(
        pcGuidCaptureDevice,
        pcGuidRenderDevice,
        pcDSCBufferDesc,
        pcDSBufferDesc,
        hWnd,
        dwLevel,
        ppDSFD,
        ppDSCBuffer8,
        ppDSBuffer8,
        pUnkOuter,
    )
}
DEFINE_GUID!{IID_IReferenceClock,
    0x56a86897, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70}
DEFINE_GUID!{IID_IDirectSound,
    0x279afa83, 0x4981, 0x11ce, 0xa5, 0x21, 0x00, 0x20, 0xaf, 0x0b, 0xe5, 0x60}
RIDL!{#[uuid(0x279afa83, 0x4981, 0x11ce, 0xa5, 0x21, 0x00, 0x20, 0xaf, 0x0b, 0xe5, 0x60)]
interface IDirectSound(IDirectSoundVtbl): IUnknown(IUnknownVtbl) {
    fn CreateSoundBuffer(
        pcDSBufferDesc: LPCDSBUFFERDESC,
        ppDSBuffer: *mut LPDIRECTSOUNDBUFFER,
        pUnkOuter: LPUNKNOWN,
    ) -> HRESULT,
    fn GetCaps(
        pDSCaps: LPDSCAPS,
    ) -> HRESULT,
    fn DuplicateSoundBuffer(
        pDSBufferOriginal: LPDIRECTSOUNDBUFFER,
        ppDSBufferDuplicate: *mut LPDIRECTSOUNDBUFFER,
    ) -> HRESULT,
    fn SetCooperativeLevel(
        hWnd: HWND,
        dwLevel: DWORD,
    ) -> HRESULT,
    fn Compact() -> HRESULT,
    fn GetSpeakerConfig(
        pdwSpeakerConfig: LPDWORD,
    ) -> HRESULT,
    fn SetSpeakerConfig(
        dwSpeakerConfig: DWORD,
    ) -> HRESULT,
    fn Initialize(
        pcGuidDevice: LPCGUID,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IDirectSound8,
    0xc50a7e93, 0xf395, 0x4834, 0x9e, 0xf6, 0x7f, 0xa9, 0x9d, 0xe5, 0x09, 0x66}
RIDL!{#[uuid(0xc50a7e93, 0xf395, 0x4834, 0x9e, 0xf6, 0x7f, 0xa9, 0x9d, 0xe5, 0x09, 0x66)]
interface IDirectSound8(IDirectSound8Vtbl): IDirectSound(IDirectSoundVtbl) {
    fn VerifyCertification(
        pdwCertified: LPDWORD,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IDirectSoundBuffer,
    0x279afa85, 0x4981, 0x11ce, 0xa5, 0x21, 0x00, 0x20, 0xaf, 0x0b, 0xe5, 0x60}
RIDL!{#[uuid(0x279afa85, 0x4981, 0x11ce, 0xa5, 0x21, 0x00, 0x20, 0xaf, 0x0b, 0xe5, 0x60)]
interface IDirectSoundBuffer(IDirectSoundBufferVtbl): IUnknown(IUnknownVtbl) {
    fn GetCaps(
        pDSBufferCaps: LPDSBCAPS,
    ) -> HRESULT,
    fn GetCurrentPosition(
        pdwCurrentPlayCursor: LPDWORD,
        pdwCurrentWriteCursor: LPDWORD,
    ) -> HRESULT,
    fn GetFormat(
        pwfxFormat: LPWAVEFORMATEX,
        dwSizeAllocated: DWORD,
        pdwSizeWritten: LPDWORD,
    ) -> HRESULT,
    fn GetVolume(
        plVolume: LPLONG,
    ) -> HRESULT,
    fn GetPan(
        plPan: LPLONG,
    ) -> HRESULT,
    fn GetFrequency(
        pdwFrequency: LPDWORD,
    ) -> HRESULT,
    fn GetStatus(
        pdwStatus: LPDWORD,
    ) -> HRESULT,
    fn Initialize(
        pDirectSound: LPDIRECTSOUND,
        pcDSBufferDesc: LPCDSBUFFERDESC,
    ) -> HRESULT,
    fn Lock(
        dwOffset: DWORD,
        dwBytes: DWORD,
        ppvAudioPtr1: *mut LPVOID,
        pdwAudioBytes1: LPDWORD,
        ppvAudioPtr2: *mut LPVOID,
        pdwAudioBytes2: LPDWORD,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn Play(
        dwReserved1: DWORD,
        dwPriority: DWORD,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn SetCurrentPosition(
        dwNewPosition: DWORD,
    ) -> HRESULT,
    fn SetFormat(
        pcfxFormat: LPCWAVEFORMATEX,
    ) -> HRESULT,
    fn SetVolume(
        lVolume: LONG,
    ) -> HRESULT,
    fn SetPan(
        lPan: LONG,
    ) -> HRESULT,
    fn SetFrequency(
        dwFrequency: DWORD,
    ) -> HRESULT,
    fn Stop() -> HRESULT,
    fn Unlock(
        pvAudioPtr1: LPVOID,
        dwAudioBytes1: DWORD,
        pvAudioPtr2: LPVOID,
        dwAudioBytes2: DWORD,
    ) -> HRESULT,
    fn Restore() -> HRESULT,
}}
DEFINE_GUID!{IID_IDirectSoundBuffer8,
    0x6825a449, 0x7524, 0x4d82, 0x92, 0x0f, 0x50, 0xe3, 0x6a, 0xb3, 0xab, 0x1e}
RIDL!{#[uuid(0x6825a449, 0x7524, 0x4d82, 0x92, 0x0f, 0x50, 0xe3, 0x6a, 0xb3, 0xab, 0x1e)]
interface IDirectSoundBuffer8(IDirectSoundBuffer8Vtbl): 
    IDirectSoundBuffer(IDirectSoundBufferVtbl) {
    fn SetFX(
        dwEffectsCount: DWORD,
        pDSFXDesc: LPDSEFFECTDESC,
        pdwResultCodes: LPDWORD,
    ) -> HRESULT,
    fn AcquireResources(
        dwFlags: DWORD,
        dwEffectsCount: DWORD,
        pdwResultCodes: LPDWORD,
    ) -> HRESULT,
    fn GetObjectInPath(
        rguidObject: REFGUID,
        dwIndex: DWORD,
        rguidInterface: REFGUID,
        ppObject: *mut LPVOID,
    ) -> HRESULT,
}}
DEFINE_GUID!{GUID_All_Objects,
    0xaa114de5, 0xc262, 0x4169, 0xa1, 0xc8, 0x23, 0xd6, 0x98, 0xcc, 0x73, 0xb5}
DEFINE_GUID!{IID_IDirectSound3DListener,
    0x279afa84, 0x4981, 0x11ce, 0xa5, 0x21, 0x00, 0x20, 0xaf, 0x0b, 0xe5, 0x60}
RIDL!{#[uuid(0x279afa84, 0x4981, 0x11ce, 0xa5, 0x21, 0x00, 0x20, 0xaf, 0x0b, 0xe5, 0x60)]
interface IDirectSound3DListener(IDirectSound3DListenerVtbl): IUnknown(IUnknownVtbl) {
   fn GetAllParameters(
        pListener: LPDS3DLISTENER,
    ) -> HRESULT,
    fn GetDistanceFactor(
        pflDistanceFactor: *mut D3DVALUE,
    ) -> HRESULT,
    fn GetDopplerFactor(
        pflDopplerFactor: *mut D3DVALUE,
    ) -> HRESULT,
    fn GetOrientation(
        pvOrientFront: *mut D3DVECTOR,
        pvOrientTop: *mut D3DVECTOR,
    ) -> HRESULT,
    fn GetPosition(
        pvPosition: *mut D3DVECTOR,
    ) -> HRESULT,
    fn GetRolloffFactor(
        pflRolloffFactor: *mut D3DVALUE,
    ) -> HRESULT,
    fn GetVelocity(
        pvVelocity: *mut D3DVECTOR,
    ) -> HRESULT,
    fn SetAllParameters(
        pcListener: LPCDS3DLISTENER,
        dwApply: DWORD,
    ) -> HRESULT,
    fn SetDistanceFactor(
        flDistanceFactor: D3DVALUE,
        dwApply: DWORD,
    ) -> HRESULT,
    fn SetDopplerFactor(
        flDopplerFactor: D3DVALUE,
        dwApply: DWORD,
    ) -> HRESULT,
    fn SetOrientation(
        xFront: D3DVALUE,
        yFront: D3DVALUE,
        zFront: D3DVALUE,
        xTop: D3DVALUE,
        yTop: D3DVALUE,
        zTop: D3DVALUE,
        dwApply: DWORD,
    ) -> HRESULT,
    fn SetPosition(
        x: D3DVALUE,
        y: D3DVALUE,
        z: D3DVALUE,
        dwApply: DWORD,
    ) -> HRESULT,
    fn SetRolloffFactor(
        flRolloffFactor: D3DVALUE,
        dwApply: DWORD,
    ) -> HRESULT,
    fn SetVelocity(
        x: D3DVALUE,
        y: D3DVALUE,
        z: D3DVALUE,
        dwApply: DWORD,
    ) -> HRESULT,
    fn CommitDeferredSettings() -> HRESULT,
}}
DEFINE_GUID!{IID_IDirectSound3DBuffer,
    0x279afa86, 0x4981, 0x11ce, 0xa5, 0x21, 0x00, 0x20, 0xaf, 0x0b, 0xe5, 0x60}
RIDL!{#[uuid(0x279afa86, 0x4981, 0x11ce, 0xa5, 0x21, 0x00, 0x20, 0xaf, 0x0b, 0xe5, 0x60)]
interface IDirectSound3DBuffer(IDirectSound3DBufferVtbl): IUnknown(IUnknownVtbl) {
    fn GetAllParameters(
        pDs3dBuffer: LPDS3DBUFFER,
    ) -> HRESULT,
    fn GetConeAngles(
        pdwInsideConeAngle: LPDWORD,
        pdwOutsideConeAngle: LPDWORD,
    ) -> HRESULT,
    fn GetConeOrientation(
        pvOrientation: *mut D3DVECTOR,
    ) -> HRESULT,
    fn GetConeOutsideVolume(
        plConeOutsideVolume: LPLONG,
    ) -> HRESULT,
    fn GetMaxDistance(
        pflMaxDistance: *mut D3DVALUE,
    ) -> HRESULT,
    fn GetMinDistance(
        pflMinDistance: *mut D3DVALUE,
    ) -> HRESULT,
    fn GetMode(
        pdwMode: LPDWORD,
    ) -> HRESULT,
    fn GetPosition(
        pvPosition: *mut D3DVECTOR,
    ) -> HRESULT,
    fn GetVelocity(
        pvVelocity: *mut D3DVECTOR,
    ) -> HRESULT,
    fn SetAllParameters(
        pcDs3dBuffer: LPCDS3DBUFFER,
        dwApply: DWORD,
    ) -> HRESULT,
    fn SetConeAngles(
        dwInsideConeAngle: DWORD,
        dwOutsideConeAngle: DWORD,
        dwApply: DWORD,
    ) -> HRESULT,
    fn SetConeOrientation(
        x: D3DVALUE,
        y: D3DVALUE,
        z: D3DVALUE,
        dwApply: DWORD,
    ) -> HRESULT,
    fn SetConeOutsideVolume(
        lConeOutsideVolume: LONG,
        dwApply: DWORD,
    ) -> HRESULT,
    fn SetMaxDistance(
        flMaxDistance: D3DVALUE,
        dwApply: DWORD,
    ) -> HRESULT,
    fn SetMinDistance(
        flMinDistance: D3DVALUE, 
        dwApply: DWORD,
    ) -> HRESULT,
    fn SetMode(
        dwMode: DWORD,
        dwApply: DWORD,
    ) -> HRESULT,
    fn SetPosition(
        x: D3DVALUE,
        y: D3DVALUE,
        z: D3DVALUE,
        dwApply: DWORD,
    ) -> HRESULT,
    fn SetVelocity(
        x: D3DVALUE,
        y: D3DVALUE,
        z: D3DVALUE,
        dwApply: DWORD,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IDirectSoundCapture,
    0xb0210781, 0x89cd, 0x11d0, 0xaf, 0x08, 0x00, 0xa0, 0xc9, 0x25, 0xcd, 0x16}
RIDL!{#[uuid(0xb0210781, 0x89cd, 0x11d0, 0xaf, 0x08, 0x00, 0xa0, 0xc9, 0x25, 0xcd, 0x16)]
interface IDirectSoundCapture(IDirectSoundCaptureVtbl): IUnknown(IUnknownVtbl) {
    fn CreateCaptureBuffer(
        pcDSCBufferDesc: LPCDSCBUFFERDESC, 
        ppDSCBuffer: *mut LPDIRECTSOUNDCAPTUREBUFFER, 
        pUnkOuter: LPUNKNOWN,
    ) -> HRESULT,
    fn GetCaps(
        pDSCCaps: LPDSCCAPS,
    ) -> HRESULT,
    fn Initialize(
        pcGuidDevice: LPCGUID,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IDirectSoundCaptureBuffer,
    0xb0210782, 0x89cd, 0x11d0, 0xaf, 0x08, 0x00, 0xa0, 0xc9, 0x25, 0xcd, 0x16}
RIDL!{#[uuid(0xb0210782, 0x89cd, 0x11d0, 0xaf, 0x08, 0x00, 0xa0, 0xc9, 0x25, 0xcd, 0x16)]
interface IDirectSoundCaptureBuffer(IDirectSoundCaptureBufferVtbl): IUnknown(IUnknownVtbl) {
    fn GetCaps(
        pDSCBCaps: LPDSCBCAPS,
    ) -> HRESULT,
    fn GetCurrentPosition(
        pdwCapturePosition: LPDWORD, 
        pdwReadPosition: LPDWORD,
    ) -> HRESULT,
    fn GetFormat(
        pwfxFormat: LPWAVEFORMATEX, 
        dwSizeAllocated: DWORD, 
        pdwSizeWritten: LPDWORD,
    ) -> HRESULT,
    fn GetStatus(
        pdwStatus: LPDWORD,
    ) -> HRESULT,
    fn Initialize(
        pDirectSoundCapture: LPDIRECTSOUNDCAPTURE,
        pcDSCBufferDesc: LPCDSCBUFFERDESC,
    ) -> HRESULT,
    fn Lock(
        dwOffset: DWORD,
        dwBytes: DWORD,
        ppvAudioPtr1: *mut LPVOID,
        pdwAudioBytes1: LPDWORD,
        ppvAudioPtr2: *mut LPVOID,
        pdwAudioBytes2: LPDWORD,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn Start(
        dwFlags: DWORD,
    ) -> HRESULT,
    fn Stop() -> HRESULT,
    fn Unlock(
        pvAudioPtr1: LPVOID,
        dwAudioBytes1: DWORD,
        pvAudioPtr2: LPVOID,
        dwAudioBytes2: DWORD,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IDirectSoundCaptureBuffer8,
    0x00990df4, 0x0dbb, 0x4872, 0x83, 0x3e, 0x6d, 0x30, 0x3e, 0x80, 0xae, 0xb6}
RIDL!{#[uuid(0x00990df4, 0x0dbb, 0x4872, 0x83, 0x3e, 0x6d, 0x30, 0x3e, 0x80, 0xae, 0xb6)]
interface IDirectSoundCaptureBuffer8(IDirectSoundCaptureBuffer8Vtbl): 
    IDirectSoundCaptureBuffer(IDirectSoundCaptureBufferVtbl) {
    fn GetObjectInPath(
        rguidObject: REFGUID,
        dwIndex: DWORD,
        rguidInterface: REFGUID,
        ppObject: LPVOID,
    ) -> HRESULT,
    fn GetFXStatus(
        dwEffectsCount: DWORD,
        pdwFXStatus: LPDWORD,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IDirectSoundNotify,
    0xb0210783, 0x89cd, 0x11d0, 0xaf, 0x08, 0x00, 0xa0, 0xc9, 0x25, 0xcd, 0x16}
RIDL!{#[uuid(0xb0210783, 0x89cd, 0x11d0, 0xaf, 0x08, 0x00, 0xa0, 0xc9, 0x25, 0xcd, 0x16)]
interface IDirectSoundNotify(IDirectSoundNotifyVtbl): IUnknown(IUnknownVtbl) {
    fn SetNotificationPositions(
        dwPositionNotifies: DWORD,
        pcPositionNotifies: LPCDSBPOSITIONNOTIFY,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IKsPropertySet,
    0x31efac30, 0x515c, 0x11d0, 0xa9, 0xaa, 0x00, 0xaa, 0x00, 0x61, 0xbe, 0x93}
DEFINE_GUID!{IID_IDirectSoundFXGargle,
    0xd616f352, 0xd622, 0x11ce, 0xaa, 0xc5, 0x00, 0x20, 0xaf, 0x0b, 0x99, 0xa3}
STRUCT!{struct DSFXGargle {
    dwRateHz: DWORD,
    dwWaveShape: DWORD,
}}
pub type LPDSFXGargle = *mut DSFXGargle;
pub const DSFXGARGLE_WAVE_TRIANGLE: DWORD = 0;
pub const DSFXGARGLE_WAVE_SQUARE: DWORD = 1;
pub type LPCDSFXGargle = *const DSFXGargle;
pub const DSFXGARGLE_RATEHZ_MIN: DWORD = 1;
pub const DSFXGARGLE_RATEHZ_MAX: DWORD = 1000;
RIDL!{#[uuid(0xd616f352, 0xd622, 0x11ce, 0xaa, 0xc5, 0x00, 0x20, 0xaf, 0x0b, 0x99, 0xa3)]
interface IDirectSoundFXGargle(IDirectSoundFXGargleVtbl): IUnknown(IUnknownVtbl) {
    fn SetAllParameters(
        pcDsFxGargle: LPCDSFXGargle,
    ) -> HRESULT,
    fn GetAllParameters(
        pDsFxGargle: LPDSFXGargle,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IDirectSoundFXChorus,
    0x880842e3, 0x145f, 0x43e6, 0xa9, 0x34, 0xa7, 0x18, 0x06, 0xe5, 0x05, 0x47}
STRUCT!{struct DSFXChorus {
    fWetDryMix: FLOAT,
    fDepth: FLOAT,
    fFeedback: FLOAT,
    fFrequency: FLOAT,
    lWaveform: LONG,
    fDelay: FLOAT,
    lPhase: LONG,
}}
pub type LPDSFXChorus = *mut DSFXChorus;
pub type LPCDSFXChorus = *const DSFXChorus;
pub const DSFXCHORUS_WAVE_TRIANGLE: LONG = 0;
pub const DSFXCHORUS_WAVE_SIN: LONG = 1;
pub const DSFXCHORUS_WETDRYMIX_MIN: FLOAT = 0.0;
pub const DSFXCHORUS_WETDRYMIX_MAX: FLOAT = 100.0;
pub const DSFXCHORUS_DEPTH_MIN: FLOAT = 0.0;
pub const DSFXCHORUS_DEPTH_MAX: FLOAT = 100.0;
pub const DSFXCHORUS_FEEDBACK_MIN: FLOAT = -99.0;
pub const DSFXCHORUS_FEEDBACK_MAX: FLOAT = 99.0;
pub const DSFXCHORUS_FREQUENCY_MIN: FLOAT = 0.0;
pub const DSFXCHORUS_FREQUENCY_MAX: FLOAT = 10.0;
pub const DSFXCHORUS_DELAY_MIN: FLOAT = 0.0;
pub const DSFXCHORUS_DELAY_MAX: FLOAT = 20.0;
pub const DSFXCHORUS_PHASE_MIN: LONG = 0;
pub const DSFXCHORUS_PHASE_MAX: LONG = 4;
pub const DSFXCHORUS_PHASE_NEG_180: LONG = 0;
pub const DSFXCHORUS_PHASE_NEG_90: LONG = 1;
pub const DSFXCHORUS_PHASE_ZERO: LONG = 2;
pub const DSFXCHORUS_PHASE_90: LONG = 3;
pub const DSFXCHORUS_PHASE_180: LONG = 4;
RIDL!{#[uuid(0x880842e3, 0x145f, 0x43e6, 0xa9, 0x34, 0xa7, 0x18, 0x06, 0xe5, 0x05, 0x47)]
interface IDirectSoundFXChorus(IDirectSoundFXChorusVtbl): IUnknown(IUnknownVtbl) {
    fn SetAllParameters(
        pcDsFxChorus: LPCDSFXChorus,
    ) -> HRESULT,
    fn GetAllParameters(
        pDsFxChorus: LPDSFXChorus,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IDirectSoundFXFlanger,
    0x903e9878, 0x2c92, 0x4072, 0x9b, 0x2c, 0xea, 0x68, 0xf5, 0x39, 0x67, 0x83}
STRUCT!{struct DSFXFlanger {
    fWetDryMix: FLOAT,
    fDepth: FLOAT,
    fFeedback: FLOAT,
    fFrequency: FLOAT,
    lWaveform: LONG,
    fDelay: FLOAT,
    lPhase: LONG,
}}
pub type LPDSFXFlanger = *mut DSFXFlanger;
pub type LPCDSFXFlanger = *const DSFXFlanger;
pub const DSFXFLANGER_WAVE_TRIANGLE: LONG = 0;
pub const DSFXFLANGER_WAVE_SIN: LONG = 1;
pub const DSFXFLANGER_WETDRYMIX_MIN: FLOAT = 0.0;
pub const DSFXFLANGER_WETDRYMIX_MAX: FLOAT = 100.0;
pub const DSFXFLANGER_FREQUENCY_MIN: FLOAT = 0.0;
pub const DSFXFLANGER_FREQUENCY_MAX: FLOAT = 10.0;
pub const DSFXFLANGER_DEPTH_MIN: FLOAT = 0.0;
pub const DSFXFLANGER_DEPTH_MAX: FLOAT = 100.0;
pub const DSFXFLANGER_PHASE_MIN: LONG = 0;
pub const DSFXFLANGER_PHASE_MAX: LONG = 4;
pub const DSFXFLANGER_FEEDBACK_MIN: FLOAT = -99.0;
pub const DSFXFLANGER_FEEDBACK_MAX: FLOAT = 99.0;
pub const DSFXFLANGER_DELAY_MIN: FLOAT = 0.0;
pub const DSFXFLANGER_DELAY_MAX: FLOAT = 4.0;
pub const DSFXFLANGER_PHASE_NEG_180: LONG = 0;
pub const DSFXFLANGER_PHASE_NEG_90: LONG = 1;
pub const DSFXFLANGER_PHASE_ZERO: LONG = 2;
pub const DSFXFLANGER_PHASE_90: LONG = 3;
pub const DSFXFLANGER_PHASE_180: LONG = 4;
RIDL!{#[uuid(0x903e9878, 0x2c92, 0x4072, 0x9b, 0x2c, 0xea, 0x68, 0xf5, 0x39, 0x67, 0x83)]
interface IDirectSoundFXFlanger(IDirectSoundFXFlangerVtbl): IUnknown(IUnknownVtbl) {
    fn SetAllParameters(
        pcDsFxFlanger: LPCDSFXFlanger,
    ) -> HRESULT,
    fn GetAllParameters(
        pDsFxFlanger: LPDSFXFlanger,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IDirectSoundFXEcho,
    0x8bd28edf, 0x50db, 0x4e92, 0xa2, 0xbd, 0x44, 0x54, 0x88, 0xd1, 0xed, 0x42}
STRUCT!{struct DSFXEcho {
    fWetDryMix: FLOAT,
    fFeedback: FLOAT,
    fLeftDelay: FLOAT,
    fRightDelay: FLOAT,
    lPanDelay: LONG,
}}
pub type LPDSFXEcho = *mut DSFXEcho;
pub type LPCDSFXEcho = *const DSFXEcho;
pub const DSFXECHO_WETDRYMIX_MIN: FLOAT = 0.0;
pub const DSFXECHO_WETDRYMIX_MAX: FLOAT = 100.0;
pub const DSFXECHO_FEEDBACK_MIN: FLOAT = 0.0;
pub const DSFXECHO_FEEDBACK_MAX: FLOAT = 100.0;
pub const DSFXECHO_LEFTDELAY_MIN: FLOAT = 1.0;
pub const DSFXECHO_LEFTDELAY_MAX: FLOAT = 2000.0;
pub const DSFXECHO_RIGHTDELAY_MIN: FLOAT = 1.0;
pub const DSFXECHO_RIGHTDELAY_MAX: FLOAT = 2000.0;
pub const DSFXECHO_PANDELAY_MIN: LONG = 0;
pub const DSFXECHO_PANDELAY_MAX: LONG = 1;
RIDL!{#[uuid(0x8bd28edf, 0x50db, 0x4e92, 0xa2, 0xbd, 0x44, 0x54, 0x88, 0xd1, 0xed, 0x42)]
interface IDirectSoundFXEcho(IDirectSoundFXEchoVtbl): IUnknown(IUnknownVtbl) {
    fn SetAllParameters(
        pcDsFxEcho: LPCDSFXEcho,
    ) -> HRESULT,
    fn GetAllParameters(
        pDsFxEcho: LPDSFXEcho,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IDirectSoundFXDistortion,
    0x8ecf4326, 0x455f, 0x4d8b, 0xbd, 0xa9, 0x8d, 0x5d, 0x3e, 0x9e, 0x3e, 0x0b}
STRUCT!{struct DSFXDistortion {
    fGain: FLOAT,
    fEdge: FLOAT,
    fPostEQCenterFrequency: FLOAT,
    fPostEQBandwidth: FLOAT,
    fPreLowpassCutoff: FLOAT,
}}
pub type LPDSFXDistortion = *mut DSFXDistortion;
pub type LPCDSFXDistortion = *const DSFXDistortion;
pub const DSFXDISTORTION_GAIN_MIN: FLOAT = -60.0;
pub const DSFXDISTORTION_GAIN_MAX: FLOAT = 0.0;
pub const DSFXDISTORTION_EDGE_MIN: FLOAT = 0.0;
pub const DSFXDISTORTION_EDGE_MAX: FLOAT = 100.0;
pub const DSFXDISTORTION_POSTEQCENTERFREQUENCY_MIN: FLOAT = 100.0;
pub const DSFXDISTORTION_POSTEQCENTERFREQUENCY_MAX: FLOAT = 8000.0;
pub const DSFXDISTORTION_POSTEQBANDWIDTH_MIN: FLOAT = 100.0;
pub const DSFXDISTORTION_POSTEQBANDWIDTH_MAX: FLOAT = 8000.0;
pub const DSFXDISTORTION_PRELOWPASSCUTOFF_MIN: FLOAT = 100.0;
pub const DSFXDISTORTION_PRELOWPASSCUTOFF_MAX: FLOAT = 8000.0;
RIDL!{#[uuid(0x8ecf4326, 0x455f, 0x4d8b, 0xbd, 0xa9, 0x8d, 0x5d, 0x3e, 0x9e, 0x3e, 0x0b)]
interface IDirectSoundFXDistortion(IDirectSoundFXDistortionVtbl): IUnknown(IUnknownVtbl) {
    fn SetAllParameters(
        pcDsFxDistortion: LPCDSFXDistortion,
    ) -> HRESULT,
    fn GetAllParameters(
        pDsFxDistortion: LPDSFXDistortion,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IDirectSoundFXCompressor,
    0x4bbd1154, 0x62f6, 0x4e2c, 0xa1, 0x5c, 0xd3, 0xb6, 0xc4, 0x17, 0xf7, 0xa0}
STRUCT!{struct DSFXCompressor {
    fGain: FLOAT,
    fAttack: FLOAT,
    fRelease: FLOAT,
    fThreshold: FLOAT,
    fRatio: FLOAT,
    fPredelay: FLOAT,
}}
pub type LPDSFXCompressor = *mut DSFXCompressor;
pub type LPCDSFXCompressor = *const DSFXCompressor;
pub const DSFXCOMPRESSOR_GAIN_MIN: FLOAT = -60.0;
pub const DSFXCOMPRESSOR_GAIN_MAX: FLOAT = 60.0;
pub const DSFXCOMPRESSOR_ATTACK_MIN: FLOAT = 0.01;
pub const DSFXCOMPRESSOR_ATTACK_MAX: FLOAT = 500.0;
pub const DSFXCOMPRESSOR_RELEASE_MIN: FLOAT = 50.0;
pub const DSFXCOMPRESSOR_RELEASE_MAX: FLOAT = 3000.0;
pub const DSFXCOMPRESSOR_THRESHOLD_MIN: FLOAT = -60.0;
pub const DSFXCOMPRESSOR_THRESHOLD_MAX: FLOAT = 0.0;
pub const DSFXCOMPRESSOR_RATIO_MIN: FLOAT = 1.0;
pub const DSFXCOMPRESSOR_RATIO_MAX: FLOAT = 100.0;
pub const DSFXCOMPRESSOR_PREDELAY_MIN: FLOAT = 0.0;
pub const DSFXCOMPRESSOR_PREDELAY_MAX: FLOAT = 4.0;
RIDL!{#[uuid(0x4bbd1154, 0x62f6, 0x4e2c, 0xa1, 0x5c, 0xd3, 0xb6, 0xc4, 0x17, 0xf7, 0xa0)]
interface IDirectSoundFXCompressor(IDirectSoundFXCompressorVtbl): IUnknown(IUnknownVtbl) {
    fn SetAllParameters(
        pcDsFxCompressor: LPCDSFXCompressor,
    ) -> HRESULT,
    fn GetAllParameters(
        pDsFxCompressor: LPDSFXCompressor,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IDirectSoundFXParamEq,
    0xc03ca9fe, 0xfe90, 0x4204, 0x80, 0x78, 0x82, 0x33, 0x4c, 0xd1, 0x77, 0xda}
STRUCT!{struct DSFXParamEq {
    fCenter: FLOAT,
    fBandwidth: FLOAT,
    fGain: FLOAT,
}}
pub type LPDSFXParamEq = *mut DSFXParamEq;
pub type LPCDSFXParamEq = *const DSFXParamEq;
pub const DSFXPARAMEQ_CENTER_MIN: FLOAT = 80.0;
pub const DSFXPARAMEQ_CENTER_MAX: FLOAT = 16000.0;
pub const DSFXPARAMEQ_BANDWIDTH_MIN: FLOAT = 1.0;
pub const DSFXPARAMEQ_BANDWIDTH_MAX: FLOAT = 36.0;
pub const DSFXPARAMEQ_GAIN_MIN: FLOAT = -15.0;
pub const DSFXPARAMEQ_GAIN_MAX: FLOAT = 15.0;
RIDL!{#[uuid(0xc03ca9fe, 0xfe90, 0x4204, 0x80, 0x78, 0x82, 0x33, 0x4c, 0xd1, 0x77, 0xda)]
interface IDirectSoundFXParamEq(IDirectSoundFXParamEqVtbl): IUnknown(IUnknownVtbl) {
    fn SetAllParameters(
        pcDsFxParamEq: LPCDSFXParamEq,
    ) -> HRESULT,
    fn GetAllParameters(
        pDsFxParamEq: LPDSFXParamEq,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IDirectSoundFXI3DL2Reverb,
    0x4b166a6a, 0x0d66, 0x43f3, 0x80, 0xe3, 0xee, 0x62, 0x80, 0xde, 0xe1, 0xa4}
STRUCT!{struct DSFXI3DL2Reverb {
    lRoom: LONG,
    lRoomHF: LONG,
    flRoomRolloffFactor: FLOAT,
    flDecayTime: FLOAT,
    flDecayHFRatio: FLOAT,
    lReflections: LONG,
    flReflectionsDelay: FLOAT,
    lReverb: LONG,
    flReverbDelay: FLOAT,
    flDiffusion: FLOAT,
    flDensity: FLOAT,
    flHFReference: FLOAT,
}}
pub type LPDSFXI3DL2Reverb = *mut DSFXI3DL2Reverb;
pub type LPCDSFXI3DL2Reverb = *const DSFXI3DL2Reverb;
pub const DSFX_I3DL2REVERB_ROOM_MIN: LONG = -10000;
pub const DSFX_I3DL2REVERB_ROOM_MAX: LONG = 0;
pub const DSFX_I3DL2REVERB_ROOM_DEFAULT: LONG = -1000;
pub const DSFX_I3DL2REVERB_ROOMHF_MIN: LONG = -10000;
pub const DSFX_I3DL2REVERB_ROOMHF_MAX: LONG = 0;
pub const DSFX_I3DL2REVERB_ROOMHF_DEFAULT: LONG = -100;
pub const DSFX_I3DL2REVERB_ROOMROLLOFFFACTOR_MIN: FLOAT = 0.0;
pub const DSFX_I3DL2REVERB_ROOMROLLOFFFACTOR_MAX: FLOAT = 10.0;
pub const DSFX_I3DL2REVERB_ROOMROLLOFFFACTOR_DEFAULT: FLOAT = 0.0;
pub const DSFX_I3DL2REVERB_DECAYTIME_MIN: FLOAT = 0.1;
pub const DSFX_I3DL2REVERB_DECAYTIME_MAX: FLOAT = 20.0;
pub const DSFX_I3DL2REVERB_DECAYTIME_DEFAULT: FLOAT = 1.49;
pub const DSFX_I3DL2REVERB_DECAYHFRATIO_MIN: FLOAT = 0.1;
pub const DSFX_I3DL2REVERB_DECAYHFRATIO_MAX: FLOAT = 2.0;
pub const DSFX_I3DL2REVERB_DECAYHFRATIO_DEFAULT: FLOAT = 0.83;
pub const DSFX_I3DL2REVERB_REFLECTIONS_MIN: LONG = -10000;
pub const DSFX_I3DL2REVERB_REFLECTIONS_MAX: LONG = 1000;
pub const DSFX_I3DL2REVERB_REFLECTIONS_DEFAULT: LONG = -2602;
pub const DSFX_I3DL2REVERB_REFLECTIONSDELAY_MIN: FLOAT = 0.0;
pub const DSFX_I3DL2REVERB_REFLECTIONSDELAY_MAX: FLOAT = 0.3;
pub const DSFX_I3DL2REVERB_REFLECTIONSDELAY_DEFAULT: FLOAT = 0.007;
pub const DSFX_I3DL2REVERB_REVERB_MIN: LONG = -10000;
pub const DSFX_I3DL2REVERB_REVERB_MAX: LONG = 2000;
pub const DSFX_I3DL2REVERB_REVERB_DEFAULT: LONG = 200;
pub const DSFX_I3DL2REVERB_REVERBDELAY_MIN: FLOAT = 0.0;
pub const DSFX_I3DL2REVERB_REVERBDELAY_MAX: FLOAT = 0.1;
pub const DSFX_I3DL2REVERB_REVERBDELAY_DEFAULT: FLOAT = 0.011;
pub const DSFX_I3DL2REVERB_DIFFUSION_MIN: FLOAT = 0.0;
pub const DSFX_I3DL2REVERB_DIFFUSION_MAX: FLOAT = 100.0;
pub const DSFX_I3DL2REVERB_DIFFUSION_DEFAULT: FLOAT = 100.0;
pub const DSFX_I3DL2REVERB_DENSITY_MIN: FLOAT = 0.0;
pub const DSFX_I3DL2REVERB_DENSITY_MAX: FLOAT = 100.0;
pub const DSFX_I3DL2REVERB_DENSITY_DEFAULT: FLOAT = 100.0;
pub const DSFX_I3DL2REVERB_HFREFERENCE_MIN: FLOAT = 20.0;
pub const DSFX_I3DL2REVERB_HFREFERENCE_MAX: FLOAT = 20000.0;
pub const DSFX_I3DL2REVERB_HFREFERENCE_DEFAULT: FLOAT = 5000.0;
pub const DSFX_I3DL2REVERB_QUALITY_MIN: LONG = 0;
pub const DSFX_I3DL2REVERB_QUALITY_MAX: LONG = 3;
pub const DSFX_I3DL2REVERB_QUALITY_DEFAULT: LONG = 2;
RIDL!{#[uuid(0x4b166a6a, 0x0d66, 0x43f3, 0x80, 0xe3, 0xee, 0x62, 0x80, 0xde, 0xe1, 0xa4)]
interface IDirectSoundFXI3DL2Reverb(IDirectSoundFXI3DL2ReverbVtbl): IUnknown(IUnknownVtbl) {
    fn SetAllParameters(
        pcDsFxI3DL2Reverb: LPCDSFXI3DL2Reverb,
    ) -> HRESULT,
    fn GetAllParameters(
        pDsFxI3DL2Reverb: LPDSFXI3DL2Reverb,
    ) -> HRESULT,
    fn SetPreset(
        dwPreset: DWORD,
    ) -> HRESULT,
    fn GetPreset(
        pdwPreset: LPDWORD,
    ) -> HRESULT,
    fn SetQuality(
        lQuality: LONG,
    ) -> HRESULT,
    fn GetQuality(
        plQuality: *mut LONG,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IDirectSoundFXWavesReverb,
    0x46858c3a, 0x0dc6, 0x45e3, 0xb7, 0x60, 0xd4, 0xee, 0xf1, 0x6c, 0xb3, 0x25}
STRUCT!{struct DSFXWavesReverb {
    fInGain: FLOAT,
    fReverbMix: FLOAT,
    fReverbTime: FLOAT,
    fHighFreqRTRatio: FLOAT,
}}
pub type LPDSFXWavesReverb = *mut DSFXWavesReverb;
pub type LPCDSFXWavesReverb = *const DSFXWavesReverb;
pub const DSFX_WAVESREVERB_INGAIN_MIN: FLOAT = -96.0;
pub const DSFX_WAVESREVERB_INGAIN_MAX: FLOAT = 0.0;
pub const DSFX_WAVESREVERB_INGAIN_DEFAULT: FLOAT = 0.0;
pub const DSFX_WAVESREVERB_REVERBMIX_MIN: FLOAT = -96.0;
pub const DSFX_WAVESREVERB_REVERBMIX_MAX: FLOAT = 0.0;
pub const DSFX_WAVESREVERB_REVERBMIX_DEFAULT: FLOAT = 0.0;
pub const DSFX_WAVESREVERB_REVERBTIME_MIN: FLOAT = 0.001;
pub const DSFX_WAVESREVERB_REVERBTIME_MAX: FLOAT = 3000.0;
pub const DSFX_WAVESREVERB_REVERBTIME_DEFAULT: FLOAT = 1000.0;
pub const DSFX_WAVESREVERB_HIGHFREQRTRATIO_MIN: FLOAT = 0.001;
pub const DSFX_WAVESREVERB_HIGHFREQRTRATIO_MAX: FLOAT = 0.999;
pub const DSFX_WAVESREVERB_HIGHFREQRTRATIO_DEFAULT: FLOAT = 0.001;
RIDL!{#[uuid(0x46858c3a, 0x0dc6, 0x45e3, 0xb7, 0x60, 0xd4, 0xee, 0xf1, 0x6c, 0xb3, 0x25)]
interface IDirectSoundFXWavesReverb(IDirectSoundFXWavesReverbVtbl): IUnknown(IUnknownVtbl) {
    fn SetAllParameters(
        pcDsFxWavesReverb: LPCDSFXWavesReverb,
    ) -> HRESULT,
    fn GetAllParameters(
        pDsFxWavesReverb: LPDSFXWavesReverb,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IDirectSoundCaptureFXAec,
    0xad74143d, 0x903d, 0x4ab7, 0x80, 0x66, 0x28, 0xd3, 0x63, 0x03, 0x6d, 0x65}
STRUCT!{struct DSCFXAec {
    fEnable: BOOL,
    fNoiseFill: BOOL,
    dwMode: DWORD,
}}
pub type LPDSCFXAec = *mut DSCFXAec;
pub type LPCDSCFXAec = *const DSCFXAec;
pub const DSCFX_AEC_MODE_PASS_THROUGH: DWORD = 0;
pub const DSCFX_AEC_MODE_HALF_DUPLEX: DWORD = 1;
pub const DSCFX_AEC_MODE_FULL_DUPLEX: DWORD = 2;
pub const DSCFX_AEC_STATUS_HISTORY_UNINITIALIZED: DWORD = 0;
pub const DSCFX_AEC_STATUS_HISTORY_CONTINUOUSLY_CONVERGED: DWORD = 1;
pub const DSCFX_AEC_STATUS_HISTORY_PREVIOUSLY_DIVERGED: DWORD = 2;
pub const DSCFX_AEC_STATUS_CURRENTLY_CONVERGED: DWORD = 8;
RIDL!{#[uuid(0xad74143d, 0x903d, 0x4ab7, 0x80, 0x66, 0x28, 0xd3, 0x63, 0x03, 0x6d, 0x65)]
interface IDirectSoundCaptureFXAec(IDirectSoundCaptureFXAecVtbl): IUnknown(IUnknownVtbl) {
    fn SetAllParameters(
        pDscFxAec: LPCDSCFXAec,
    ) -> HRESULT,
    fn GetAllParameters(
        pDscFxAec: LPDSCFXAec,
    ) -> HRESULT,
    fn GetStatus(
        pdwStatus: LPDWORD,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
}}
DEFINE_GUID!{IID_IDirectSoundCaptureFXNoiseSuppress,
    0xed311e41, 0xfbae, 0x4175, 0x96, 0x25, 0xcd, 0x08, 0x54, 0xf6, 0x93, 0xca}
STRUCT!{struct DSCFXNoiseSuppress {
    fEnable: BOOL,
}}
pub type LPDSCFXNoiseSuppress = *mut DSCFXNoiseSuppress;
pub type LPCDSCFXNoiseSuppress = *const DSCFXNoiseSuppress;
RIDL!{#[uuid(0xed311e41, 0xfbae, 0x4175, 0x96, 0x25, 0xcd, 0x08, 0x54, 0xf6, 0x93, 0xca)]
interface IDirectSoundCaptureFXNoiseSuppress(IDirectSoundCaptureFXNoiseSuppressVtbl): 
    IUnknown(IUnknownVtbl) {
    fn SetAllParameters(
        pcDscFxNoiseSuppress: LPCDSCFXNoiseSuppress,
    ) -> HRESULT,
    fn GetAllParameters(
        pDscFxNoiseSuppress: LPDSCFXNoiseSuppress,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
}}
DEFINE_GUID!{IID_IDirectSoundFullDuplex,
    0xedcb4c7a, 0xdaab, 0x4216, 0xa4, 0x2e, 0x6c, 0x50, 0x59, 0x6d, 0xdc, 0x1d}
RIDL!{#[uuid(0xedcb4c7a, 0xdaab, 0x4216, 0xa4, 0x2e, 0x6c, 0x50, 0x59, 0x6d, 0xdc, 0x1d)]
interface IDirectSoundFullDuplex(IDirectSoundFullDuplexVtbl): IUnknown(IUnknownVtbl) {
    fn Initialize(
        pCaptureGuid: LPCGUID,
        pRenderGuid: LPCGUID,
        lpDscBufferDesc: LPCDSCBUFFERDESC,
        lpDsBufferDesc: LPCDSBUFFERDESC,
        hWnd: HWND,
        dwLevel: DWORD,
        lplpDirectSoundCaptureBuffer8: LPLPDIRECTSOUNDCAPTUREBUFFER8,
        lplpDirectSoundBuffer8: LPLPDIRECTSOUNDBUFFER8,
    ) -> HANDLE,
}}
pub const DS_OK: HRESULT = S_OK;
pub const DS_NO_VIRTUALIZATION: HRESULT = MAKE_HRESULT!(0, _FACDS, 10);
pub const DSERR_ALLOCATED: HRESULT = MAKE_HRESULT!(1, _FACDS, 10);
pub const DSERR_CONTROLUNAVAIL: HRESULT = MAKE_HRESULT!(1, _FACDS, 30);
pub const DSERR_INVALIDPARAM: HRESULT = E_INVALIDARG;
pub const DSERR_INVALIDCALL: HRESULT = MAKE_HRESULT!(1, _FACDS, 50);
pub const DSERR_GENERIC: HRESULT = E_FAIL;
pub const DSERR_PRIOLEVELNEEDED: HRESULT = MAKE_HRESULT!(1, _FACDS, 70);
pub const DSERR_OUTOFMEMORY: HRESULT = E_OUTOFMEMORY;
pub const DSERR_BADFORMAT: HRESULT = MAKE_HRESULT!(1, _FACDS, 100);
pub const DSERR_UNSUPPORTED: HRESULT = E_NOTIMPL;
pub const DSERR_NODRIVER: HRESULT = MAKE_HRESULT!(1, _FACDS, 120);
pub const DSERR_ALREADYINITIALIZED: HRESULT = MAKE_HRESULT!(1, _FACDS, 130);
pub const DSERR_NOAGGREGATION: HRESULT = CLASS_E_NOAGGREGATION;
pub const DSERR_BUFFERLOST: HRESULT = MAKE_HRESULT!(1, _FACDS, 150);
pub const DSERR_OTHERAPPHASPRIO: HRESULT = MAKE_HRESULT!(1, _FACDS, 160);
pub const DSERR_UNINITIALIZED: HRESULT = MAKE_HRESULT!(1, _FACDS, 170);
pub const DSERR_NOINTERFACE: HRESULT = E_NOINTERFACE;
pub const DSERR_ACCESSDENIED: HRESULT = E_ACCESSDENIED;
pub const DSERR_BUFFERTOOSMALL: HRESULT = MAKE_HRESULT!(1, _FACDS, 180);
pub const DSERR_DS8_REQUIRED: HRESULT = MAKE_HRESULT!(1, _FACDS, 190);
pub const DSERR_SENDLOOP: HRESULT = MAKE_HRESULT!(1, _FACDS, 200);
pub const DSERR_BADSENDBUFFERGUID: HRESULT = MAKE_HRESULT!(1, _FACDS, 210);
pub const DSERR_OBJNOTFOUND: HRESULT = MAKE_HRESULT!(1, _FACDS, 4449);
pub const DSERR_FXUNAVAILABLE: HRESULT = MAKE_HRESULT!(1, _FACDS, 220);
pub const DSCAPS_PRIMARYMONO: DWORD = 0x00000001;
pub const DSCAPS_PRIMARYSTEREO: DWORD = 0x00000002;
pub const DSCAPS_PRIMARY8BIT: DWORD = 0x00000004;
pub const DSCAPS_PRIMARY16BIT: DWORD = 0x00000008;
pub const DSCAPS_CONTINUOUSRATE: DWORD = 0x00000010;
pub const DSCAPS_EMULDRIVER: DWORD = 0x00000020;
pub const DSCAPS_CERTIFIED: DWORD = 0x00000040;
pub const DSCAPS_SECONDARYMONO: DWORD = 0x00000100;
pub const DSCAPS_SECONDARYSTEREO: DWORD = 0x00000200;
pub const DSCAPS_SECONDARY8BIT: DWORD = 0x00000400;
pub const DSCAPS_SECONDARY16BIT: DWORD = 0x00000800;
pub const DSSCL_NORMAL: DWORD = 0x00000001;
pub const DSSCL_PRIORITY: DWORD = 0x00000002;
pub const DSSCL_EXCLUSIVE: DWORD = 0x00000003;
pub const DSSCL_WRITEPRIMARY: DWORD = 0x00000004;
pub const DSSPEAKER_DIRECTOUT: DWORD = 0x00000000;
pub const DSSPEAKER_HEADPHONE: DWORD = 0x00000001;
pub const DSSPEAKER_MONO: DWORD = 0x00000002;
pub const DSSPEAKER_QUAD: DWORD = 0x00000003;
pub const DSSPEAKER_STEREO: DWORD = 0x00000004;
pub const DSSPEAKER_SURROUND: DWORD = 0x00000005;
pub const DSSPEAKER_5POINT1: DWORD = 0x00000006;
pub const DSSPEAKER_7POINT1: DWORD = 0x00000007;
pub const DSSPEAKER_7POINT1_SURROUND: DWORD = 0x00000008;
pub const DSSPEAKER_5POINT1_SURROUND: DWORD = 0x00000009;
pub const DSSPEAKER_7POINT1_WIDE: DWORD = DSSPEAKER_7POINT1;
pub const DSSPEAKER_5POINT1_BACK: DWORD = DSSPEAKER_5POINT1;
pub const DSSPEAKER_GEOMETRY_MIN: DWORD = 0x00000005;
pub const DSSPEAKER_GEOMETRY_NARROW: DWORD = 0x0000000A;
pub const DSSPEAKER_GEOMETRY_WIDE: DWORD = 0x00000014;
pub const DSSPEAKER_GEOMETRY_MAX: DWORD = 0x000000B4;
#[inline]
pub fn DSSPEAKER_COMBINED(c: DWORD, g: DWORD) -> DWORD {
    (c as BYTE as DWORD) | ((g as BYTE as DWORD) << 16)
}
#[inline]
pub fn DSSPEAKER_CONFIG(a: DWORD) -> BYTE {
    a as BYTE
}
#[inline]
pub fn DSSPEAKER_GEOMETRY(a: DWORD) -> BYTE {
    (a >> 16) as BYTE
}
pub const DSBCAPS_PRIMARYBUFFER: DWORD = 0x00000001;
pub const DSBCAPS_STATIC: DWORD = 0x00000002;
pub const DSBCAPS_LOCHARDWARE: DWORD = 0x00000004;
pub const DSBCAPS_LOCSOFTWARE: DWORD = 0x00000008;
pub const DSBCAPS_CTRL3D: DWORD = 0x00000010;
pub const DSBCAPS_CTRLFREQUENCY: DWORD = 0x00000020;
pub const DSBCAPS_CTRLPAN: DWORD = 0x00000040;
pub const DSBCAPS_CTRLVOLUME: DWORD = 0x00000080;
pub const DSBCAPS_CTRLPOSITIONNOTIFY: DWORD = 0x00000100;
pub const DSBCAPS_CTRLFX: DWORD = 0x00000200;
pub const DSBCAPS_STICKYFOCUS: DWORD = 0x00004000;
pub const DSBCAPS_GLOBALFOCUS: DWORD = 0x00008000;
pub const DSBCAPS_GETCURRENTPOSITION2: DWORD = 0x00010000;
pub const DSBCAPS_MUTE3DATMAXDISTANCE: DWORD = 0x00020000;
pub const DSBCAPS_LOCDEFER: DWORD = 0x00040000;
pub const DSBCAPS_TRUEPLAYPOSITION: DWORD = 0x00080000;
pub const DSBPLAY_LOOPING: DWORD = 0x00000001;
pub const DSBPLAY_LOCHARDWARE: DWORD = 0x00000002;
pub const DSBPLAY_LOCSOFTWARE: DWORD = 0x00000004;
pub const DSBPLAY_TERMINATEBY_TIME: DWORD = 0x00000008;
pub const DSBPLAY_TERMINATEBY_DISTANCE: DWORD = 0x000000010;
pub const DSBPLAY_TERMINATEBY_PRIORITY: DWORD = 0x000000020;
pub const DSBSTATUS_PLAYING: DWORD = 0x00000001;
pub const DSBSTATUS_BUFFERLOST: DWORD = 0x00000002;
pub const DSBSTATUS_LOOPING: DWORD = 0x00000004;
pub const DSBSTATUS_LOCHARDWARE: DWORD = 0x00000008;
pub const DSBSTATUS_LOCSOFTWARE: DWORD = 0x00000010;
pub const DSBSTATUS_TERMINATED: DWORD = 0x00000020;
pub const DSBLOCK_FROMWRITECURSOR: DWORD = 0x00000001;
pub const DSBLOCK_ENTIREBUFFER: DWORD = 0x00000002;
pub const DSBFREQUENCY_ORIGINAL: DWORD = 0;
pub const DSBFREQUENCY_MIN: DWORD = 100;
pub const DSBFREQUENCY_MAX: DWORD = 200000;
pub const DSBPAN_LEFT: LONG = -10000;
pub const DSBPAN_CENTER: LONG = 0;
pub const DSBPAN_RIGHT: LONG = 10000;
pub const DSBVOLUME_MIN: LONG = -10000;
pub const DSBVOLUME_MAX: LONG = 0;
pub const DSBSIZE_MIN: DWORD = 4;
pub const DSBSIZE_MAX: DWORD = 0x0FFFFFFF;
pub const DSBSIZE_FX_MIN: DWORD = 150;
pub const DSBNOTIFICATIONS_MAX: DWORD = 100000;
pub const DS3DMODE_NORMAL: DWORD = 0x00000000;
pub const DS3DMODE_HEADRELATIVE: DWORD = 0x00000001;
pub const DS3DMODE_DISABLE: DWORD = 0x00000002;
pub const DS3D_IMMEDIATE: DWORD = 0x00000000;
pub const DS3D_DEFERRED: DWORD = 0x00000001;
pub const DS3D_MINDISTANCEFACTOR: D3DVALUE = -3.40282347e+38;
pub const DS3D_MAXDISTANCEFACTOR: D3DVALUE = 3.40282347e+38;
pub const DS3D_DEFAULTDISTANCEFACTOR: D3DVALUE = 1.0;
pub const DS3D_MINROLLOFFFACTOR: D3DVALUE = 0.0;
pub const DS3D_MAXROLLOFFFACTOR: D3DVALUE = 10.0;
pub const DS3D_DEFAULTROLLOFFFACTOR: D3DVALUE = 1.0;
pub const DS3D_MINDOPPLERFACTOR: D3DVALUE = 0.0;
pub const DS3D_MAXDOPPLERFACTOR: D3DVALUE = 10.0;
pub const DS3D_DEFAULTDOPPLERFACTOR: D3DVALUE = 1.0;
pub const DS3D_DEFAULTMINDISTANCE: D3DVALUE = 1.0;
pub const DS3D_DEFAULTMAXDISTANCE: D3DVALUE = 1000000000.0;
pub const DS3D_MINCONEANGLE: DWORD = 0;
pub const DS3D_MAXCONEANGLE: DWORD = 360;
pub const DS3D_DEFAULTCONEANGLE: DWORD = 360;
pub const DS3D_DEFAULTCONEOUTSIDEVOLUME: LONG = DSBVOLUME_MAX;
pub const DSCCAPS_EMULDRIVER: DWORD = DSCAPS_EMULDRIVER;
pub const DSCCAPS_CERTIFIED: DWORD = DSCAPS_CERTIFIED;
pub const DSCCAPS_MULTIPLECAPTURE: DWORD = 0x00000001;
pub const DSCBCAPS_WAVEMAPPED: DWORD = 0x80000000;
pub const DSCBCAPS_CTRLFX: DWORD = 0x00000200;
pub const DSCBLOCK_ENTIREBUFFER: DWORD = 0x00000001;
pub const DSCBSTATUS_CAPTURING: DWORD = 0x00000001;
pub const DSCBSTATUS_LOOPING: DWORD = 0x00000002;
pub const DSCBSTART_LOOPING: DWORD = 0x00000001;
pub const DSBPN_OFFSETSTOP: DWORD = 0xFFFFFFFF;
pub const DS_CERTIFIED: DWORD = 0x00000000;
pub const DS_UNCERTIFIED: DWORD = 0x00000001;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_DEFAULT: DWORD = 0;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_GENERIC: DWORD = 1;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_PADDEDCELL: DWORD = 2;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_ROOM: DWORD = 3;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_BATHROOM: DWORD = 4;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_LIVINGROOM: DWORD = 5;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_STONEROOM: DWORD = 6;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_AUDITORIUM: DWORD = 7;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_CONCERTHALL: DWORD = 8;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_CAVE: DWORD = 9;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_ARENA: DWORD = 10;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_HANGAR: DWORD = 11;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_CARPETEDHALLWAY: DWORD = 12;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_HALLWAY: DWORD = 13;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_STONECORRIDOR: DWORD = 14;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_ALLEY: DWORD = 15;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_FOREST: DWORD = 16;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_CITY: DWORD = 17;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_MOUNTAINS: DWORD = 18;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_QUARRY: DWORD = 19;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_PLAIN: DWORD = 20;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_PARKINGLOT: DWORD = 21;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_SEWERPIPE: DWORD = 22;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_UNDERWATER: DWORD = 23;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_SMALLROOM: DWORD = 24;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_MEDIUMROOM: DWORD = 25;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_LARGEROOM: DWORD = 26;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_MEDIUMHALL: DWORD = 27;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_LARGEHALL: DWORD = 28;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_PLATE: DWORD = 29;
pub const I3DL2_ENVIRONMENT_PRESET_DEFAULT: DSFXI3DL2Reverb = DSFXI3DL2Reverb {
    lRoom: -1000,
    lRoomHF: -100,
    flRoomRolloffFactor: 0.0,
    flDecayTime: 1.49,
    flDecayHFRatio: 0.83,
    lReflections: -2602,
    flReflectionsDelay: 0.007,
    lReverb: 200,
    flReverbDelay: 0.011,
    flDiffusion: 100.0,
    flDensity: 100.0,
    flHFReference: 5000.0,
};
pub const I3DL2_ENVIRONMENT_PRESET_GENERIC: DSFXI3DL2Reverb = DSFXI3DL2Reverb {
    lRoom: -1000,
    lRoomHF: -100,
    flRoomRolloffFactor: 0.0,
    flDecayTime: 1.49,
    flDecayHFRatio: 0.83,
    lReflections: -2602,
    flReflectionsDelay: 0.007,
    lReverb: 200,
    flReverbDelay: 0.011,
    flDiffusion: 100.0,
    flDensity: 100.0,
    flHFReference: 5000.0,
};
pub const I3DL2_ENVIRONMENT_PRESET_PADDEDCELL: DSFXI3DL2Reverb = DSFXI3DL2Reverb {
    lRoom: -1000,
    lRoomHF: -6000,
    flRoomRolloffFactor: 0.0,
    flDecayTime: 0.17,
    flDecayHFRatio: 0.10,
    lReflections: -1204,
    flReflectionsDelay: 0.001,
    lReverb: 207,
    flReverbDelay: 0.002,
    flDiffusion: 100.0,
    flDensity: 100.0,
    flHFReference: 5000.0,
};
pub const I3DL2_ENVIRONMENT_PRESET_ROOM: DSFXI3DL2Reverb = DSFXI3DL2Reverb {
    lRoom: -1000,
    lRoomHF: -454,
    flRoomRolloffFactor: 0.0,
    flDecayTime: 0.40,
    flDecayHFRatio: 0.83,
    lReflections: -1646,
    flReflectionsDelay: 0.002,
    lReverb: 53,
    flReverbDelay: 0.003,
    flDiffusion: 100.0,
    flDensity: 100.0,
    flHFReference: 5000.0,
};
pub const I3DL2_ENVIRONMENT_PRESET_BATHROOM: DSFXI3DL2Reverb = DSFXI3DL2Reverb {
    lRoom: -1000,
    lRoomHF: -1200,
    flRoomRolloffFactor: 0.0,
    flDecayTime: 1.49,
    flDecayHFRatio: 0.54,
    lReflections: -370,
    flReflectionsDelay: 0.007,
    lReverb: 1030,
    flReverbDelay: 0.011,
    flDiffusion: 100.0,
    flDensity: 60.0,
    flHFReference: 5000.0,
};
pub const I3DL2_ENVIRONMENT_PRESET_LIVINGROOM: DSFXI3DL2Reverb = DSFXI3DL2Reverb {
    lRoom: -1000,
    lRoomHF: -6000,
    flRoomRolloffFactor: 0.0,
    flDecayTime: 0.50,
    flDecayHFRatio: 0.10,
    lReflections: -1376,
    flReflectionsDelay: 0.003,
    lReverb: -1104,
    flReverbDelay: 0.004,
    flDiffusion: 100.0,
    flDensity: 100.0,
    flHFReference: 5000.0,
};
pub const I3DL2_ENVIRONMENT_PRESET_STONEROOM: DSFXI3DL2Reverb = DSFXI3DL2Reverb {
    lRoom: -1000,
    lRoomHF: -300,
    flRoomRolloffFactor: 0.0,
    flDecayTime: 2.31,
    flDecayHFRatio: 0.64,
    lReflections: -711,
    flReflectionsDelay: 0.012,
    lReverb: 83,
    flReverbDelay: 0.017,
    flDiffusion: 100.0,
    flDensity: 100.0,
    flHFReference: 5000.0,
};
pub const I3DL2_ENVIRONMENT_PRESET_AUDITORIUM: DSFXI3DL2Reverb = DSFXI3DL2Reverb {
    lRoom: -1000,
    lRoomHF: -476,
    flRoomRolloffFactor: 0.0,
    flDecayTime: 4.32,
    flDecayHFRatio: 0.59,
    lReflections: -789,
    flReflectionsDelay: 0.020,
    lReverb: -289,
    flReverbDelay: 0.030,
    flDiffusion: 100.0,
    flDensity: 100.0,
    flHFReference: 5000.0,
};
pub const I3DL2_ENVIRONMENT_PRESET_CONCERTHALL: DSFXI3DL2Reverb = DSFXI3DL2Reverb {
    lRoom: -1000,
    lRoomHF: -500,
    flRoomRolloffFactor: 0.0,
    flDecayTime: 3.92,
    flDecayHFRatio: 0.70,
    lReflections: -1230,
    flReflectionsDelay: 0.020,
    lReverb: -2,
    flReverbDelay: 0.029,
    flDiffusion: 100.0,
    flDensity: 100.0,
    flHFReference: 5000.0,
};
pub const I3DL2_ENVIRONMENT_PRESET_CAVE: DSFXI3DL2Reverb = DSFXI3DL2Reverb {
    lRoom: -1000,
    lRoomHF: 0,
    flRoomRolloffFactor: 0.0,
    flDecayTime: 2.91,
    flDecayHFRatio: 1.30,
    lReflections: -602,
    flReflectionsDelay: 0.015,
    lReverb: -302,
    flReverbDelay: 0.022,
    flDiffusion: 100.0,
    flDensity: 100.0,
    flHFReference: 5000.0,
};
pub const I3DL2_ENVIRONMENT_PRESET_ARENA: DSFXI3DL2Reverb = DSFXI3DL2Reverb {
    lRoom: -1000,
    lRoomHF: -698,
    flRoomRolloffFactor: 0.0,
    flDecayTime: 7.24,
    flDecayHFRatio: 0.33,
    lReflections: -1166,
    flReflectionsDelay: 0.020,
    lReverb: 16,
    flReverbDelay: 0.030,
    flDiffusion: 100.0,
    flDensity: 100.0,
    flHFReference: 5000.0,
};
pub const I3DL2_ENVIRONMENT_PRESET_HANGAR: DSFXI3DL2Reverb = DSFXI3DL2Reverb {
    lRoom: -1000,
    lRoomHF: -1000,
    flRoomRolloffFactor: 0.0,
    flDecayTime: 10.05,
    flDecayHFRatio: 0.23,
    lReflections: -602,
    flReflectionsDelay: 0.020,
    lReverb: 198,
    flReverbDelay: 0.030,
    flDiffusion: 100.0,
    flDensity: 100.0,
    flHFReference: 5000.0,
};
pub const I3DL2_ENVIRONMENT_PRESET_CARPETEDHALLWAY: DSFXI3DL2Reverb = DSFXI3DL2Reverb {
    lRoom: -1000,
    lRoomHF: -4000,
    flRoomRolloffFactor: 0.0,
    flDecayTime: 0.30,
    flDecayHFRatio: 0.10,
    lReflections: -1831,
    flReflectionsDelay: 0.002,
    lReverb: -1630,
    flReverbDelay: 0.030,
    flDiffusion: 100.0,
    flDensity: 100.0,
    flHFReference: 5000.0,
};
pub const I3DL2_ENVIRONMENT_PRESET_HALLWAY: DSFXI3DL2Reverb = DSFXI3DL2Reverb {
    lRoom: -1000,
    lRoomHF: -300,
    flRoomRolloffFactor: 0.0,
    flDecayTime: 1.49,
    flDecayHFRatio: 0.59,
    lReflections: -1219,
    flReflectionsDelay: 0.007,
    lReverb: 441,
    flReverbDelay: 0.011,
    flDiffusion: 100.0,
    flDensity: 100.0,
    flHFReference: 5000.0,
};
pub const I3DL2_ENVIRONMENT_PRESET_STONECORRIDOR: DSFXI3DL2Reverb = DSFXI3DL2Reverb {
    lRoom: -1000,
    lRoomHF: -237,
    flRoomRolloffFactor: 0.0,
    flDecayTime: 2.70,
    flDecayHFRatio: 0.79,
    lReflections: -1214,
    flReflectionsDelay: 0.013,
    lReverb: 395,
    flReverbDelay: 0.020,
    flDiffusion: 100.0,
    flDensity: 100.0,
    flHFReference: 5000.0,
};
pub const I3DL2_ENVIRONMENT_PRESET_ALLEY: DSFXI3DL2Reverb = DSFXI3DL2Reverb {
    lRoom: -1000,
    lRoomHF: -270,
    flRoomRolloffFactor: 0.0,
    flDecayTime: 1.49,
    flDecayHFRatio: 0.86,
    lReflections: -1204,
    flReflectionsDelay: 0.007,
    lReverb: -4,
    flReverbDelay: 0.011,
    flDiffusion: 100.0,
    flDensity: 100.0,
    flHFReference: 5000.0,
};
pub const I3DL2_ENVIRONMENT_PRESET_FOREST: DSFXI3DL2Reverb = DSFXI3DL2Reverb {
    lRoom: -1000,
    lRoomHF: -3300,
    flRoomRolloffFactor: 0.0,
    flDecayTime: 1.49,
    flDecayHFRatio: 0.54,
    lReflections: -2560,
    flReflectionsDelay: 0.162,
    lReverb: -613,
    flReverbDelay: 0.088,
    flDiffusion: 79.0,
    flDensity: 100.0,
    flHFReference: 5000.0,
};
pub const I3DL2_ENVIRONMENT_PRESET_CITY: DSFXI3DL2Reverb = DSFXI3DL2Reverb {
    lRoom: -1000,
    lRoomHF: -800,
    flRoomRolloffFactor: 0.0,
    flDecayTime: 1.49,
    flDecayHFRatio: 0.67,
    lReflections: -2273,
    flReflectionsDelay: 0.007,
    lReverb: -2217,
    flReverbDelay: 0.011,
    flDiffusion: 50.0,
    flDensity: 100.0,
    flHFReference: 5000.0,
};
pub const I3DL2_ENVIRONMENT_PRESET_MOUNTAINS: DSFXI3DL2Reverb = DSFXI3DL2Reverb {
    lRoom: -1000,
    lRoomHF: -2500,
    flRoomRolloffFactor: 0.0,
    flDecayTime: 1.49,
    flDecayHFRatio: 0.21,
    lReflections: -2780,
    flReflectionsDelay: 0.300,
    lReverb: -2014,
    flReverbDelay: 0.100,
    flDiffusion: 27.0,
    flDensity: 100.0,
    flHFReference: 5000.0,
};
pub const I3DL2_ENVIRONMENT_PRESET_QUARRY: DSFXI3DL2Reverb = DSFXI3DL2Reverb {
    lRoom: -1000,
    lRoomHF: -1000,
    flRoomRolloffFactor: 0.0,
    flDecayTime: 1.49,
    flDecayHFRatio: 0.83,
    lReflections: -10000,
    flReflectionsDelay: 0.061,
    lReverb: 500,
    flReverbDelay: 0.025,
    flDiffusion: 100.0,
    flDensity: 100.0,
    flHFReference: 5000.0,
};
pub const I3DL2_ENVIRONMENT_PRESET_PLAIN: DSFXI3DL2Reverb = DSFXI3DL2Reverb {
    lRoom: -1000,
    lRoomHF: -2000,
    flRoomRolloffFactor: 0.0,
    flDecayTime: 1.49,
    flDecayHFRatio: 0.50,
    lReflections: -2466,
    flReflectionsDelay: 0.179,
    lReverb: -2514,
    flReverbDelay: 0.100,
    flDiffusion: 21.0,
    flDensity: 100.0,
    flHFReference: 5000.0,
};
pub const I3DL2_ENVIRONMENT_PRESET_PARKINGLOT: DSFXI3DL2Reverb = DSFXI3DL2Reverb {
    lRoom: -1000,
    lRoomHF: 0,
    flRoomRolloffFactor: 0.0,
    flDecayTime: 1.65,
    flDecayHFRatio: 1.50,
    lReflections: -1363,
    flReflectionsDelay: 0.008,
    lReverb: -1153,
    flReverbDelay: 0.012,
    flDiffusion: 100.0,
    flDensity: 100.0,
    flHFReference: 5000.0,
};
pub const I3DL2_ENVIRONMENT_PRESET_SEWERPIPE: DSFXI3DL2Reverb = DSFXI3DL2Reverb {
    lRoom: -1000,
    lRoomHF: -1000,
    flRoomRolloffFactor: 0.0,
    flDecayTime: 2.81,
    flDecayHFRatio: 0.14,
    lReflections: 429,
    flReflectionsDelay: 0.014,
    lReverb: 648,
    flReverbDelay: 0.021,
    flDiffusion: 80.0,
    flDensity: 60.0,
    flHFReference: 5000.0,
};
pub const I3DL2_ENVIRONMENT_PRESET_UNDERWATER: DSFXI3DL2Reverb = DSFXI3DL2Reverb {
    lRoom: -1000,
    lRoomHF: -4000,
    flRoomRolloffFactor: 0.0,
    flDecayTime: 1.49,
    flDecayHFRatio: 0.10,
    lReflections: -449,
    flReflectionsDelay: 0.007,
    lReverb: 1700,
    flReverbDelay: 0.011,
    flDiffusion: 100.0,
    flDensity: 100.0,
    flHFReference: 5000.0,
};
pub const I3DL2_ENVIRONMENT_PRESET_SMALLROOM: DSFXI3DL2Reverb = DSFXI3DL2Reverb {
    lRoom: -1000,
    lRoomHF: -600,
    flRoomRolloffFactor: 0.0,
    flDecayTime: 1.10,
    flDecayHFRatio: 0.83,
    lReflections: -400,
    flReflectionsDelay: 0.005,
    lReverb: 500,
    flReverbDelay: 0.010,
    flDiffusion: 100.0,
    flDensity: 100.0,
    flHFReference: 5000.0,
};
pub const I3DL2_ENVIRONMENT_PRESET_MEDIUMROOM: DSFXI3DL2Reverb = DSFXI3DL2Reverb {
    lRoom: -1000,
    lRoomHF: -600,
    flRoomRolloffFactor: 0.0,
    flDecayTime: 1.30,
    flDecayHFRatio: 0.83,
    lReflections: -1000,
    flReflectionsDelay: 0.010,
    lReverb: -200,
    flReverbDelay: 0.020,
    flDiffusion: 100.0,
    flDensity: 100.0,
    flHFReference: 5000.0,
};
pub const I3DL2_ENVIRONMENT_PRESET_LARGEROOM: DSFXI3DL2Reverb = DSFXI3DL2Reverb {
    lRoom: -1000,
    lRoomHF: -600,
    flRoomRolloffFactor: 0.0,
    flDecayTime: 1.50,
    flDecayHFRatio: 0.83,
    lReflections: -1600,
    flReflectionsDelay: 0.020,
    lReverb: -1000,
    flReverbDelay: 0.040,
    flDiffusion: 100.0,
    flDensity: 100.0,
    flHFReference: 5000.0,
};
pub const I3DL2_ENVIRONMENT_PRESET_MEDIUMHALL: DSFXI3DL2Reverb = DSFXI3DL2Reverb {
    lRoom: -1000,
    lRoomHF: -600,
    flRoomRolloffFactor: 0.0,
    flDecayTime: 1.80,
    flDecayHFRatio: 0.70,
    lReflections: -1300,
    flReflectionsDelay: 0.015,
    lReverb: -800,
    flReverbDelay: 0.030,
    flDiffusion: 100.0,
    flDensity: 100.0,
    flHFReference: 5000.0,
};
pub const I3DL2_ENVIRONMENT_PRESET_LARGEHALL: DSFXI3DL2Reverb = DSFXI3DL2Reverb {
    lRoom: -1000,
    lRoomHF: -600,
    flRoomRolloffFactor: 0.0,
    flDecayTime: 1.80,
    flDecayHFRatio: 0.70,
    lReflections: -2000,
    flReflectionsDelay: 0.030,
    lReverb: -1400,
    flReverbDelay: 0.060,
    flDiffusion: 100.0,
    flDensity: 100.0,
    flHFReference: 5000.0,
};
pub const I3DL2_ENVIRONMENT_PRESET_PLATE: DSFXI3DL2Reverb = DSFXI3DL2Reverb {
    lRoom: -1000,
    lRoomHF: -200,
    flRoomRolloffFactor: 0.0,
    flDecayTime: 1.30,
    flDecayHFRatio: 0.90,
    lReflections: 0,
    flReflectionsDelay: 0.002,
    lReverb: 0,
    flReverbDelay: 0.010,
    flDiffusion: 100.0,
    flDensity: 75.0,
    flHFReference: 5000.0,
};
pub const DS3DALG_DEFAULT: GUID = GUID_NULL;
DEFINE_GUID!{DS3DALG_NO_VIRTUALIZATION,
    0xc241333f, 0x1c1b, 0x11d2, 0x94, 0xf5, 0x00, 0xc0, 0x4f, 0xc2, 0x8a, 0xca}
DEFINE_GUID!{DS3DALG_HRTF_FULL,
    0xc2413340, 0x1c1b, 0x11d2, 0x94, 0xf5, 0x00, 0xc0, 0x4f, 0xc2, 0x8a, 0xca}
DEFINE_GUID!{DS3DALG_HRTF_LIGHT,
    0xc2413342, 0x1c1b, 0x11d2, 0x94, 0xf5, 0x00, 0xc0, 0x4f, 0xc2, 0x8a, 0xca}
DEFINE_GUID!{GUID_DSFX_STANDARD_GARGLE,
    0xdafd8210, 0x5711, 0x4b91, 0x9f, 0xe3, 0xf7, 0x5b, 0x7a, 0xe2, 0x79, 0xbf}
DEFINE_GUID!{GUID_DSFX_STANDARD_CHORUS,
    0xefe6629c, 0x81f7, 0x4281, 0xbd, 0x91, 0xc9, 0xd6, 0x04, 0xa9, 0x5a, 0xf6}
DEFINE_GUID!{GUID_DSFX_STANDARD_FLANGER,
    0xefca3d92, 0xdfd8, 0x4672, 0xa6, 0x03, 0x74, 0x20, 0x89, 0x4b, 0xad, 0x98}
DEFINE_GUID!{GUID_DSFX_STANDARD_ECHO,
    0xef3e932c, 0xd40b, 0x4f51, 0x8c, 0xcf, 0x3f, 0x98, 0xf1, 0xb2, 0x9d, 0x5d}
DEFINE_GUID!{GUID_DSFX_STANDARD_DISTORTION,
    0xef114c90, 0xcd1d, 0x484e, 0x96, 0xe5, 0x09, 0xcf, 0xaf, 0x91, 0x2a, 0x21}
DEFINE_GUID!{GUID_DSFX_STANDARD_COMPRESSOR,
    0xef011f79, 0x4000, 0x406d, 0x87, 0xaf, 0xbf, 0xfb, 0x3f, 0xc3, 0x9d, 0x57}
DEFINE_GUID!{GUID_DSFX_STANDARD_PARAMEQ,
    0x120ced89, 0x3bf4, 0x4173, 0xa1, 0x32, 0x3c, 0xb4, 0x06, 0xcf, 0x32, 0x31}
DEFINE_GUID!{GUID_DSFX_STANDARD_I3DL2REVERB,
    0xef985e71, 0xd5c7, 0x42d4, 0xba, 0x4d, 0x2d, 0x07, 0x3e, 0x2e, 0x96, 0xf4}
DEFINE_GUID!{GUID_DSFX_WAVES_REVERB,
    0x87fc0268, 0x9a55, 0x4360, 0x95, 0xaa, 0x00, 0x4a, 0x1d, 0x9d, 0xe2, 0x6c}
DEFINE_GUID!{GUID_DSCFX_CLASS_AEC,
    0xbf963d80, 0xc559, 0x11d0, 0x8a, 0x2b, 0x00, 0xa0, 0xc9, 0x25, 0x5a, 0xc1}
DEFINE_GUID!{GUID_DSCFX_MS_AEC,
    0xcdebb919, 0x379a, 0x488a, 0x87, 0x65, 0xf5, 0x3c, 0xfd, 0x36, 0xde, 0x40}
DEFINE_GUID!{GUID_DSCFX_SYSTEM_AEC,
    0x1c22c56d, 0x9879, 0x4f5b, 0xa3, 0x89, 0x27, 0x99, 0x6d, 0xdc, 0x28, 0x10}
DEFINE_GUID!{GUID_DSCFX_CLASS_NS,
    0xe07f903f, 0x62fd, 0x4e60, 0x8c, 0xdd, 0xde, 0xa7, 0x23, 0x66, 0x65, 0xb5}
DEFINE_GUID!{GUID_DSCFX_MS_NS,
    0x11c5c73b, 0x66e9, 0x4ba1, 0xa0, 0xba, 0xe8, 0x14, 0xc6, 0xee, 0xd9, 0x2d}
DEFINE_GUID!{GUID_DSCFX_SYSTEM_NS,
    0x5ab0882e, 0x7274, 0x4516, 0x87, 0x7d, 0x4e, 0xee, 0x99, 0xba, 0x4f, 0xd0}
