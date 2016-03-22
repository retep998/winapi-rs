// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to dsound.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn DirectSoundCaptureCreate(
        pcGuidDevice: LPCGUID, ppDSC: *mut LPDIRECTSOUNDCAPTURE, pUnkOuter: LPUNKNOWN,
    ) -> HRESULT;
    pub fn DirectSoundCaptureCreate8(
        pcGuidDevice: LPCGUID, ppDSC8: *mut LPDIRECTSOUNDCAPTURE8, pUnkOuter: LPUNKNOWN,
    ) -> HRESULT;
    pub fn DirectSoundCaptureEnumerateA(
        pDSEnumCallback: LPDSENUMCALLBACKA, pContext: LPVOID,
    ) -> HRESULT;
    pub fn DirectSoundCaptureEnumerateW(
        pDSEnumCallback: LPDSENUMCALLBACKW, pContext: LPVOID,
    ) -> HRESULT;
    pub fn DirectSoundCreate(
        pcGuidDevice: LPCGUID, ppDS: *mut LPDIRECTSOUND, pUnkOuter: LPUNKNOWN,
    ) -> HRESULT;
    pub fn DirectSoundCreate8(
        pcGuidDevice: LPCGUID, ppDS8: *mut LPDIRECTSOUND8, pUnkOuter: LPUNKNOWN,
    ) -> HRESULT;
    pub fn DirectSoundEnumerateA(
        pDSEnumCallback: LPDSENUMCALLBACKA, pContext: LPVOID,
    ) -> HRESULT;
    pub fn DirectSoundEnumerateA(
        pDSEnumCallback: LPDSENUMCALLBACKW, pContext: LPVOID,
    ) -> HRESULT;
    pub fn DirectSoundFullDuplexCreate(
        pcGuidCaptureDevice: LPCGUID, pcGuidRenderDevice: LPCGUID,
        pcDSCBufferDesc: LPCDSCBUFFERDESC, pcDSBufferDesc: LPCDSBUFFERDESC, hWnd: HWND,
        dwLevel: DWORD, ppDSFD: *mut LPDIRECTSOUNDFULLDUPLEX,
        ppDSCBuffer8: *mut LPDIRECTSOUNDCAPTUREBUFFER8, ppDSBuffer8: *mut LPDIRECTSOUNDBUFFER8,
        pUnkOuter: LPUNKNOWN,
    ) -> HRESULT;
    pub fn GetDeviceID(pGuidSrc: LPCGUID, pGuidDest: LPGUID) -> HRESULT;
}
