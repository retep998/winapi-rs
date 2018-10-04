// Copyright © 2017-2018 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::{UINT_PTR};
use shared::guiddef::{GUID, LPGUID, REFGUID, REFIID};
use shared::minwindef::{
    BOOL, BYTE, DWORD, FILETIME, HINSTANCE, LPDWORD, LPLONG, LPVOID, MAKELONG, MAX_PATH, UINT, WORD
};
use shared::ntdef::{CHAR, HANDLE, HRESULT, LONG, LPCSTR, LPCWSTR, LPSTR, LPWSTR, WCHAR};
use shared::windef::{HWND, POINT, RECT};
use shared::winerror::{
    CLASS_E_NOAGGREGATION, ERROR_ALREADY_INITIALIZED, ERROR_BAD_DRIVER_LEVEL, ERROR_BUSY, 
    ERROR_FILE_NOT_FOUND, ERROR_INVALID_ACCESS, ERROR_NOT_READY, ERROR_OLD_WIN_VERSION, 
    ERROR_READ_FAULT, ERROR_RMODE_APP, E_ACCESSDENIED, E_FAIL, E_INVALIDARG, E_NOINTERFACE, 
    E_NOTIMPL, E_OUTOFMEMORY, FACILITY_WIN32, REGDB_E_CLASSNOTREG, SEVERITY_ERROR, S_FALSE, S_OK
};
use um::mmsystem::{MMRESULT};
use um::unknwnbase::{IUnknown, IUnknownVtbl, LPUNKNOWN};
pub const DIRECTINPUT_VERSION: DWORD = 0x0800;
DEFINE_GUID!{CLSID_DirectInput,
    0x25e609e0, 0xb259, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
DEFINE_GUID!{CLSID_DirectInputDevice,
    0x25e609e1, 0xb259, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
DEFINE_GUID!{CLSID_DirectInput8,
    0x25e609e4, 0xb259, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
DEFINE_GUID!{CLSID_DirectInputDevice8,
    0x25e609e5, 0xb259, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
DEFINE_GUID!{IID_IDirectInputA,
    0x89521360, 0xaa8a, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
DEFINE_GUID!{IID_IDirectInputW,
    0x89521361, 0xaa8a, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
DEFINE_GUID!{IID_IDirectInput2A,
    0x5944e662, 0xaa8a, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
DEFINE_GUID!{IID_IDirectInput2W,
    0x5944e663, 0xaa8a, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
DEFINE_GUID!{IID_IDirectInput7A,
    0x9a4cb684, 0x236d, 0x11d3, 0x8e, 0x9d, 0x00, 0xc0, 0x4f, 0x68, 0x44, 0xae}
DEFINE_GUID!{IID_IDirectInput7W,
    0x9a4cb685, 0x236d, 0x11d3, 0x8e, 0x9d, 0x00, 0xc0, 0x4f, 0x68, 0x44, 0xae}
DEFINE_GUID!{IID_IDirectInput8A,
    0xbf798030, 0x483a, 0x4da2, 0xaa, 0x99, 0x5d, 0x64, 0xed, 0x36, 0x97, 0x00}
DEFINE_GUID!{IID_IDirectInput8W,
    0xbf798031, 0x483a, 0x4da2, 0xaa, 0x99, 0x5d, 0x64, 0xed, 0x36, 0x97, 0x00}
DEFINE_GUID!{IID_IDirectInputDeviceA,
    0x5944e680, 0xc92e, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
DEFINE_GUID!{IID_IDirectInputDeviceW,
    0x5944e681, 0xc92e, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
DEFINE_GUID!{IID_IDirectInputDevice2A,
    0x5944e682, 0xc92e, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
DEFINE_GUID!{IID_IDirectInputDevice2W,
    0x5944e683, 0xc92e, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
DEFINE_GUID!{IID_IDirectInputDevice7A,
    0x57d7c6bc, 0x2356, 0x11d3, 0x8e, 0x9d, 0x00, 0xc0, 0x4f, 0x68, 0x44, 0xae}
DEFINE_GUID!{IID_IDirectInputDevice7W,
    0x57d7c6bd, 0x2356, 0x11d3, 0x8e, 0x9d, 0x00, 0xc0, 0x4f, 0x68, 0x44, 0xae}
DEFINE_GUID!{IID_IDirectInputDevice8A,
    0x54d41080, 0xdc15, 0x4833, 0xa4, 0x1b, 0x74, 0x8f, 0x73, 0xa3, 0x81, 0x79}
DEFINE_GUID!{IID_IDirectInputDevice8W,
    0x54d41081, 0xdc15, 0x4833, 0xa4, 0x1b, 0x74, 0x8f, 0x73, 0xa3, 0x81, 0x79}
DEFINE_GUID!{IID_IDirectInputEffect,
    0xe7e1f7c0, 0x88d2, 0x11d0, 0x9a, 0xd0, 0x00, 0xa0, 0xc9, 0xa0, 0x6e, 0x35}
DEFINE_GUID!{GUID_XAxis,
    0xa36d02e0, 0xc9f3, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
DEFINE_GUID!{GUID_YAxis,
    0xa36d02e1, 0xc9f3, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
DEFINE_GUID!{GUID_ZAxis,
    0xa36d02e2, 0xc9f3, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
DEFINE_GUID!{GUID_RxAxis,
    0xa36d02f4, 0xc9f3, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
DEFINE_GUID!{GUID_RyAxis,
    0xa36d02f5, 0xc9f3, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
DEFINE_GUID!{GUID_RzAxis,
    0xa36d02e3, 0xc9f3, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
DEFINE_GUID!{GUID_Slider,
    0xa36d02e4, 0xc9f3, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
DEFINE_GUID!{GUID_Button,
    0xa36d02f0, 0xc9f3, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
DEFINE_GUID!{GUID_Key,
    0x55728220, 0xd33c, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
DEFINE_GUID!{GUID_POV,
    0xa36d02f2, 0xc9f3, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
DEFINE_GUID!{GUID_Unknown,
    0xa36d02f3, 0xc9f3, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
DEFINE_GUID!{GUID_SysMouse,
    0x6f1d2b60, 0xd5a0, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
DEFINE_GUID!{GUID_SysKeyboard,
    0x6f1d2b61, 0xd5a0, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
DEFINE_GUID!{GUID_Joystick,
    0x6f1d2b70, 0xd5a0, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
DEFINE_GUID!{GUID_SysMouseEm,
    0x6f1d2b80, 0xd5a0, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
DEFINE_GUID!{GUID_SysMouseEm2,
    0x6f1d2b81, 0xd5a0, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
DEFINE_GUID!{GUID_SysKeyboardEm,
    0x6f1d2b82, 0xd5a0, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
DEFINE_GUID!{GUID_SysKeyboardEm2,
    0x6f1d2b83, 0xd5a0, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
DEFINE_GUID!{GUID_ConstantForce,
    0x13541c20, 0x8e33, 0x11d0, 0x9a, 0xd0, 0x00, 0xa0, 0xc9, 0xa0, 0x6e, 0x35}
DEFINE_GUID!{GUID_RampForce,
    0x13541c21, 0x8e33, 0x11d0, 0x9a, 0xd0, 0x00, 0xa0, 0xc9, 0xa0, 0x6e, 0x35}
DEFINE_GUID!{GUID_Square,
    0x13541c22, 0x8e33, 0x11d0, 0x9a, 0xd0, 0x00, 0xa0, 0xc9, 0xa0, 0x6e, 0x35}
DEFINE_GUID!{GUID_Sine,
    0x13541c23, 0x8e33, 0x11d0, 0x9a, 0xd0, 0x00, 0xa0, 0xc9, 0xa0, 0x6e, 0x35}
DEFINE_GUID!{GUID_Triangle,
    0x13541c24, 0x8e33, 0x11d0, 0x9a, 0xd0, 0x00, 0xa0, 0xc9, 0xa0, 0x6e, 0x35}
DEFINE_GUID!{GUID_SawtoothUp,
    0x13541c25, 0x8e33, 0x11d0, 0x9a, 0xd0, 0x00, 0xa0, 0xc9, 0xa0, 0x6e, 0x35}
DEFINE_GUID!{GUID_SawtoothDown,
    0x13541c26, 0x8e33, 0x11d0, 0x9a, 0xd0, 0x00, 0xa0, 0xc9, 0xa0, 0x6e, 0x35}
DEFINE_GUID!{GUID_Spring,
    0x13541c27, 0x8e33, 0x11d0, 0x9a, 0xd0, 0x00, 0xa0, 0xc9, 0xa0, 0x6e, 0x35}
DEFINE_GUID!{GUID_Damper,
    0x13541c28, 0x8e33, 0x11d0, 0x9a, 0xd0, 0x00, 0xa0, 0xc9, 0xa0, 0x6e, 0x35}
DEFINE_GUID!{GUID_Inertia,
    0x13541c29, 0x8e33, 0x11d0, 0x9a, 0xd0, 0x00, 0xa0, 0xc9, 0xa0, 0x6e, 0x35}
DEFINE_GUID!{GUID_Friction,
    0x13541c2a, 0x8e33, 0x11d0, 0x9a, 0xd0, 0x00, 0xa0, 0xc9, 0xa0, 0x6e, 0x35}
DEFINE_GUID!{GUID_CustomForce,
    0x13541c2b, 0x8e33, 0x11d0, 0x9a, 0xd0, 0x00, 0xa0, 0xc9, 0xa0, 0x6e, 0x35}
pub const DIEFT_ALL: DWORD = 0x00000000;
pub const DIEFT_CONSTANTFORCE: DWORD = 0x00000001;
pub const DIEFT_RAMPFORCE: DWORD = 0x00000002;
pub const DIEFT_PERIODIC: DWORD = 0x00000003;
pub const DIEFT_CONDITION: DWORD = 0x00000004;
pub const DIEFT_CUSTOMFORCE: DWORD = 0x00000005;
pub const DIEFT_HARDWARE: DWORD = 0x000000FF;
pub const DIEFT_FFATTACK: DWORD = 0x00000200;
pub const DIEFT_FFFADE: DWORD = 0x00000400;
pub const DIEFT_SATURATION: DWORD = 0x00000800;
pub const DIEFT_POSNEGCOEFFICIENTS: DWORD = 0x00001000;
pub const DIEFT_POSNEGSATURATION: DWORD = 0x00002000;
pub const DIEFT_DEADBAND: DWORD = 0x00004000;
pub const DIEFT_STARTDELAY: DWORD = 0x00008000;
#[inline]
pub fn DIEFT_GETTYPE(eft: DWORD) -> BYTE {
    (eft & 0xff) as BYTE
}
pub const DI_DEGREES : DWORD = 100;
pub const DI_FFNOMINALMAX : LONG = 10000;
pub const DI_SECONDS : DWORD = 1000000;
STRUCT!{struct DICONSTANTFORCE {
    lMagnitude: LONG,
}}
pub type LPDICONSTANTFORCE = *mut DICONSTANTFORCE;
pub type LPCDICONSTANTFORCE = *const DICONSTANTFORCE;
STRUCT!{struct DIRAMPFORCE {
    lStart: LONG,
    lEnd: LONG,
}}
pub type LPDIRAMPFORCE = *mut DIRAMPFORCE;
pub type LPCDIRAMPFORCE = *const DIRAMPFORCE;
STRUCT!{struct DIPERIODIC {
    dwMagnitude: DWORD,
    lOffset: LONG,
    dwPhase: DWORD,
    dwPeriod: DWORD,
}}
pub type LPDIPERIODIC = *mut DIPERIODIC;
pub type LPCDIPERIODIC = *const DIPERIODIC;
STRUCT!{struct DICONDITION {
    lOffset: LONG,
    lPositiveCoefficient: LONG,
    lNegativeCoefficient: LONG,
    dwPositiveSaturation: DWORD,
    dwNegativeSaturation: DWORD,
    lDeadBand: LONG,
}}
pub type LPDICONDITION = *mut DICONDITION;
pub type LPCDICONDITION = *const DICONDITION;
STRUCT!{struct DICUSTOMFORCE {
    cChannels: DWORD,
    dwSamplePeriod: DWORD,
    cSamples: DWORD,
    rglForceData: LPLONG,
}}
pub type LPDICUSTOMFORCE = *mut DICUSTOMFORCE;
pub type LPCDICUSTOMFORCE = *const DICUSTOMFORCE;
STRUCT!{struct DIENVELOPE {
    dwSize: DWORD, 
    dwAttackLevel: DWORD,
    dwAttackTime: DWORD, 
    dwFadeLevel: DWORD,
    dwFadeTime: DWORD, 
}}
pub type LPDIENVELOPE = *mut DIENVELOPE;
pub type LPCDIENVELOPE = *const DIENVELOPE;
STRUCT!{struct DIEFFECT_DX5 {
    dwSize: DWORD,
    dwFlags: DWORD,
    dwDuration: DWORD,
    dwSamplePeriod: DWORD,
    dwGain: DWORD,
    dwTriggerButton: DWORD,
    dwTriggerRepeatInterval: DWORD,
    cAxes: DWORD,
    rgdwAxes: LPDWORD,
    rglDirection: LPLONG,
    lpEnvelope: LPDIENVELOPE,
    cbTypeSpecificParams: DWORD,
    lpvTypeSpecificParams: LPVOID,
}}
pub type LPDIEFFECT_DX5 = *mut DIEFFECT_DX5;
pub type LPCDIEFFECT_DX5 = *const DIEFFECT_DX5;
STRUCT!{struct DIEFFECT {
    dwSize: DWORD,
    dwFlags: DWORD,
    dwDuration: DWORD,
    dwSamplePeriod: DWORD,
    dwGain: DWORD,
    dwTriggerButton: DWORD,
    dwTriggerRepeatInterval: DWORD,
    cAxes: DWORD,
    rgdwAxes: LPDWORD,
    rglDirection: LPLONG,
    lpEnvelope: LPDIENVELOPE,
    cbTypeSpecificParams: DWORD,
    lpvTypeSpecificParams: LPVOID,
    dwStartDelay: DWORD,
}}
pub type LPDIEFFECT = *mut DIEFFECT;
pub type DIEFFECT_DX6 = DIEFFECT;
pub type LPDIEFFECT_DX6 = LPDIEFFECT;
pub type LPCDIEFFECT = *const DIEFFECT;
STRUCT!{struct DIFILEEFFECT {
    dwSize: DWORD,
    GuidEffect: GUID,
    lpDiEffect: LPCDIEFFECT,
    szFriendlyName: [CHAR; MAX_PATH],
}}
pub type LPDIFILEEFFECT = *mut DIFILEEFFECT;
pub type LPCDIFILEEFFECT = *const DIFILEEFFECT;
FN!{stdcall LPDIENUMEFFECTSINFILECALLBACK(
    lpDiFileEf: LPCDIFILEEFFECT,
    pvRef: LPVOID,
) -> BOOL}
pub const DIEFF_OBJECTIDS: DWORD = 0x00000001;
pub const DIEFF_OBJECTOFFSETS: DWORD = 0x00000002;
pub const DIEFF_CARTESIAN: DWORD = 0x00000010;
pub const DIEFF_POLAR: DWORD = 0x00000020;
pub const DIEFF_SPHERICAL: DWORD = 0x00000040;
pub const DIEP_DURATION: DWORD = 0x00000001;
pub const DIEP_SAMPLEPERIOD: DWORD = 0x00000002;
pub const DIEP_GAIN: DWORD = 0x00000004;
pub const DIEP_TRIGGERBUTTON: DWORD = 0x00000008;
pub const DIEP_TRIGGERREPEATINTERVAL: DWORD = 0x00000010;
pub const DIEP_AXES: DWORD = 0x00000020;
pub const DIEP_DIRECTION: DWORD = 0x00000040;
pub const DIEP_ENVELOPE: DWORD = 0x00000080;
pub const DIEP_TYPESPECIFICPARAMS: DWORD = 0x00000100;
pub const DIEP_STARTDELAY: DWORD = 0x00000200;
pub const DIEP_ALLPARAMS_DX5: DWORD = 0x000001FF;
pub const DIEP_ALLPARAMS: DWORD = 0x000003FF;
pub const DIEP_START: DWORD = 0x20000000;
pub const DIEP_NORESTART: DWORD = 0x40000000;
pub const DIEP_NODOWNLOAD: DWORD = 0x80000000;
pub const DIEB_NOTRIGGER: DWORD = 0xFFFFFFFF;
pub const DIES_SOLO: DWORD = 0x00000001;
pub const DIES_NODOWNLOAD: DWORD = 0x80000000;
pub const DIEGES_PLAYING: DWORD = 0x00000001;
pub const DIEGES_EMULATED: DWORD = 0x00000002;
STRUCT!{struct DIEFFESCAPE {
    dwSize: DWORD,
    dwCommand: DWORD,
    lpvInBuffer: LPVOID,
    cbInBuffer: DWORD,
    lpvOutBuffer: LPVOID,
    cbOutBuffer: DWORD,
}}
pub type LPDIEFFESCAPE = *mut DIEFFESCAPE;
RIDL!{#[uuid(0xe7e1f7c0, 0x88d2, 0x11d0, 0x9a, 0xd0, 0x00, 0xa0, 0xc9, 0xa0, 0x6e, 0x35)]
interface IDirectInputEffect(IDirectInputEffectVtbl): IUnknown(IUnknownVtbl) {
    fn Initialize(
        hinst: HINSTANCE,
        dwVersion: DWORD,
        rguid: REFGUID,
    ) -> HRESULT,
    fn GetEffectGuid(
        pguid: LPGUID,
    ) -> HRESULT,
    fn GetParameters(
        peff: LPDIEFFECT,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn SetParameters(
        peff: LPCDIEFFECT,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn Start(
        dwIterations: DWORD,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn Stop() -> HRESULT,
    fn GetEffectStatus(
        pdwFlags: LPDWORD,
    ) -> HRESULT,
    fn Download() -> HRESULT,
    fn Unload() -> HRESULT,
    fn Escape(
        pesc: LPDIEFFESCAPE,
    ) -> HRESULT,
}}
pub type LPDIRECTINPUTEFFECT = *mut IDirectInputEffect;
pub const DI8DEVCLASS_ALL: DWORD = 0;
pub const DI8DEVCLASS_DEVICE: DWORD = 1;
pub const DI8DEVCLASS_POINTER: DWORD = 2;
pub const DI8DEVCLASS_KEYBOARD: DWORD = 3;
pub const DI8DEVCLASS_GAMECTRL: DWORD = 4;
pub const DI8DEVTYPE_DEVICE: DWORD = 0x11;
pub const DI8DEVTYPE_MOUSE: DWORD = 0x12;
pub const DI8DEVTYPE_KEYBOARD: DWORD = 0x13;
pub const DI8DEVTYPE_JOYSTICK: DWORD = 0x14;
pub const DI8DEVTYPE_GAMEPAD: DWORD = 0x15;
pub const DI8DEVTYPE_DRIVING: DWORD = 0x16;
pub const DI8DEVTYPE_FLIGHT: DWORD = 0x17;
pub const DI8DEVTYPE_1STPERSON: DWORD = 0x18;
pub const DI8DEVTYPE_DEVICECTRL: DWORD = 0x19;
pub const DI8DEVTYPE_SCREENPOINTER: DWORD = 0x1A;
pub const DI8DEVTYPE_REMOTE: DWORD = 0x1B;
pub const DI8DEVTYPE_SUPPLEMENTAL: DWORD = 0x1C;
pub const DIDEVTYPE_HID: DWORD = 0x00010000;
pub const DI8DEVTYPEMOUSE_UNKNOWN: DWORD = 1;
pub const DI8DEVTYPEMOUSE_TRADITIONAL: DWORD = 2;
pub const DI8DEVTYPEMOUSE_FINGERSTICK: DWORD = 3;
pub const DI8DEVTYPEMOUSE_TOUCHPAD: DWORD = 4;
pub const DI8DEVTYPEMOUSE_TRACKBALL: DWORD = 5;
pub const DI8DEVTYPEMOUSE_ABSOLUTE: DWORD = 6;
pub const DI8DEVTYPEKEYBOARD_UNKNOWN: DWORD = 0;
pub const DI8DEVTYPEKEYBOARD_PCXT: DWORD = 1;
pub const DI8DEVTYPEKEYBOARD_OLIVETTI: DWORD = 2;
pub const DI8DEVTYPEKEYBOARD_PCAT: DWORD = 3;
pub const DI8DEVTYPEKEYBOARD_PCENH: DWORD = 4;
pub const DI8DEVTYPEKEYBOARD_NOKIA1050: DWORD = 5;
pub const DI8DEVTYPEKEYBOARD_NOKIA9140: DWORD = 6;
pub const DI8DEVTYPEKEYBOARD_NEC98: DWORD = 7;
pub const DI8DEVTYPEKEYBOARD_NEC98LAPTOP: DWORD = 8;
pub const DI8DEVTYPEKEYBOARD_NEC98106: DWORD = 9;
pub const DI8DEVTYPEKEYBOARD_JAPAN106: DWORD = 10;
pub const DI8DEVTYPEKEYBOARD_JAPANAX: DWORD = 11;
pub const DI8DEVTYPEKEYBOARD_J3100: DWORD = 12;
pub const DI8DEVTYPE_LIMITEDGAMESUBTYPE: DWORD = 1;
pub const DI8DEVTYPEJOYSTICK_LIMITED: DWORD = DI8DEVTYPE_LIMITEDGAMESUBTYPE;
pub const DI8DEVTYPEJOYSTICK_STANDARD: DWORD = 2;
pub const DI8DEVTYPEGAMEPAD_LIMITED: DWORD = DI8DEVTYPE_LIMITEDGAMESUBTYPE;
pub const DI8DEVTYPEGAMEPAD_STANDARD: DWORD = 2;
pub const DI8DEVTYPEGAMEPAD_TILT: DWORD = 3;
pub const DI8DEVTYPEDRIVING_LIMITED: DWORD = DI8DEVTYPE_LIMITEDGAMESUBTYPE;
pub const DI8DEVTYPEDRIVING_COMBINEDPEDALS: DWORD = 2;
pub const DI8DEVTYPEDRIVING_DUALPEDALS: DWORD = 3;
pub const DI8DEVTYPEDRIVING_THREEPEDALS: DWORD = 4;
pub const DI8DEVTYPEDRIVING_HANDHELD: DWORD = 5;
pub const DI8DEVTYPEFLIGHT_LIMITED: DWORD = DI8DEVTYPE_LIMITEDGAMESUBTYPE;
pub const DI8DEVTYPEFLIGHT_STICK: DWORD = 2;
pub const DI8DEVTYPEFLIGHT_YOKE: DWORD = 3;
pub const DI8DEVTYPEFLIGHT_RC: DWORD = 4;
pub const DI8DEVTYPE1STPERSON_LIMITED: DWORD = DI8DEVTYPE_LIMITEDGAMESUBTYPE;
pub const DI8DEVTYPE1STPERSON_UNKNOWN: DWORD = 2;
pub const DI8DEVTYPE1STPERSON_SIXDOF: DWORD = 3;
pub const DI8DEVTYPE1STPERSON_SHOOTER: DWORD = 4;
pub const DI8DEVTYPESCREENPTR_UNKNOWN: DWORD = 2;
pub const DI8DEVTYPESCREENPTR_LIGHTGUN: DWORD = 3;
pub const DI8DEVTYPESCREENPTR_LIGHTPEN: DWORD = 4;
pub const DI8DEVTYPESCREENPTR_TOUCH: DWORD = 5;
pub const DI8DEVTYPEREMOTE_UNKNOWN: DWORD = 2;
pub const DI8DEVTYPEDEVICECTRL_UNKNOWN: DWORD = 2;
pub const DI8DEVTYPEDEVICECTRL_COMMSSELECTION: DWORD = 3;
pub const DI8DEVTYPEDEVICECTRL_COMMSSELECTION_HARDWIRED: DWORD = 4;
pub const DI8DEVTYPESUPPLEMENTAL_UNKNOWN: DWORD = 2;
pub const DI8DEVTYPESUPPLEMENTAL_2NDHANDCONTROLLER: DWORD = 3;
pub const DI8DEVTYPESUPPLEMENTAL_HEADTRACKER: DWORD = 4;
pub const DI8DEVTYPESUPPLEMENTAL_HANDTRACKER: DWORD = 5;
pub const DI8DEVTYPESUPPLEMENTAL_SHIFTSTICKGATE: DWORD = 6;
pub const DI8DEVTYPESUPPLEMENTAL_SHIFTER: DWORD = 7;
pub const DI8DEVTYPESUPPLEMENTAL_THROTTLE: DWORD = 8;
pub const DI8DEVTYPESUPPLEMENTAL_SPLITTHROTTLE: DWORD = 9;
pub const DI8DEVTYPESUPPLEMENTAL_COMBINEDPEDALS: DWORD = 10;
pub const DI8DEVTYPESUPPLEMENTAL_DUALPEDALS: DWORD = 11;
pub const DI8DEVTYPESUPPLEMENTAL_THREEPEDALS: DWORD = 12;
pub const DI8DEVTYPESUPPLEMENTAL_RUDDERPEDALS: DWORD = 13;
#[inline]
pub fn GET_DIDEVICE_TYPE(dwDevType: DWORD) -> BYTE {
    (dwDevType & 0xff) as BYTE
}
#[inline]
pub fn GET_DIDEVICE_SUBTYPE(dwDevType: DWORD) -> BYTE {
    ((dwDevType >> 8) & 0xff) as BYTE
}
STRUCT!{struct DIDEVCAPS_DX3 {
    dwSize: DWORD,
    dwFlags: DWORD,
    dwDevType: DWORD,
    dwAxes: DWORD,
    dwButtons: DWORD,
    dwPOVs: DWORD,
}}
pub type LPDIDEVCAPS_DX3 = *mut DIDEVCAPS_DX3;
STRUCT!{struct DIDEVCAPS {
    dwSize: DWORD,
    dwFlags: DWORD,
    dwDevType: DWORD,
    dwAxes: DWORD,
    dwButtons: DWORD,
    dwPOVs: DWORD,
    dwFFSamplePeriod: DWORD,
    dwFFMinTimeResolution: DWORD,
    dwFirmwareRevision: DWORD,
    dwHardwareRevision: DWORD,
    dwFFDriverVersion: DWORD,
}}
pub type LPDIDEVCAPS = *mut DIDEVCAPS;
pub const DIDC_ATTACHED: DWORD = 0x00000001;
pub const DIDC_POLLEDDEVICE: DWORD = 0x00000002;
pub const DIDC_EMULATED: DWORD = 0x00000004;
pub const DIDC_POLLEDDATAFORMAT: DWORD = 0x00000008;
pub const DIDC_FORCEFEEDBACK: DWORD = 0x00000100;
pub const DIDC_FFATTACK: DWORD = 0x00000200;
pub const DIDC_FFFADE: DWORD = 0x00000400;
pub const DIDC_SATURATION: DWORD = 0x00000800;
pub const DIDC_POSNEGCOEFFICIENTS: DWORD = 0x00001000;
pub const DIDC_POSNEGSATURATION: DWORD = 0x00002000;
pub const DIDC_DEADBAND: DWORD = 0x00004000;
pub const DIDC_STARTDELAY: DWORD = 0x00008000;
pub const DIDC_ALIAS: DWORD = 0x00010000;
pub const DIDC_PHANTOM: DWORD = 0x00020000;
pub const DIDC_HIDDEN: DWORD = 0x00040000;
pub const DIDFT_ALL: DWORD = 0x00000000;
pub const DIDFT_RELAXIS: DWORD = 0x00000001;
pub const DIDFT_ABSAXIS: DWORD = 0x00000002;
pub const DIDFT_AXIS: DWORD = 0x00000003;
pub const DIDFT_PSHBUTTON: DWORD = 0x00000004;
pub const DIDFT_TGLBUTTON: DWORD = 0x00000008;
pub const DIDFT_BUTTON: DWORD = 0x0000000C;
pub const DIDFT_POV: DWORD = 0x00000010;
pub const DIDFT_COLLECTION: DWORD = 0x00000040;
pub const DIDFT_NODATA: DWORD = 0x00000080;
pub const DIDFT_ANYINSTANCE: DWORD = 0x00FFFF00;
pub const DIDFT_INSTANCEMASK: DWORD = DIDFT_ANYINSTANCE;
#[inline]
pub fn DIDFT_MAKEINSTANCE(dft: DWORD) -> WORD {
    (dft as WORD) << 8
}
#[inline]
pub fn DIDFT_GETTYPE(dft: DWORD) -> BYTE {
    (dft & 0xff) as BYTE
}
#[inline]
pub fn DIDFT_GETINSTANCE(dft: DWORD) -> WORD {
    ((dft >> 8) & 0xffff) as WORD
}
pub const DIDFT_FFACTUATOR: DWORD = 0x01000000;
pub const DIDFT_FFEFFECTTRIGGER: DWORD = 0x02000000;
pub const DIDFT_OUTPUT: DWORD = 0x10000000;
pub const DIDFT_VENDORDEFINED: DWORD = 0x04000000;
pub const DIDFT_ALIAS: DWORD = 0x08000000;
#[inline]
pub fn DIDFT_ENUMCOLLECTION(dft: DWORD) -> WORD {
    ((dft << 8) & 0xffff) as WORD
}
pub const DIDFT_NOCOLLECTION: DWORD = 0x00FFFF00;
STRUCT!{struct DIOBJECTDATAFORMAT {
    pguid: *const GUID,
    dwOfs: DWORD,
    dwType: DWORD,
    dwFlags: DWORD,
}}
pub type LPDIOBJECTDATAFORMAT = *mut DIOBJECTDATAFORMAT;
pub type LPCDIOBJECTDATAFORMAT = *const DIOBJECTDATAFORMAT;
STRUCT!{struct DIDATAFORMAT {
    dwSize: DWORD,
    dwObjSize: DWORD,
    dwFlags: DWORD,
    dwDataSize: DWORD,
    dwNumObjs: DWORD,
    rgodf: LPDIOBJECTDATAFORMAT,
}}
pub type LPDIDATAFORMAT = *mut DIDATAFORMAT;
pub type LPCDIDATAFORMAT = *const DIDATAFORMAT;
pub const DIDF_ABSAXIS: DWORD = 0x00000001;
pub const DIDF_RELAXIS: DWORD = 0x00000002;
extern "system" {
    pub static c_dfDIMouse: DIDATAFORMAT;
    pub static c_dfDIMouse2: DIDATAFORMAT;
    pub static c_dfDIKeyboard: DIDATAFORMAT;
    pub static c_dfDIJoystick: DIDATAFORMAT;
    pub static c_dfDIJoystick2: DIDATAFORMAT;
    pub fn GetdfDIJoystick() -> LPCDIDATAFORMAT;
}
STRUCT!{struct DIACTIONA {
    uAppData: UINT_PTR,
    dwSemantic: DWORD,
    dwFlags: DWORD,
    u: DIACTIONA_u,
    guidInstance: GUID,
    dwObjID: DWORD,
    dwHow: DWORD,
}}
UNION!{union DIACTIONA_u {
    [u32; 1] [u64; 1],
    lptszActionName lptszActionName_mut: LPCSTR,
    uResIdString uResIdString_mut: UINT,
}}
pub type LPDIACTIONA = *mut DIACTIONA;
STRUCT!{struct DIACTIONW {
    uAppData: UINT_PTR,
    dwSemantic: DWORD,
    dwFlags: DWORD,
    u: DIACTIONW_u,
    guidInstance: GUID,
    dwObjID: DWORD,
    dwHow: DWORD,
}}
UNION!{union DIACTIONW_u {
    [u32; 1] [u64; 1],
    lptszActionName lptszActionName_mut: LPCWSTR,
    uResIdString uResIdString_mut: UINT,
}}
pub type LPDIACTIONW = *mut DIACTIONW;
pub type LPCDIACTIONA = *const DIACTIONA;
pub type LPCDIACTIONW = *const DIACTIONW;
pub const DIA_FORCEFEEDBACK: DWORD = 0x00000001;
pub const DIA_APPMAPPED: DWORD = 0x00000002;
pub const DIA_APPNOMAP: DWORD = 0x00000004;
pub const DIA_NORANGE: DWORD = 0x00000008;
pub const DIA_APPFIXED: DWORD = 0x00000010;
pub const DIAH_UNMAPPED: DWORD = 0x00000000;
pub const DIAH_USERCONFIG: DWORD = 0x00000001;
pub const DIAH_APPREQUESTED: DWORD = 0x00000002;
pub const DIAH_HWAPP: DWORD = 0x00000004;
pub const DIAH_HWDEFAULT: DWORD = 0x00000008;
pub const DIAH_DEFAULT: DWORD = 0x00000020;
pub const DIAH_ERROR: DWORD = 0x80000000;
STRUCT!{struct DIACTIONFORMATA {
    dwSize: DWORD,
    dwActionSize: DWORD,
    dwDataSize: DWORD,
    dwNumActions: DWORD,
    rgoAction: LPDIACTIONA,
    guidActionMap: GUID,
    dwGenre: DWORD,
    dwBufferSize: DWORD,
    lAxisMin: LONG,
    lAxisMax: LONG,
    hInstString: HINSTANCE,
    ftTimeStamp: FILETIME,
    dwCRC: DWORD,
    tszActionMap: [CHAR; MAX_PATH],
}}
pub type LPDIACTIONFORMATA = *mut DIACTIONFORMATA;
STRUCT!{struct DIACTIONFORMATW {
    dwSize: DWORD,
    dwActionSize: DWORD,
    dwDataSize: DWORD,
    dwNumActions: DWORD,
    rgoAction: LPDIACTIONW,
    guidActionMap: GUID,
    dwGenre: DWORD,
    dwBufferSize: DWORD,
    lAxisMin: LONG,
    lAxisMax: LONG,
    hInstString: HINSTANCE,
    ftTimeStamp: FILETIME,
    dwCRC: DWORD,
    tszActionMap: [WCHAR; MAX_PATH],
}}
pub type LPDIACTIONFORMATW = *mut DIACTIONFORMATW;
pub type LPCDIACTIONFORMATA = *const DIACTIONFORMATA;
pub type LPCDIACTIONFORMATW = *const DIACTIONFORMATW;
pub const DIAFTS_NEWDEVICELOW: DWORD = 0xFFFFFFFF;
pub const DIAFTS_NEWDEVICEHIGH: DWORD = 0xFFFFFFFF;
pub const DIAFTS_UNUSEDDEVICELOW: DWORD = 0x00000000;
pub const DIAFTS_UNUSEDDEVICEHIGH: DWORD = 0x00000000;
pub const DIDBAM_DEFAULT: DWORD = 0x00000000;
pub const DIDBAM_PRESERVE: DWORD = 0x00000001;
pub const DIDBAM_INITIALIZE: DWORD = 0x00000002;
pub const DIDBAM_HWDEFAULTS: DWORD = 0x00000004;
pub const DIDSAM_DEFAULT: DWORD = 0x00000000;
pub const DIDSAM_NOUSER: DWORD = 0x00000001;
pub const DIDSAM_FORCESAVE: DWORD = 0x00000002;
pub const DICD_DEFAULT: DWORD = 0x00000000;
pub const DICD_EDIT: DWORD = 0x00000001;
pub type D3DCOLOR = DWORD;
STRUCT!{struct DICOLORSET {
    dwSize: DWORD,
    cTextFore: D3DCOLOR,
    cTextHighlight: D3DCOLOR,
    cCalloutLine: D3DCOLOR,
    cCalloutHighlight: D3DCOLOR,
    cBorder: D3DCOLOR,
    cControlFill: D3DCOLOR,
    cHighlightFill: D3DCOLOR,
    cAreaFill: D3DCOLOR,
}}
pub type LPDICOLORSET = *mut DICOLORSET;
pub type LPCDICOLORSET = *const DICOLORSET;
STRUCT!{struct DICONFIGUREDEVICESPARAMSA {
    dwSize: DWORD,
    dwcUsers: DWORD,
    lptszUserNames: LPSTR,
    dwcFormats: DWORD,
    lprgFormats: LPDIACTIONFORMATA,
    hwnd: HWND,
    dics: DICOLORSET,
    lpUnkDDSTarget: *mut IUnknown,
}}
pub type LPDICONFIGUREDEVICESPARAMSA = *mut DICONFIGUREDEVICESPARAMSA;
STRUCT!{struct DICONFIGUREDEVICESPARAMSW {
    dwSize: DWORD,
    dwcUsers: DWORD,
    lptszUserNames: LPWSTR,
    dwcFormats: DWORD,
    lprgFormats: LPDIACTIONFORMATW,
    hwnd: HWND,
    dics: DICOLORSET,
    lpUnkDDSTarget: *mut IUnknown,
}}
pub type LPDICONFIGUREDEVICESPARAMSW = *mut DICONFIGUREDEVICESPARAMSW;
pub type LPCDICONFIGUREDEVICESPARAMSA = *const DICONFIGUREDEVICESPARAMSA;
pub type LPCDICONFIGUREDEVICESPARAMSW = *const DICONFIGUREDEVICESPARAMSW;
pub const DIDIFT_CONFIGURATION: DWORD = 0x00000001;
pub const DIDIFT_OVERLAY: DWORD = 0x00000002;
pub const DIDAL_CENTERED: DWORD = 0x00000000;
pub const DIDAL_LEFTALIGNED: DWORD = 0x00000001;
pub const DIDAL_RIGHTALIGNED: DWORD = 0x00000002;
pub const DIDAL_MIDDLE: DWORD = 0x00000000;
pub const DIDAL_TOPALIGNED: DWORD = 0x00000004;
pub const DIDAL_BOTTOMALIGNED: DWORD = 0x00000008;
STRUCT!{struct DIDEVICEIMAGEINFOA {
    tszImagePath: [CHAR; MAX_PATH],
    dwFlags: DWORD,
    dwViewID: DWORD,
    rcOverlay: RECT,
    dwObjID: DWORD,
    dwcValidPts: DWORD,
    rgptCalloutLine: [POINT; 5],
    rcCalloutRect: RECT,
    dwTextAlign: DWORD,
}}
pub type LPDIDEVICEIMAGEINFOA = *mut DIDEVICEIMAGEINFOA;
STRUCT!{struct DIDEVICEIMAGEINFOW {
    tszImagePath: [WCHAR; MAX_PATH],
    dwFlags: DWORD,
    dwViewID: DWORD,
    rcOverlay: RECT,
    dwObjID: DWORD,
    dwcValidPts: DWORD,
    rgptCalloutLine: [POINT; 5],
    rcCalloutRect: RECT,
    dwTextAlign: DWORD,
}}
pub type LPDIDEVICEIMAGEINFOW = *mut DIDEVICEIMAGEINFOW;
pub type LPCDIDEVICEIMAGEINFOA = *const DIDEVICEIMAGEINFOA;
pub type LPCDIDEVICEIMAGEINFOW = *const DIDEVICEIMAGEINFOW;
STRUCT!{struct DIDEVICEIMAGEINFOHEADERA {
    dwSize: DWORD,
    dwSizeImageInfo: DWORD,
    dwcViews: DWORD,
    dwcButtons: DWORD,
    dwcAxes: DWORD,
    dwcPOVs: DWORD,
    dwBufferSize: DWORD,
    dwBufferUsed: DWORD,
    lprgImageInfoArray: LPDIDEVICEIMAGEINFOA,
}}
pub type LPDIDEVICEIMAGEINFOHEADERA = *mut DIDEVICEIMAGEINFOHEADERA;
STRUCT!{struct DIDEVICEIMAGEINFOHEADERW {
    dwSize: DWORD,
    dwSizeImageInfo: DWORD,
    dwcViews: DWORD,
    dwcButtons: DWORD,
    dwcAxes: DWORD,
    dwcPOVs: DWORD,
    dwBufferSize: DWORD,
    dwBufferUsed: DWORD,
    lprgImageInfoArray: LPDIDEVICEIMAGEINFOW,
}}
pub type LPDIDEVICEIMAGEINFOHEADERW = *mut DIDEVICEIMAGEINFOHEADERW;
pub type LPCDIDEVICEIMAGEINFOHEADERA = *const DIDEVICEIMAGEINFOHEADERA;
pub type LPCDIDEVICEIMAGEINFOHEADERW = *const DIDEVICEIMAGEINFOHEADERW;
STRUCT!{struct DIDEVICEOBJECTINSTANCE_DX3A {
    dwSize: DWORD,
    guidType: GUID,
    dwOfs: DWORD,
    dwType: DWORD,
    dwFlags: DWORD,
    tszName: [CHAR; MAX_PATH],
}}
pub type LPDIDEVICEOBJECTINSTANCE_DX3A = *mut DIDEVICEOBJECTINSTANCE_DX3A;
STRUCT!{struct DIDEVICEOBJECTINSTANCE_DX3W {
    dwSize: DWORD,
    guidType: GUID,
    dwOfs: DWORD,
    dwType: DWORD,
    dwFlags: DWORD,
    tszName: [WCHAR; MAX_PATH],
}}
pub type LPDIDEVICEOBJECTINSTANCE_DX3W = *mut DIDEVICEOBJECTINSTANCE_DX3W;
pub type LPCDIDEVICEOBJECTINSTANCE_DX3A = *const DIDEVICEOBJECTINSTANCE_DX3A;
pub type LPCDIDEVICEOBJECTINSTANCE_DX3W = *const DIDEVICEOBJECTINSTANCE_DX3W;
STRUCT!{struct DIDEVICEOBJECTINSTANCEA {
    dwSize: DWORD,
    guidType: GUID,
    dwOfs: DWORD,
    dwType: DWORD,
    dwFlags: DWORD,
    tszName: [CHAR; MAX_PATH],
    dwFFMaxForce: DWORD,
    dwFFForceResolution: DWORD,
    wCollectionNumber: WORD,
    wDesignatorIndex: WORD,
    wUsagePage: WORD,
    wUsage: WORD,
    dwDimension: DWORD,
    wExponent: WORD,
    wReportId: WORD,
}}
pub type LPDIDEVICEOBJECTINSTANCEA = *mut DIDEVICEOBJECTINSTANCEA;
STRUCT!{struct DIDEVICEOBJECTINSTANCEW {
    dwSize: DWORD,
    guidType: GUID,
    dwOfs: DWORD,
    dwType: DWORD,
    dwFlags: DWORD,
    tszName: [WCHAR; MAX_PATH],
    dwFFMaxForce: DWORD,
    dwFFForceResolution: DWORD,
    wCollectionNumber: WORD,
    wDesignatorIndex: WORD,
    wUsagePage: WORD,
    wUsage: WORD,
    dwDimension: DWORD,
    wExponent: WORD,
    wReportId: WORD,
}}
pub type LPDIDEVICEOBJECTINSTANCEW = *mut DIDEVICEOBJECTINSTANCEW;
pub type LPCDIDEVICEOBJECTINSTANCEA = *const DIDEVICEOBJECTINSTANCEA;
pub type LPCDIDEVICEOBJECTINSTANCEW = *const DIDEVICEOBJECTINSTANCEW;
FN!{stdcall LPDIENUMDEVICEOBJECTSCALLBACKA(
    lpddoi: LPCDIDEVICEOBJECTINSTANCEA,
    pvRef: LPVOID,
) -> BOOL}
FN!{stdcall LPDIENUMDEVICEOBJECTSCALLBACKW(
    lpddoi: LPCDIDEVICEOBJECTINSTANCEW,
    pvRef: LPVOID,
) -> BOOL}
pub const DIDOI_FFACTUATOR: DWORD = 0x00000001;
pub const DIDOI_FFEFFECTTRIGGER: DWORD = 0x00000002;
pub const DIDOI_POLLED: DWORD = 0x00008000;
pub const DIDOI_ASPECTPOSITION: DWORD = 0x00000100;
pub const DIDOI_ASPECTVELOCITY: DWORD = 0x00000200;
pub const DIDOI_ASPECTACCEL: DWORD = 0x00000300;
pub const DIDOI_ASPECTFORCE: DWORD = 0x00000400;
pub const DIDOI_ASPECTMASK: DWORD = 0x00000F00;
pub const DIDOI_GUIDISUSAGE: DWORD = 0x00010000;
STRUCT!{struct DIPROPHEADER {
    dwSize: DWORD,
    dwHeaderSize: DWORD,
    dwObj: DWORD,
    dwHow: DWORD,
}}
pub type LPDIPROPHEADER = *mut DIPROPHEADER;
pub type LPCDIPROPHEADER = *const DIPROPHEADER;
pub const DIPH_DEVICE: DWORD = 0;
pub const DIPH_BYOFFSET: DWORD = 1;
pub const DIPH_BYID: DWORD = 2;
pub const DIPH_BYUSAGE: DWORD = 3;
#[inline]
pub fn DIMAKEUSAGEDWORD(UsagePage: WORD, Usage: WORD) -> DWORD {
    MAKELONG(Usage, UsagePage) as DWORD
}
STRUCT!{struct DIPROPDWORD {
    diph: DIPROPHEADER,
    dwData: DWORD,
}}
pub type LPDIPROPDWORD = *mut DIPROPDWORD;
pub type LPCDIPROPDWORD = *const DIPROPDWORD;
STRUCT!{struct DIPROPPOINTER {
    diph: DIPROPHEADER,
    uData: UINT_PTR,
}}
pub type LPDIPROPPOINTER = *mut DIPROPPOINTER;
pub type LPCDIPROPPOINTER = *const DIPROPPOINTER;
STRUCT!{struct DIPROPRANGE {
    diph: DIPROPHEADER,
    lMin: LONG,
    lMax: LONG,
}}
pub type LPDIPROPRANGE = *mut DIPROPRANGE;
pub type LPCDIPROPRANGE = *const DIPROPRANGE;
pub const DIPROPRANGE_NOMIN: LONG = 0x80000000 as LONG;
pub const DIPROPRANGE_NOMAX: LONG = 0x7fffffff as LONG;
STRUCT!{struct DIPROPCAL {
    diph: DIPROPHEADER,
    lMin: LONG,
    lCenter: LONG,
    lMax: LONG,
}}
pub type LPDIPROPCAL = *mut DIPROPCAL;
pub type LPCDIPROPCAL = *const DIPROPCAL;
STRUCT!{struct DIPROPCALPOV {
    diph: DIPROPHEADER,
    lMin: [LONG; 5],
    lMax: [LONG; 5],
}}
pub type LPDIPROPCALPOV = *mut DIPROPCALPOV;
pub type LPCDIPROPCALPOV = *const DIPROPCALPOV;
STRUCT!{struct DIPROPGUIDANDPATH {
    diph: DIPROPHEADER,
    guidClass: GUID,
    wszPath: [WCHAR; MAX_PATH],
}}
pub type LPDIPROPGUIDANDPATH = *mut DIPROPGUIDANDPATH;
pub type LPCDIPROPGUIDANDPATH = *const DIPROPGUIDANDPATH;
STRUCT!{struct DIPROPSTRING {
    diph: DIPROPHEADER,
    wsz: [WCHAR; MAX_PATH],
}}
pub type LPDIPROPSTRING = *mut DIPROPSTRING;
pub type LPCDIPROPSTRING = *const DIPROPSTRING;
pub const MAXCPOINTSNUM: usize = 8;
STRUCT!{struct CPOINT {
    lP: LONG,
    dwLog: DWORD,
}}
pub type PCPOINT = *mut CPOINT;
STRUCT!{struct DIPROPCPOINTS {
    diph: DIPROPHEADER,
    dwCPointsNum: DWORD,
    cp: [CPOINT; MAXCPOINTSNUM],
}}
pub type LPDIPROPCPOINTS = *mut DIPROPCPOINTS;
pub type LPCDIPROPCPOINTS = *const DIPROPCPOINTS;
pub const DIPROP_BUFFERSIZE: REFGUID = 1 as REFGUID;
pub const DIPROP_AXISMODE: REFGUID = 2 as REFGUID;
pub const DIPROPAXISMODE_ABS: DWORD = 0;
pub const DIPROPAXISMODE_REL: DWORD = 1;
pub const DIPROP_GRANULARITY: REFGUID = 3 as REFGUID;
pub const DIPROP_RANGE: REFGUID = 4 as REFGUID;
pub const DIPROP_DEADZONE: REFGUID = 5 as REFGUID;
pub const DIPROP_SATURATION: REFGUID = 6 as REFGUID;
pub const DIPROP_FFGAIN: REFGUID = 7 as REFGUID;
pub const DIPROP_FFLOAD: REFGUID = 8 as REFGUID;
pub const DIPROP_AUTOCENTER: REFGUID = 9 as REFGUID;
pub const DIPROPAUTOCENTER_OFF: DWORD = 0;
pub const DIPROPAUTOCENTER_ON: DWORD = 1;
pub const DIPROP_CALIBRATIONMODE: REFGUID = 10 as REFGUID;
pub const DIPROPCALIBRATIONMODE_COOKED: DWORD = 0;
pub const DIPROPCALIBRATIONMODE_RAW: DWORD = 1;
pub const DIPROP_CALIBRATION: REFGUID = 11 as REFGUID;
pub const DIPROP_GUIDANDPATH: REFGUID = 12 as REFGUID;
pub const DIPROP_INSTANCENAME: REFGUID = 13 as REFGUID;
pub const DIPROP_PRODUCTNAME: REFGUID = 14 as REFGUID;
pub const DIPROP_JOYSTICKID: REFGUID = 15 as REFGUID;
pub const DIPROP_GETPORTDISPLAYNAME: REFGUID = 16 as REFGUID;
pub const DIPROP_PHYSICALRANGE: REFGUID = 18 as REFGUID;
pub const DIPROP_LOGICALRANGE: REFGUID = 19 as REFGUID;
pub const DIPROP_KEYNAME: REFGUID = 20 as REFGUID;
pub const DIPROP_CPOINTS: REFGUID = 21 as REFGUID;
pub const DIPROP_APPDATA: REFGUID = 22 as REFGUID;
pub const DIPROP_SCANCODE: REFGUID = 23 as REFGUID;
pub const DIPROP_VIDPID: REFGUID = 24 as REFGUID;
pub const DIPROP_USERNAME: REFGUID = 25 as REFGUID;
pub const DIPROP_TYPENAME: REFGUID = 26 as REFGUID;
STRUCT!{struct DIDEVICEOBJECTDATA_DX3 {
    dwOfs: DWORD,
    dwData: DWORD,
    dwTimeStamp: DWORD,
    dwSequence: DWORD,
}}
pub type LPDIDEVICEOBJECTDATA_DX3 = *mut DIDEVICEOBJECTDATA_DX3;
pub type LPCDIDEVICEOBJECTDATA_DX3 = *const DIDEVICEOBJECTDATA_DX3;
STRUCT!{struct DIDEVICEOBJECTDATA {
    dwOfs: DWORD,
    dwData: DWORD,
    dwTimeStamp: DWORD,
    dwSequence: DWORD,
    uAppData: UINT_PTR,
}}
pub type LPDIDEVICEOBJECTDATA = *mut DIDEVICEOBJECTDATA;
pub type LPCDIDEVICEOBJECTDATA = *const DIDEVICEOBJECTDATA;
pub const DIGDD_PEEK: DWORD = 0x00000001;
pub const DISCL_EXCLUSIVE: DWORD = 0x00000001;
pub const DISCL_NONEXCLUSIVE: DWORD = 0x00000002;
pub const DISCL_FOREGROUND: DWORD = 0x00000004;
pub const DISCL_BACKGROUND: DWORD = 0x00000008;
pub const DISCL_NOWINKEY: DWORD = 0x00000010;
STRUCT!{struct DIDEVICEINSTANCE_DX3A {
    dwSize: DWORD,
    guidInstance: GUID,
    guidProduct: GUID,
    dwDevType: DWORD,
    tszInstanceName: [CHAR; MAX_PATH],
    tszProductName: [CHAR; MAX_PATH],
}}
pub type LPDIDEVICEINSTANCE_DX3A = *mut DIDEVICEINSTANCE_DX3A;
STRUCT!{struct DIDEVICEINSTANCE_DX3W {
    dwSize: DWORD,
    guidInstance: GUID,
    guidProduct: GUID,
    dwDevType: DWORD,
    tszInstanceName: [WCHAR; MAX_PATH],
    tszProductName: [WCHAR; MAX_PATH],
}}
pub type LPDIDEVICEINSTANCE_DX3W = *mut DIDEVICEINSTANCE_DX3W;
pub type LPCDIDEVICEINSTANCE_DX3A = *const DIDEVICEINSTANCE_DX3A;
pub type LPCDIDEVICEINSTANCE_DX3W = *const DIDEVICEINSTANCE_DX3W;
STRUCT!{struct DIDEVICEINSTANCEA {
    dwSize: DWORD,
    guidInstance: GUID,
    guidProduct: GUID,
    dwDevType: DWORD,
    tszInstanceName: [CHAR; MAX_PATH],
    tszProductName: [CHAR; MAX_PATH],
    guidFFDriver: GUID,
    wUsagePage: WORD,
    wUsage: WORD,
}}
pub type LPDIDEVICEINSTANCEA = *mut DIDEVICEINSTANCEA;
STRUCT!{struct DIDEVICEINSTANCEW {
    dwSize: DWORD,
    guidInstance: GUID,
    guidProduct: GUID,
    dwDevType: DWORD,
    tszInstanceName: [WCHAR; MAX_PATH],
    tszProductName: [WCHAR; MAX_PATH],
    guidFFDriver: GUID,
    wUsagePage: WORD,
    wUsage: WORD,
}}
pub type LPDIDEVICEINSTANCEW = *mut DIDEVICEINSTANCEW;
pub type LPCDIDEVICEINSTANCEA = *const DIDEVICEINSTANCEA;
pub type LPCDIDEVICEINSTANCEW = *const DIDEVICEINSTANCEW;
RIDL!{#[uuid(0x5944e681, 0xc92e, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00)]
interface IDirectInputDeviceW(IDirectInputDeviceWVtbl): IUnknown(IUnknownVtbl) {
    fn GetCapabilities(
        lpDIDevCaps: LPDIDEVCAPS,
    ) -> HRESULT,
    fn EnumObjects(
        lpCallback: LPDIENUMDEVICEOBJECTSCALLBACKW,
        pvRef: LPVOID,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetProperty(
        rguidProp: REFGUID,
        pdiph: LPDIPROPHEADER,
    ) -> HRESULT,
    fn SetProperty(
        rguidProp: REFGUID,
        pdiph: LPCDIPROPHEADER,
    ) -> HRESULT,
    fn Acquire() -> HRESULT,
    fn Unacquire() -> HRESULT,
    fn GetDeviceState(
        cbData: DWORD,
        lpvData: LPVOID,
    ) -> HRESULT,
    fn GetDeviceData(
        cbObjectData: DWORD,
        rgdod: LPDIDEVICEOBJECTDATA,
        pdwInOut: LPDWORD,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn SetDataFormat(
        lpdf: LPCDIDATAFORMAT,
    ) -> HRESULT,
    fn SetEventNotification(
        hEvent: HANDLE,
    ) -> HRESULT,
    fn SetCooperativeLevel(
        hwnd: HWND,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetObjectInfo(
        pdidoi: LPDIDEVICEOBJECTINSTANCEW,
        dwObj: DWORD,
        dwHow: DWORD,
    ) -> HRESULT,
    fn GetDeviceInfo(
        pdidi: LPDIDEVICEINSTANCEW,
    ) -> HRESULT,
    fn RunControlPanel(
        hwndOwner: HWND,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn Initialize(
        hinst: HINSTANCE,
        dwVersion: DWORD,
        rguid: REFGUID,
    ) -> HRESULT,
}}
pub type LPDIRECTINPUTDEVICEW = *mut IDirectInputDeviceW;
RIDL!{#[uuid(0x5944e680, 0xc92e, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00)]
interface IDirectInputDeviceA(IDirectInputDeviceAVtbl): IUnknown(IUnknownVtbl) {
    fn GetCapabilities(
        lpDIDevCaps: LPDIDEVCAPS,
    ) -> HRESULT,
    fn EnumObjects(
        lpCallback: LPDIENUMDEVICEOBJECTSCALLBACKA,
        pvRef: LPVOID,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetProperty(
        rguidProp: REFGUID,
        pdiph: LPDIPROPHEADER,
    ) -> HRESULT,
    fn SetProperty(
        rguidProp: REFGUID,
        pdiph: LPCDIPROPHEADER,
    ) -> HRESULT,
    fn Acquire() -> HRESULT,
    fn Unacquire() -> HRESULT,
    fn GetDeviceState(
        cbData: DWORD,
        lpvData: LPVOID,
    ) -> HRESULT,
    fn GetDeviceData(
        cbObjectData: DWORD,
        rgdod: LPDIDEVICEOBJECTDATA,
        pdwInOut: LPDWORD,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn SetDataFormat(
        lpdf: LPCDIDATAFORMAT,
    ) -> HRESULT,
    fn SetEventNotification(
        hEvent: HANDLE,
    ) -> HRESULT,
    fn SetCooperativeLevel(
        hwnd: HWND,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetObjectInfo(
        pdidoi: LPDIDEVICEOBJECTINSTANCEA,
        dwObj: DWORD,
        dwHow: DWORD,
    ) -> HRESULT,
    fn GetDeviceInfo(
        pdidi: LPDIDEVICEINSTANCEA,
    ) -> HRESULT,
    fn RunControlPanel(
        hwndOwner: HWND,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn Initialize(
        hinst: HINSTANCE,
        dwVersion: DWORD,
        rguid: REFGUID,
    ) -> HRESULT,
}}
pub type LPDIRECTINPUTDEVICEA = *mut IDirectInputDeviceA;
pub const DISFFC_RESET: DWORD = 0x00000001;
pub const DISFFC_STOPALL: DWORD = 0x00000002;
pub const DISFFC_PAUSE: DWORD = 0x00000004;
pub const DISFFC_CONTINUE: DWORD = 0x00000008;
pub const DISFFC_SETACTUATORSON: DWORD = 0x00000010;
pub const DISFFC_SETACTUATORSOFF: DWORD = 0x00000020;
pub const DIGFFS_EMPTY: DWORD = 0x00000001;
pub const DIGFFS_STOPPED: DWORD = 0x00000002;
pub const DIGFFS_PAUSED: DWORD = 0x00000004;
pub const DIGFFS_ACTUATORSON: DWORD = 0x00000010;
pub const DIGFFS_ACTUATORSOFF: DWORD = 0x00000020;
pub const DIGFFS_POWERON: DWORD = 0x00000040;
pub const DIGFFS_POWEROFF: DWORD = 0x00000080;
pub const DIGFFS_SAFETYSWITCHON: DWORD = 0x00000100;
pub const DIGFFS_SAFETYSWITCHOFF: DWORD = 0x00000200;
pub const DIGFFS_USERFFSWITCHON: DWORD = 0x00000400;
pub const DIGFFS_USERFFSWITCHOFF: DWORD = 0x00000800;
pub const DIGFFS_DEVICELOST: DWORD = 0x80000000;
STRUCT!{struct DIEFFECTINFOA {
    dwSize: DWORD,
    guid: GUID,
    dwEffType: DWORD,
    dwStaticParams: DWORD,
    dwDynamicParams: DWORD,
    tszName: [CHAR; MAX_PATH],
}}
pub type LPDIEFFECTINFOA = *mut DIEFFECTINFOA;
STRUCT!{struct DIEFFECTINFOW {
    dwSize: DWORD,
    guid: GUID,
    dwEffType: DWORD,
    dwStaticParams: DWORD,
    dwDynamicParams: DWORD,
    tszName: [WCHAR; MAX_PATH],
}}
pub type LPDIEFFECTINFOW = *mut DIEFFECTINFOW;
pub type LPCDIEFFECTINFOA = *const DIEFFECTINFOA;
pub type LPCDIEFFECTINFOW = *const DIEFFECTINFOW;
pub const DISDD_CONTINUE: DWORD = 0x00000001;
FN!{stdcall LPDIENUMEFFECTSCALLBACKA(
    pdei: LPCDIEFFECTINFOA,
    pvRef: LPVOID,
) -> BOOL}
FN!{stdcall LPDIENUMEFFECTSCALLBACKW(
    pdei: LPCDIEFFECTINFOW,
    pvRef: LPVOID,
) -> BOOL}
FN!{stdcall LPDIENUMCREATEDEFFECTOBJECTSCALLBACK(
    peff: LPDIRECTINPUTEFFECT,
    pvRef: LPVOID,
) -> BOOL}
RIDL!{#[uuid(0x5944e683, 0xc92e, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00)]
interface IDirectInputDevice2W(IDirectInputDevice2WVtbl): 
    IDirectInputDeviceW(IDirectInputDeviceWVtbl) {
    fn CreateEffect(
        rguid: REFGUID,
        lpeff: LPCDIEFFECT,
        ppdeff: *mut LPDIRECTINPUTEFFECT,
        punkOuter: LPUNKNOWN,
    ) -> HRESULT,
    fn EnumEffects(
        lpCallback: LPDIENUMEFFECTSCALLBACKW,
        pvRef: LPVOID,
        dwEffType: DWORD,
    ) -> HRESULT,
    fn GetEffectInfo(
        pdei: LPDIEFFECTINFOW,
        rguid: REFGUID,
    ) -> HRESULT,
    fn GetForceFeedbackState(
        pdwOut: LPDWORD,
    ) -> HRESULT,
    fn SendForceFeedbackCommand(
        dwFlags: DWORD,
    ) -> HRESULT,
    fn EnumCreatedEffectObjects(
        lpCallback: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK,
        pvRef: LPVOID,
        fl: DWORD,
    ) -> HRESULT,
    fn Escape(
        pesc: LPDIEFFESCAPE,
    ) -> HRESULT,
    fn Poll() -> HRESULT,
    fn SendDeviceData(
        cbObjectData: DWORD,
        rgdod: LPCDIDEVICEOBJECTDATA,
        pdwInOut: LPDWORD,
        fl: DWORD,
    ) -> HRESULT,
}}
pub type LPDIRECTINPUTDEVICE2W = *mut IDirectInputDevice2W;
RIDL!{#[uuid(0x5944e682, 0xc92e, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00)]
interface IDirectInputDevice2A(IDirectInputDevice2AVtbl): 
    IDirectInputDeviceA(IDirectInputDeviceAVtbl) {
    fn CreateEffect(
        rguid: REFGUID,
        lpeff: LPCDIEFFECT,
        ppdeff: *mut LPDIRECTINPUTEFFECT,
        punkOuter: LPUNKNOWN,
    ) -> HRESULT,
    fn EnumEffects(
        lpCallback: LPDIENUMEFFECTSCALLBACKA,
        pvRef: LPVOID,
        dwEffType: DWORD,
    ) -> HRESULT,
    fn GetEffectInfo(
        pdei: LPDIEFFECTINFOA,
        rguid: REFGUID,
    ) -> HRESULT,
    fn GetForceFeedbackState(
        pdwOut: LPDWORD,
    ) -> HRESULT,
    fn SendForceFeedbackCommand(
        dwFlags: DWORD,
    ) -> HRESULT,
    fn EnumCreatedEffectObjects(
        lpCallback: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK,
        pvRef: LPVOID,
        fl: DWORD,
    ) -> HRESULT,
    fn Escape(
        pesc: LPDIEFFESCAPE,
    ) -> HRESULT,
    fn Poll() -> HRESULT,
    fn SendDeviceData(
        cbObjectData: DWORD,
        rgdod: LPCDIDEVICEOBJECTDATA,
        pdwInOut: LPDWORD,
        fl: DWORD,
    ) -> HRESULT,
}}
pub type LPDIRECTINPUTDEVICE2A = *mut IDirectInputDevice2A;
pub const DIFEF_DEFAULT: DWORD = 0x00000000;
pub const DIFEF_INCLUDENONSTANDARD: DWORD = 0x00000001;
pub const DIFEF_MODIFYIFNEEDED: DWORD = 0x00000010;
RIDL!{#[uuid(0x57d7c6bd, 0x2356, 0x11d3, 0x8e, 0x9d, 0x00, 0xc0, 0x4f, 0x68, 0x44, 0xae)]
interface IDirectInputDevice7W(IDirectInputDevice7WVtbl): 
    IDirectInputDevice2W(IDirectInputDevice2WVtbl) {
    fn EnumEffectsInFile(
        lpszFileName: LPCWSTR,
        pec: LPDIENUMEFFECTSINFILECALLBACK,
        pvRef: LPVOID,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn WriteEffectToFile(
        lpszFileName: LPCWSTR,
        dwEntries: DWORD,
        rgDiFileEft: LPDIFILEEFFECT,
        dwFlags: DWORD,
    ) -> HRESULT,
}}
pub type LPDIRECTINPUTDEVICE7W = *mut IDirectInputDevice7W;
RIDL!{#[uuid(0x57d7c6bc, 0x2356, 0x11d3, 0x8e, 0x9d, 0x00, 0xc0, 0x4f, 0x68, 0x44, 0xae)]
interface IDirectInputDevice7A(IDirectInputDevice7AVtbl): 
    IDirectInputDevice2A(IDirectInputDevice2AVtbl) {
    fn EnumEffectsInFile(
        lpszFileName: LPCSTR,
        pec: LPDIENUMEFFECTSINFILECALLBACK,
        pvRef: LPVOID,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn WriteEffectToFile(
        lpszFileName: LPCSTR,
        dwEntries: DWORD,
        rgDiFileEft: LPDIFILEEFFECT,
        dwFlags: DWORD,
    ) -> HRESULT,
}}
pub type LPDIRECTINPUTDEVICE7A = *mut IDirectInputDevice7A;
RIDL!{#[uuid(0x54d41081, 0xdc15, 0x4833, 0xa4, 0x1b, 0x74, 0x8f, 0x73, 0xa3, 0x81, 0x79)]
interface IDirectInputDevice8W(IDirectInputDevice8WVtbl): IUnknown(IUnknownVtbl) {
    fn GetCapabilities(
        lpDIDevCaps: LPDIDEVCAPS,
    ) -> HRESULT,
    fn EnumObjects(
        lpCallback: LPDIENUMDEVICEOBJECTSCALLBACKW,
        pvRef: LPVOID,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetProperty(
        rguidProp: REFGUID,
        pdiph: LPDIPROPHEADER,
    ) -> HRESULT,
    fn SetProperty(
        rguidProp: REFGUID,
        pdiph: LPCDIPROPHEADER,
    ) -> HRESULT,
    fn Acquire() -> HRESULT,
    fn Unacquire() -> HRESULT,
    fn GetDeviceState(
        cbData: DWORD,
        lpvData: LPVOID,
    ) -> HRESULT,
    fn GetDeviceData(
        cbObjectData: DWORD,
        rgdod: LPDIDEVICEOBJECTDATA,
        pdwInOut: LPDWORD,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn SetDataFormat(
        lpdf: LPCDIDATAFORMAT,
    ) -> HRESULT,
    fn SetEventNotification(
        hEvent: HANDLE,
    ) -> HRESULT,
    fn SetCooperativeLevel(
        hwnd: HWND,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetObjectInfo(
        pdidoi: LPDIDEVICEOBJECTINSTANCEW,
        dwObj: DWORD,
        dwHow: DWORD,
    ) -> HRESULT,
    fn GetDeviceInfo(
        pdidi: LPDIDEVICEINSTANCEW,
    ) -> HRESULT,
    fn RunControlPanel(
        hwndOwner: HWND,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn Initialize(
        hinst: HINSTANCE,
        dwVersion: DWORD,
        rguid: REFGUID,
    ) -> HRESULT,
    fn CreateEffect(
        rguid: REFGUID,
        lpeff: LPCDIEFFECT,
        ppdeff: *mut LPDIRECTINPUTEFFECT,
        punkOuter: LPUNKNOWN,
    ) -> HRESULT,
    fn EnumEffects(
        lpCallback: LPDIENUMEFFECTSCALLBACKW,
        pvRef: LPVOID,
        dwEffType: DWORD,
    ) -> HRESULT,
    fn GetEffectInfo(
        pdei: LPDIEFFECTINFOW,
        rguid: REFGUID,
    ) -> HRESULT,
    fn GetForceFeedbackState(
        pdwOut: LPDWORD,
    ) -> HRESULT,
    fn SendForceFeedbackCommand(
        dwFlags: DWORD,
    ) -> HRESULT,
    fn EnumCreatedEffectObjects(
        lpCallback: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK,
        pvRef: LPVOID,
        fl: DWORD,
    ) -> HRESULT,
    fn Escape(
        pesc: LPDIEFFESCAPE,
    ) -> HRESULT,
    fn Poll() -> HRESULT,
    fn SendDeviceData(
        cbObjectData: DWORD,
        rgdod: LPCDIDEVICEOBJECTDATA,
        pdwInOut: LPDWORD,
        fl: DWORD,
    ) -> HRESULT,
    fn EnumEffectsInFile(
        lpszFileName: LPCWSTR,
        pec: LPDIENUMEFFECTSINFILECALLBACK,
        pvRef: LPVOID,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn WriteEffectToFile(
        lpszFileName: LPCWSTR,
        dwEntries: DWORD,
        rgDiFileEft: LPDIFILEEFFECT,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn BuildActionMap(
        lpdiaf: LPDIACTIONFORMATW,
        lpszUserName: LPCWSTR,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn SetActionMap(
        lpdiActionFormat: LPDIACTIONFORMATW,
        lptszUserName: LPCWSTR,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetImageInfo(
        lpdiDevImageInfoHeader: LPDIDEVICEIMAGEINFOHEADERW,
    ) -> HRESULT,
}}
pub type LPDIRECTINPUTDEVICE8W = *mut IDirectInputDevice8W;
RIDL!{#[uuid(0x54d41080, 0xdc15, 0x4833, 0xa4, 0x1b, 0x74, 0x8f, 0x73, 0xa3, 0x81, 0x79)]
interface IDirectInputDevice8A(IDirectInputDevice8AVtbl): IUnknown(IUnknownVtbl) {
    fn GetCapabilities(
        lpDIDevCaps: LPDIDEVCAPS,
    ) -> HRESULT,
    fn EnumObjects(
        lpCallback: LPDIENUMDEVICEOBJECTSCALLBACKA,
        pvRef: LPVOID,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetProperty(
        rguidProp: REFGUID,
        pdiph: LPDIPROPHEADER,
    ) -> HRESULT,
    fn SetProperty(
        rguidProp: REFGUID,
        pdiph: LPCDIPROPHEADER,
    ) -> HRESULT,
    fn Acquire() -> HRESULT,
    fn Unacquire() -> HRESULT,
    fn GetDeviceState(
        cbData: DWORD,
        lpvData: LPVOID,
    ) -> HRESULT,
    fn GetDeviceData(
        cbObjectData: DWORD,
        rgdod: LPDIDEVICEOBJECTDATA,
        pdwInOut: LPDWORD,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn SetDataFormat(
        lpdf: LPCDIDATAFORMAT,
    ) -> HRESULT,
    fn SetEventNotification(
        hEvent: HANDLE,
    ) -> HRESULT,
    fn SetCooperativeLevel(
        hwnd: HWND,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetObjectInfo(
        pdidoi: LPDIDEVICEOBJECTINSTANCEA,
        dwObj: DWORD,
        dwHow: DWORD,
    ) -> HRESULT,
    fn GetDeviceInfo(
        pdidi: LPDIDEVICEINSTANCEA,
    ) -> HRESULT,
    fn RunControlPanel(
        hwndOwner: HWND,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn Initialize(
        hinst: HINSTANCE,
        dwVersion: DWORD,
        rguid: REFGUID,
    ) -> HRESULT,
    fn CreateEffect(
        rguid: REFGUID,
        lpeff: LPCDIEFFECT,
        ppdeff: *mut LPDIRECTINPUTEFFECT,
        punkOuter: LPUNKNOWN,
    ) -> HRESULT,
    fn EnumEffects(
        lpCallback: LPDIENUMEFFECTSCALLBACKA,
        pvRef: LPVOID,
        dwEffType: DWORD,
    ) -> HRESULT,
    fn GetEffectInfo(
        pdei: LPDIEFFECTINFOA,
        rguid: REFGUID,
    ) -> HRESULT,
    fn GetForceFeedbackState(
        pdwOut: LPDWORD,
    ) -> HRESULT,
    fn SendForceFeedbackCommand(
        dwFlags: DWORD,
    ) -> HRESULT,
    fn EnumCreatedEffectObjects(
        lpCallback: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK,
        pvRef: LPVOID,
        fl: DWORD,
    ) -> HRESULT,
    fn Escape(
        pesc: LPDIEFFESCAPE,
    ) -> HRESULT,
    fn Poll() -> HRESULT,
    fn SendDeviceData(
        cbObjectData: DWORD,
        rgdod: LPCDIDEVICEOBJECTDATA,
        pdwInOut: LPDWORD,
        fl: DWORD,
    ) -> HRESULT,
    fn EnumEffectsInFile(
        lpszFileName: LPCSTR,
        pec: LPDIENUMEFFECTSINFILECALLBACK,
        pvRef: LPVOID,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn WriteEffectToFile(
        lpszFileName: LPCSTR,
        dwEntries: DWORD,
        rgDiFileEft: LPDIFILEEFFECT,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn BuildActionMap(
        lpdiaf: LPDIACTIONFORMATA,
        lpszUserName: LPCSTR,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn SetActionMap(
        lpdiActionFormat: LPDIACTIONFORMATA,
        lptszUserName: LPCSTR,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetImageInfo(
        lpdiDevImageInfoHeader: LPDIDEVICEIMAGEINFOHEADERA,
    ) -> HRESULT,
}}
pub type LPDIRECTINPUTDEVICE8A = *mut IDirectInputDevice8A;
STRUCT!{struct DIMOUSESTATE {
    lX: LONG,
    lY: LONG,
    lZ: LONG,
    rgbButtons: [BYTE; 4],
}}
pub type LPDIMOUSESTATE = *mut DIMOUSESTATE;
STRUCT!{struct DIMOUSESTATE2 {
    lX: LONG,
    lY: LONG,
    lZ: LONG,
    rgbButtons: [BYTE; 8],
}}
pub type LPDIMOUSESTATE2 = *mut DIMOUSESTATE2;
pub const DIMOFS_X: usize = 0;
pub const DIMOFS_Y: usize = 4;
pub const DIMOFS_Z: usize = 8;
pub const DIMOFS_BUTTON0: usize = 12;
pub const DIMOFS_BUTTON1: usize = 13;
pub const DIMOFS_BUTTON2: usize = 14;
pub const DIMOFS_BUTTON3: usize = 15;
pub const DIMOFS_BUTTON4: usize = 16;
pub const DIMOFS_BUTTON5: usize = 17;
pub const DIMOFS_BUTTON6: usize = 18;
pub const DIMOFS_BUTTON7: usize = 19;
pub const DIK_ESCAPE: DWORD = 0x01;
pub const DIK_1: DWORD = 0x02;
pub const DIK_2: DWORD = 0x03;
pub const DIK_3: DWORD = 0x04;
pub const DIK_4: DWORD = 0x05;
pub const DIK_5: DWORD = 0x06;
pub const DIK_6: DWORD = 0x07;
pub const DIK_7: DWORD = 0x08;
pub const DIK_8: DWORD = 0x09;
pub const DIK_9: DWORD = 0x0A;
pub const DIK_0: DWORD = 0x0B;
pub const DIK_MINUS: DWORD = 0x0C;
pub const DIK_EQUALS: DWORD = 0x0D;
pub const DIK_BACK: DWORD = 0x0E;
pub const DIK_TAB: DWORD = 0x0F;
pub const DIK_Q: DWORD = 0x10;
pub const DIK_W: DWORD = 0x11;
pub const DIK_E: DWORD = 0x12;
pub const DIK_R: DWORD = 0x13;
pub const DIK_T: DWORD = 0x14;
pub const DIK_Y: DWORD = 0x15;
pub const DIK_U: DWORD = 0x16;
pub const DIK_I: DWORD = 0x17;
pub const DIK_O: DWORD = 0x18;
pub const DIK_P: DWORD = 0x19;
pub const DIK_LBRACKET: DWORD = 0x1A;
pub const DIK_RBRACKET: DWORD = 0x1B;
pub const DIK_RETURN: DWORD = 0x1C;
pub const DIK_LCONTROL: DWORD = 0x1D;
pub const DIK_A: DWORD = 0x1E;
pub const DIK_S: DWORD = 0x1F;
pub const DIK_D: DWORD = 0x20;
pub const DIK_F: DWORD = 0x21;
pub const DIK_G: DWORD = 0x22;
pub const DIK_H: DWORD = 0x23;
pub const DIK_J: DWORD = 0x24;
pub const DIK_K: DWORD = 0x25;
pub const DIK_L: DWORD = 0x26;
pub const DIK_SEMICOLON: DWORD = 0x27;
pub const DIK_APOSTROPHE: DWORD = 0x28;
pub const DIK_GRAVE: DWORD = 0x29;
pub const DIK_LSHIFT: DWORD = 0x2A;
pub const DIK_BACKSLASH: DWORD = 0x2B;
pub const DIK_Z: DWORD = 0x2C;
pub const DIK_X: DWORD = 0x2D;
pub const DIK_C: DWORD = 0x2E;
pub const DIK_V: DWORD = 0x2F;
pub const DIK_B: DWORD = 0x30;
pub const DIK_N: DWORD = 0x31;
pub const DIK_M: DWORD = 0x32;
pub const DIK_COMMA: DWORD = 0x33;
pub const DIK_PERIOD: DWORD = 0x34;
pub const DIK_SLASH: DWORD = 0x35;
pub const DIK_RSHIFT: DWORD = 0x36;
pub const DIK_MULTIPLY: DWORD = 0x37;
pub const DIK_LMENU: DWORD = 0x38;
pub const DIK_SPACE: DWORD = 0x39;
pub const DIK_CAPITAL: DWORD = 0x3A;
pub const DIK_F1: DWORD = 0x3B;
pub const DIK_F2: DWORD = 0x3C;
pub const DIK_F3: DWORD = 0x3D;
pub const DIK_F4: DWORD = 0x3E;
pub const DIK_F5: DWORD = 0x3F;
pub const DIK_F6: DWORD = 0x40;
pub const DIK_F7: DWORD = 0x41;
pub const DIK_F8: DWORD = 0x42;
pub const DIK_F9: DWORD = 0x43;
pub const DIK_F10: DWORD = 0x44;
pub const DIK_NUMLOCK: DWORD = 0x45;
pub const DIK_SCROLL: DWORD = 0x46;
pub const DIK_NUMPAD7: DWORD = 0x47;
pub const DIK_NUMPAD8: DWORD = 0x48;
pub const DIK_NUMPAD9: DWORD = 0x49;
pub const DIK_SUBTRACT: DWORD = 0x4A;
pub const DIK_NUMPAD4: DWORD = 0x4B;
pub const DIK_NUMPAD5: DWORD = 0x4C;
pub const DIK_NUMPAD6: DWORD = 0x4D;
pub const DIK_ADD: DWORD = 0x4E;
pub const DIK_NUMPAD1: DWORD = 0x4F;
pub const DIK_NUMPAD2: DWORD = 0x50;
pub const DIK_NUMPAD3: DWORD = 0x51;
pub const DIK_NUMPAD0: DWORD = 0x52;
pub const DIK_DECIMAL: DWORD = 0x53;
pub const DIK_OEM_102: DWORD = 0x56;
pub const DIK_F11: DWORD = 0x57;
pub const DIK_F12: DWORD = 0x58;
pub const DIK_F13: DWORD = 0x64;
pub const DIK_F14: DWORD = 0x65;
pub const DIK_F15: DWORD = 0x66;
pub const DIK_KANA: DWORD = 0x70;
pub const DIK_ABNT_C1: DWORD = 0x73;
pub const DIK_CONVERT: DWORD = 0x79;
pub const DIK_NOCONVERT: DWORD = 0x7B;
pub const DIK_YEN: DWORD = 0x7D;
pub const DIK_ABNT_C2: DWORD = 0x7E;
pub const DIK_NUMPADEQUALS: DWORD = 0x8D;
pub const DIK_PREVTRACK: DWORD = 0x90;
pub const DIK_AT: DWORD = 0x91;
pub const DIK_COLON: DWORD = 0x92;
pub const DIK_UNDERLINE: DWORD = 0x93;
pub const DIK_KANJI: DWORD = 0x94;
pub const DIK_STOP: DWORD = 0x95;
pub const DIK_AX: DWORD = 0x96;
pub const DIK_UNLABELED: DWORD = 0x97;
pub const DIK_NEXTTRACK: DWORD = 0x99;
pub const DIK_NUMPADENTER: DWORD = 0x9C;
pub const DIK_RCONTROL: DWORD = 0x9D;
pub const DIK_MUTE: DWORD = 0xA0;
pub const DIK_CALCULATOR: DWORD = 0xA1;
pub const DIK_PLAYPAUSE: DWORD = 0xA2;
pub const DIK_MEDIASTOP: DWORD = 0xA4;
pub const DIK_VOLUMEDOWN: DWORD = 0xAE;
pub const DIK_VOLUMEUP: DWORD = 0xB0;
pub const DIK_WEBHOME: DWORD = 0xB2;
pub const DIK_NUMPADCOMMA: DWORD = 0xB3;
pub const DIK_DIVIDE: DWORD = 0xB5;
pub const DIK_SYSRQ: DWORD = 0xB7;
pub const DIK_RMENU: DWORD = 0xB8;
pub const DIK_PAUSE: DWORD = 0xC5;
pub const DIK_HOME: DWORD = 0xC7;
pub const DIK_UP: DWORD = 0xC8;
pub const DIK_PRIOR: DWORD = 0xC9;
pub const DIK_LEFT: DWORD = 0xCB;
pub const DIK_RIGHT: DWORD = 0xCD;
pub const DIK_END: DWORD = 0xCF;
pub const DIK_DOWN: DWORD = 0xD0;
pub const DIK_NEXT: DWORD = 0xD1;
pub const DIK_INSERT: DWORD = 0xD2;
pub const DIK_DELETE: DWORD = 0xD3;
pub const DIK_LWIN: DWORD = 0xDB;
pub const DIK_RWIN: DWORD = 0xDC;
pub const DIK_APPS: DWORD = 0xDD;
pub const DIK_POWER: DWORD = 0xDE;
pub const DIK_SLEEP: DWORD = 0xDF;
pub const DIK_WAKE: DWORD = 0xE3;
pub const DIK_WEBSEARCH: DWORD = 0xE5;
pub const DIK_WEBFAVORITES: DWORD = 0xE6;
pub const DIK_WEBREFRESH: DWORD = 0xE7;
pub const DIK_WEBSTOP: DWORD = 0xE8;
pub const DIK_WEBFORWARD: DWORD = 0xE9;
pub const DIK_WEBBACK: DWORD = 0xEA;
pub const DIK_MYCOMPUTER: DWORD = 0xEB;
pub const DIK_MAIL: DWORD = 0xEC;
pub const DIK_MEDIASELECT: DWORD = 0xED;
pub const DIK_BACKSPACE: DWORD = DIK_BACK;
pub const DIK_NUMPADSTAR: DWORD = DIK_MULTIPLY;
pub const DIK_LALT: DWORD = DIK_LMENU;
pub const DIK_CAPSLOCK: DWORD = DIK_CAPITAL;
pub const DIK_NUMPADMINUS: DWORD = DIK_SUBTRACT;
pub const DIK_NUMPADPLUS: DWORD = DIK_ADD;
pub const DIK_NUMPADPERIOD: DWORD = DIK_DECIMAL;
pub const DIK_NUMPADSLASH: DWORD = DIK_DIVIDE;
pub const DIK_RALT: DWORD = DIK_RMENU;
pub const DIK_UPARROW: DWORD = DIK_UP;
pub const DIK_PGUP: DWORD = DIK_PRIOR;
pub const DIK_LEFTARROW: DWORD = DIK_LEFT;
pub const DIK_RIGHTARROW: DWORD = DIK_RIGHT;
pub const DIK_DOWNARROW: DWORD = DIK_DOWN;
pub const DIK_PGDN: DWORD = DIK_NEXT;
pub const DIK_CIRCUMFLEX: DWORD = DIK_PREVTRACK;
STRUCT!{struct DIJOYSTATE {
    lX: LONG,
    lY: LONG,
    lZ: LONG,
    lRx: LONG,
    lRy: LONG,
    lRz: LONG,
    rglSlider: [LONG; 2],
    rgdwPOV: [DWORD; 4],
    rgbButtons: [BYTE; 32],
}}
pub type LPDIJOYSTATE = *mut DIJOYSTATE;
STRUCT!{struct DIJOYSTATE2 {
    lX: LONG,
    lY: LONG,
    lZ: LONG,
    lRx: LONG,
    lRy: LONG,
    lRz: LONG,
    rglSlider: [LONG; 2],
    rgdwPOV: [DWORD; 4],
    rgbButtons: [BYTE; 128],
    lVX: LONG,
    lVY: LONG,
    lVZ: LONG,
    lVRx: LONG,
    lVRy: LONG,
    lVRz: LONG,
    rglVSlider: [LONG; 2],
    lAX: LONG,
    lAY: LONG,
    lAZ: LONG,
    lARx: LONG,
    lARy: LONG,
    lARz: LONG,
    rglASlider: [LONG; 2],
    lFX: LONG,
    lFY: LONG,
    lFZ: LONG,
    lFRx: LONG,
    lFRy: LONG,
    lFRz: LONG,
    rglFSlider: [LONG; 2],
}}
pub type LPDIJOYSTATE2 = *mut DIJOYSTATE2;
pub const DIJOFS_X: usize = 0;
pub const DIJOFS_Y: usize = 4;
pub const DIJOFS_Z: usize = 8;
pub const DIJOFS_RX: usize = 12;
pub const DIJOFS_RY: usize = 16;
pub const DIJOFS_RZ: usize = 20;
#[inline]
pub fn DIJOFS_SLIDER(n: usize) -> usize {
    24 + n * 4
}
#[inline]
pub fn DIJOFS_POV(n: usize) -> usize {
    32 + n * 4
}
#[inline]
pub fn DIJOFS_BUTTON(n: usize) -> usize {
    48 + n
}
pub const DIJOFS_BUTTON0: usize = 48 + 0;
pub const DIJOFS_BUTTON1: usize = 48 + 1;
pub const DIJOFS_BUTTON2: usize = 48 + 2;
pub const DIJOFS_BUTTON3: usize = 48 + 3;
pub const DIJOFS_BUTTON4: usize = 48 + 4;
pub const DIJOFS_BUTTON5: usize = 48 + 5;
pub const DIJOFS_BUTTON6: usize = 48 + 6;
pub const DIJOFS_BUTTON7: usize = 48 + 7;
pub const DIJOFS_BUTTON8: usize = 48 + 8;
pub const DIJOFS_BUTTON9: usize = 48 + 9;
pub const DIJOFS_BUTTON10: usize = 48 + 10;
pub const DIJOFS_BUTTON11: usize = 48 + 11;
pub const DIJOFS_BUTTON12: usize = 48 + 12;
pub const DIJOFS_BUTTON13: usize = 48 + 13;
pub const DIJOFS_BUTTON14: usize = 48 + 14;
pub const DIJOFS_BUTTON15: usize = 48 + 15;
pub const DIJOFS_BUTTON16: usize = 48 + 16;
pub const DIJOFS_BUTTON17: usize = 48 + 17;
pub const DIJOFS_BUTTON18: usize = 48 + 18;
pub const DIJOFS_BUTTON19: usize = 48 + 19;
pub const DIJOFS_BUTTON20: usize = 48 + 20;
pub const DIJOFS_BUTTON21: usize = 48 + 21;
pub const DIJOFS_BUTTON22: usize = 48 + 22;
pub const DIJOFS_BUTTON23: usize = 48 + 23;
pub const DIJOFS_BUTTON24: usize = 48 + 24;
pub const DIJOFS_BUTTON25: usize = 48 + 25;
pub const DIJOFS_BUTTON26: usize = 48 + 26;
pub const DIJOFS_BUTTON27: usize = 48 + 27;
pub const DIJOFS_BUTTON28: usize = 48 + 28;
pub const DIJOFS_BUTTON29: usize = 48 + 29;
pub const DIJOFS_BUTTON30: usize = 48 + 30;
pub const DIJOFS_BUTTON31: usize = 48 + 31;
pub const DIENUM_STOP: BOOL = 0;
pub const DIENUM_CONTINUE: BOOL = 1;
FN!{stdcall LPDIENUMDEVICESCALLBACKA(
    lpddi: LPCDIDEVICEINSTANCEA,
    pvRef: LPVOID,
) -> BOOL}
FN!{stdcall LPDIENUMDEVICESCALLBACKW(
    lpddi: LPCDIDEVICEINSTANCEW,
    pvRef: LPVOID,
) -> BOOL}
FN!{stdcall LPDICONFIGUREDEVICESCALLBACK(
    lpDDSTarget: *mut IUnknown,
    pvRef: LPVOID,
) -> BOOL}
pub const DIEDFL_ALLDEVICES: DWORD = 0x00000000;
pub const DIEDFL_ATTACHEDONLY: DWORD = 0x00000001;
pub const DIEDFL_FORCEFEEDBACK: DWORD = 0x00000100;
pub const DIEDFL_INCLUDEALIASES: DWORD = 0x00010000;
pub const DIEDFL_INCLUDEPHANTOMS: DWORD = 0x00020000;
pub const DIEDFL_INCLUDEHIDDEN: DWORD = 0x00040000;
FN!{stdcall LPDIENUMDEVICESBYSEMANTICSCBA(
    lpddi: LPCDIDEVICEINSTANCEA,
    lpdid: LPDIRECTINPUTDEVICE8A,
    dwFlags: DWORD,
    dwRemaining: DWORD,
    pvRef: LPVOID,
) -> BOOL}
FN!{stdcall LPDIENUMDEVICESBYSEMANTICSCBW(
    lpddi: LPCDIDEVICEINSTANCEW,
    lpdid: LPDIRECTINPUTDEVICE8W,
    dwFlags: DWORD,
    dwRemaining: DWORD,
    pvRef: LPVOID,
) -> BOOL}
pub const DIEDBS_MAPPEDPRI1: DWORD = 0x00000001;
pub const DIEDBS_MAPPEDPRI2: DWORD = 0x00000002;
pub const DIEDBS_RECENTDEVICE: DWORD = 0x00000010;
pub const DIEDBS_NEWDEVICE: DWORD = 0x00000020;
pub const DIEDBSFL_ATTACHEDONLY: DWORD = 0x00000000;
pub const DIEDBSFL_THISUSER: DWORD = 0x00000010;
pub const DIEDBSFL_FORCEFEEDBACK: DWORD = DIEDFL_FORCEFEEDBACK;
pub const DIEDBSFL_AVAILABLEDEVICES: DWORD = 0x00001000;
pub const DIEDBSFL_MULTIMICEKEYBOARDS: DWORD = 0x00002000;
pub const DIEDBSFL_NONGAMINGDEVICES: DWORD = 0x00004000;
pub const DIEDBSFL_VALID: DWORD = 0x00007110;
RIDL!{#[uuid(0x89521361, 0xaa8a, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00)]
interface IDirectInputW(IDirectInputWVtbl): IUnknown(IUnknownVtbl) {
    fn CreateDevice(
        rguid: REFGUID,
        lplpDirectInputDevice: *mut LPDIRECTINPUTDEVICEW,
        pUnkOuter: LPUNKNOWN,
    ) -> HRESULT,
    fn EnumDevices(
        dwDevType: DWORD,
        lpCallback: LPDIENUMDEVICESCALLBACKW,
        pvRef: LPVOID,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetDeviceStatus(
        rguidInstance: REFGUID,
    ) -> HRESULT,
    fn RunControlPanel(
        hwndOwner: HWND,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn Initialize(
        hinst: HINSTANCE,
        dwVersion: DWORD,
    ) -> HRESULT,
}}
pub type LPDIRECTINPUTW = *mut IDirectInputW;
RIDL!{#[uuid(0x89521360, 0xaa8a, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00)]
interface IDirectInputA(IDirectInputAVtbl): IUnknown(IUnknownVtbl) {
    fn CreateDevice(
        rguid: REFGUID,
        lplpDirectInputDevice: *mut LPDIRECTINPUTDEVICEA,
        pUnkOuter: LPUNKNOWN,
    ) -> HRESULT,
    fn EnumDevices(
        dwDevType: DWORD,
        lpCallback: LPDIENUMDEVICESCALLBACKA,
        pvRef: LPVOID,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetDeviceStatus(
        rguidInstance: REFGUID,
    ) -> HRESULT,
    fn RunControlPanel(
        hwndOwner: HWND,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn Initialize(
        hinst: HINSTANCE,
        dwVersion: DWORD,
    ) -> HRESULT,
}}
pub type LPDIRECTINPUTA = *mut IDirectInputA;
RIDL!{#[uuid(0x5944e663, 0xaa8a, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00)]
interface IDirectInput2W(IDirectInput2WVtbl): IDirectInputW(IDirectInputWVtbl) {
    fn FindDevice(
        rguidClass: REFGUID,
        ptszName: LPCWSTR,
        pguidInstance: LPGUID,
    ) -> HRESULT,
}}
pub type LPDIRECTINPUT2W = *mut IDirectInput2W;
RIDL!{#[uuid(0x5944e662, 0xaa8a, 0x11cf, 0xbf, 0xc7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00)]
interface IDirectInput2A(IDirectInput2AVtbl): IDirectInputA(IDirectInputAVtbl) {
    fn FindDevice(
        rguidClass: REFGUID,
        ptszName: LPCSTR,
        pguidInstance: LPGUID,
    ) -> HRESULT,
}}
pub type LPDIRECTINPUT2A = *mut IDirectInput2A;
RIDL!{#[uuid(0x9a4cb685, 0x236d, 0x11d3, 0x8e, 0x9d, 0x00, 0xc0, 0x4f, 0x68, 0x44, 0xae)]
interface IDirectInput7W(IDirectInput7WVtbl): IDirectInput2W(IDirectInput2WVtbl) {
    fn CreateDeviceEx(
        rguid: REFGUID,
        riid: REFIID,
        pvOut: *mut LPVOID,
        pUnkOuter: LPUNKNOWN,
    ) -> HRESULT,
}}
pub type LPDIRECTINPUT7W = *mut IDirectInput7W;
RIDL!{#[uuid(0x9a4cb684, 0x236d, 0x11d3, 0x8e, 0x9d, 0x00, 0xc0, 0x4f, 0x68, 0x44, 0xae)]
interface IDirectInput7A(IDirectInput7AVtbl): IDirectInput2A(IDirectInput2AVtbl) {
    fn CreateDeviceEx(
        rguid: REFGUID,
        riid: REFIID,
        pvOut: *mut LPVOID,
        pUnkOuter: LPUNKNOWN,
    ) -> HRESULT,
}}
pub type LPDIRECTINPUT7A = *mut IDirectInput7A;
RIDL!{#[uuid(0xbf798031, 0x483a, 0x4da2, 0xaa, 0x99, 0x5d, 0x64, 0xed, 0x36, 0x97, 0x00)]
interface IDirectInput8W(IDirectInput8WVtbl): IUnknown(IUnknownVtbl) {
    fn CreateDevice(
        rguid: REFGUID,
        lplpDirectInputDevice: *mut LPDIRECTINPUTDEVICEW,
        pUnkOuter: LPUNKNOWN,
    ) -> HRESULT,
    fn EnumDevices(
        dwDevType: DWORD,
        lpCallback: LPDIENUMDEVICESCALLBACKW,
        pvRef: LPVOID,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetDeviceStatus(
        rguidInstance: REFGUID,
    ) -> HRESULT,
    fn RunControlPanel(
        hwndOwner: HWND,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn Initialize(
        hinst: HINSTANCE,
        dwVersion: DWORD,
    ) -> HRESULT,
    fn FindDevice(
        rguidClass: REFGUID,
        ptszName: LPCWSTR,
        pguidInstance: LPGUID,
    ) -> HRESULT,
    fn EnumDevicesBySemantics(
        ptszUserName: LPCWSTR,
        lpdiActionFormat: LPDIACTIONFORMATW,
        lpCallback: LPDIENUMDEVICESBYSEMANTICSCBW,
        pvRef: LPVOID,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn ConfigureDevices(
        lpdiCallback: LPDICONFIGUREDEVICESCALLBACK,
        lpdiCDParams: LPDICONFIGUREDEVICESPARAMSW,
        dwFlags: DWORD,
        pvRefData: LPVOID,
    ) -> HRESULT,
}}
pub type LPDIRECTINPUT8W = *mut IDirectInput8W;
RIDL!{#[uuid(0xbf798030, 0x483a, 0x4da2, 0xaa, 0x99, 0x5d, 0x64, 0xed, 0x36, 0x97, 0x00)]
interface IDirectInput8A(IDirectInput8AVtbl): IUnknown(IUnknownVtbl) {
    fn CreateDevice(
        rguid: REFGUID,
        lplpDirectInputDevice: *mut LPDIRECTINPUTDEVICEA,
        pUnkOuter: LPUNKNOWN,
    ) -> HRESULT,
    fn EnumDevices(
        dwDevType: DWORD,
        lpCallback: LPDIENUMDEVICESCALLBACKA,
        pvRef: LPVOID,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetDeviceStatus(
        rguidInstance: REFGUID,
    ) -> HRESULT,
    fn RunControlPanel(
        hwndOwner: HWND,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn Initialize(
        hinst: HINSTANCE,
        dwVersion: DWORD,
    ) -> HRESULT,
    fn FindDevice(
        rguidClass: REFGUID,
        ptszName: LPCSTR,
        pguidInstance: LPGUID,
    ) -> HRESULT,
    fn EnumDevicesBySemantics(
        ptszUserName: LPCSTR,
        lpdiActionFormat: LPDIACTIONFORMATA,
        lpCallback: LPDIENUMDEVICESBYSEMANTICSCBA,
        pvRef: LPVOID,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn ConfigureDevices(
        lpdiCallback: LPDICONFIGUREDEVICESCALLBACK,
        lpdiCDParams: LPDICONFIGUREDEVICESPARAMSA,
        dwFlags: DWORD,
        pvRefData: LPVOID,
    ) -> HRESULT,
}}
pub type LPDIRECTINPUT8A = *mut IDirectInput8A;
extern "system" {
    pub fn DirectInput8Create(
        hinst: HINSTANCE,
        dwVersion: DWORD,
        riidltf: REFIID,
        ppvOut: *mut LPVOID,
        punkOuter: LPUNKNOWN,
    ) -> HRESULT;
    pub fn DirectInputCreateA(
        hinst: HINSTANCE,
        dwVersion: DWORD,
        ppDI: *mut LPDIRECTINPUTA,
        punkOuter: LPUNKNOWN,
    ) -> HRESULT;
    pub fn DirectInputCreateW(
        hinst: HINSTANCE,
        dwVersion: DWORD,
        ppDI: *mut LPDIRECTINPUTW,
        punkOuter: LPUNKNOWN,
    ) -> HRESULT;
    pub fn DirectInputCreateEx(
        hinst: HINSTANCE,
        dwVersion: DWORD,
        riidltd: REFIID,
        ppvOut: *mut LPVOID,
        punkOuter: LPUNKNOWN,
    ) -> HRESULT;
}
pub const DI_OK: HRESULT = S_OK;
pub const DI_NOTATTACHED: HRESULT = S_FALSE;
pub const DI_BUFFEROVERFLOW: HRESULT = S_FALSE;
pub const DI_PROPNOEFFECT: HRESULT = S_FALSE;
pub const DI_NOEFFECT: HRESULT = S_FALSE;
pub const DI_POLLEDDEVICE: HRESULT = 0x00000002;
pub const DI_DOWNLOADSKIPPED: HRESULT = 0x00000003;
pub const DI_EFFECTRESTARTED: HRESULT = 0x00000004;
pub const DI_TRUNCATED: HRESULT = 0x00000008;
pub const DI_SETTINGSNOTSAVED: HRESULT = 0x0000000B;
pub const DI_TRUNCATEDANDRESTARTED: HRESULT = 0x0000000C;
pub const DI_WRITEPROTECT: HRESULT = 0x00000013;
pub const DIERR_OLDDIRECTINPUTVERSION: HRESULT = MAKE_HRESULT!(
    SEVERITY_ERROR, 
    FACILITY_WIN32, 
    ERROR_OLD_WIN_VERSION as HRESULT
);
pub const DIERR_BETADIRECTINPUTVERSION: HRESULT = MAKE_HRESULT!(
    SEVERITY_ERROR, 
    FACILITY_WIN32, 
    ERROR_RMODE_APP as HRESULT
);
pub const DIERR_BADDRIVERVER: HRESULT = MAKE_HRESULT!(
    SEVERITY_ERROR, 
    FACILITY_WIN32, 
    ERROR_BAD_DRIVER_LEVEL as HRESULT
);
pub const DIERR_DEVICENOTREG: HRESULT = REGDB_E_CLASSNOTREG;
pub const DIERR_NOTFOUND: HRESULT = MAKE_HRESULT!(
    SEVERITY_ERROR, 
    FACILITY_WIN32, 
    ERROR_FILE_NOT_FOUND as HRESULT
);
pub const DIERR_OBJECTNOTFOUND: HRESULT = MAKE_HRESULT!(
    SEVERITY_ERROR, 
    FACILITY_WIN32, 
    ERROR_FILE_NOT_FOUND as HRESULT
);
pub const DIERR_INVALIDPARAM: HRESULT = E_INVALIDARG;
pub const DIERR_NOINTERFACE: HRESULT = E_NOINTERFACE;
pub const DIERR_GENERIC: HRESULT = E_FAIL;
pub const DIERR_OUTOFMEMORY: HRESULT = E_OUTOFMEMORY;
pub const DIERR_UNSUPPORTED: HRESULT = E_NOTIMPL;
pub const DIERR_NOTINITIALIZED: HRESULT = MAKE_HRESULT!(
    SEVERITY_ERROR,
    FACILITY_WIN32,
    ERROR_NOT_READY as HRESULT
);
pub const DIERR_ALREADYINITIALIZED: HRESULT = MAKE_HRESULT!(
    SEVERITY_ERROR,
    FACILITY_WIN32,
    ERROR_ALREADY_INITIALIZED as HRESULT
);
pub const DIERR_NOAGGREGATION: HRESULT = CLASS_E_NOAGGREGATION;
pub const DIERR_OTHERAPPHASPRIO: HRESULT = E_ACCESSDENIED;
pub const DIERR_INPUTLOST: HRESULT = MAKE_HRESULT!(
    SEVERITY_ERROR,
    FACILITY_WIN32,
    ERROR_READ_FAULT as HRESULT
);
pub const DIERR_ACQUIRED: HRESULT = MAKE_HRESULT!(
    SEVERITY_ERROR,
    FACILITY_WIN32,
    ERROR_BUSY as HRESULT
);
pub const DIERR_NOTACQUIRED: HRESULT = MAKE_HRESULT!(
    SEVERITY_ERROR,
    FACILITY_WIN32,
    ERROR_INVALID_ACCESS as HRESULT
);
pub const DIERR_READONLY: HRESULT = E_ACCESSDENIED;
pub const DIERR_HANDLEEXISTS: HRESULT = E_ACCESSDENIED;
pub const E_PENDING: HRESULT = 0x8000000A;
pub const DIERR_INSUFFICIENTPRIVS: HRESULT = 0x80040200;
pub const DIERR_DEVICEFULL: HRESULT = 0x80040201;
pub const DIERR_MOREDATA: HRESULT = 0x80040202;
pub const DIERR_NOTDOWNLOADED: HRESULT = 0x80040203;
pub const DIERR_HASEFFECTS: HRESULT = 0x80040204;
pub const DIERR_NOTEXCLUSIVEACQUIRED: HRESULT = 0x80040205;
pub const DIERR_INCOMPLETEEFFECT: HRESULT = 0x80040206;
pub const DIERR_NOTBUFFERED: HRESULT = 0x80040207;
pub const DIERR_EFFECTPLAYING: HRESULT = 0x80040208;
pub const DIERR_UNPLUGGED: HRESULT = 0x80040209;
pub const DIERR_REPORTFULL: HRESULT = 0x8004020A;
pub const DIERR_MAPFILEFAIL: HRESULT = 0x8004020B;
pub const DIKEYBOARD_ESCAPE: DWORD = 0x81000401;
pub const DIKEYBOARD_1: DWORD = 0x81000402;
pub const DIKEYBOARD_2: DWORD = 0x81000403;
pub const DIKEYBOARD_3: DWORD = 0x81000404;
pub const DIKEYBOARD_4: DWORD = 0x81000405;
pub const DIKEYBOARD_5: DWORD = 0x81000406;
pub const DIKEYBOARD_6: DWORD = 0x81000407;
pub const DIKEYBOARD_7: DWORD = 0x81000408;
pub const DIKEYBOARD_8: DWORD = 0x81000409;
pub const DIKEYBOARD_9: DWORD = 0x8100040A;
pub const DIKEYBOARD_0: DWORD = 0x8100040B;
pub const DIKEYBOARD_MINUS: DWORD = 0x8100040C;
pub const DIKEYBOARD_EQUALS: DWORD = 0x8100040D;
pub const DIKEYBOARD_BACK: DWORD = 0x8100040E;
pub const DIKEYBOARD_TAB: DWORD = 0x8100040F;
pub const DIKEYBOARD_Q: DWORD = 0x81000410;
pub const DIKEYBOARD_W: DWORD = 0x81000411;
pub const DIKEYBOARD_E: DWORD = 0x81000412;
pub const DIKEYBOARD_R: DWORD = 0x81000413;
pub const DIKEYBOARD_T: DWORD = 0x81000414;
pub const DIKEYBOARD_Y: DWORD = 0x81000415;
pub const DIKEYBOARD_U: DWORD = 0x81000416;
pub const DIKEYBOARD_I: DWORD = 0x81000417;
pub const DIKEYBOARD_O: DWORD = 0x81000418;
pub const DIKEYBOARD_P: DWORD = 0x81000419;
pub const DIKEYBOARD_LBRACKET: DWORD = 0x8100041A;
pub const DIKEYBOARD_RBRACKET: DWORD = 0x8100041B;
pub const DIKEYBOARD_RETURN: DWORD = 0x8100041C;
pub const DIKEYBOARD_LCONTROL: DWORD = 0x8100041D;
pub const DIKEYBOARD_A: DWORD = 0x8100041E;
pub const DIKEYBOARD_S: DWORD = 0x8100041F;
pub const DIKEYBOARD_D: DWORD = 0x81000420;
pub const DIKEYBOARD_F: DWORD = 0x81000421;
pub const DIKEYBOARD_G: DWORD = 0x81000422;
pub const DIKEYBOARD_H: DWORD = 0x81000423;
pub const DIKEYBOARD_J: DWORD = 0x81000424;
pub const DIKEYBOARD_K: DWORD = 0x81000425;
pub const DIKEYBOARD_L: DWORD = 0x81000426;
pub const DIKEYBOARD_SEMICOLON: DWORD = 0x81000427;
pub const DIKEYBOARD_APOSTROPHE: DWORD = 0x81000428;
pub const DIKEYBOARD_GRAVE: DWORD = 0x81000429;
pub const DIKEYBOARD_LSHIFT: DWORD = 0x8100042A;
pub const DIKEYBOARD_BACKSLASH: DWORD = 0x8100042B;
pub const DIKEYBOARD_Z: DWORD = 0x8100042C;
pub const DIKEYBOARD_X: DWORD = 0x8100042D;
pub const DIKEYBOARD_C: DWORD = 0x8100042E;
pub const DIKEYBOARD_V: DWORD = 0x8100042F;
pub const DIKEYBOARD_B: DWORD = 0x81000430;
pub const DIKEYBOARD_N: DWORD = 0x81000431;
pub const DIKEYBOARD_M: DWORD = 0x81000432;
pub const DIKEYBOARD_COMMA: DWORD = 0x81000433;
pub const DIKEYBOARD_PERIOD: DWORD = 0x81000434;
pub const DIKEYBOARD_SLASH: DWORD = 0x81000435;
pub const DIKEYBOARD_RSHIFT: DWORD = 0x81000436;
pub const DIKEYBOARD_MULTIPLY: DWORD = 0x81000437;
pub const DIKEYBOARD_LMENU: DWORD = 0x81000438;
pub const DIKEYBOARD_SPACE: DWORD = 0x81000439;
pub const DIKEYBOARD_CAPITAL: DWORD = 0x8100043A;
pub const DIKEYBOARD_F1: DWORD = 0x8100043B;
pub const DIKEYBOARD_F2: DWORD = 0x8100043C;
pub const DIKEYBOARD_F3: DWORD = 0x8100043D;
pub const DIKEYBOARD_F4: DWORD = 0x8100043E;
pub const DIKEYBOARD_F5: DWORD = 0x8100043F;
pub const DIKEYBOARD_F6: DWORD = 0x81000440;
pub const DIKEYBOARD_F7: DWORD = 0x81000441;
pub const DIKEYBOARD_F8: DWORD = 0x81000442;
pub const DIKEYBOARD_F9: DWORD = 0x81000443;
pub const DIKEYBOARD_F10: DWORD = 0x81000444;
pub const DIKEYBOARD_NUMLOCK: DWORD = 0x81000445;
pub const DIKEYBOARD_SCROLL: DWORD = 0x81000446;
pub const DIKEYBOARD_NUMPAD7: DWORD = 0x81000447;
pub const DIKEYBOARD_NUMPAD8: DWORD = 0x81000448;
pub const DIKEYBOARD_NUMPAD9: DWORD = 0x81000449;
pub const DIKEYBOARD_SUBTRACT: DWORD = 0x8100044A;
pub const DIKEYBOARD_NUMPAD4: DWORD = 0x8100044B;
pub const DIKEYBOARD_NUMPAD5: DWORD = 0x8100044C;
pub const DIKEYBOARD_NUMPAD6: DWORD = 0x8100044D;
pub const DIKEYBOARD_ADD: DWORD = 0x8100044E;
pub const DIKEYBOARD_NUMPAD1: DWORD = 0x8100044F;
pub const DIKEYBOARD_NUMPAD2: DWORD = 0x81000450;
pub const DIKEYBOARD_NUMPAD3: DWORD = 0x81000451;
pub const DIKEYBOARD_NUMPAD0: DWORD = 0x81000452;
pub const DIKEYBOARD_DECIMAL: DWORD = 0x81000453;
pub const DIKEYBOARD_OEM_102: DWORD = 0x81000456;
pub const DIKEYBOARD_F11: DWORD = 0x81000457;
pub const DIKEYBOARD_F12: DWORD = 0x81000458;
pub const DIKEYBOARD_F13: DWORD = 0x81000464;
pub const DIKEYBOARD_F14: DWORD = 0x81000465;
pub const DIKEYBOARD_F15: DWORD = 0x81000466;
pub const DIKEYBOARD_KANA: DWORD = 0x81000470;
pub const DIKEYBOARD_ABNT_C1: DWORD = 0x81000473;
pub const DIKEYBOARD_CONVERT: DWORD = 0x81000479;
pub const DIKEYBOARD_NOCONVERT: DWORD = 0x8100047B;
pub const DIKEYBOARD_YEN: DWORD = 0x8100047D;
pub const DIKEYBOARD_ABNT_C2: DWORD = 0x8100047E;
pub const DIKEYBOARD_NUMPADEQUALS: DWORD = 0x8100048D;
pub const DIKEYBOARD_PREVTRACK: DWORD = 0x81000490;
pub const DIKEYBOARD_AT: DWORD = 0x81000491;
pub const DIKEYBOARD_COLON: DWORD = 0x81000492;
pub const DIKEYBOARD_UNDERLINE: DWORD = 0x81000493;
pub const DIKEYBOARD_KANJI: DWORD = 0x81000494;
pub const DIKEYBOARD_STOP: DWORD = 0x81000495;
pub const DIKEYBOARD_AX: DWORD = 0x81000496;
pub const DIKEYBOARD_UNLABELED: DWORD = 0x81000497;
pub const DIKEYBOARD_NEXTTRACK: DWORD = 0x81000499;
pub const DIKEYBOARD_NUMPADENTER: DWORD = 0x8100049C;
pub const DIKEYBOARD_RCONTROL: DWORD = 0x8100049D;
pub const DIKEYBOARD_MUTE: DWORD = 0x810004A0;
pub const DIKEYBOARD_CALCULATOR: DWORD = 0x810004A1;
pub const DIKEYBOARD_PLAYPAUSE: DWORD = 0x810004A2;
pub const DIKEYBOARD_MEDIASTOP: DWORD = 0x810004A4;
pub const DIKEYBOARD_VOLUMEDOWN: DWORD = 0x810004AE;
pub const DIKEYBOARD_VOLUMEUP: DWORD = 0x810004B0;
pub const DIKEYBOARD_WEBHOME: DWORD = 0x810004B2;
pub const DIKEYBOARD_NUMPADCOMMA: DWORD = 0x810004B3;
pub const DIKEYBOARD_DIVIDE: DWORD = 0x810004B5;
pub const DIKEYBOARD_SYSRQ: DWORD = 0x810004B7;
pub const DIKEYBOARD_RMENU: DWORD = 0x810004B8;
pub const DIKEYBOARD_PAUSE: DWORD = 0x810004C5;
pub const DIKEYBOARD_HOME: DWORD = 0x810004C7;
pub const DIKEYBOARD_UP: DWORD = 0x810004C8;
pub const DIKEYBOARD_PRIOR: DWORD = 0x810004C9;
pub const DIKEYBOARD_LEFT: DWORD = 0x810004CB;
pub const DIKEYBOARD_RIGHT: DWORD = 0x810004CD;
pub const DIKEYBOARD_END: DWORD = 0x810004CF;
pub const DIKEYBOARD_DOWN: DWORD = 0x810004D0;
pub const DIKEYBOARD_NEXT: DWORD = 0x810004D1;
pub const DIKEYBOARD_INSERT: DWORD = 0x810004D2;
pub const DIKEYBOARD_DELETE: DWORD = 0x810004D3;
pub const DIKEYBOARD_LWIN: DWORD = 0x810004DB;
pub const DIKEYBOARD_RWIN: DWORD = 0x810004DC;
pub const DIKEYBOARD_APPS: DWORD = 0x810004DD;
pub const DIKEYBOARD_POWER: DWORD = 0x810004DE;
pub const DIKEYBOARD_SLEEP: DWORD = 0x810004DF;
pub const DIKEYBOARD_WAKE: DWORD = 0x810004E3;
pub const DIKEYBOARD_WEBSEARCH: DWORD = 0x810004E5;
pub const DIKEYBOARD_WEBFAVORITES: DWORD = 0x810004E6;
pub const DIKEYBOARD_WEBREFRESH: DWORD = 0x810004E7;
pub const DIKEYBOARD_WEBSTOP: DWORD = 0x810004E8;
pub const DIKEYBOARD_WEBFORWARD: DWORD = 0x810004E9;
pub const DIKEYBOARD_WEBBACK: DWORD = 0x810004EA;
pub const DIKEYBOARD_MYCOMPUTER: DWORD = 0x810004EB;
pub const DIKEYBOARD_MAIL: DWORD = 0x810004EC;
pub const DIKEYBOARD_MEDIASELECT: DWORD = 0x810004ED;
pub const DIMOUSE_XAXISAB: DWORD = 0x82000200 | DIMOFS_X as DWORD;
pub const DIMOUSE_YAXISAB: DWORD = 0x82000200 | DIMOFS_Y as DWORD;
pub const DIMOUSE_XAXIS: DWORD = 0x82000300 | DIMOFS_X as DWORD;
pub const DIMOUSE_YAXIS: DWORD = 0x82000300 | DIMOFS_Y as DWORD;
pub const DIMOUSE_WHEEL: DWORD = 0x82000300 | DIMOFS_Z as DWORD;
pub const DIMOUSE_BUTTON0: DWORD = 0x82000400 | DIMOFS_BUTTON0 as DWORD;
pub const DIMOUSE_BUTTON1: DWORD = 0x82000400 | DIMOFS_BUTTON1 as DWORD;
pub const DIMOUSE_BUTTON2: DWORD = 0x82000400 | DIMOFS_BUTTON2 as DWORD;
pub const DIMOUSE_BUTTON3: DWORD = 0x82000400 | DIMOFS_BUTTON3 as DWORD;
pub const DIMOUSE_BUTTON4: DWORD = 0x82000400 | DIMOFS_BUTTON4 as DWORD;
pub const DIMOUSE_BUTTON5: DWORD = 0x82000400 | DIMOFS_BUTTON5 as DWORD;
pub const DIMOUSE_BUTTON6: DWORD = 0x82000400 | DIMOFS_BUTTON6 as DWORD;
pub const DIMOUSE_BUTTON7: DWORD = 0x82000400 | DIMOFS_BUTTON7 as DWORD;
pub const DIVOICE_CHANNEL1: DWORD = 0x83000401;
pub const DIVOICE_CHANNEL2: DWORD = 0x83000402;
pub const DIVOICE_CHANNEL3: DWORD = 0x83000403;
pub const DIVOICE_CHANNEL4: DWORD = 0x83000404;
pub const DIVOICE_CHANNEL5: DWORD = 0x83000405;
pub const DIVOICE_CHANNEL6: DWORD = 0x83000406;
pub const DIVOICE_CHANNEL7: DWORD = 0x83000407;
pub const DIVOICE_CHANNEL8: DWORD = 0x83000408;
pub const DIVOICE_TEAM: DWORD = 0x83000409;
pub const DIVOICE_ALL: DWORD = 0x8300040A;
pub const DIVOICE_RECORDMUTE: DWORD = 0x8300040B;
pub const DIVOICE_PLAYBACKMUTE: DWORD = 0x8300040C;
pub const DIVOICE_TRANSMIT: DWORD = 0x8300040D;
pub const DIVOICE_VOICECOMMAND: DWORD = 0x83000410;
pub const DIVIRTUAL_DRIVING_RACE: DWORD = 0x01000000;
pub const DIAXIS_DRIVINGR_STEER: DWORD = 0x01008A01;
pub const DIAXIS_DRIVINGR_ACCELERATE: DWORD = 0x01039202;
pub const DIAXIS_DRIVINGR_BRAKE: DWORD = 0x01041203;
pub const DIBUTTON_DRIVINGR_SHIFTUP: DWORD = 0x01000C01;
pub const DIBUTTON_DRIVINGR_SHIFTDOWN: DWORD = 0x01000C02;
pub const DIBUTTON_DRIVINGR_VIEW: DWORD = 0x01001C03;
pub const DIBUTTON_DRIVINGR_MENU: DWORD = 0x010004FD;
pub const DIAXIS_DRIVINGR_ACCEL_AND_BRAKE: DWORD = 0x01014A04;
pub const DIHATSWITCH_DRIVINGR_GLANCE: DWORD = 0x01004601;
pub const DIBUTTON_DRIVINGR_BRAKE: DWORD = 0x01004C04;
pub const DIBUTTON_DRIVINGR_DASHBOARD: DWORD = 0x01004405;
pub const DIBUTTON_DRIVINGR_AIDS: DWORD = 0x01004406;
pub const DIBUTTON_DRIVINGR_MAP: DWORD = 0x01004407;
pub const DIBUTTON_DRIVINGR_BOOST: DWORD = 0x01004408;
pub const DIBUTTON_DRIVINGR_PIT: DWORD = 0x01004409;
pub const DIBUTTON_DRIVINGR_ACCELERATE_LINK: DWORD = 0x0103D4E0;
pub const DIBUTTON_DRIVINGR_STEER_LEFT_LINK: DWORD = 0x0100CCE4;
pub const DIBUTTON_DRIVINGR_STEER_RIGHT_LINK: DWORD = 0x0100CCEC;
pub const DIBUTTON_DRIVINGR_GLANCE_LEFT_LINK: DWORD = 0x0107C4E4;
pub const DIBUTTON_DRIVINGR_GLANCE_RIGHT_LINK: DWORD = 0x0107C4EC;
pub const DIBUTTON_DRIVINGR_DEVICE: DWORD = 0x010044FE;
pub const DIBUTTON_DRIVINGR_PAUSE: DWORD = 0x010044FC;
pub const DIVIRTUAL_DRIVING_COMBAT: DWORD = 0x02000000;
pub const DIAXIS_DRIVINGC_STEER: DWORD = 0x02008A01;
pub const DIAXIS_DRIVINGC_ACCELERATE: DWORD = 0x02039202;
pub const DIAXIS_DRIVINGC_BRAKE: DWORD = 0x02041203;
pub const DIBUTTON_DRIVINGC_FIRE: DWORD = 0x02000C01;
pub const DIBUTTON_DRIVINGC_WEAPONS: DWORD = 0x02000C02;
pub const DIBUTTON_DRIVINGC_TARGET: DWORD = 0x02000C03;
pub const DIBUTTON_DRIVINGC_MENU: DWORD = 0x020004FD;
pub const DIAXIS_DRIVINGC_ACCEL_AND_BRAKE: DWORD = 0x02014A04;
pub const DIHATSWITCH_DRIVINGC_GLANCE: DWORD = 0x02004601;
pub const DIBUTTON_DRIVINGC_SHIFTUP: DWORD = 0x02004C04;
pub const DIBUTTON_DRIVINGC_SHIFTDOWN: DWORD = 0x02004C05;
pub const DIBUTTON_DRIVINGC_DASHBOARD: DWORD = 0x02004406;
pub const DIBUTTON_DRIVINGC_AIDS: DWORD = 0x02004407;
pub const DIBUTTON_DRIVINGC_BRAKE: DWORD = 0x02004C08;
pub const DIBUTTON_DRIVINGC_FIRESECONDARY: DWORD = 0x02004C09;
pub const DIBUTTON_DRIVINGC_ACCELERATE_LINK: DWORD = 0x0203D4E0;
pub const DIBUTTON_DRIVINGC_STEER_LEFT_LINK: DWORD = 0x0200CCE4;
pub const DIBUTTON_DRIVINGC_STEER_RIGHT_LINK: DWORD = 0x0200CCEC;
pub const DIBUTTON_DRIVINGC_GLANCE_LEFT_LINK: DWORD = 0x0207C4E4;
pub const DIBUTTON_DRIVINGC_GLANCE_RIGHT_LINK: DWORD = 0x0207C4EC;
pub const DIBUTTON_DRIVINGC_DEVICE: DWORD = 0x020044FE;
pub const DIBUTTON_DRIVINGC_PAUSE: DWORD = 0x020044FC;
pub const DIVIRTUAL_DRIVING_TANK: DWORD = 0x03000000;
pub const DIAXIS_DRIVINGT_STEER: DWORD = 0x03008A01;
pub const DIAXIS_DRIVINGT_BARREL: DWORD = 0x03010202;
pub const DIAXIS_DRIVINGT_ACCELERATE: DWORD = 0x03039203;
pub const DIAXIS_DRIVINGT_ROTATE: DWORD = 0x03020204;
pub const DIBUTTON_DRIVINGT_FIRE: DWORD = 0x03000C01;
pub const DIBUTTON_DRIVINGT_WEAPONS: DWORD = 0x03000C02;
pub const DIBUTTON_DRIVINGT_TARGET: DWORD = 0x03000C03;
pub const DIBUTTON_DRIVINGT_MENU: DWORD = 0x030004FD;
pub const DIHATSWITCH_DRIVINGT_GLANCE: DWORD = 0x03004601;
pub const DIAXIS_DRIVINGT_BRAKE: DWORD = 0x03045205;
pub const DIAXIS_DRIVINGT_ACCEL_AND_BRAKE: DWORD = 0x03014A06;
pub const DIBUTTON_DRIVINGT_VIEW: DWORD = 0x03005C04;
pub const DIBUTTON_DRIVINGT_DASHBOARD: DWORD = 0x03005C05;
pub const DIBUTTON_DRIVINGT_BRAKE: DWORD = 0x03004C06;
pub const DIBUTTON_DRIVINGT_FIRESECONDARY: DWORD = 0x03004C07;
pub const DIBUTTON_DRIVINGT_ACCELERATE_LINK: DWORD = 0x0303D4E0;
pub const DIBUTTON_DRIVINGT_STEER_LEFT_LINK: DWORD = 0x0300CCE4;
pub const DIBUTTON_DRIVINGT_STEER_RIGHT_LINK: DWORD = 0x0300CCEC;
pub const DIBUTTON_DRIVINGT_BARREL_UP_LINK: DWORD = 0x030144E0;
pub const DIBUTTON_DRIVINGT_BARREL_DOWN_LINK: DWORD = 0x030144E8;
pub const DIBUTTON_DRIVINGT_ROTATE_LEFT_LINK: DWORD = 0x030244E4;
pub const DIBUTTON_DRIVINGT_ROTATE_RIGHT_LINK: DWORD = 0x030244EC;
pub const DIBUTTON_DRIVINGT_GLANCE_LEFT_LINK: DWORD = 0x0307C4E4;
pub const DIBUTTON_DRIVINGT_GLANCE_RIGHT_LINK: DWORD = 0x0307C4EC;
pub const DIBUTTON_DRIVINGT_DEVICE: DWORD = 0x030044FE;
pub const DIBUTTON_DRIVINGT_PAUSE: DWORD = 0x030044FC;
pub const DIVIRTUAL_FLYING_CIVILIAN: DWORD = 0x04000000;
pub const DIAXIS_FLYINGC_BANK: DWORD = 0x04008A01;
pub const DIAXIS_FLYINGC_PITCH: DWORD = 0x04010A02;
pub const DIAXIS_FLYINGC_THROTTLE: DWORD = 0x04039203;
pub const DIBUTTON_FLYINGC_VIEW: DWORD = 0x04002401;
pub const DIBUTTON_FLYINGC_DISPLAY: DWORD = 0x04002402;
pub const DIBUTTON_FLYINGC_GEAR: DWORD = 0x04002C03;
pub const DIBUTTON_FLYINGC_MENU: DWORD = 0x040004FD;
pub const DIHATSWITCH_FLYINGC_GLANCE: DWORD = 0x04004601;
pub const DIAXIS_FLYINGC_BRAKE: DWORD = 0x04046A04;
pub const DIAXIS_FLYINGC_RUDDER: DWORD = 0x04025205;
pub const DIAXIS_FLYINGC_FLAPS: DWORD = 0x04055A06;
pub const DIBUTTON_FLYINGC_FLAPSUP: DWORD = 0x04006404;
pub const DIBUTTON_FLYINGC_FLAPSDOWN: DWORD = 0x04006405;
pub const DIBUTTON_FLYINGC_BRAKE_LINK: DWORD = 0x04046CE0;
pub const DIBUTTON_FLYINGC_FASTER_LINK: DWORD = 0x0403D4E0;
pub const DIBUTTON_FLYINGC_SLOWER_LINK: DWORD = 0x0403D4E8;
pub const DIBUTTON_FLYINGC_GLANCE_LEFT_LINK: DWORD = 0x0407C4E4;
pub const DIBUTTON_FLYINGC_GLANCE_RIGHT_LINK: DWORD = 0x0407C4EC;
pub const DIBUTTON_FLYINGC_GLANCE_UP_LINK: DWORD = 0x0407C4E0;
pub const DIBUTTON_FLYINGC_GLANCE_DOWN_LINK: DWORD = 0x0407C4E8;
pub const DIBUTTON_FLYINGC_DEVICE: DWORD = 0x040044FE;
pub const DIBUTTON_FLYINGC_PAUSE: DWORD = 0x040044FC;
pub const DIVIRTUAL_FLYING_MILITARY: DWORD = 0x05000000;
pub const DIAXIS_FLYINGM_BANK: DWORD = 0x05008A01;
pub const DIAXIS_FLYINGM_PITCH: DWORD = 0x05010A02;
pub const DIAXIS_FLYINGM_THROTTLE: DWORD = 0x05039203;
pub const DIBUTTON_FLYINGM_FIRE: DWORD = 0x05000C01;
pub const DIBUTTON_FLYINGM_WEAPONS: DWORD = 0x05000C02;
pub const DIBUTTON_FLYINGM_TARGET: DWORD = 0x05000C03;
pub const DIBUTTON_FLYINGM_MENU: DWORD = 0x050004FD;
pub const DIHATSWITCH_FLYINGM_GLANCE: DWORD = 0x05004601;
pub const DIBUTTON_FLYINGM_COUNTER: DWORD = 0x05005C04;
pub const DIAXIS_FLYINGM_RUDDER: DWORD = 0x05024A04;
pub const DIAXIS_FLYINGM_BRAKE: DWORD = 0x05046205;
pub const DIBUTTON_FLYINGM_VIEW: DWORD = 0x05006405;
pub const DIBUTTON_FLYINGM_DISPLAY: DWORD = 0x05006406;
pub const DIAXIS_FLYINGM_FLAPS: DWORD = 0x05055206;
pub const DIBUTTON_FLYINGM_FLAPSUP: DWORD = 0x05005407;
pub const DIBUTTON_FLYINGM_FLAPSDOWN: DWORD = 0x05005408;
pub const DIBUTTON_FLYINGM_FIRESECONDARY: DWORD = 0x05004C09;
pub const DIBUTTON_FLYINGM_GEAR: DWORD = 0x0500640A;
pub const DIBUTTON_FLYINGM_BRAKE_LINK: DWORD = 0x050464E0;
pub const DIBUTTON_FLYINGM_FASTER_LINK: DWORD = 0x0503D4E0;
pub const DIBUTTON_FLYINGM_SLOWER_LINK: DWORD = 0x0503D4E8;
pub const DIBUTTON_FLYINGM_GLANCE_LEFT_LINK: DWORD = 0x0507C4E4;
pub const DIBUTTON_FLYINGM_GLANCE_RIGHT_LINK: DWORD = 0x0507C4EC;
pub const DIBUTTON_FLYINGM_GLANCE_UP_LINK: DWORD = 0x0507C4E0;
pub const DIBUTTON_FLYINGM_GLANCE_DOWN_LINK: DWORD = 0x0507C4E8;
pub const DIBUTTON_FLYINGM_DEVICE: DWORD = 0x050044FE;
pub const DIBUTTON_FLYINGM_PAUSE: DWORD = 0x050044FC;
pub const DIVIRTUAL_FLYING_HELICOPTER: DWORD = 0x06000000;
pub const DIAXIS_FLYINGH_BANK: DWORD = 0x06008A01;
pub const DIAXIS_FLYINGH_PITCH: DWORD = 0x06010A02;
pub const DIAXIS_FLYINGH_COLLECTIVE: DWORD = 0x06018A03;
pub const DIBUTTON_FLYINGH_FIRE: DWORD = 0x06001401;
pub const DIBUTTON_FLYINGH_WEAPONS: DWORD = 0x06001402;
pub const DIBUTTON_FLYINGH_TARGET: DWORD = 0x06001403;
pub const DIBUTTON_FLYINGH_MENU: DWORD = 0x060004FD;
pub const DIHATSWITCH_FLYINGH_GLANCE: DWORD = 0x06004601;
pub const DIAXIS_FLYINGH_TORQUE: DWORD = 0x06025A04;
pub const DIAXIS_FLYINGH_THROTTLE: DWORD = 0x0603DA05;
pub const DIBUTTON_FLYINGH_COUNTER: DWORD = 0x06005404;
pub const DIBUTTON_FLYINGH_VIEW: DWORD = 0x06006405;
pub const DIBUTTON_FLYINGH_GEAR: DWORD = 0x06006406;
pub const DIBUTTON_FLYINGH_FIRESECONDARY: DWORD = 0x06004C07;
pub const DIBUTTON_FLYINGH_FASTER_LINK: DWORD = 0x0603DCE0;
pub const DIBUTTON_FLYINGH_SLOWER_LINK: DWORD = 0x0603DCE8;
pub const DIBUTTON_FLYINGH_GLANCE_LEFT_LINK: DWORD = 0x0607C4E4;
pub const DIBUTTON_FLYINGH_GLANCE_RIGHT_LINK: DWORD = 0x0607C4EC;
pub const DIBUTTON_FLYINGH_GLANCE_UP_LINK: DWORD = 0x0607C4E0;
pub const DIBUTTON_FLYINGH_GLANCE_DOWN_LINK: DWORD = 0x0607C4E8;
pub const DIBUTTON_FLYINGH_DEVICE: DWORD = 0x060044FE;
pub const DIBUTTON_FLYINGH_PAUSE: DWORD = 0x060044FC;
pub const DIVIRTUAL_SPACESIM: DWORD = 0x07000000;
pub const DIAXIS_SPACESIM_LATERAL: DWORD = 0x07008201;
pub const DIAXIS_SPACESIM_MOVE: DWORD = 0x07010202;
pub const DIAXIS_SPACESIM_THROTTLE: DWORD = 0x07038203;
pub const DIBUTTON_SPACESIM_FIRE: DWORD = 0x07000401;
pub const DIBUTTON_SPACESIM_WEAPONS: DWORD = 0x07000402;
pub const DIBUTTON_SPACESIM_TARGET: DWORD = 0x07000403;
pub const DIBUTTON_SPACESIM_MENU: DWORD = 0x070004FD;
pub const DIHATSWITCH_SPACESIM_GLANCE: DWORD = 0x07004601;
pub const DIAXIS_SPACESIM_CLIMB: DWORD = 0x0701C204;
pub const DIAXIS_SPACESIM_ROTATE: DWORD = 0x07024205;
pub const DIBUTTON_SPACESIM_VIEW: DWORD = 0x07004404;
pub const DIBUTTON_SPACESIM_DISPLAY: DWORD = 0x07004405;
pub const DIBUTTON_SPACESIM_RAISE: DWORD = 0x07004406;
pub const DIBUTTON_SPACESIM_LOWER: DWORD = 0x07004407;
pub const DIBUTTON_SPACESIM_GEAR: DWORD = 0x07004408;
pub const DIBUTTON_SPACESIM_FIRESECONDARY: DWORD = 0x07004409;
pub const DIBUTTON_SPACESIM_LEFT_LINK: DWORD = 0x0700C4E4;
pub const DIBUTTON_SPACESIM_RIGHT_LINK: DWORD = 0x0700C4EC;
pub const DIBUTTON_SPACESIM_FORWARD_LINK: DWORD = 0x070144E0;
pub const DIBUTTON_SPACESIM_BACKWARD_LINK: DWORD = 0x070144E8;
pub const DIBUTTON_SPACESIM_FASTER_LINK: DWORD = 0x0703C4E0;
pub const DIBUTTON_SPACESIM_SLOWER_LINK: DWORD = 0x0703C4E8;
pub const DIBUTTON_SPACESIM_TURN_LEFT_LINK: DWORD = 0x070244E4;
pub const DIBUTTON_SPACESIM_TURN_RIGHT_LINK: DWORD = 0x070244EC;
pub const DIBUTTON_SPACESIM_GLANCE_LEFT_LINK: DWORD = 0x0707C4E4;
pub const DIBUTTON_SPACESIM_GLANCE_RIGHT_LINK: DWORD = 0x0707C4EC;
pub const DIBUTTON_SPACESIM_GLANCE_UP_LINK: DWORD = 0x0707C4E0;
pub const DIBUTTON_SPACESIM_GLANCE_DOWN_LINK: DWORD = 0x0707C4E8;
pub const DIBUTTON_SPACESIM_DEVICE: DWORD = 0x070044FE;
pub const DIBUTTON_SPACESIM_PAUSE: DWORD = 0x070044FC;
pub const DIVIRTUAL_FIGHTING_HAND2HAND: DWORD = 0x08000000;
pub const DIAXIS_FIGHTINGH_LATERAL: DWORD = 0x08008201;
pub const DIAXIS_FIGHTINGH_MOVE: DWORD = 0x08010202;
pub const DIBUTTON_FIGHTINGH_PUNCH: DWORD = 0x08000401;
pub const DIBUTTON_FIGHTINGH_KICK: DWORD = 0x08000402;
pub const DIBUTTON_FIGHTINGH_BLOCK: DWORD = 0x08000403;
pub const DIBUTTON_FIGHTINGH_CROUCH: DWORD = 0x08000404;
pub const DIBUTTON_FIGHTINGH_JUMP: DWORD = 0x08000405;
pub const DIBUTTON_FIGHTINGH_SPECIAL1: DWORD = 0x08000406;
pub const DIBUTTON_FIGHTINGH_SPECIAL2: DWORD = 0x08000407;
pub const DIBUTTON_FIGHTINGH_MENU: DWORD = 0x080004FD;
pub const DIBUTTON_FIGHTINGH_SELECT: DWORD = 0x08004408;
pub const DIHATSWITCH_FIGHTINGH_SLIDE: DWORD = 0x08004601;
pub const DIBUTTON_FIGHTINGH_DISPLAY: DWORD = 0x08004409;
pub const DIAXIS_FIGHTINGH_ROTATE: DWORD = 0x08024203;
pub const DIBUTTON_FIGHTINGH_DODGE: DWORD = 0x0800440A;
pub const DIBUTTON_FIGHTINGH_LEFT_LINK: DWORD = 0x0800C4E4;
pub const DIBUTTON_FIGHTINGH_RIGHT_LINK: DWORD = 0x0800C4EC;
pub const DIBUTTON_FIGHTINGH_FORWARD_LINK: DWORD = 0x080144E0;
pub const DIBUTTON_FIGHTINGH_BACKWARD_LINK: DWORD = 0x080144E8;
pub const DIBUTTON_FIGHTINGH_DEVICE: DWORD = 0x080044FE;
pub const DIBUTTON_FIGHTINGH_PAUSE: DWORD = 0x080044FC;
pub const DIVIRTUAL_FIGHTING_FPS: DWORD = 0x09000000;
pub const DIAXIS_FPS_ROTATE: DWORD = 0x09008201;
pub const DIAXIS_FPS_MOVE: DWORD = 0x09010202;
pub const DIBUTTON_FPS_FIRE: DWORD = 0x09000401;
pub const DIBUTTON_FPS_WEAPONS: DWORD = 0x09000402;
pub const DIBUTTON_FPS_APPLY: DWORD = 0x09000403;
pub const DIBUTTON_FPS_SELECT: DWORD = 0x09000404;
pub const DIBUTTON_FPS_CROUCH: DWORD = 0x09000405;
pub const DIBUTTON_FPS_JUMP: DWORD = 0x09000406;
pub const DIAXIS_FPS_LOOKUPDOWN: DWORD = 0x09018203;
pub const DIBUTTON_FPS_STRAFE: DWORD = 0x09000407;
pub const DIBUTTON_FPS_MENU: DWORD = 0x090004FD;
pub const DIHATSWITCH_FPS_GLANCE: DWORD = 0x09004601;
pub const DIBUTTON_FPS_DISPLAY: DWORD = 0x09004408;
pub const DIAXIS_FPS_SIDESTEP: DWORD = 0x09024204;
pub const DIBUTTON_FPS_DODGE: DWORD = 0x09004409;
pub const DIBUTTON_FPS_GLANCEL: DWORD = 0x0900440A;
pub const DIBUTTON_FPS_GLANCER: DWORD = 0x0900440B;
pub const DIBUTTON_FPS_FIRESECONDARY: DWORD = 0x0900440C;
pub const DIBUTTON_FPS_ROTATE_LEFT_LINK: DWORD = 0x0900C4E4;
pub const DIBUTTON_FPS_ROTATE_RIGHT_LINK: DWORD = 0x0900C4EC;
pub const DIBUTTON_FPS_FORWARD_LINK: DWORD = 0x090144E0;
pub const DIBUTTON_FPS_BACKWARD_LINK: DWORD = 0x090144E8;
pub const DIBUTTON_FPS_GLANCE_UP_LINK: DWORD = 0x0901C4E0;
pub const DIBUTTON_FPS_GLANCE_DOWN_LINK: DWORD = 0x0901C4E8;
pub const DIBUTTON_FPS_STEP_LEFT_LINK: DWORD = 0x090244E4;
pub const DIBUTTON_FPS_STEP_RIGHT_LINK: DWORD = 0x090244EC;
pub const DIBUTTON_FPS_DEVICE: DWORD = 0x090044FE;
pub const DIBUTTON_FPS_PAUSE: DWORD = 0x090044FC;
pub const DIVIRTUAL_FIGHTING_THIRDPERSON: DWORD = 0x0A000000;
pub const DIAXIS_TPS_TURN: DWORD = 0x0A020201;
pub const DIAXIS_TPS_MOVE: DWORD = 0x0A010202;
pub const DIBUTTON_TPS_RUN: DWORD = 0x0A000401;
pub const DIBUTTON_TPS_ACTION: DWORD = 0x0A000402;
pub const DIBUTTON_TPS_SELECT: DWORD = 0x0A000403;
pub const DIBUTTON_TPS_USE: DWORD = 0x0A000404;
pub const DIBUTTON_TPS_JUMP: DWORD = 0x0A000405;
pub const DIBUTTON_TPS_MENU: DWORD = 0x0A0004FD;
pub const DIHATSWITCH_TPS_GLANCE: DWORD = 0x0A004601;
pub const DIBUTTON_TPS_VIEW: DWORD = 0x0A004406;
pub const DIBUTTON_TPS_STEPLEFT: DWORD = 0x0A004407;
pub const DIBUTTON_TPS_STEPRIGHT: DWORD = 0x0A004408;
pub const DIAXIS_TPS_STEP: DWORD = 0x0A00C203;
pub const DIBUTTON_TPS_DODGE: DWORD = 0x0A004409;
pub const DIBUTTON_TPS_INVENTORY: DWORD = 0x0A00440A;
pub const DIBUTTON_TPS_TURN_LEFT_LINK: DWORD = 0x0A0244E4;
pub const DIBUTTON_TPS_TURN_RIGHT_LINK: DWORD = 0x0A0244EC;
pub const DIBUTTON_TPS_FORWARD_LINK: DWORD = 0x0A0144E0;
pub const DIBUTTON_TPS_BACKWARD_LINK: DWORD = 0x0A0144E8;
pub const DIBUTTON_TPS_GLANCE_UP_LINK: DWORD = 0x0A07C4E0;
pub const DIBUTTON_TPS_GLANCE_DOWN_LINK: DWORD = 0x0A07C4E8;
pub const DIBUTTON_TPS_GLANCE_LEFT_LINK: DWORD = 0x0A07C4E4;
pub const DIBUTTON_TPS_GLANCE_RIGHT_LINK: DWORD = 0x0A07C4EC;
pub const DIBUTTON_TPS_DEVICE: DWORD = 0x0A0044FE;
pub const DIBUTTON_TPS_PAUSE: DWORD = 0x0A0044FC;
pub const DIVIRTUAL_STRATEGY_ROLEPLAYING: DWORD = 0x0B000000;
pub const DIAXIS_STRATEGYR_LATERAL: DWORD = 0x0B008201;
pub const DIAXIS_STRATEGYR_MOVE: DWORD = 0x0B010202;
pub const DIBUTTON_STRATEGYR_GET: DWORD = 0x0B000401;
pub const DIBUTTON_STRATEGYR_APPLY: DWORD = 0x0B000402;
pub const DIBUTTON_STRATEGYR_SELECT: DWORD = 0x0B000403;
pub const DIBUTTON_STRATEGYR_ATTACK: DWORD = 0x0B000404;
pub const DIBUTTON_STRATEGYR_CAST: DWORD = 0x0B000405;
pub const DIBUTTON_STRATEGYR_CROUCH: DWORD = 0x0B000406;
pub const DIBUTTON_STRATEGYR_JUMP: DWORD = 0x0B000407;
pub const DIBUTTON_STRATEGYR_MENU: DWORD = 0x0B0004FD;
pub const DIHATSWITCH_STRATEGYR_GLANCE: DWORD = 0x0B004601;
pub const DIBUTTON_STRATEGYR_MAP: DWORD = 0x0B004408;
pub const DIBUTTON_STRATEGYR_DISPLAY: DWORD = 0x0B004409;
pub const DIAXIS_STRATEGYR_ROTATE: DWORD = 0x0B024203;
pub const DIBUTTON_STRATEGYR_LEFT_LINK: DWORD = 0x0B00C4E4;
pub const DIBUTTON_STRATEGYR_RIGHT_LINK: DWORD = 0x0B00C4EC;
pub const DIBUTTON_STRATEGYR_FORWARD_LINK: DWORD = 0x0B0144E0;
pub const DIBUTTON_STRATEGYR_BACK_LINK: DWORD = 0x0B0144E8;
pub const DIBUTTON_STRATEGYR_ROTATE_LEFT_LINK: DWORD = 0x0B0244E4;
pub const DIBUTTON_STRATEGYR_ROTATE_RIGHT_LINK: DWORD = 0x0B0244EC;
pub const DIBUTTON_STRATEGYR_DEVICE: DWORD = 0x0B0044FE;
pub const DIBUTTON_STRATEGYR_PAUSE: DWORD = 0x0B0044FC;
pub const DIVIRTUAL_STRATEGY_TURN: DWORD = 0x0C000000;
pub const DIAXIS_STRATEGYT_LATERAL: DWORD = 0x0C008201;
pub const DIAXIS_STRATEGYT_MOVE: DWORD = 0x0C010202;
pub const DIBUTTON_STRATEGYT_SELECT: DWORD = 0x0C000401;
pub const DIBUTTON_STRATEGYT_INSTRUCT: DWORD = 0x0C000402;
pub const DIBUTTON_STRATEGYT_APPLY: DWORD = 0x0C000403;
pub const DIBUTTON_STRATEGYT_TEAM: DWORD = 0x0C000404;
pub const DIBUTTON_STRATEGYT_TURN: DWORD = 0x0C000405;
pub const DIBUTTON_STRATEGYT_MENU: DWORD = 0x0C0004FD;
pub const DIBUTTON_STRATEGYT_ZOOM: DWORD = 0x0C004406;
pub const DIBUTTON_STRATEGYT_MAP: DWORD = 0x0C004407;
pub const DIBUTTON_STRATEGYT_DISPLAY: DWORD = 0x0C004408;
pub const DIBUTTON_STRATEGYT_LEFT_LINK: DWORD = 0x0C00C4E4;
pub const DIBUTTON_STRATEGYT_RIGHT_LINK: DWORD = 0x0C00C4EC;
pub const DIBUTTON_STRATEGYT_FORWARD_LINK: DWORD = 0x0C0144E0;
pub const DIBUTTON_STRATEGYT_BACK_LINK: DWORD = 0x0C0144E8;
pub const DIBUTTON_STRATEGYT_DEVICE: DWORD = 0x0C0044FE;
pub const DIBUTTON_STRATEGYT_PAUSE: DWORD = 0x0C0044FC;
pub const DIVIRTUAL_SPORTS_HUNTING: DWORD = 0x0D000000;
pub const DIAXIS_HUNTING_LATERAL: DWORD = 0x0D008201;
pub const DIAXIS_HUNTING_MOVE: DWORD = 0x0D010202;
pub const DIBUTTON_HUNTING_FIRE: DWORD = 0x0D000401;
pub const DIBUTTON_HUNTING_AIM: DWORD = 0x0D000402;
pub const DIBUTTON_HUNTING_WEAPON: DWORD = 0x0D000403;
pub const DIBUTTON_HUNTING_BINOCULAR: DWORD = 0x0D000404;
pub const DIBUTTON_HUNTING_CALL: DWORD = 0x0D000405;
pub const DIBUTTON_HUNTING_MAP: DWORD = 0x0D000406;
pub const DIBUTTON_HUNTING_SPECIAL: DWORD = 0x0D000407;
pub const DIBUTTON_HUNTING_MENU: DWORD = 0x0D0004FD;
pub const DIHATSWITCH_HUNTING_GLANCE: DWORD = 0x0D004601;
pub const DIBUTTON_HUNTING_DISPLAY: DWORD = 0x0D004408;
pub const DIAXIS_HUNTING_ROTATE: DWORD = 0x0D024203;
pub const DIBUTTON_HUNTING_CROUCH: DWORD = 0x0D004409;
pub const DIBUTTON_HUNTING_JUMP: DWORD = 0x0D00440A;
pub const DIBUTTON_HUNTING_FIRESECONDARY: DWORD = 0x0D00440B;
pub const DIBUTTON_HUNTING_LEFT_LINK: DWORD = 0x0D00C4E4;
pub const DIBUTTON_HUNTING_RIGHT_LINK: DWORD = 0x0D00C4EC;
pub const DIBUTTON_HUNTING_FORWARD_LINK: DWORD = 0x0D0144E0;
pub const DIBUTTON_HUNTING_BACK_LINK: DWORD = 0x0D0144E8;
pub const DIBUTTON_HUNTING_ROTATE_LEFT_LINK: DWORD = 0x0D0244E4;
pub const DIBUTTON_HUNTING_ROTATE_RIGHT_LINK: DWORD = 0x0D0244EC;
pub const DIBUTTON_HUNTING_DEVICE: DWORD = 0x0D0044FE;
pub const DIBUTTON_HUNTING_PAUSE: DWORD = 0x0D0044FC;
pub const DIVIRTUAL_SPORTS_FISHING: DWORD = 0x0E000000;
pub const DIAXIS_FISHING_LATERAL: DWORD = 0x0E008201;
pub const DIAXIS_FISHING_MOVE: DWORD = 0x0E010202;
pub const DIBUTTON_FISHING_CAST: DWORD = 0x0E000401;
pub const DIBUTTON_FISHING_TYPE: DWORD = 0x0E000402;
pub const DIBUTTON_FISHING_BINOCULAR: DWORD = 0x0E000403;
pub const DIBUTTON_FISHING_BAIT: DWORD = 0x0E000404;
pub const DIBUTTON_FISHING_MAP: DWORD = 0x0E000405;
pub const DIBUTTON_FISHING_MENU: DWORD = 0x0E0004FD;
pub const DIHATSWITCH_FISHING_GLANCE: DWORD = 0x0E004601;
pub const DIBUTTON_FISHING_DISPLAY: DWORD = 0x0E004406;
pub const DIAXIS_FISHING_ROTATE: DWORD = 0x0E024203;
pub const DIBUTTON_FISHING_CROUCH: DWORD = 0x0E004407;
pub const DIBUTTON_FISHING_JUMP: DWORD = 0x0E004408;
pub const DIBUTTON_FISHING_LEFT_LINK: DWORD = 0x0E00C4E4;
pub const DIBUTTON_FISHING_RIGHT_LINK: DWORD = 0x0E00C4EC;
pub const DIBUTTON_FISHING_FORWARD_LINK: DWORD = 0x0E0144E0;
pub const DIBUTTON_FISHING_BACK_LINK: DWORD = 0x0E0144E8;
pub const DIBUTTON_FISHING_ROTATE_LEFT_LINK: DWORD = 0x0E0244E4;
pub const DIBUTTON_FISHING_ROTATE_RIGHT_LINK: DWORD = 0x0E0244EC;
pub const DIBUTTON_FISHING_DEVICE: DWORD = 0x0E0044FE;
pub const DIBUTTON_FISHING_PAUSE: DWORD = 0x0E0044FC;
pub const DIVIRTUAL_SPORTS_BASEBALL_BAT: DWORD = 0x0F000000;
pub const DIAXIS_BASEBALLB_LATERAL: DWORD = 0x0F008201;
pub const DIAXIS_BASEBALLB_MOVE: DWORD = 0x0F010202;
pub const DIBUTTON_BASEBALLB_SELECT: DWORD = 0x0F000401;
pub const DIBUTTON_BASEBALLB_NORMAL: DWORD = 0x0F000402;
pub const DIBUTTON_BASEBALLB_POWER: DWORD = 0x0F000403;
pub const DIBUTTON_BASEBALLB_BUNT: DWORD = 0x0F000404;
pub const DIBUTTON_BASEBALLB_STEAL: DWORD = 0x0F000405;
pub const DIBUTTON_BASEBALLB_BURST: DWORD = 0x0F000406;
pub const DIBUTTON_BASEBALLB_SLIDE: DWORD = 0x0F000407;
pub const DIBUTTON_BASEBALLB_CONTACT: DWORD = 0x0F000408;
pub const DIBUTTON_BASEBALLB_MENU: DWORD = 0x0F0004FD;
pub const DIBUTTON_BASEBALLB_NOSTEAL: DWORD = 0x0F004409;
pub const DIBUTTON_BASEBALLB_BOX: DWORD = 0x0F00440A;
pub const DIBUTTON_BASEBALLB_LEFT_LINK: DWORD = 0x0F00C4E4;
pub const DIBUTTON_BASEBALLB_RIGHT_LINK: DWORD = 0x0F00C4EC;
pub const DIBUTTON_BASEBALLB_FORWARD_LINK: DWORD = 0x0F0144E0;
pub const DIBUTTON_BASEBALLB_BACK_LINK: DWORD = 0x0F0144E8;
pub const DIBUTTON_BASEBALLB_DEVICE: DWORD = 0x0F0044FE;
pub const DIBUTTON_BASEBALLB_PAUSE: DWORD = 0x0F0044FC;
pub const DIVIRTUAL_SPORTS_BASEBALL_PITCH: DWORD = 0x10000000;
pub const DIAXIS_BASEBALLP_LATERAL: DWORD = 0x10008201;
pub const DIAXIS_BASEBALLP_MOVE: DWORD = 0x10010202;
pub const DIBUTTON_BASEBALLP_SELECT: DWORD = 0x10000401;
pub const DIBUTTON_BASEBALLP_PITCH: DWORD = 0x10000402;
pub const DIBUTTON_BASEBALLP_BASE: DWORD = 0x10000403;
pub const DIBUTTON_BASEBALLP_THROW: DWORD = 0x10000404;
pub const DIBUTTON_BASEBALLP_FAKE: DWORD = 0x10000405;
pub const DIBUTTON_BASEBALLP_MENU: DWORD = 0x100004FD;
pub const DIBUTTON_BASEBALLP_WALK: DWORD = 0x10004406;
pub const DIBUTTON_BASEBALLP_LOOK: DWORD = 0x10004407;
pub const DIBUTTON_BASEBALLP_LEFT_LINK: DWORD = 0x1000C4E4;
pub const DIBUTTON_BASEBALLP_RIGHT_LINK: DWORD = 0x1000C4EC;
pub const DIBUTTON_BASEBALLP_FORWARD_LINK: DWORD = 0x100144E0;
pub const DIBUTTON_BASEBALLP_BACK_LINK: DWORD = 0x100144E8;
pub const DIBUTTON_BASEBALLP_DEVICE: DWORD = 0x100044FE;
pub const DIBUTTON_BASEBALLP_PAUSE: DWORD = 0x100044FC;
pub const DIVIRTUAL_SPORTS_BASEBALL_FIELD: DWORD = 0x11000000;
pub const DIAXIS_BASEBALLF_LATERAL: DWORD = 0x11008201;
pub const DIAXIS_BASEBALLF_MOVE: DWORD = 0x11010202;
pub const DIBUTTON_BASEBALLF_NEAREST: DWORD = 0x11000401;
pub const DIBUTTON_BASEBALLF_THROW1: DWORD = 0x11000402;
pub const DIBUTTON_BASEBALLF_THROW2: DWORD = 0x11000403;
pub const DIBUTTON_BASEBALLF_BURST: DWORD = 0x11000404;
pub const DIBUTTON_BASEBALLF_JUMP: DWORD = 0x11000405;
pub const DIBUTTON_BASEBALLF_DIVE: DWORD = 0x11000406;
pub const DIBUTTON_BASEBALLF_MENU: DWORD = 0x110004FD;
pub const DIBUTTON_BASEBALLF_SHIFTIN: DWORD = 0x11004407;
pub const DIBUTTON_BASEBALLF_SHIFTOUT: DWORD = 0x11004408;
pub const DIBUTTON_BASEBALLF_AIM_LEFT_LINK: DWORD = 0x1100C4E4;
pub const DIBUTTON_BASEBALLF_AIM_RIGHT_LINK: DWORD = 0x1100C4EC;
pub const DIBUTTON_BASEBALLF_FORWARD_LINK: DWORD = 0x110144E0;
pub const DIBUTTON_BASEBALLF_BACK_LINK: DWORD = 0x110144E8;
pub const DIBUTTON_BASEBALLF_DEVICE: DWORD = 0x110044FE;
pub const DIBUTTON_BASEBALLF_PAUSE: DWORD = 0x110044FC;
pub const DIVIRTUAL_SPORTS_BASKETBALL_OFFENSE: DWORD = 0x12000000;
pub const DIAXIS_BBALLO_LATERAL: DWORD = 0x12008201;
pub const DIAXIS_BBALLO_MOVE: DWORD = 0x12010202;
pub const DIBUTTON_BBALLO_SHOOT: DWORD = 0x12000401;
pub const DIBUTTON_BBALLO_DUNK: DWORD = 0x12000402;
pub const DIBUTTON_BBALLO_PASS: DWORD = 0x12000403;
pub const DIBUTTON_BBALLO_FAKE: DWORD = 0x12000404;
pub const DIBUTTON_BBALLO_SPECIAL: DWORD = 0x12000405;
pub const DIBUTTON_BBALLO_PLAYER: DWORD = 0x12000406;
pub const DIBUTTON_BBALLO_BURST: DWORD = 0x12000407;
pub const DIBUTTON_BBALLO_CALL: DWORD = 0x12000408;
pub const DIBUTTON_BBALLO_MENU: DWORD = 0x120004FD;
pub const DIHATSWITCH_BBALLO_GLANCE: DWORD = 0x12004601;
pub const DIBUTTON_BBALLO_SCREEN: DWORD = 0x12004409;
pub const DIBUTTON_BBALLO_PLAY: DWORD = 0x1200440A;
pub const DIBUTTON_BBALLO_JAB: DWORD = 0x1200440B;
pub const DIBUTTON_BBALLO_POST: DWORD = 0x1200440C;
pub const DIBUTTON_BBALLO_TIMEOUT: DWORD = 0x1200440D;
pub const DIBUTTON_BBALLO_SUBSTITUTE: DWORD = 0x1200440E;
pub const DIBUTTON_BBALLO_LEFT_LINK: DWORD = 0x1200C4E4;
pub const DIBUTTON_BBALLO_RIGHT_LINK: DWORD = 0x1200C4EC;
pub const DIBUTTON_BBALLO_FORWARD_LINK: DWORD = 0x120144E0;
pub const DIBUTTON_BBALLO_BACK_LINK: DWORD = 0x120144E8;
pub const DIBUTTON_BBALLO_DEVICE: DWORD = 0x120044FE;
pub const DIBUTTON_BBALLO_PAUSE: DWORD = 0x120044FC;
pub const DIVIRTUAL_SPORTS_BASKETBALL_DEFENSE: DWORD = 0x13000000;
pub const DIAXIS_BBALLD_LATERAL: DWORD = 0x13008201;
pub const DIAXIS_BBALLD_MOVE: DWORD = 0x13010202;
pub const DIBUTTON_BBALLD_JUMP: DWORD = 0x13000401;
pub const DIBUTTON_BBALLD_STEAL: DWORD = 0x13000402;
pub const DIBUTTON_BBALLD_FAKE: DWORD = 0x13000403;
pub const DIBUTTON_BBALLD_SPECIAL: DWORD = 0x13000404;
pub const DIBUTTON_BBALLD_PLAYER: DWORD = 0x13000405;
pub const DIBUTTON_BBALLD_BURST: DWORD = 0x13000406;
pub const DIBUTTON_BBALLD_PLAY: DWORD = 0x13000407;
pub const DIBUTTON_BBALLD_MENU: DWORD = 0x130004FD;
pub const DIHATSWITCH_BBALLD_GLANCE: DWORD = 0x13004601;
pub const DIBUTTON_BBALLD_TIMEOUT: DWORD = 0x13004408;
pub const DIBUTTON_BBALLD_SUBSTITUTE: DWORD = 0x13004409;
pub const DIBUTTON_BBALLD_LEFT_LINK: DWORD = 0x1300C4E4;
pub const DIBUTTON_BBALLD_RIGHT_LINK: DWORD = 0x1300C4EC;
pub const DIBUTTON_BBALLD_FORWARD_LINK: DWORD = 0x130144E0;
pub const DIBUTTON_BBALLD_BACK_LINK: DWORD = 0x130144E8;
pub const DIBUTTON_BBALLD_DEVICE: DWORD = 0x130044FE;
pub const DIBUTTON_BBALLD_PAUSE: DWORD = 0x130044FC;
pub const DIVIRTUAL_SPORTS_FOOTBALL_FIELD: DWORD = 0x14000000;
pub const DIBUTTON_FOOTBALLP_PLAY: DWORD = 0x14000401;
pub const DIBUTTON_FOOTBALLP_SELECT: DWORD = 0x14000402;
pub const DIBUTTON_FOOTBALLP_HELP: DWORD = 0x14000403;
pub const DIBUTTON_FOOTBALLP_MENU: DWORD = 0x140004FD;
pub const DIBUTTON_FOOTBALLP_DEVICE: DWORD = 0x140044FE;
pub const DIBUTTON_FOOTBALLP_PAUSE: DWORD = 0x140044FC;
pub const DIVIRTUAL_SPORTS_FOOTBALL_QBCK: DWORD = 0x15000000;
pub const DIAXIS_FOOTBALLQ_LATERAL: DWORD = 0x15008201;
pub const DIAXIS_FOOTBALLQ_MOVE: DWORD = 0x15010202;
pub const DIBUTTON_FOOTBALLQ_SELECT: DWORD = 0x15000401;
pub const DIBUTTON_FOOTBALLQ_SNAP: DWORD = 0x15000402;
pub const DIBUTTON_FOOTBALLQ_JUMP: DWORD = 0x15000403;
pub const DIBUTTON_FOOTBALLQ_SLIDE: DWORD = 0x15000404;
pub const DIBUTTON_FOOTBALLQ_PASS: DWORD = 0x15000405;
pub const DIBUTTON_FOOTBALLQ_FAKE: DWORD = 0x15000406;
pub const DIBUTTON_FOOTBALLQ_MENU: DWORD = 0x150004FD;
pub const DIBUTTON_FOOTBALLQ_FAKESNAP: DWORD = 0x15004407;
pub const DIBUTTON_FOOTBALLQ_MOTION: DWORD = 0x15004408;
pub const DIBUTTON_FOOTBALLQ_AUDIBLE: DWORD = 0x15004409;
pub const DIBUTTON_FOOTBALLQ_LEFT_LINK: DWORD = 0x1500C4E4;
pub const DIBUTTON_FOOTBALLQ_RIGHT_LINK: DWORD = 0x1500C4EC;
pub const DIBUTTON_FOOTBALLQ_FORWARD_LINK: DWORD = 0x150144E0;
pub const DIBUTTON_FOOTBALLQ_BACK_LINK: DWORD = 0x150144E8;
pub const DIBUTTON_FOOTBALLQ_DEVICE: DWORD = 0x150044FE;
pub const DIBUTTON_FOOTBALLQ_PAUSE: DWORD = 0x150044FC;
pub const DIVIRTUAL_SPORTS_FOOTBALL_OFFENSE: DWORD = 0x16000000;
pub const DIAXIS_FOOTBALLO_LATERAL: DWORD = 0x16008201;
pub const DIAXIS_FOOTBALLO_MOVE: DWORD = 0x16010202;
pub const DIBUTTON_FOOTBALLO_JUMP: DWORD = 0x16000401;
pub const DIBUTTON_FOOTBALLO_LEFTARM: DWORD = 0x16000402;
pub const DIBUTTON_FOOTBALLO_RIGHTARM: DWORD = 0x16000403;
pub const DIBUTTON_FOOTBALLO_THROW: DWORD = 0x16000404;
pub const DIBUTTON_FOOTBALLO_SPIN: DWORD = 0x16000405;
pub const DIBUTTON_FOOTBALLO_MENU: DWORD = 0x160004FD;
pub const DIBUTTON_FOOTBALLO_JUKE: DWORD = 0x16004406;
pub const DIBUTTON_FOOTBALLO_SHOULDER: DWORD = 0x16004407;
pub const DIBUTTON_FOOTBALLO_TURBO: DWORD = 0x16004408;
pub const DIBUTTON_FOOTBALLO_DIVE: DWORD = 0x16004409;
pub const DIBUTTON_FOOTBALLO_ZOOM: DWORD = 0x1600440A;
pub const DIBUTTON_FOOTBALLO_SUBSTITUTE: DWORD = 0x1600440B;
pub const DIBUTTON_FOOTBALLO_LEFT_LINK: DWORD = 0x1600C4E4;
pub const DIBUTTON_FOOTBALLO_RIGHT_LINK: DWORD = 0x1600C4EC;
pub const DIBUTTON_FOOTBALLO_FORWARD_LINK: DWORD = 0x160144E0;
pub const DIBUTTON_FOOTBALLO_BACK_LINK: DWORD = 0x160144E8;
pub const DIBUTTON_FOOTBALLO_DEVICE: DWORD = 0x160044FE;
pub const DIBUTTON_FOOTBALLO_PAUSE: DWORD = 0x160044FC;
pub const DIVIRTUAL_SPORTS_FOOTBALL_DEFENSE: DWORD = 0x17000000;
pub const DIAXIS_FOOTBALLD_LATERAL: DWORD = 0x17008201;
pub const DIAXIS_FOOTBALLD_MOVE: DWORD = 0x17010202;
pub const DIBUTTON_FOOTBALLD_PLAY: DWORD = 0x17000401;
pub const DIBUTTON_FOOTBALLD_SELECT: DWORD = 0x17000402;
pub const DIBUTTON_FOOTBALLD_JUMP: DWORD = 0x17000403;
pub const DIBUTTON_FOOTBALLD_TACKLE: DWORD = 0x17000404;
pub const DIBUTTON_FOOTBALLD_FAKE: DWORD = 0x17000405;
pub const DIBUTTON_FOOTBALLD_SUPERTACKLE: DWORD = 0x17000406;
pub const DIBUTTON_FOOTBALLD_MENU: DWORD = 0x170004FD;
pub const DIBUTTON_FOOTBALLD_SPIN: DWORD = 0x17004407;
pub const DIBUTTON_FOOTBALLD_SWIM: DWORD = 0x17004408;
pub const DIBUTTON_FOOTBALLD_BULLRUSH: DWORD = 0x17004409;
pub const DIBUTTON_FOOTBALLD_RIP: DWORD = 0x1700440A;
pub const DIBUTTON_FOOTBALLD_AUDIBLE: DWORD = 0x1700440B;
pub const DIBUTTON_FOOTBALLD_ZOOM: DWORD = 0x1700440C;
pub const DIBUTTON_FOOTBALLD_SUBSTITUTE: DWORD = 0x1700440D;
pub const DIBUTTON_FOOTBALLD_LEFT_LINK: DWORD = 0x1700C4E4;
pub const DIBUTTON_FOOTBALLD_RIGHT_LINK: DWORD = 0x1700C4EC;
pub const DIBUTTON_FOOTBALLD_FORWARD_LINK: DWORD = 0x170144E0;
pub const DIBUTTON_FOOTBALLD_BACK_LINK: DWORD = 0x170144E8;
pub const DIBUTTON_FOOTBALLD_DEVICE: DWORD = 0x170044FE;
pub const DIBUTTON_FOOTBALLD_PAUSE: DWORD = 0x170044FC;
pub const DIVIRTUAL_SPORTS_GOLF: DWORD = 0x18000000;
pub const DIAXIS_GOLF_LATERAL: DWORD = 0x18008201;
pub const DIAXIS_GOLF_MOVE: DWORD = 0x18010202;
pub const DIBUTTON_GOLF_SWING: DWORD = 0x18000401;
pub const DIBUTTON_GOLF_SELECT: DWORD = 0x18000402;
pub const DIBUTTON_GOLF_UP: DWORD = 0x18000403;
pub const DIBUTTON_GOLF_DOWN: DWORD = 0x18000404;
pub const DIBUTTON_GOLF_TERRAIN: DWORD = 0x18000405;
pub const DIBUTTON_GOLF_FLYBY: DWORD = 0x18000406;
pub const DIBUTTON_GOLF_MENU: DWORD = 0x180004FD;
pub const DIHATSWITCH_GOLF_SCROLL: DWORD = 0x18004601;
pub const DIBUTTON_GOLF_ZOOM: DWORD = 0x18004407;
pub const DIBUTTON_GOLF_TIMEOUT: DWORD = 0x18004408;
pub const DIBUTTON_GOLF_SUBSTITUTE: DWORD = 0x18004409;
pub const DIBUTTON_GOLF_LEFT_LINK: DWORD = 0x1800C4E4;
pub const DIBUTTON_GOLF_RIGHT_LINK: DWORD = 0x1800C4EC;
pub const DIBUTTON_GOLF_FORWARD_LINK: DWORD = 0x180144E0;
pub const DIBUTTON_GOLF_BACK_LINK: DWORD = 0x180144E8;
pub const DIBUTTON_GOLF_DEVICE: DWORD = 0x180044FE;
pub const DIBUTTON_GOLF_PAUSE: DWORD = 0x180044FC;
pub const DIVIRTUAL_SPORTS_HOCKEY_OFFENSE: DWORD = 0x19000000;
pub const DIAXIS_HOCKEYO_LATERAL: DWORD = 0x19008201;
pub const DIAXIS_HOCKEYO_MOVE: DWORD = 0x19010202;
pub const DIBUTTON_HOCKEYO_SHOOT: DWORD = 0x19000401;
pub const DIBUTTON_HOCKEYO_PASS: DWORD = 0x19000402;
pub const DIBUTTON_HOCKEYO_BURST: DWORD = 0x19000403;
pub const DIBUTTON_HOCKEYO_SPECIAL: DWORD = 0x19000404;
pub const DIBUTTON_HOCKEYO_FAKE: DWORD = 0x19000405;
pub const DIBUTTON_HOCKEYO_MENU: DWORD = 0x190004FD;
pub const DIHATSWITCH_HOCKEYO_SCROLL: DWORD = 0x19004601;
pub const DIBUTTON_HOCKEYO_ZOOM: DWORD = 0x19004406;
pub const DIBUTTON_HOCKEYO_STRATEGY: DWORD = 0x19004407;
pub const DIBUTTON_HOCKEYO_TIMEOUT: DWORD = 0x19004408;
pub const DIBUTTON_HOCKEYO_SUBSTITUTE: DWORD = 0x19004409;
pub const DIBUTTON_HOCKEYO_LEFT_LINK: DWORD = 0x1900C4E4;
pub const DIBUTTON_HOCKEYO_RIGHT_LINK: DWORD = 0x1900C4EC;
pub const DIBUTTON_HOCKEYO_FORWARD_LINK: DWORD = 0x190144E0;
pub const DIBUTTON_HOCKEYO_BACK_LINK: DWORD = 0x190144E8;
pub const DIBUTTON_HOCKEYO_DEVICE: DWORD = 0x190044FE;
pub const DIBUTTON_HOCKEYO_PAUSE: DWORD = 0x190044FC;
pub const DIVIRTUAL_SPORTS_HOCKEY_DEFENSE: DWORD = 0x1A000000;
pub const DIAXIS_HOCKEYD_LATERAL: DWORD = 0x1A008201;
pub const DIAXIS_HOCKEYD_MOVE: DWORD = 0x1A010202;
pub const DIBUTTON_HOCKEYD_PLAYER: DWORD = 0x1A000401;
pub const DIBUTTON_HOCKEYD_STEAL: DWORD = 0x1A000402;
pub const DIBUTTON_HOCKEYD_BURST: DWORD = 0x1A000403;
pub const DIBUTTON_HOCKEYD_BLOCK: DWORD = 0x1A000404;
pub const DIBUTTON_HOCKEYD_FAKE: DWORD = 0x1A000405;
pub const DIBUTTON_HOCKEYD_MENU: DWORD = 0x1A0004FD;
pub const DIHATSWITCH_HOCKEYD_SCROLL: DWORD = 0x1A004601;
pub const DIBUTTON_HOCKEYD_ZOOM: DWORD = 0x1A004406;
pub const DIBUTTON_HOCKEYD_STRATEGY: DWORD = 0x1A004407;
pub const DIBUTTON_HOCKEYD_TIMEOUT: DWORD = 0x1A004408;
pub const DIBUTTON_HOCKEYD_SUBSTITUTE: DWORD = 0x1A004409;
pub const DIBUTTON_HOCKEYD_LEFT_LINK: DWORD = 0x1A00C4E4;
pub const DIBUTTON_HOCKEYD_RIGHT_LINK: DWORD = 0x1A00C4EC;
pub const DIBUTTON_HOCKEYD_FORWARD_LINK: DWORD = 0x1A0144E0;
pub const DIBUTTON_HOCKEYD_BACK_LINK: DWORD = 0x1A0144E8;
pub const DIBUTTON_HOCKEYD_DEVICE: DWORD = 0x1A0044FE;
pub const DIBUTTON_HOCKEYD_PAUSE: DWORD = 0x1A0044FC;
pub const DIVIRTUAL_SPORTS_HOCKEY_GOALIE: DWORD = 0x1B000000;
pub const DIAXIS_HOCKEYG_LATERAL: DWORD = 0x1B008201;
pub const DIAXIS_HOCKEYG_MOVE: DWORD = 0x1B010202;
pub const DIBUTTON_HOCKEYG_PASS: DWORD = 0x1B000401;
pub const DIBUTTON_HOCKEYG_POKE: DWORD = 0x1B000402;
pub const DIBUTTON_HOCKEYG_STEAL: DWORD = 0x1B000403;
pub const DIBUTTON_HOCKEYG_BLOCK: DWORD = 0x1B000404;
pub const DIBUTTON_HOCKEYG_MENU: DWORD = 0x1B0004FD;
pub const DIHATSWITCH_HOCKEYG_SCROLL: DWORD = 0x1B004601;
pub const DIBUTTON_HOCKEYG_ZOOM: DWORD = 0x1B004405;
pub const DIBUTTON_HOCKEYG_STRATEGY: DWORD = 0x1B004406;
pub const DIBUTTON_HOCKEYG_TIMEOUT: DWORD = 0x1B004407;
pub const DIBUTTON_HOCKEYG_SUBSTITUTE: DWORD = 0x1B004408;
pub const DIBUTTON_HOCKEYG_LEFT_LINK: DWORD = 0x1B00C4E4;
pub const DIBUTTON_HOCKEYG_RIGHT_LINK: DWORD = 0x1B00C4EC;
pub const DIBUTTON_HOCKEYG_FORWARD_LINK: DWORD = 0x1B0144E0;
pub const DIBUTTON_HOCKEYG_BACK_LINK: DWORD = 0x1B0144E8;
pub const DIBUTTON_HOCKEYG_DEVICE: DWORD = 0x1B0044FE;
pub const DIBUTTON_HOCKEYG_PAUSE: DWORD = 0x1B0044FC;
pub const DIVIRTUAL_SPORTS_BIKING_MOUNTAIN: DWORD = 0x1C000000;
pub const DIAXIS_BIKINGM_TURN: DWORD = 0x1C008201;
pub const DIAXIS_BIKINGM_PEDAL: DWORD = 0x1C010202;
pub const DIBUTTON_BIKINGM_JUMP: DWORD = 0x1C000401;
pub const DIBUTTON_BIKINGM_CAMERA: DWORD = 0x1C000402;
pub const DIBUTTON_BIKINGM_SPECIAL1: DWORD = 0x1C000403;
pub const DIBUTTON_BIKINGM_SELECT: DWORD = 0x1C000404;
pub const DIBUTTON_BIKINGM_SPECIAL2: DWORD = 0x1C000405;
pub const DIBUTTON_BIKINGM_MENU: DWORD = 0x1C0004FD;
pub const DIHATSWITCH_BIKINGM_SCROLL: DWORD = 0x1C004601;
pub const DIBUTTON_BIKINGM_ZOOM: DWORD = 0x1C004406;
pub const DIAXIS_BIKINGM_BRAKE: DWORD = 0x1C044203;
pub const DIBUTTON_BIKINGM_LEFT_LINK: DWORD = 0x1C00C4E4;
pub const DIBUTTON_BIKINGM_RIGHT_LINK: DWORD = 0x1C00C4EC;
pub const DIBUTTON_BIKINGM_FASTER_LINK: DWORD = 0x1C0144E0;
pub const DIBUTTON_BIKINGM_SLOWER_LINK: DWORD = 0x1C0144E8;
pub const DIBUTTON_BIKINGM_BRAKE_BUTTON_LINK: DWORD = 0x1C0444E8;
pub const DIBUTTON_BIKINGM_DEVICE: DWORD = 0x1C0044FE;
pub const DIBUTTON_BIKINGM_PAUSE: DWORD = 0x1C0044FC;
pub const DIVIRTUAL_SPORTS_SKIING: DWORD = 0x1D000000;
pub const DIAXIS_SKIING_TURN: DWORD = 0x1D008201;
pub const DIAXIS_SKIING_SPEED: DWORD = 0x1D010202;
pub const DIBUTTON_SKIING_JUMP: DWORD = 0x1D000401;
pub const DIBUTTON_SKIING_CROUCH: DWORD = 0x1D000402;
pub const DIBUTTON_SKIING_CAMERA: DWORD = 0x1D000403;
pub const DIBUTTON_SKIING_SPECIAL1: DWORD = 0x1D000404;
pub const DIBUTTON_SKIING_SELECT: DWORD = 0x1D000405;
pub const DIBUTTON_SKIING_SPECIAL2: DWORD = 0x1D000406;
pub const DIBUTTON_SKIING_MENU: DWORD = 0x1D0004FD;
pub const DIHATSWITCH_SKIING_GLANCE: DWORD = 0x1D004601;
pub const DIBUTTON_SKIING_ZOOM: DWORD = 0x1D004407;
pub const DIBUTTON_SKIING_LEFT_LINK: DWORD = 0x1D00C4E4;
pub const DIBUTTON_SKIING_RIGHT_LINK: DWORD = 0x1D00C4EC;
pub const DIBUTTON_SKIING_FASTER_LINK: DWORD = 0x1D0144E0;
pub const DIBUTTON_SKIING_SLOWER_LINK: DWORD = 0x1D0144E8;
pub const DIBUTTON_SKIING_DEVICE: DWORD = 0x1D0044FE;
pub const DIBUTTON_SKIING_PAUSE: DWORD = 0x1D0044FC;
pub const DIVIRTUAL_SPORTS_SOCCER_OFFENSE: DWORD = 0x1E000000;
pub const DIAXIS_SOCCERO_LATERAL: DWORD = 0x1E008201;
pub const DIAXIS_SOCCERO_MOVE: DWORD = 0x1E010202;
pub const DIAXIS_SOCCERO_BEND: DWORD = 0x1E018203;
pub const DIBUTTON_SOCCERO_SHOOT: DWORD = 0x1E000401;
pub const DIBUTTON_SOCCERO_PASS: DWORD = 0x1E000402;
pub const DIBUTTON_SOCCERO_FAKE: DWORD = 0x1E000403;
pub const DIBUTTON_SOCCERO_PLAYER: DWORD = 0x1E000404;
pub const DIBUTTON_SOCCERO_SPECIAL1: DWORD = 0x1E000405;
pub const DIBUTTON_SOCCERO_SELECT: DWORD = 0x1E000406;
pub const DIBUTTON_SOCCERO_MENU: DWORD = 0x1E0004FD;
pub const DIHATSWITCH_SOCCERO_GLANCE: DWORD = 0x1E004601;
pub const DIBUTTON_SOCCERO_SUBSTITUTE: DWORD = 0x1E004407;
pub const DIBUTTON_SOCCERO_SHOOTLOW: DWORD = 0x1E004408;
pub const DIBUTTON_SOCCERO_SHOOTHIGH: DWORD = 0x1E004409;
pub const DIBUTTON_SOCCERO_PASSTHRU: DWORD = 0x1E00440A;
pub const DIBUTTON_SOCCERO_SPRINT: DWORD = 0x1E00440B;
pub const DIBUTTON_SOCCERO_CONTROL: DWORD = 0x1E00440C;
pub const DIBUTTON_SOCCERO_HEAD: DWORD = 0x1E00440D;
pub const DIBUTTON_SOCCERO_LEFT_LINK: DWORD = 0x1E00C4E4;
pub const DIBUTTON_SOCCERO_RIGHT_LINK: DWORD = 0x1E00C4EC;
pub const DIBUTTON_SOCCERO_FORWARD_LINK: DWORD = 0x1E0144E0;
pub const DIBUTTON_SOCCERO_BACK_LINK: DWORD = 0x1E0144E8;
pub const DIBUTTON_SOCCERO_DEVICE: DWORD = 0x1E0044FE;
pub const DIBUTTON_SOCCERO_PAUSE: DWORD = 0x1E0044FC;
pub const DIVIRTUAL_SPORTS_SOCCER_DEFENSE: DWORD = 0x1F000000;
pub const DIAXIS_SOCCERD_LATERAL: DWORD = 0x1F008201;
pub const DIAXIS_SOCCERD_MOVE: DWORD = 0x1F010202;
pub const DIBUTTON_SOCCERD_BLOCK: DWORD = 0x1F000401;
pub const DIBUTTON_SOCCERD_STEAL: DWORD = 0x1F000402;
pub const DIBUTTON_SOCCERD_FAKE: DWORD = 0x1F000403;
pub const DIBUTTON_SOCCERD_PLAYER: DWORD = 0x1F000404;
pub const DIBUTTON_SOCCERD_SPECIAL: DWORD = 0x1F000405;
pub const DIBUTTON_SOCCERD_SELECT: DWORD = 0x1F000406;
pub const DIBUTTON_SOCCERD_SLIDE: DWORD = 0x1F000407;
pub const DIBUTTON_SOCCERD_MENU: DWORD = 0x1F0004FD;
pub const DIHATSWITCH_SOCCERD_GLANCE: DWORD = 0x1F004601;
pub const DIBUTTON_SOCCERD_FOUL: DWORD = 0x1F004408;
pub const DIBUTTON_SOCCERD_HEAD: DWORD = 0x1F004409;
pub const DIBUTTON_SOCCERD_CLEAR: DWORD = 0x1F00440A;
pub const DIBUTTON_SOCCERD_GOALIECHARGE: DWORD = 0x1F00440B;
pub const DIBUTTON_SOCCERD_SUBSTITUTE: DWORD = 0x1F00440C;
pub const DIBUTTON_SOCCERD_LEFT_LINK: DWORD = 0x1F00C4E4;
pub const DIBUTTON_SOCCERD_RIGHT_LINK: DWORD = 0x1F00C4EC;
pub const DIBUTTON_SOCCERD_FORWARD_LINK: DWORD = 0x1F0144E0;
pub const DIBUTTON_SOCCERD_BACK_LINK: DWORD = 0x1F0144E8;
pub const DIBUTTON_SOCCERD_DEVICE: DWORD = 0x1F0044FE;
pub const DIBUTTON_SOCCERD_PAUSE: DWORD = 0x1F0044FC;
pub const DIVIRTUAL_SPORTS_RACQUET: DWORD = 0x20000000;
pub const DIAXIS_RACQUET_LATERAL: DWORD = 0x20008201;
pub const DIAXIS_RACQUET_MOVE: DWORD = 0x20010202;
pub const DIBUTTON_RACQUET_SWING: DWORD = 0x20000401;
pub const DIBUTTON_RACQUET_BACKSWING: DWORD = 0x20000402;
pub const DIBUTTON_RACQUET_SMASH: DWORD = 0x20000403;
pub const DIBUTTON_RACQUET_SPECIAL: DWORD = 0x20000404;
pub const DIBUTTON_RACQUET_SELECT: DWORD = 0x20000405;
pub const DIBUTTON_RACQUET_MENU: DWORD = 0x200004FD;
pub const DIHATSWITCH_RACQUET_GLANCE: DWORD = 0x20004601;
pub const DIBUTTON_RACQUET_TIMEOUT: DWORD = 0x20004406;
pub const DIBUTTON_RACQUET_SUBSTITUTE: DWORD = 0x20004407;
pub const DIBUTTON_RACQUET_LEFT_LINK: DWORD = 0x2000C4E4;
pub const DIBUTTON_RACQUET_RIGHT_LINK: DWORD = 0x2000C4EC;
pub const DIBUTTON_RACQUET_FORWARD_LINK: DWORD = 0x200144E0;
pub const DIBUTTON_RACQUET_BACK_LINK: DWORD = 0x200144E8;
pub const DIBUTTON_RACQUET_DEVICE: DWORD = 0x200044FE;
pub const DIBUTTON_RACQUET_PAUSE: DWORD = 0x200044FC;
pub const DIVIRTUAL_ARCADE_SIDE2SIDE: DWORD = 0x21000000;
pub const DIAXIS_ARCADES_LATERAL: DWORD = 0x21008201;
pub const DIAXIS_ARCADES_MOVE: DWORD = 0x21010202;
pub const DIBUTTON_ARCADES_THROW: DWORD = 0x21000401;
pub const DIBUTTON_ARCADES_CARRY: DWORD = 0x21000402;
pub const DIBUTTON_ARCADES_ATTACK: DWORD = 0x21000403;
pub const DIBUTTON_ARCADES_SPECIAL: DWORD = 0x21000404;
pub const DIBUTTON_ARCADES_SELECT: DWORD = 0x21000405;
pub const DIBUTTON_ARCADES_MENU: DWORD = 0x210004FD;
pub const DIHATSWITCH_ARCADES_VIEW: DWORD = 0x21004601;
pub const DIBUTTON_ARCADES_LEFT_LINK: DWORD = 0x2100C4E4;
pub const DIBUTTON_ARCADES_RIGHT_LINK: DWORD = 0x2100C4EC;
pub const DIBUTTON_ARCADES_FORWARD_LINK: DWORD = 0x210144E0;
pub const DIBUTTON_ARCADES_BACK_LINK: DWORD = 0x210144E8;
pub const DIBUTTON_ARCADES_VIEW_UP_LINK: DWORD = 0x2107C4E0;
pub const DIBUTTON_ARCADES_VIEW_DOWN_LINK: DWORD = 0x2107C4E8;
pub const DIBUTTON_ARCADES_VIEW_LEFT_LINK: DWORD = 0x2107C4E4;
pub const DIBUTTON_ARCADES_VIEW_RIGHT_LINK: DWORD = 0x2107C4EC;
pub const DIBUTTON_ARCADES_DEVICE: DWORD = 0x210044FE;
pub const DIBUTTON_ARCADES_PAUSE: DWORD = 0x210044FC;
pub const DIVIRTUAL_ARCADE_PLATFORM: DWORD = 0x22000000;
pub const DIAXIS_ARCADEP_LATERAL: DWORD = 0x22008201;
pub const DIAXIS_ARCADEP_MOVE: DWORD = 0x22010202;
pub const DIBUTTON_ARCADEP_JUMP: DWORD = 0x22000401;
pub const DIBUTTON_ARCADEP_FIRE: DWORD = 0x22000402;
pub const DIBUTTON_ARCADEP_CROUCH: DWORD = 0x22000403;
pub const DIBUTTON_ARCADEP_SPECIAL: DWORD = 0x22000404;
pub const DIBUTTON_ARCADEP_SELECT: DWORD = 0x22000405;
pub const DIBUTTON_ARCADEP_MENU: DWORD = 0x220004FD;
pub const DIHATSWITCH_ARCADEP_VIEW: DWORD = 0x22004601;
pub const DIBUTTON_ARCADEP_FIRESECONDARY: DWORD = 0x22004406;
pub const DIBUTTON_ARCADEP_LEFT_LINK: DWORD = 0x2200C4E4;
pub const DIBUTTON_ARCADEP_RIGHT_LINK: DWORD = 0x2200C4EC;
pub const DIBUTTON_ARCADEP_FORWARD_LINK: DWORD = 0x220144E0;
pub const DIBUTTON_ARCADEP_BACK_LINK: DWORD = 0x220144E8;
pub const DIBUTTON_ARCADEP_VIEW_UP_LINK: DWORD = 0x2207C4E0;
pub const DIBUTTON_ARCADEP_VIEW_DOWN_LINK: DWORD = 0x2207C4E8;
pub const DIBUTTON_ARCADEP_VIEW_LEFT_LINK: DWORD = 0x2207C4E4;
pub const DIBUTTON_ARCADEP_VIEW_RIGHT_LINK: DWORD = 0x2207C4EC;
pub const DIBUTTON_ARCADEP_DEVICE: DWORD = 0x220044FE;
pub const DIBUTTON_ARCADEP_PAUSE: DWORD = 0x220044FC;
pub const DIVIRTUAL_CAD_2DCONTROL: DWORD = 0x23000000;
pub const DIAXIS_2DCONTROL_LATERAL: DWORD = 0x23008201;
pub const DIAXIS_2DCONTROL_MOVE: DWORD = 0x23010202;
pub const DIAXIS_2DCONTROL_INOUT: DWORD = 0x23018203;
pub const DIBUTTON_2DCONTROL_SELECT: DWORD = 0x23000401;
pub const DIBUTTON_2DCONTROL_SPECIAL1: DWORD = 0x23000402;
pub const DIBUTTON_2DCONTROL_SPECIAL: DWORD = 0x23000403;
pub const DIBUTTON_2DCONTROL_SPECIAL2: DWORD = 0x23000404;
pub const DIBUTTON_2DCONTROL_MENU: DWORD = 0x230004FD;
pub const DIHATSWITCH_2DCONTROL_HATSWITCH: DWORD = 0x23004601;
pub const DIAXIS_2DCONTROL_ROTATEZ: DWORD = 0x23024204;
pub const DIBUTTON_2DCONTROL_DISPLAY: DWORD = 0x23004405;
pub const DIBUTTON_2DCONTROL_DEVICE: DWORD = 0x230044FE;
pub const DIBUTTON_2DCONTROL_PAUSE: DWORD = 0x230044FC;
pub const DIVIRTUAL_CAD_3DCONTROL: DWORD = 0x24000000;
pub const DIAXIS_3DCONTROL_LATERAL: DWORD = 0x24008201;
pub const DIAXIS_3DCONTROL_MOVE: DWORD = 0x24010202;
pub const DIAXIS_3DCONTROL_INOUT: DWORD = 0x24018203;
pub const DIBUTTON_3DCONTROL_SELECT: DWORD = 0x24000401;
pub const DIBUTTON_3DCONTROL_SPECIAL1: DWORD = 0x24000402;
pub const DIBUTTON_3DCONTROL_SPECIAL: DWORD = 0x24000403;
pub const DIBUTTON_3DCONTROL_SPECIAL2: DWORD = 0x24000404;
pub const DIBUTTON_3DCONTROL_MENU: DWORD = 0x240004FD;
pub const DIHATSWITCH_3DCONTROL_HATSWITCH: DWORD = 0x24004601;
pub const DIAXIS_3DCONTROL_ROTATEX: DWORD = 0x24034204;
pub const DIAXIS_3DCONTROL_ROTATEY: DWORD = 0x2402C205;
pub const DIAXIS_3DCONTROL_ROTATEZ: DWORD = 0x24024206;
pub const DIBUTTON_3DCONTROL_DISPLAY: DWORD = 0x24004405;
pub const DIBUTTON_3DCONTROL_DEVICE: DWORD = 0x240044FE;
pub const DIBUTTON_3DCONTROL_PAUSE: DWORD = 0x240044FC;
pub const DIVIRTUAL_CAD_FLYBY: DWORD = 0x25000000;
pub const DIAXIS_CADF_LATERAL: DWORD = 0x25008201;
pub const DIAXIS_CADF_MOVE: DWORD = 0x25010202;
pub const DIAXIS_CADF_INOUT: DWORD = 0x25018203;
pub const DIBUTTON_CADF_SELECT: DWORD = 0x25000401;
pub const DIBUTTON_CADF_SPECIAL1: DWORD = 0x25000402;
pub const DIBUTTON_CADF_SPECIAL: DWORD = 0x25000403;
pub const DIBUTTON_CADF_SPECIAL2: DWORD = 0x25000404;
pub const DIBUTTON_CADF_MENU: DWORD = 0x250004FD;
pub const DIHATSWITCH_CADF_HATSWITCH: DWORD = 0x25004601;
pub const DIAXIS_CADF_ROTATEX: DWORD = 0x25034204;
pub const DIAXIS_CADF_ROTATEY: DWORD = 0x2502C205;
pub const DIAXIS_CADF_ROTATEZ: DWORD = 0x25024206;
pub const DIBUTTON_CADF_DISPLAY: DWORD = 0x25004405;
pub const DIBUTTON_CADF_DEVICE: DWORD = 0x250044FE;
pub const DIBUTTON_CADF_PAUSE: DWORD = 0x250044FC;
pub const DIVIRTUAL_CAD_MODEL: DWORD = 0x26000000;
pub const DIAXIS_CADM_LATERAL: DWORD = 0x26008201;
pub const DIAXIS_CADM_MOVE: DWORD = 0x26010202;
pub const DIAXIS_CADM_INOUT: DWORD = 0x26018203;
pub const DIBUTTON_CADM_SELECT: DWORD = 0x26000401;
pub const DIBUTTON_CADM_SPECIAL1: DWORD = 0x26000402;
pub const DIBUTTON_CADM_SPECIAL: DWORD = 0x26000403;
pub const DIBUTTON_CADM_SPECIAL2: DWORD = 0x26000404;
pub const DIBUTTON_CADM_MENU: DWORD = 0x260004FD;
pub const DIHATSWITCH_CADM_HATSWITCH: DWORD = 0x26004601;
pub const DIAXIS_CADM_ROTATEX: DWORD = 0x26034204;
pub const DIAXIS_CADM_ROTATEY: DWORD = 0x2602C205;
pub const DIAXIS_CADM_ROTATEZ: DWORD = 0x26024206;
pub const DIBUTTON_CADM_DISPLAY: DWORD = 0x26004405;
pub const DIBUTTON_CADM_DEVICE: DWORD = 0x260044FE;
pub const DIBUTTON_CADM_PAUSE: DWORD = 0x260044FC;
pub const DIVIRTUAL_REMOTE_CONTROL: DWORD = 0x27000000;
pub const DIAXIS_REMOTE_SLIDER: DWORD = 0x27050201;
pub const DIBUTTON_REMOTE_MUTE: DWORD = 0x27000401;
pub const DIBUTTON_REMOTE_SELECT: DWORD = 0x27000402;
pub const DIBUTTON_REMOTE_PLAY: DWORD = 0x27002403;
pub const DIBUTTON_REMOTE_CUE: DWORD = 0x27002404;
pub const DIBUTTON_REMOTE_REVIEW: DWORD = 0x27002405;
pub const DIBUTTON_REMOTE_CHANGE: DWORD = 0x27002406;
pub const DIBUTTON_REMOTE_RECORD: DWORD = 0x27002407;
pub const DIBUTTON_REMOTE_MENU: DWORD = 0x270004FD;
pub const DIAXIS_REMOTE_SLIDER2: DWORD = 0x27054202;
pub const DIBUTTON_REMOTE_TV: DWORD = 0x27005C08;
pub const DIBUTTON_REMOTE_CABLE: DWORD = 0x27005C09;
pub const DIBUTTON_REMOTE_CD: DWORD = 0x27005C0A;
pub const DIBUTTON_REMOTE_VCR: DWORD = 0x27005C0B;
pub const DIBUTTON_REMOTE_TUNER: DWORD = 0x27005C0C;
pub const DIBUTTON_REMOTE_DVD: DWORD = 0x27005C0D;
pub const DIBUTTON_REMOTE_ADJUST: DWORD = 0x27005C0E;
pub const DIBUTTON_REMOTE_DIGIT0: DWORD = 0x2700540F;
pub const DIBUTTON_REMOTE_DIGIT1: DWORD = 0x27005410;
pub const DIBUTTON_REMOTE_DIGIT2: DWORD = 0x27005411;
pub const DIBUTTON_REMOTE_DIGIT3: DWORD = 0x27005412;
pub const DIBUTTON_REMOTE_DIGIT4: DWORD = 0x27005413;
pub const DIBUTTON_REMOTE_DIGIT5: DWORD = 0x27005414;
pub const DIBUTTON_REMOTE_DIGIT6: DWORD = 0x27005415;
pub const DIBUTTON_REMOTE_DIGIT7: DWORD = 0x27005416;
pub const DIBUTTON_REMOTE_DIGIT8: DWORD = 0x27005417;
pub const DIBUTTON_REMOTE_DIGIT9: DWORD = 0x27005418;
pub const DIBUTTON_REMOTE_DEVICE: DWORD = 0x270044FE;
pub const DIBUTTON_REMOTE_PAUSE: DWORD = 0x270044FC;
pub const DIVIRTUAL_BROWSER_CONTROL: DWORD = 0x28000000;
pub const DIAXIS_BROWSER_LATERAL: DWORD = 0x28008201;
pub const DIAXIS_BROWSER_MOVE: DWORD = 0x28010202;
pub const DIBUTTON_BROWSER_SELECT: DWORD = 0x28000401;
pub const DIAXIS_BROWSER_VIEW: DWORD = 0x28018203;
pub const DIBUTTON_BROWSER_REFRESH: DWORD = 0x28000402;
pub const DIBUTTON_BROWSER_MENU: DWORD = 0x280004FD;
pub const DIBUTTON_BROWSER_SEARCH: DWORD = 0x28004403;
pub const DIBUTTON_BROWSER_STOP: DWORD = 0x28004404;
pub const DIBUTTON_BROWSER_HOME: DWORD = 0x28004405;
pub const DIBUTTON_BROWSER_FAVORITES: DWORD = 0x28004406;
pub const DIBUTTON_BROWSER_NEXT: DWORD = 0x28004407;
pub const DIBUTTON_BROWSER_PREVIOUS: DWORD = 0x28004408;
pub const DIBUTTON_BROWSER_HISTORY: DWORD = 0x28004409;
pub const DIBUTTON_BROWSER_PRINT: DWORD = 0x2800440A;
pub const DIBUTTON_BROWSER_DEVICE: DWORD = 0x280044FE;
pub const DIBUTTON_BROWSER_PAUSE: DWORD = 0x280044FC;
pub const DIVIRTUAL_DRIVING_MECHA: DWORD = 0x29000000;
pub const DIAXIS_MECHA_STEER: DWORD = 0x29008201;
pub const DIAXIS_MECHA_TORSO: DWORD = 0x29010202;
pub const DIAXIS_MECHA_ROTATE: DWORD = 0x29020203;
pub const DIAXIS_MECHA_THROTTLE: DWORD = 0x29038204;
pub const DIBUTTON_MECHA_FIRE: DWORD = 0x29000401;
pub const DIBUTTON_MECHA_WEAPONS: DWORD = 0x29000402;
pub const DIBUTTON_MECHA_TARGET: DWORD = 0x29000403;
pub const DIBUTTON_MECHA_REVERSE: DWORD = 0x29000404;
pub const DIBUTTON_MECHA_ZOOM: DWORD = 0x29000405;
pub const DIBUTTON_MECHA_JUMP: DWORD = 0x29000406;
pub const DIBUTTON_MECHA_MENU: DWORD = 0x290004FD;
pub const DIBUTTON_MECHA_CENTER: DWORD = 0x29004407;
pub const DIHATSWITCH_MECHA_GLANCE: DWORD = 0x29004601;
pub const DIBUTTON_MECHA_VIEW: DWORD = 0x29004408;
pub const DIBUTTON_MECHA_FIRESECONDARY: DWORD = 0x29004409;
pub const DIBUTTON_MECHA_LEFT_LINK: DWORD = 0x2900C4E4;
pub const DIBUTTON_MECHA_RIGHT_LINK: DWORD = 0x2900C4EC;
pub const DIBUTTON_MECHA_FORWARD_LINK: DWORD = 0x290144E0;
pub const DIBUTTON_MECHA_BACK_LINK: DWORD = 0x290144E8;
pub const DIBUTTON_MECHA_ROTATE_LEFT_LINK: DWORD = 0x290244E4;
pub const DIBUTTON_MECHA_ROTATE_RIGHT_LINK: DWORD = 0x290244EC;
pub const DIBUTTON_MECHA_FASTER_LINK: DWORD = 0x2903C4E0;
pub const DIBUTTON_MECHA_SLOWER_LINK: DWORD = 0x2903C4E8;
pub const DIBUTTON_MECHA_DEVICE: DWORD = 0x290044FE;
pub const DIBUTTON_MECHA_PAUSE: DWORD = 0x290044FC;
pub const DIAXIS_ANY_X_1: DWORD = 0xFF00C201;
pub const DIAXIS_ANY_X_2: DWORD = 0xFF00C202;
pub const DIAXIS_ANY_Y_1: DWORD = 0xFF014201;
pub const DIAXIS_ANY_Y_2: DWORD = 0xFF014202;
pub const DIAXIS_ANY_Z_1: DWORD = 0xFF01C201;
pub const DIAXIS_ANY_Z_2: DWORD = 0xFF01C202;
pub const DIAXIS_ANY_R_1: DWORD = 0xFF024201;
pub const DIAXIS_ANY_R_2: DWORD = 0xFF024202;
pub const DIAXIS_ANY_U_1: DWORD = 0xFF02C201;
pub const DIAXIS_ANY_U_2: DWORD = 0xFF02C202;
pub const DIAXIS_ANY_V_1: DWORD = 0xFF034201;
pub const DIAXIS_ANY_V_2: DWORD = 0xFF034202;
pub const DIAXIS_ANY_A_1: DWORD = 0xFF03C201;
pub const DIAXIS_ANY_A_2: DWORD = 0xFF03C202;
pub const DIAXIS_ANY_B_1: DWORD = 0xFF044201;
pub const DIAXIS_ANY_B_2: DWORD = 0xFF044202;
pub const DIAXIS_ANY_C_1: DWORD = 0xFF04C201;
pub const DIAXIS_ANY_C_2: DWORD = 0xFF04C202;
pub const DIAXIS_ANY_S_1: DWORD = 0xFF054201;
pub const DIAXIS_ANY_S_2: DWORD = 0xFF054202;
pub const DIAXIS_ANY_1: DWORD = 0xFF004201;
pub const DIAXIS_ANY_2: DWORD = 0xFF004202;
pub const DIAXIS_ANY_3: DWORD = 0xFF004203;
pub const DIAXIS_ANY_4: DWORD = 0xFF004204;
pub const DIPOV_ANY_1: DWORD = 0xFF004601;
pub const DIPOV_ANY_2: DWORD = 0xFF004602;
pub const DIPOV_ANY_3: DWORD = 0xFF004603;
pub const DIPOV_ANY_4: DWORD = 0xFF004604;
#[inline]
pub fn DIBUTTON_ANY(instance: DWORD) -> DWORD {
    0xFF004400 | instance
}
pub const JOY_PASSDRIVERDATA: DWORD = 0x10000000;
extern "system" {
    pub fn joyConfigChanged(
        dwFlags: DWORD,
    ) -> MMRESULT;
    pub fn ShowJoyCPL(
        hWnd: HWND,
    );
}
FN!{stdcall LPFNSHOWJOYCPL(
    hWnd: HWND,
) -> ()}
pub const JOY_OEMPOLL_PASSDRIVERDATA: DWORD = 7;
pub const JOY_HWS_ISHEADTRACKER: DWORD = 0x02000000;
pub const JOY_HWS_ISGAMEPORTDRIVER: DWORD = 0x04000000;
pub const JOY_HWS_ISANALOGPORTDRIVER: DWORD = 0x08000000;
pub const JOY_HWS_AUTOLOAD: DWORD = 0x10000000;
pub const JOY_HWS_NODEVNODE: DWORD = 0x20000000;
pub const JOY_HWS_ISGAMEPORTBUS: DWORD = 0x80000000;
pub const JOY_HWS_GAMEPORTBUSBUSY: DWORD = 0x00000001;
pub const JOY_US_VOLATILE: DWORD = 0x00000008;