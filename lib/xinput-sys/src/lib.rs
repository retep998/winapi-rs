// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to xinput.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn XInputEnable(enable: BOOL);
    pub fn XInputGetAudioDeviceIds(
        dwUserIndex: DWORD, pRenderDeviceId: LPWSTR, pRenderCount: *mut UINT,
        pCaptureDeviceId: LPWSTR, pCaptureCount: *mut UINT
    ) -> DWORD;
    pub fn XInputGetBatteryInformation(
        dwUserIndex: DWORD, devType: BYTE, pBatteryInformation: *mut XINPUT_BATTERY_INFORMATION
    ) -> DWORD;
    pub fn XInputGetCapabilities(
        dwUserIndex: DWORD, dwFlags: DWORD, pCapabilities: *mut XINPUT_CAPABILITIES
    ) -> DWORD;
    pub fn XInputGetDSoundAudioDeviceGuids(
        dwUserIndex: DWORD, pDSoundRenderGuid: *mut GUID, pDSoundCaptureGuid: *mut GUID
    ) -> DWORD;
    pub fn XInputGetKeystroke(
        dwUserIndex: DWORD, dwReserved: DWORD, pKeystroke: PXINPUT_KEYSTROKE
    ) -> DWORD;
    pub fn XInputGetState(dwUserIndex: DWORD, pState: *mut XINPUT_STATE) -> DWORD;
    pub fn XInputSetState(dwUserIndex: DWORD, pVibration: *mut XINPUT_VIBRATION) -> DWORD;
}
