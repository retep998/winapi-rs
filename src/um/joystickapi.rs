// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! ApiSet Contract for api-ms-win-mm-joystick-l1-1-0
use shared::basetsd::UINT_PTR;
use shared::guiddef::GUID;
use shared::minwindef::{BOOL, DWORD, UINT, WORD};
use shared::windef::HWND;
use um::mmsyscom::{JOYERR_BASE, LPUINT, MAXPNAMELEN, MAX_JOYSTICKOEMVXDNAME, MMRESULT};
use um::winnt::{CHAR, WCHAR};
pub const JOYERR_NOERROR: MMRESULT = 0;
pub const JOYERR_PARMS: MMRESULT = JOYERR_BASE + 5;
pub const JOYERR_NOCANDO: MMRESULT = JOYERR_BASE + 6;
pub const JOYERR_UNPLUGGED: MMRESULT = JOYERR_BASE + 7;
pub const JOY_BUTTON1: UINT = 0x0001;
pub const JOY_BUTTON2: UINT = 0x0002;
pub const JOY_BUTTON3: UINT = 0x0004;
pub const JOY_BUTTON4: UINT = 0x0008;
pub const JOY_BUTTON1CHG: UINT = 0x0100;
pub const JOY_BUTTON2CHG: UINT = 0x0200;
pub const JOY_BUTTON3CHG: UINT = 0x0400;
pub const JOY_BUTTON4CHG: UINT = 0x0800;
pub const JOY_BUTTON5: DWORD = 0x00000010;
pub const JOY_BUTTON6: DWORD = 0x00000020;
pub const JOY_BUTTON7: DWORD = 0x00000040;
pub const JOY_BUTTON8: DWORD = 0x00000080;
pub const JOY_BUTTON9: DWORD = 0x00000100;
pub const JOY_BUTTON10: DWORD = 0x00000200;
pub const JOY_BUTTON11: DWORD = 0x00000400;
pub const JOY_BUTTON12: DWORD = 0x00000800;
pub const JOY_BUTTON13: DWORD = 0x00001000;
pub const JOY_BUTTON14: DWORD = 0x00002000;
pub const JOY_BUTTON15: DWORD = 0x00004000;
pub const JOY_BUTTON16: DWORD = 0x00008000;
pub const JOY_BUTTON17: DWORD = 0x00010000;
pub const JOY_BUTTON18: DWORD = 0x00020000;
pub const JOY_BUTTON19: DWORD = 0x00040000;
pub const JOY_BUTTON20: DWORD = 0x00080000;
pub const JOY_BUTTON21: DWORD = 0x00100000;
pub const JOY_BUTTON22: DWORD = 0x00200000;
pub const JOY_BUTTON23: DWORD = 0x00400000;
pub const JOY_BUTTON24: DWORD = 0x00800000;
pub const JOY_BUTTON25: DWORD = 0x01000000;
pub const JOY_BUTTON26: DWORD = 0x02000000;
pub const JOY_BUTTON27: DWORD = 0x04000000;
pub const JOY_BUTTON28: DWORD = 0x08000000;
pub const JOY_BUTTON29: DWORD = 0x10000000;
pub const JOY_BUTTON30: DWORD = 0x20000000;
pub const JOY_BUTTON31: DWORD = 0x40000000;
pub const JOY_BUTTON32: DWORD = 0x80000000;
pub const JOY_POVCENTERED: WORD = -1i16 as u16;
pub const JOY_POVFORWARD: WORD = 0;
pub const JOY_POVRIGHT: WORD = 9000;
pub const JOY_POVBACKWARD: WORD = 18000;
pub const JOY_POVLEFT: WORD = 27000;
pub const JOY_RETURNX: DWORD = 0x00000001;
pub const JOY_RETURNY: DWORD = 0x00000002;
pub const JOY_RETURNZ: DWORD = 0x00000004;
pub const JOY_RETURNR: DWORD = 0x00000008;
pub const JOY_RETURNU: DWORD = 0x00000010;
pub const JOY_RETURNV: DWORD = 0x00000020;
pub const JOY_RETURNPOV: DWORD = 0x00000040;
pub const JOY_RETURNBUTTONS: DWORD = 0x00000080;
pub const JOY_RETURNRAWDATA: DWORD = 0x00000100;
pub const JOY_RETURNPOVCTS: DWORD = 0x00000200;
pub const JOY_RETURNCENTERED: DWORD = 0x00000400;
pub const JOY_USEDEADZONE: DWORD = 0x00000800;
pub const JOY_RETURNALL: DWORD = JOY_RETURNX | JOY_RETURNY | JOY_RETURNZ | JOY_RETURNR |
    JOY_RETURNU | JOY_RETURNV | JOY_RETURNPOV | JOY_RETURNBUTTONS;
