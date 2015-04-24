// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to xinput.
#![feature(no_std)]
#![no_std]
#![allow(non_snake_case)]
extern crate winapi;
use winapi::*;

#[repr(C)]
pub struct XINPUT_BATTERY_INFORMATION {
    pub BatteryType: BYTE,
    pub BatteryLevel: BYTE
}

#[repr(C)]
pub struct XINPUT_CAPABILITIES {
    pub Type: BYTE,
    pub SubType: BYTE,
    pub Flags: WORD,
    pub Gamepad: XINPUT_GAMEPAD,
    pub Vibration: XINPUT_VIBRATION
}

#[repr(C)]
pub struct XINPUT_GAMEPAD {
    pub wButtons: WORD,
    pub bLeftTrigger: BYTE,
    pub bRightTrigger: BYTE,
    pub sThumbLX: SHORT,
    pub sThumbLY: SHORT,
    pub sThumbRX: SHORT,
    pub sThumbRY: SHORT
}

#[repr(C)]
pub struct XINPUT_KEYSTROKE {
    pub VirtualKey: WORD,
    pub Unicode: WCHAR,
    pub Flags: WORD,
    pub UserIndex: BYTE,
    pub HidCode: BYTE
}

#[repr(C)]
pub struct XINPUT_STATE {
    pub dwPacketNumber: DWORD,
    pub Gamepad: XINPUT_GAMEPAD
}

#[repr(C)]
pub struct XINPUT_VIBRATION {
    pub wLeftMotorSpeed: WORD,
    pub wRightMotorSpeed: WORD
}

extern "system" {
    pub fn XInputEnable(enable: BOOL);

    pub fn XInputGetAudioDeviceIds(
        dwUserIndex: DWORD,
        pRenderDeviceId: LPWSTR,
        pRenderCount: &mut UINT,
        pCaptureDeviceId: LPWSTR,
        pCaptureCount: &mut UINT) -> DWORD;

    pub fn XInputGetBatteryInformation(
        dwUserIndex: DWORD,
        devType: BYTE,
        pBatteryInformation: &mut XINPUT_BATTERY_INFORMATION) -> DWORD;

    pub fn XInputGetCapabilities(
        dwUserIndex: DWORD,
        dwFlags: DWORD,
        pCapabilities: &mut XINPUT_CAPABILITIES) -> DWORD;

    // DWORD WINAPI XInputGetDSoundAudioDeviceGuids(
    //   DWORD dwUserIndex,
    //   GUID* pDSoundRenderGuid,
    //   GUID* pDSoundCaptureGuid
    // );

    pub fn XInputGetKeystroke(
        dwUserIndex: DWORD,
        dwReserved: DWORD,
        pKeystroke: &mut XINPUT_KEYSTROKE) -> DWORD;

    pub fn XInputGetState(
        dwUserIndex: DWORD,
        pState: &mut XINPUT_STATE) -> DWORD;

    pub fn XInputSetState(
        dwUserIndex: DWORD,
        pVibration: &mut XINPUT_VIBRATION) -> DWORD;
}
