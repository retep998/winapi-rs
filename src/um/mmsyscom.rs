// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Commonm Include file for Multimedia API's
use shared::basetsd::DWORD_PTR;
use shared::minwindef::{BYTE, DWORD, UINT};
pub const MAXPNAMELEN: usize = 32;
pub const MAXERRORLENGTH: usize = 256;
pub const MAX_JOYSTICKOEMVXDNAME: usize = 260;
pub type MMVERSION = UINT;
pub type MMRESULT = UINT;
pub type LPUINT = *mut UINT;
STRUCT!{#[repr(packed)] struct MMTIME_u_smpte {
    hour: BYTE,
    min: BYTE,
    sec: BYTE,
    frame: BYTE,
    fps: BYTE,
    dummy: BYTE,
    pad: [BYTE; 2],
}}
STRUCT!{#[repr(packed)] struct MMTIME_u_midi {
    songptrpos: DWORD,
}}
UNION!{union MMTIME_u {
    [u8; 8],
    ms ms_mut: DWORD,
    sample sample_mut: DWORD,
    cb cb_mut: DWORD,
    ticks ticks_mut: DWORD,
    smpte smpte_mut: MMTIME_u_smpte,
    midi midi_mut: MMTIME_u_midi,
}}
STRUCT!{#[repr(packed)] struct MMTIME {
    wType: UINT,
    u: MMTIME_u,
}}
pub type PMMTIME = *mut MMTIME;
pub type NPMMTIME = *mut MMTIME;
pub type LPMMTIME = *mut MMTIME;
pub const TIME_MS: UINT = 0x0001;
pub const TIME_SAMPLES: UINT = 0x0002;
pub const TIME_BYTES: UINT = 0x0004;
pub const TIME_SMPTE: UINT = 0x0008;
pub const TIME_MIDI: UINT = 0x0010;
pub const TIME_TICKS: UINT = 0x0020;
#[inline]
pub fn MAKEFOURCC(ch0: BYTE, ch1: BYTE, ch2: BYTE, ch3: BYTE) -> DWORD {
    (ch0 as DWORD) | ((ch1 as DWORD) << 8) | ((ch2 as DWORD) << 16) | ((ch3 as DWORD) << 24)
}
pub const MM_JOY1MOVE: UINT = 0x3A0;
pub const MM_JOY2MOVE: UINT = 0x3A1;
pub const MM_JOY1ZMOVE: UINT = 0x3A2;
pub const MM_JOY2ZMOVE: UINT = 0x3A3;
pub const MM_JOY1BUTTONDOWN: UINT = 0x3B5;
pub const MM_JOY2BUTTONDOWN: UINT = 0x3B6;
pub const MM_JOY1BUTTONUP: UINT = 0x3B7;
pub const MM_JOY2BUTTONUP: UINT = 0x3B8;
pub const MM_MCINOTIFY: UINT = 0x3B9;
pub const MM_WOM_OPEN: UINT = 0x3BB;
pub const MM_WOM_CLOSE: UINT = 0x3BC;
pub const MM_WOM_DONE: UINT = 0x3BD;
pub const MM_WIM_OPEN: UINT = 0x3BE;
pub const MM_WIM_CLOSE: UINT = 0x3BF;
pub const MM_WIM_DATA: UINT = 0x3C0;
pub const MM_MIM_OPEN: UINT = 0x3C1;
pub const MM_MIM_CLOSE: UINT = 0x3C2;
pub const MM_MIM_DATA: UINT = 0x3C3;
pub const MM_MIM_LONGDATA: UINT = 0x3C4;
pub const MM_MIM_ERROR: UINT = 0x3C5;
pub const MM_MIM_LONGERROR: UINT = 0x3C6;
pub const MM_MOM_OPEN: UINT = 0x3C7;
pub const MM_MOM_CLOSE: UINT = 0x3C8;
pub const MM_MOM_DONE: UINT = 0x3C9;
pub const MM_DRVM_OPEN: UINT = 0x3D0;
pub const MM_DRVM_CLOSE: UINT = 0x3D1;
pub const MM_DRVM_DATA: UINT = 0x3D2;
pub const MM_DRVM_ERROR: UINT = 0x3D3;
pub const MM_STREAM_OPEN: UINT = 0x3D4;
pub const MM_STREAM_CLOSE: UINT = 0x3D5;
pub const MM_STREAM_DONE: UINT = 0x3D6;
pub const MM_STREAM_ERROR: UINT = 0x3D7;
pub const MM_MOM_POSITIONCB: UINT = 0x3CA;
pub const MM_MCISIGNAL: UINT = 0x3CB;
pub const MM_MIM_MOREDATA: UINT = 0x3CC;
pub const MM_MIXM_LINE_CHANGE: UINT = 0x3D0;
pub const MM_MIXM_CONTROL_CHANGE: UINT = 0x3D1;
pub const MMSYSERR_BASE: MMRESULT = 0;
pub const WAVERR_BASE: MMRESULT = 32;
pub const MIDIERR_BASE: MMRESULT = 64;
pub const TIMERR_BASE: MMRESULT = 96;
pub const JOYERR_BASE: MMRESULT = 160;
pub const MCIERR_BASE: MMRESULT = 256;
pub const MIXERR_BASE: MMRESULT = 1024;
pub const MCI_STRING_OFFSET: UINT = 512;
pub const MCI_VD_OFFSET: UINT = 1024;
pub const MCI_CD_OFFSET: UINT = 1088;
pub const MCI_WAVE_OFFSET: UINT = 1152;
pub const MCI_SEQ_OFFSET: UINT = 1216;
pub const MMSYSERR_NOERROR: UINT = 0;
pub const MMSYSERR_ERROR: UINT = MMSYSERR_BASE + 1;
pub const MMSYSERR_BADDEVICEID: UINT = MMSYSERR_BASE + 2;
pub const MMSYSERR_NOTENABLED: UINT = MMSYSERR_BASE + 3;
pub const MMSYSERR_ALLOCATED: UINT = MMSYSERR_BASE + 4;
pub const MMSYSERR_INVALHANDLE: UINT = MMSYSERR_BASE + 5;
pub const MMSYSERR_NODRIVER: UINT = MMSYSERR_BASE + 6;
pub const MMSYSERR_NOMEM: UINT = MMSYSERR_BASE + 7;
pub const MMSYSERR_NOTSUPPORTED: UINT = MMSYSERR_BASE + 8;
pub const MMSYSERR_BADERRNUM: UINT = MMSYSERR_BASE + 9;
pub const MMSYSERR_INVALFLAG: UINT = MMSYSERR_BASE + 10;
pub const MMSYSERR_INVALPARAM: UINT = MMSYSERR_BASE + 11;
pub const MMSYSERR_HANDLEBUSY: UINT = MMSYSERR_BASE + 12;
pub const MMSYSERR_INVALIDALIAS: UINT = MMSYSERR_BASE + 13;
pub const MMSYSERR_BADDB: UINT = MMSYSERR_BASE + 14;
pub const MMSYSERR_KEYNOTFOUND: UINT = MMSYSERR_BASE + 15;
pub const MMSYSERR_READERROR: UINT = MMSYSERR_BASE + 16;
pub const MMSYSERR_WRITEERROR: UINT = MMSYSERR_BASE + 17;
pub const MMSYSERR_DELETEERROR: UINT = MMSYSERR_BASE + 18;
pub const MMSYSERR_VALNOTFOUND: UINT = MMSYSERR_BASE + 19;
pub const MMSYSERR_NODRIVERCB: UINT = MMSYSERR_BASE + 20;
pub const MMSYSERR_MOREDATA: UINT = MMSYSERR_BASE + 21;
pub const MMSYSERR_LASTERROR: UINT = MMSYSERR_BASE + 21;
DECLARE_HANDLE!(HDRVR, HDRVR__);
pub const CALLBACK_TYPEMASK: DWORD = 0x00070000;
pub const CALLBACK_NULL: DWORD = 0x00000000;
pub const CALLBACK_WINDOW: DWORD = 0x00010000;
pub const CALLBACK_TASK: DWORD = 0x00020000;
pub const CALLBACK_FUNCTION: DWORD = 0x00030000;
pub const CALLBACK_THREAD: DWORD = CALLBACK_TASK;
pub const CALLBACK_EVENT: DWORD = 0x00050000;
FN!{stdcall PDRVCALLBACK(
    hdrvr: HDRVR,
    uMsg: UINT,
    dwUser: DWORD_PTR,
    dw1: DWORD_PTR,
    dw2: DWORD_PTR,
) -> ()}
pub type LPDRVCALLBACK = PDRVCALLBACK;