pub const JOY_CAL_READALWAYS: DWORD = 0x00010000;
pub const JOY_CAL_READXYONLY: DWORD = 0x00020000;
pub const JOY_CAL_READ3: DWORD = 0x00040000;
pub const JOY_CAL_READ4: DWORD = 0x00080000;
pub const JOY_CAL_READXONLY: DWORD = 0x00100000;
pub const JOY_CAL_READYONLY: DWORD = 0x00200000;
pub const JOY_CAL_READ5: DWORD = 0x00400000;
pub const JOY_CAL_READ6: DWORD = 0x00800000;
pub const JOY_CAL_READZONLY: DWORD = 0x01000000;
pub const JOY_CAL_READRONLY: DWORD = 0x02000000;
pub const JOY_CAL_READUONLY: DWORD = 0x04000000;
pub const JOY_CAL_READVONLY: DWORD = 0x08000000;
pub const JOYSTICKID1: UINT = 0;
pub const JOYSTICKID2: UINT = 1;
pub const JOYCAPS_HASZ: UINT = 0x0001;
pub const JOYCAPS_HASR: UINT = 0x0002;
pub const JOYCAPS_HASU: UINT = 0x0004;
pub const JOYCAPS_HASV: UINT = 0x0008;
pub const JOYCAPS_HASPOV: UINT = 0x0010;
pub const JOYCAPS_POV4DIR: UINT = 0x0020;
pub const JOYCAPS_POVCTS: UINT = 0x0040;
STRUCT!{#[repr(packed)] struct JOYCAPSA {
    wMid: WORD,
    wPid: WORD,
    szPname: [CHAR; MAXPNAMELEN],
    wXmin: UINT,
    wXmax: UINT,
    wYmin: UINT,
    wYmax: UINT,
    wZmin: UINT,
    wZmax: UINT,
    wNumButtons: UINT,
    wPeriodMin: UINT,
    wPeriodMax: UINT,
    wRmin: UINT,
    wRmax: UINT,
    wUmin: UINT,
    wUmax: UINT,
    wVmin: UINT,
    wVmax: UINT,
    wCaps: UINT,
    wMaxAxes: UINT,
    wNumAxes: UINT,
    wMaxButtons: UINT,
    szRegKey: [CHAR; MAXPNAMELEN],
    szOEMVxD: [CHAR; MAX_JOYSTICKOEMVXDNAME],
}}
pub type PJOYCAPSA = *mut JOYCAPSA;
pub type NPJOYCAPSA = *mut JOYCAPSA;
pub type LPJOYCAPSA = *mut JOYCAPSA;
STRUCT!{#[repr(packed)] struct JOYCAPSW {
    wMid: WORD,
    wPid: WORD,
    szPname: [WCHAR; MAXPNAMELEN],
    wXmin: UINT,
    wXmax: UINT,
    wYmin: UINT,
    wYmax: UINT,
    wZmin: UINT,
    wZmax: UINT,
    wNumButtons: UINT,
    wPeriodMin: UINT,
    wPeriodMax: UINT,
    wRmin: UINT,
    wRmax: UINT,
    wUmin: UINT,
    wUmax: UINT,
    wVmin: UINT,
    wVmax: UINT,
    wCaps: UINT,
    wMaxAxes: UINT,
    wNumAxes: UINT,
    wMaxButtons: UINT,
    szRegKey: [WCHAR; MAXPNAMELEN],
    szOEMVxD: [WCHAR; MAX_JOYSTICKOEMVXDNAME],
}}
pub type PJOYCAPSW = *mut JOYCAPSW;
pub type NPJOYCAPSW = *mut JOYCAPSW;
pub type LPJOYCAPSW = *mut JOYCAPSW;
STRUCT!{#[repr(packed)] struct JOYCAPS2A {
    wMid: WORD,
    wPid: WORD,
    szPname: [CHAR; MAXPNAMELEN],
    wXmin: UINT,
    wXmax: UINT,
    wYmin: UINT,
    wYmax: UINT,
    wZmin: UINT,
    wZmax: UINT,
    wNumButtons: UINT,
    wPeriodMin: UINT,
    wPeriodMax: UINT,
    wRmin: UINT,
    wRmax: UINT,
    wUmin: UINT,
    wUmax: UINT,
    wVmin: UINT,
    wVmax: UINT,
    wCaps: UINT,
    wMaxAxes: UINT,
    wNumAxes: UINT,
    wMaxButtons: UINT,
    szRegKey: [CHAR; MAXPNAMELEN],
    szOEMVxD: [CHAR; MAX_JOYSTICKOEMVXDNAME],
    ManufacturerGuid: GUID,
    ProductGuid: GUID,
    NameGuid: GUID,
}}
pub type PJOYCAPS2A = *mut JOYCAPS2A;
pub type NPJOYCAPS2A = *mut JOYCAPS2A;
pub type LPJOYCAPS2A = *mut JOYCAPS2A;
STRUCT!{#[repr(packed)] struct JOYCAPS2W {
    wMid: WORD,
    wPid: WORD,
    szPname: [WCHAR; MAXPNAMELEN],
    wXmin: UINT,
    wXmax: UINT,
    wYmin: UINT,
    wYmax: UINT,
    wZmin: UINT,
    wZmax: UINT,
    wNumButtons: UINT,
    wPeriodMin: UINT,
    wPeriodMax: UINT,
    wRmin: UINT,
    wRmax: UINT,
    wUmin: UINT,
    wUmax: UINT,
    wVmin: UINT,
    wVmax: UINT,
    wCaps: UINT,
    wMaxAxes: UINT,
    wNumAxes: UINT,
    wMaxButtons: UINT,
    szRegKey: [WCHAR; MAXPNAMELEN],
    szOEMVxD: [WCHAR; MAX_JOYSTICKOEMVXDNAME],
    ManufacturerGuid: GUID,
    ProductGuid: GUID,
    NameGuid: GUID,
}}
pub type PJOYCAPS2W = *mut JOYCAPS2W;
pub type NPJOYCAPS2W = *mut JOYCAPS2W;
pub type LPJOYCAPS2W = *mut JOYCAPS2W;
STRUCT!{#[repr(packed)] struct JOYINFO {
    wXpos: UINT,
    wYpos: UINT,
    wZpos: UINT,
    wButtons: UINT,
}}
pub type PJOYINFO = *mut JOYINFO;
pub type NPJOYINFO = *mut JOYINFO;
pub type LPJOYINFO = *mut JOYINFO;
STRUCT!{#[repr(packed)] struct JOYINFOEX {
    dwSize: DWORD,
    dwFlags: DWORD,
    dwXpos: DWORD,
    dwYpos: DWORD,
    dwZpos: DWORD,
    dwRpos: DWORD,
    dwUpos: DWORD,
    dwVpos: DWORD,
    dwButtons: DWORD,
    dwButtonNumber: DWORD,
    dwPOV: DWORD,
    dwReserved1: DWORD,
    dwReserved2: DWORD,
}}
pub type PJOYINFOEX = *mut JOYINFOEX;
pub type NPJOYINFOEX = *mut JOYINFOEX;
pub type LPJOYINFOEX = *mut JOYINFOEX;
extern "system" {
    pub fn joyGetPosEx(
        uJoyID: UINT,
        pji: LPJOYINFOEX,
    ) -> MMRESULT;
    pub fn joyGetNumDevs() -> UINT;
    pub fn joyGetDevCapsA(
        uJoyID: UINT_PTR,
        pjc: LPJOYCAPSA,
        cbjc: UINT,
    ) -> MMRESULT;
    pub fn joyGetDevCapsW(
        uJoyID: UINT_PTR,
        pjc: LPJOYCAPSW,
        cbjc: UINT,
    ) -> MMRESULT;
    pub fn joyGetPos(
        uJoyID: UINT,
        pji: LPJOYINFO,
    ) -> MMRESULT;
    pub fn joyGetThreshold(
        uJoyID: UINT,
        puThreshold: LPUINT,
    ) -> MMRESULT;
    pub fn joyReleaseCapture(
        uJoyID: UINT,
    ) -> MMRESULT;
    pub fn joySetCapture(
        hwnd: HWND,
        uJoyID: UINT,
        uPeriod: UINT,
        fChanged: BOOL,
    ) -> MMRESULT;
    pub fn joySetThreshold(
        uJoyID: UINT,
        uThreshold: UINT,
    ) -> MMRESULT;
    pub fn joyConfigChanged(
        dwFlags: DWORD,
    ) -> MMRESULT;
}
