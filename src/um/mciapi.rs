// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! ApiSet Contract for api-ms-win-mm-mci-l1-1-0
use ctypes::c_int;
use shared::basetsd::{DWORD_PTR};
use shared::minwindef::{BOOL, BYTE, DWORD, HTASK, LPDWORD, UINT, WORD, WPARAM};
use shared::windef::{HDC, HWND, RECT};
use um::mmiscapi::DRV_MCI_FIRST;
use um::mmsyscom::{
    MCIERR_BASE, MCI_CD_OFFSET, MCI_SEQ_OFFSET, MCI_STRING_OFFSET, MCI_VD_OFFSET, MCI_WAVE_OFFSET,
};
use um::winnt::{HANDLE, LPCSTR, LPCWSTR, LPSTR, LPWSTR};
pub type MCIERROR = DWORD;
pub type MCIDEVICEID = UINT;
FN!{stdcall YIELDPROC(
    mciId: MCIDEVICEID,
    dwYieldData: DWORD,
) -> UINT}
extern "system" {
    pub fn mciSendCommandA(
        mciId: MCIDEVICEID,
        uMsg: UINT,
        dwParam1: DWORD_PTR,
        dwParam2: DWORD_PTR,
    ) -> MCIERROR;
    pub fn mciSendCommandW(
        mciId: MCIDEVICEID,
        uMsg: UINT,
        dwParam1: DWORD_PTR,
        dwParam2: DWORD_PTR,
    ) -> MCIERROR;
    pub fn mciSendStringA(
        lpstrCommand: LPCSTR,
        lpstrReturnString: LPSTR,
        uReturnLength: UINT,
        hwndCallback: HWND,
    ) -> MCIERROR;
    pub fn mciSendStringW(
        lpstrCommand: LPCWSTR,
        lpstrReturnString: LPWSTR,
        uReturnLength: UINT,
        hwndCallback: HWND,
    ) -> MCIERROR;
    pub fn mciGetDeviceIDA(
        pszDevice: LPCSTR,
    ) -> MCIDEVICEID;
    pub fn mciGetDeviceIDW(
        pszDevice: LPCWSTR,
    ) -> MCIDEVICEID;
    pub fn mciGetDeviceIDFromElementIDA(
        dwElementID: DWORD,
        lpstrType: LPCSTR,
    ) -> MCIDEVICEID;
    pub fn mciGetDeviceIDFromElementIDW(
        dwElementID: DWORD,
        lpstrType: LPCWSTR,
    ) -> MCIDEVICEID;
    pub fn mciGetErrorStringA(
        mcierr: MCIERROR,
        pszText: LPSTR,
        cchText: UINT,
    ) -> BOOL;
    pub fn mciGetErrorStringW(
        mcierr: MCIERROR,
        pszText: LPWSTR,
        cchText: UINT,
    ) -> BOOL;
    pub fn mciSetYieldProc(
        mciId: MCIDEVICEID,
        fpYieldProc: YIELDPROC,
        dwYieldData: DWORD,
    ) -> BOOL;
    pub fn mciGetCreatorTask(
        mciId: MCIDEVICEID,
    ) -> HTASK;
    pub fn mciGetYieldProc(
        mciId: MCIDEVICEID,
        pdwYieldData: LPDWORD,
    ) -> YIELDPROC;
}
pub const MCIERR_INVALID_DEVICE_ID: UINT = MCIERR_BASE + 1;
pub const MCIERR_UNRECOGNIZED_KEYWORD: UINT = MCIERR_BASE + 3;
pub const MCIERR_UNRECOGNIZED_COMMAND: UINT = MCIERR_BASE + 5;
pub const MCIERR_HARDWARE: UINT = MCIERR_BASE + 6;
pub const MCIERR_INVALID_DEVICE_NAME: UINT = MCIERR_BASE + 7;
pub const MCIERR_OUT_OF_MEMORY: UINT = MCIERR_BASE + 8;
pub const MCIERR_DEVICE_OPEN: UINT = MCIERR_BASE + 9;
pub const MCIERR_CANNOT_LOAD_DRIVER: UINT = MCIERR_BASE + 10;
pub const MCIERR_MISSING_COMMAND_STRING: UINT = MCIERR_BASE + 11;
pub const MCIERR_PARAM_OVERFLOW: UINT = MCIERR_BASE + 12;
pub const MCIERR_MISSING_STRING_ARGUMENT: UINT = MCIERR_BASE + 13;
pub const MCIERR_BAD_INTEGER: UINT = MCIERR_BASE + 14;
pub const MCIERR_PARSER_INTERNAL: UINT = MCIERR_BASE + 15;
pub const MCIERR_DRIVER_INTERNAL: UINT = MCIERR_BASE + 16;
pub const MCIERR_MISSING_PARAMETER: UINT = MCIERR_BASE + 17;
pub const MCIERR_UNSUPPORTED_FUNCTION: UINT = MCIERR_BASE + 18;
pub const MCIERR_FILE_NOT_FOUND: UINT = MCIERR_BASE + 19;
pub const MCIERR_DEVICE_NOT_READY: UINT = MCIERR_BASE + 20;
pub const MCIERR_INTERNAL: UINT = MCIERR_BASE + 21;
pub const MCIERR_DRIVER: UINT = MCIERR_BASE + 22;
pub const MCIERR_CANNOT_USE_ALL: UINT = MCIERR_BASE + 23;
pub const MCIERR_MULTIPLE: UINT = MCIERR_BASE + 24;
pub const MCIERR_EXTENSION_NOT_FOUND: UINT = MCIERR_BASE + 25;
pub const MCIERR_OUTOFRANGE: UINT = MCIERR_BASE + 26;
pub const MCIERR_FLAGS_NOT_COMPATIBLE: UINT = MCIERR_BASE + 28;
pub const MCIERR_FILE_NOT_SAVED: UINT = MCIERR_BASE + 30;
pub const MCIERR_DEVICE_TYPE_REQUIRED: UINT = MCIERR_BASE + 31;
pub const MCIERR_DEVICE_LOCKED: UINT = MCIERR_BASE + 32;
pub const MCIERR_DUPLICATE_ALIAS: UINT = MCIERR_BASE + 33;
pub const MCIERR_BAD_CONSTANT: UINT = MCIERR_BASE + 34;
pub const MCIERR_MUST_USE_SHAREABLE: UINT = MCIERR_BASE + 35;
pub const MCIERR_MISSING_DEVICE_NAME: UINT = MCIERR_BASE + 36;
pub const MCIERR_BAD_TIME_FORMAT: UINT = MCIERR_BASE + 37;
pub const MCIERR_NO_CLOSING_QUOTE: UINT = MCIERR_BASE + 38;
pub const MCIERR_DUPLICATE_FLAGS: UINT = MCIERR_BASE + 39;
pub const MCIERR_INVALID_FILE: UINT = MCIERR_BASE + 40;
pub const MCIERR_NULL_PARAMETER_BLOCK: UINT = MCIERR_BASE + 41;
pub const MCIERR_UNNAMED_RESOURCE: UINT = MCIERR_BASE + 42;
pub const MCIERR_NEW_REQUIRES_ALIAS: UINT = MCIERR_BASE + 43;
pub const MCIERR_NOTIFY_ON_AUTO_OPEN: UINT = MCIERR_BASE + 44;
pub const MCIERR_NO_ELEMENT_ALLOWED: UINT = MCIERR_BASE + 45;
pub const MCIERR_NONAPPLICABLE_FUNCTION: UINT = MCIERR_BASE + 46;
pub const MCIERR_ILLEGAL_FOR_AUTO_OPEN: UINT = MCIERR_BASE + 47;
pub const MCIERR_FILENAME_REQUIRED: UINT = MCIERR_BASE + 48;
pub const MCIERR_EXTRA_CHARACTERS: UINT = MCIERR_BASE + 49;
pub const MCIERR_DEVICE_NOT_INSTALLED: UINT = MCIERR_BASE + 50;
pub const MCIERR_GET_CD: UINT = MCIERR_BASE + 51;
pub const MCIERR_SET_CD: UINT = MCIERR_BASE + 52;
pub const MCIERR_SET_DRIVE: UINT = MCIERR_BASE + 53;
pub const MCIERR_DEVICE_LENGTH: UINT = MCIERR_BASE + 54;
pub const MCIERR_DEVICE_ORD_LENGTH: UINT = MCIERR_BASE + 55;
pub const MCIERR_NO_INTEGER: UINT = MCIERR_BASE + 56;
pub const MCIERR_WAVE_OUTPUTSINUSE: UINT = MCIERR_BASE + 64;
pub const MCIERR_WAVE_SETOUTPUTINUSE: UINT = MCIERR_BASE + 65;
pub const MCIERR_WAVE_INPUTSINUSE: UINT = MCIERR_BASE + 66;
pub const MCIERR_WAVE_SETINPUTINUSE: UINT = MCIERR_BASE + 67;
pub const MCIERR_WAVE_OUTPUTUNSPECIFIED: UINT = MCIERR_BASE + 68;
pub const MCIERR_WAVE_INPUTUNSPECIFIED: UINT = MCIERR_BASE + 69;
pub const MCIERR_WAVE_OUTPUTSUNSUITABLE: UINT = MCIERR_BASE + 70;
pub const MCIERR_WAVE_SETOUTPUTUNSUITABLE: UINT = MCIERR_BASE + 71;
pub const MCIERR_WAVE_INPUTSUNSUITABLE: UINT = MCIERR_BASE + 72;
pub const MCIERR_WAVE_SETINPUTUNSUITABLE: UINT = MCIERR_BASE + 73;
pub const MCIERR_SEQ_DIV_INCOMPATIBLE: UINT = MCIERR_BASE + 80;
pub const MCIERR_SEQ_PORT_INUSE: UINT = MCIERR_BASE + 81;
pub const MCIERR_SEQ_PORT_NONEXISTENT: UINT = MCIERR_BASE + 82;
pub const MCIERR_SEQ_PORT_MAPNODEVICE: UINT = MCIERR_BASE + 83;
pub const MCIERR_SEQ_PORT_MISCERROR: UINT = MCIERR_BASE + 84;
pub const MCIERR_SEQ_TIMER: UINT = MCIERR_BASE + 85;
pub const MCIERR_SEQ_PORTUNSPECIFIED: UINT = MCIERR_BASE + 86;
pub const MCIERR_SEQ_NOMIDIPRESENT: UINT = MCIERR_BASE + 87;
pub const MCIERR_NO_WINDOW: UINT = MCIERR_BASE + 90;
pub const MCIERR_CREATEWINDOW: UINT = MCIERR_BASE + 91;
pub const MCIERR_FILE_READ: UINT = MCIERR_BASE + 92;
pub const MCIERR_FILE_WRITE: UINT = MCIERR_BASE + 93;
pub const MCIERR_NO_IDENTITY: UINT = MCIERR_BASE + 94;
pub const MCIERR_CUSTOM_DRIVER_BASE: UINT = MCIERR_BASE + 256;
pub const MCI_FIRST: UINT = DRV_MCI_FIRST;
pub const MCI_OPEN: UINT = 0x0803;
pub const MCI_CLOSE: UINT = 0x0804;
pub const MCI_ESCAPE: UINT = 0x0805;
pub const MCI_PLAY: UINT = 0x0806;
pub const MCI_SEEK: UINT = 0x0807;
pub const MCI_STOP: UINT = 0x0808;
pub const MCI_PAUSE: UINT = 0x0809;
pub const MCI_INFO: UINT = 0x080A;
pub const MCI_GETDEVCAPS: UINT = 0x080B;
pub const MCI_SPIN: UINT = 0x080C;
pub const MCI_SET: UINT = 0x080D;
pub const MCI_STEP: UINT = 0x080E;
pub const MCI_RECORD: UINT = 0x080F;
pub const MCI_SYSINFO: UINT = 0x0810;
pub const MCI_BREAK: UINT = 0x0811;
pub const MCI_SAVE: UINT = 0x0813;
pub const MCI_STATUS: UINT = 0x0814;
pub const MCI_CUE: UINT = 0x0830;
pub const MCI_REALIZE: UINT = 0x0840;
pub const MCI_WINDOW: UINT = 0x0841;
pub const MCI_PUT: UINT = 0x0842;
pub const MCI_WHERE: UINT = 0x0843;
pub const MCI_FREEZE: UINT = 0x0844;
pub const MCI_UNFREEZE: UINT = 0x0845;
pub const MCI_LOAD: UINT = 0x0850;
pub const MCI_CUT: UINT = 0x0851;
pub const MCI_COPY: UINT = 0x0852;
pub const MCI_PASTE: UINT = 0x0853;
pub const MCI_UPDATE: UINT = 0x0854;
pub const MCI_RESUME: UINT = 0x0855;
pub const MCI_DELETE: UINT = 0x0856;
pub const MCI_USER_MESSAGES: UINT = DRV_MCI_FIRST + 0x400;
pub const MCI_LAST: UINT = 0x0FFF;
pub const MCI_ALL_DEVICE_ID: MCIDEVICEID = -1i32 as u32;
pub const MCI_DEVTYPE_VCR: UINT = 513;
pub const MCI_DEVTYPE_VIDEODISC: UINT = 514;
pub const MCI_DEVTYPE_OVERLAY: UINT = 515;
pub const MCI_DEVTYPE_CD_AUDIO: UINT = 516;
pub const MCI_DEVTYPE_DAT: UINT = 517;
pub const MCI_DEVTYPE_SCANNER: UINT = 518;
pub const MCI_DEVTYPE_ANIMATION: UINT = 519;
pub const MCI_DEVTYPE_DIGITAL_VIDEO: UINT = 520;
pub const MCI_DEVTYPE_OTHER: UINT = 521;
pub const MCI_DEVTYPE_WAVEFORM_AUDIO: UINT = 522;
pub const MCI_DEVTYPE_SEQUENCER: UINT = 523;
pub const MCI_DEVTYPE_FIRST: UINT = MCI_DEVTYPE_VCR;
pub const MCI_DEVTYPE_LAST: UINT = MCI_DEVTYPE_SEQUENCER;
pub const MCI_DEVTYPE_FIRST_USER: UINT = 0x1000;
pub const MCI_MODE_NOT_READY: UINT = MCI_STRING_OFFSET + 12;
pub const MCI_MODE_STOP: UINT = MCI_STRING_OFFSET + 13;
pub const MCI_MODE_PLAY: UINT = MCI_STRING_OFFSET + 14;
pub const MCI_MODE_RECORD: UINT = MCI_STRING_OFFSET + 15;
pub const MCI_MODE_SEEK: UINT = MCI_STRING_OFFSET + 16;
pub const MCI_MODE_PAUSE: UINT = MCI_STRING_OFFSET + 17;
pub const MCI_MODE_OPEN: UINT = MCI_STRING_OFFSET + 18;
pub const MCI_FORMAT_MILLISECONDS: DWORD = 0;
pub const MCI_FORMAT_HMS: DWORD = 1;
pub const MCI_FORMAT_MSF: DWORD = 2;
pub const MCI_FORMAT_FRAMES: DWORD = 3;
pub const MCI_FORMAT_SMPTE_24: DWORD = 4;
pub const MCI_FORMAT_SMPTE_25: DWORD = 5;
pub const MCI_FORMAT_SMPTE_30: DWORD = 6;
pub const MCI_FORMAT_SMPTE_30DROP: DWORD = 7;
pub const MCI_FORMAT_BYTES: DWORD = 8;
pub const MCI_FORMAT_SAMPLES: DWORD = 9;
pub const MCI_FORMAT_TMSF: DWORD = 10;
#[inline]
pub fn MCI_MSF_MINUTE(msf: DWORD) -> BYTE {
    msf as BYTE
}
#[inline]
pub fn MCI_MSF_SECOND(msf: DWORD) -> BYTE {
    ((msf as WORD) >> 8) as BYTE
}
#[inline]
pub fn MCI_MSF_FRAME(msf: DWORD) -> BYTE {
    (msf >> 16) as BYTE
}
#[inline]
pub fn MCI_MAKE_MSF(m: BYTE, s: BYTE, f: BYTE) -> DWORD {
    (m as DWORD) | ((s as DWORD) << 8) | ((f as DWORD) << 16)
}
#[inline]
pub fn MCI_TMSF_TRACK(tmsf: DWORD) -> BYTE {
    tmsf as BYTE
}
#[inline]
pub fn MCI_TMSF_MINUTE(tmsf: DWORD) -> BYTE {
    ((tmsf as WORD) >> 8) as BYTE
}
#[inline]
pub fn MCI_TMSF_SECOND(tmsf: DWORD) -> BYTE {
    (tmsf >> 16) as BYTE
}
#[inline]
pub fn MCI_TMSF_FRAME(tmsf: DWORD) -> BYTE {
    (tmsf >> 24) as BYTE
}
#[inline]
pub fn MCI_MAKE_TMSF(t: BYTE, m: BYTE, s: BYTE, f: BYTE) -> DWORD {
    (t as DWORD) | ((m as DWORD) << 8) | ((s as DWORD) << 16) | ((f as DWORD) << 24)
}
#[inline]
pub fn MCI_HMS_HOUR(hms: DWORD) -> BYTE {
    hms as BYTE
}
#[inline]
pub fn MCI_HMS_MINUTE(hms: DWORD) -> BYTE {
    ((hms as WORD) >> 8) as BYTE
}
#[inline]
pub fn MCI_HMS_SECOND(hms: DWORD) -> BYTE {
    (hms >> 16) as BYTE
}
#[inline]
pub fn MCI_MAKE_HMS(h: BYTE, m: BYTE, s: BYTE) -> DWORD {
    (h as DWORD) | ((m as DWORD) << 8) | ((s as DWORD) << 16)
}
pub const MCI_NOTIFY_SUCCESSFUL: WPARAM = 0x0001;
pub const MCI_NOTIFY_SUPERSEDED: WPARAM = 0x0002;
pub const MCI_NOTIFY_ABORTED: WPARAM = 0x0004;
pub const MCI_NOTIFY_FAILURE: WPARAM = 0x0008;
pub const MCI_NOTIFY: DWORD = 0x00000001;
pub const MCI_WAIT: DWORD = 0x00000002;
pub const MCI_FROM: DWORD = 0x00000004;
pub const MCI_TO: DWORD = 0x00000008;
pub const MCI_TRACK: DWORD = 0x00000010;
pub const MCI_OPEN_SHAREABLE: DWORD = 0x00000100;
pub const MCI_OPEN_ELEMENT: DWORD = 0x00000200;
pub const MCI_OPEN_ALIAS: DWORD = 0x00000400;
pub const MCI_OPEN_ELEMENT_ID: DWORD = 0x00000800;
pub const MCI_OPEN_TYPE_ID: DWORD = 0x00001000;
pub const MCI_OPEN_TYPE: DWORD = 0x00002000;
pub const MCI_SEEK_TO_START: DWORD = 0x00000100;
pub const MCI_SEEK_TO_END: DWORD = 0x00000200;
pub const MCI_STATUS_ITEM: DWORD = 0x00000100;
pub const MCI_STATUS_START: DWORD = 0x00000200;
pub const MCI_STATUS_LENGTH: DWORD = 0x00000001;
pub const MCI_STATUS_POSITION: DWORD = 0x00000002;
pub const MCI_STATUS_NUMBER_OF_TRACKS: DWORD = 0x00000003;
pub const MCI_STATUS_MODE: DWORD = 0x00000004;
pub const MCI_STATUS_MEDIA_PRESENT: DWORD = 0x00000005;
pub const MCI_STATUS_TIME_FORMAT: DWORD = 0x00000006;
pub const MCI_STATUS_READY: DWORD = 0x00000007;
pub const MCI_STATUS_CURRENT_TRACK: DWORD = 0x00000008;
pub const MCI_INFO_PRODUCT: DWORD = 0x00000100;
pub const MCI_INFO_FILE: DWORD = 0x00000200;
pub const MCI_INFO_MEDIA_UPC: DWORD = 0x00000400;
pub const MCI_INFO_MEDIA_IDENTITY: DWORD = 0x00000800;
pub const MCI_INFO_NAME: DWORD = 0x00001000;
pub const MCI_INFO_COPYRIGHT: DWORD = 0x00002000;
pub const MCI_GETDEVCAPS_ITEM: DWORD = 0x00000100;
pub const MCI_GETDEVCAPS_CAN_RECORD: DWORD = 0x00000001;
pub const MCI_GETDEVCAPS_HAS_AUDIO: DWORD = 0x00000002;
pub const MCI_GETDEVCAPS_HAS_VIDEO: DWORD = 0x00000003;
pub const MCI_GETDEVCAPS_DEVICE_TYPE: DWORD = 0x00000004;
pub const MCI_GETDEVCAPS_USES_FILES: DWORD = 0x00000005;
pub const MCI_GETDEVCAPS_COMPOUND_DEVICE: DWORD = 0x00000006;
pub const MCI_GETDEVCAPS_CAN_EJECT: DWORD = 0x00000007;
pub const MCI_GETDEVCAPS_CAN_PLAY: DWORD = 0x00000008;
pub const MCI_GETDEVCAPS_CAN_SAVE: DWORD = 0x00000009;
pub const MCI_SYSINFO_QUANTITY: DWORD = 0x00000100;
pub const MCI_SYSINFO_OPEN: DWORD = 0x00000200;
pub const MCI_SYSINFO_NAME: DWORD = 0x00000400;
pub const MCI_SYSINFO_INSTALLNAME: DWORD = 0x00000800;
pub const MCI_SET_DOOR_OPEN: DWORD = 0x00000100;
pub const MCI_SET_DOOR_CLOSED: DWORD = 0x00000200;
pub const MCI_SET_TIME_FORMAT: DWORD = 0x00000400;
pub const MCI_SET_AUDIO: DWORD = 0x00000800;
pub const MCI_SET_VIDEO: DWORD = 0x00001000;
pub const MCI_SET_ON: DWORD = 0x00002000;
pub const MCI_SET_OFF: DWORD = 0x00004000;
pub const MCI_SET_AUDIO_ALL: DWORD = 0x00000000;
pub const MCI_SET_AUDIO_LEFT: DWORD = 0x00000001;
pub const MCI_SET_AUDIO_RIGHT: DWORD = 0x00000002;
pub const MCI_BREAK_KEY: DWORD = 0x00000100;
pub const MCI_BREAK_HWND: DWORD = 0x00000200;
pub const MCI_BREAK_OFF: DWORD = 0x00000400;
pub const MCI_RECORD_INSERT: DWORD = 0x00000100;
pub const MCI_RECORD_OVERWRITE: DWORD = 0x00000200;
pub const MCI_SAVE_FILE: DWORD = 0x00000100;
pub const MCI_LOAD_FILE: DWORD = 0x00000100;
STRUCT!{#[repr(packed)] struct MCI_GENERIC_PARMS {
    dwCallback: DWORD_PTR,
}}
pub type PMCI_GENERIC_PARMS = *mut MCI_GENERIC_PARMS;
pub type LPMCI_GENERIC_PARMS = *mut MCI_GENERIC_PARMS;
STRUCT!{#[repr(packed)] struct MCI_OPEN_PARMSA {
    dwCallback: DWORD_PTR,
    wDeviceID: MCIDEVICEID,
    lpstrDeviceType: LPCSTR,
    lpstrElementName: LPCSTR,
    lpstrAlias: LPCSTR,
}}
pub type PMCI_OPEN_PARMSA = *mut MCI_OPEN_PARMSA;
pub type LPMCI_OPEN_PARMSA = *mut MCI_OPEN_PARMSA;
STRUCT!{#[repr(packed)] struct MCI_OPEN_PARMSW {
    dwCallback: DWORD_PTR,
    wDeviceID: MCIDEVICEID,
    lpstrDeviceType: LPCWSTR,
    lpstrElementName: LPCWSTR,
    lpstrAlias: LPCWSTR,
}}
pub type PMCI_OPEN_PARMSW = *mut MCI_OPEN_PARMSW;
pub type LPMCI_OPEN_PARMSW = *mut MCI_OPEN_PARMSW;
STRUCT!{#[repr(packed)] struct MCI_PLAY_PARMS {
    dwCallback: DWORD_PTR,
    dwFrom: DWORD,
    dwTo: DWORD,
}}
pub type PMCI_PLAY_PARMS = *mut MCI_PLAY_PARMS;
pub type LPMCI_PLAY_PARMS = *mut MCI_PLAY_PARMS;
STRUCT!{#[repr(packed)] struct MCI_SEEK_PARMS {
    dwCallback: DWORD_PTR,
    dwTo: DWORD,
}}
pub type PMCI_SEEK_PARMS = *mut MCI_SEEK_PARMS;
pub type LPMCI_SEEK_PARMS = *mut MCI_SEEK_PARMS;
STRUCT!{#[repr(packed)] struct MCI_STATUS_PARMS {
    dwCallback: DWORD_PTR,
    dwReturn: DWORD_PTR,
    dwItem: DWORD,
    dwTrack: DWORD,
}}
pub type PMCI_STATUS_PARMS = *mut MCI_STATUS_PARMS;
pub type LPMCI_STATUS_PARMS = *mut MCI_STATUS_PARMS;
STRUCT!{#[repr(packed)] struct MCI_INFO_PARMSA {
    dwCallback: DWORD_PTR,
    lpstrReturn: LPSTR,
    dwRetSize: DWORD,
}}
pub type LPMCI_INFO_PARMSA = *mut MCI_INFO_PARMSA;
STRUCT!{#[repr(packed)] struct MCI_INFO_PARMSW {
    dwCallback: DWORD_PTR,
    lpstrReturn: LPWSTR,
    dwRetSize: DWORD,
}}
pub type LPMCI_INFO_PARMSW = *mut MCI_INFO_PARMSW;
STRUCT!{#[repr(packed)] struct MCI_GETDEVCAPS_PARMS {
    dwCallback: DWORD_PTR,
    dwReturn: DWORD,
    dwItem: DWORD,
}}
pub type PMCI_GETDEVCAPS_PARMS = *mut MCI_GETDEVCAPS_PARMS;
pub type LPMCI_GETDEVCAPS_PARMS = *mut MCI_GETDEVCAPS_PARMS;
STRUCT!{#[repr(packed)] struct MCI_SYSINFO_PARMSA {
    dwCallback: DWORD_PTR,
    lpstrReturn: LPSTR,
    dwRetSize: DWORD,
    dwNumber: DWORD,
    wDeviceType: UINT,
}}
pub type PMCI_SYSINFO_PARMSA = *mut MCI_SYSINFO_PARMSA;
pub type LPMCI_SYSINFO_PARMSA = *mut MCI_SYSINFO_PARMSA;
STRUCT!{#[repr(packed)] struct MCI_SYSINFO_PARMSW {
    dwCallback: DWORD_PTR,
    lpstrReturn: LPWSTR,
    dwRetSize: DWORD,
    dwNumber: DWORD,
    wDeviceType: UINT,
}}
pub type PMCI_SYSINFO_PARMSW = *mut MCI_SYSINFO_PARMSW;
pub type LPMCI_SYSINFO_PARMSW = *mut MCI_SYSINFO_PARMSW;
STRUCT!{#[repr(packed)] struct MCI_SET_PARMS {
    dwCallback: DWORD_PTR,
    dwTimeFormat: DWORD,
    dwAudio: DWORD,
}}
pub type PMCI_SET_PARMS = *mut MCI_SET_PARMS;
pub type LPMCI_SET_PARMS = *mut MCI_SET_PARMS;
STRUCT!{#[repr(packed)] struct MCI_BREAK_PARMS {
    dwCallback: DWORD_PTR,
    nVirtKey: c_int,
    hwndBreak: HWND,
}}
pub type PMCI_BREAK_PARMS = *mut MCI_BREAK_PARMS;
pub type LPMCI_BREAK_PARMS = *mut MCI_BREAK_PARMS;
STRUCT!{#[repr(packed)] struct MCI_SAVE_PARMSA {
    dwCallback: DWORD_PTR,
    lpfilename: LPCSTR,
}}
pub type PMCI_SAVE_PARMSA = *mut MCI_SAVE_PARMSA;
pub type LPMCI_SAVE_PARMSA = *mut MCI_SAVE_PARMSA;
STRUCT!{#[repr(packed)] struct MCI_SAVE_PARMSW {
    dwCallback: DWORD_PTR,
    lpfilename: LPCWSTR,
}}
pub type PMCI_SAVE_PARMSW = *mut MCI_SAVE_PARMSW;
pub type LPMCI_SAVE_PARMSW = *mut MCI_SAVE_PARMSW;
STRUCT!{#[repr(packed)] struct MCI_LOAD_PARMSA {
    dwCallback: DWORD_PTR,
    lpfilename: LPCSTR,
}}
pub type PMCI_LOAD_PARMSA = *mut MCI_LOAD_PARMSA;
pub type LPMCI_LOAD_PARMSA = *mut MCI_LOAD_PARMSA;
STRUCT!{#[repr(packed)] struct MCI_LOAD_PARMSW {
    dwCallback: DWORD_PTR,
    lpfilename: LPCWSTR,
}}
pub type PMCI_LOAD_PARMSW = *mut MCI_LOAD_PARMSW;
pub type LPMCI_LOAD_PARMSW = *mut MCI_LOAD_PARMSW;
STRUCT!{#[repr(packed)] struct MCI_RECORD_PARMS {
    dwCallback: DWORD_PTR,
    dwFrom: DWORD,
    dwTo: DWORD,
}}
pub type LPMCI_RECORD_PARMS = *mut MCI_RECORD_PARMS;
pub const MCI_VD_MODE_PARK: DWORD = MCI_VD_OFFSET + 1;
pub const MCI_VD_MEDIA_CLV: DWORD = MCI_VD_OFFSET + 2;
pub const MCI_VD_MEDIA_CAV: DWORD = MCI_VD_OFFSET + 3;
pub const MCI_VD_MEDIA_OTHER: DWORD = MCI_VD_OFFSET + 4;
pub const MCI_VD_FORMAT_TRACK: DWORD = 0x4001;
pub const MCI_VD_PLAY_REVERSE: DWORD = 0x00010000;
pub const MCI_VD_PLAY_FAST: DWORD = 0x00020000;
pub const MCI_VD_PLAY_SPEED: DWORD = 0x00040000;
pub const MCI_VD_PLAY_SCAN: DWORD = 0x00080000;
pub const MCI_VD_PLAY_SLOW: DWORD = 0x00100000;
pub const MCI_VD_SEEK_REVERSE: DWORD = 0x00010000;
pub const MCI_VD_STATUS_SPEED: DWORD = 0x00004002;
pub const MCI_VD_STATUS_FORWARD: DWORD = 0x00004003;
pub const MCI_VD_STATUS_MEDIA_TYPE: DWORD = 0x00004004;
pub const MCI_VD_STATUS_SIDE: DWORD = 0x00004005;
pub const MCI_VD_STATUS_DISC_SIZE: DWORD = 0x00004006;
pub const MCI_VD_GETDEVCAPS_CLV: DWORD = 0x00010000;
pub const MCI_VD_GETDEVCAPS_CAV: DWORD = 0x00020000;
pub const MCI_VD_SPIN_UP: DWORD = 0x00010000;
pub const MCI_VD_SPIN_DOWN: DWORD = 0x00020000;
pub const MCI_VD_GETDEVCAPS_CAN_REVERSE: DWORD = 0x00004002;
pub const MCI_VD_GETDEVCAPS_FAST_RATE: DWORD = 0x00004003;
pub const MCI_VD_GETDEVCAPS_SLOW_RATE: DWORD = 0x00004004;
pub const MCI_VD_GETDEVCAPS_NORMAL_RATE: DWORD = 0x00004005;
pub const MCI_VD_STEP_FRAMES: DWORD = 0x00010000;
pub const MCI_VD_STEP_REVERSE: DWORD = 0x00020000;
pub const MCI_VD_ESCAPE_STRING: DWORD = 0x00000100;
STRUCT!{#[repr(packed)] struct MCI_VD_PLAY_PARMS {
    dwCallback: DWORD_PTR,
    dwFrom: DWORD,
    dwTo: DWORD,
    dwSpeed: DWORD,
}}
pub type PMCI_VD_PLAY_PARMS = *mut MCI_VD_PLAY_PARMS;
pub type LPMCI_VD_PLAY_PARMS = *mut MCI_VD_PLAY_PARMS;
STRUCT!{#[repr(packed)] struct MCI_VD_STEP_PARMS {
    dwCallback: DWORD_PTR,
    dwFrames: DWORD,
}}
pub type PMCI_VD_STEP_PARMS = *mut MCI_VD_STEP_PARMS;
pub type LPMCI_VD_STEP_PARMS = *mut MCI_VD_STEP_PARMS;
STRUCT!{#[repr(packed)] struct MCI_VD_ESCAPE_PARMSA {
    dwCallback: DWORD_PTR,
    lpstrCommand: LPCSTR,
}}
pub type PMCI_VD_ESCAPE_PARMSA = *mut MCI_VD_ESCAPE_PARMSA;
pub type LPMCI_VD_ESCAPE_PARMSA = *mut MCI_VD_ESCAPE_PARMSA;
STRUCT!{#[repr(packed)] struct MCI_VD_ESCAPE_PARMSW {
    dwCallback: DWORD_PTR,
    lpstrCommand: LPCWSTR,
}}
pub type PMCI_VD_ESCAPE_PARMSW = *mut MCI_VD_ESCAPE_PARMSW;
pub type LPMCI_VD_ESCAPE_PARMSW = *mut MCI_VD_ESCAPE_PARMSW;
pub const MCI_CDA_STATUS_TYPE_TRACK: DWORD = 0x00004001;
pub const MCI_CDA_TRACK_AUDIO: DWORD = MCI_CD_OFFSET + 0;
pub const MCI_CDA_TRACK_OTHER: DWORD = MCI_CD_OFFSET + 1;
pub const MCI_WAVE_PCM: DWORD = MCI_WAVE_OFFSET + 0;
pub const MCI_WAVE_MAPPER: DWORD = MCI_WAVE_OFFSET + 1;
pub const MCI_WAVE_OPEN_BUFFER: DWORD = 0x00010000;
pub const MCI_WAVE_SET_FORMATTAG: DWORD = 0x00010000;
pub const MCI_WAVE_SET_CHANNELS: DWORD = 0x00020000;
pub const MCI_WAVE_SET_SAMPLESPERSEC: DWORD = 0x00040000;
pub const MCI_WAVE_SET_AVGBYTESPERSEC: DWORD = 0x00080000;
pub const MCI_WAVE_SET_BLOCKALIGN: DWORD = 0x00100000;
pub const MCI_WAVE_SET_BITSPERSAMPLE: DWORD = 0x00200000;
pub const MCI_WAVE_INPUT: DWORD = 0x00400000;
pub const MCI_WAVE_OUTPUT: DWORD = 0x00800000;
pub const MCI_WAVE_STATUS_FORMATTAG: DWORD = 0x00004001;
pub const MCI_WAVE_STATUS_CHANNELS: DWORD = 0x00004002;
pub const MCI_WAVE_STATUS_SAMPLESPERSEC: DWORD = 0x00004003;
pub const MCI_WAVE_STATUS_AVGBYTESPERSEC: DWORD = 0x00004004;
pub const MCI_WAVE_STATUS_BLOCKALIGN: DWORD = 0x00004005;
pub const MCI_WAVE_STATUS_BITSPERSAMPLE: DWORD = 0x00004006;
pub const MCI_WAVE_STATUS_LEVEL: DWORD = 0x00004007;
pub const MCI_WAVE_SET_ANYINPUT: DWORD = 0x04000000;
pub const MCI_WAVE_SET_ANYOUTPUT: DWORD = 0x08000000;
pub const MCI_WAVE_GETDEVCAPS_INPUTS: DWORD = 0x00004001;
pub const MCI_WAVE_GETDEVCAPS_OUTPUTS: DWORD = 0x00004002;
STRUCT!{#[repr(packed)] struct MCI_WAVE_OPEN_PARMSA {
    dwCallback: DWORD_PTR,
    wDeviceID: MCIDEVICEID,
    lpstrDeviceType: LPCSTR,
    lpstrElementName: LPCSTR,
    lpstrAlias: LPCSTR,
    dwBufferSeconds: DWORD,
}}
pub type PMCI_WAVE_OPEN_PARMSA = *mut MCI_WAVE_OPEN_PARMSA;
pub type LPMCI_WAVE_OPEN_PARMSA = *mut MCI_WAVE_OPEN_PARMSA;
STRUCT!{#[repr(packed)] struct MCI_WAVE_OPEN_PARMSW {
    dwCallback: DWORD_PTR,
    wDeviceID: MCIDEVICEID,
    lpstrDeviceType: LPCWSTR,
    lpstrElementName: LPCWSTR,
    lpstrAlias: LPCWSTR,
    dwBufferSeconds: DWORD,
}}
pub type PMCI_WAVE_OPEN_PARMSW = *mut MCI_WAVE_OPEN_PARMSW;
pub type LPMCI_WAVE_OPEN_PARMSW = *mut MCI_WAVE_OPEN_PARMSW;
STRUCT!{#[repr(packed)] struct MCI_WAVE_DELETE_PARMS {
    dwCallback: DWORD_PTR,
    dwFrom: DWORD,
    dwTo: DWORD,
}}
pub type PMCI_WAVE_DELETE_PARMS = *mut MCI_WAVE_DELETE_PARMS;
pub type LPMCI_WAVE_DELETE_PARMS = *mut MCI_WAVE_DELETE_PARMS;
STRUCT!{#[repr(packed)] struct MCI_WAVE_SET_PARMS {
    dwCallback: DWORD_PTR,
    dwTimeFormat: DWORD,
    dwAudio: DWORD,
    wInput: UINT,
    wOutput: UINT,
    wFormatTag: WORD,
    wReserved2: WORD,
    nChannels: WORD,
    wReserved3: WORD,
    nSamplesPerSec: DWORD,
    nAvgBytesPerSec: DWORD,
    nBlockAlign: WORD,
    wReserved4: WORD,
    wBitsPerSample: WORD,
    wReserved5: WORD,
}}
pub type PMCI_WAVE_SET_PARMS = *mut MCI_WAVE_SET_PARMS;
pub type LPMCI_WAVE_SET_PARMS = *mut MCI_WAVE_SET_PARMS;
pub const MCI_SEQ_DIV_PPQN: DWORD = 0 + MCI_SEQ_OFFSET;
pub const MCI_SEQ_DIV_SMPTE_24: DWORD = 1 + MCI_SEQ_OFFSET;
pub const MCI_SEQ_DIV_SMPTE_25: DWORD = 2 + MCI_SEQ_OFFSET;
pub const MCI_SEQ_DIV_SMPTE_30DROP: DWORD = 3 + MCI_SEQ_OFFSET;
pub const MCI_SEQ_DIV_SMPTE_30: DWORD = 4 + MCI_SEQ_OFFSET;
pub const MCI_SEQ_FORMAT_SONGPTR: DWORD = 0x4001;
pub const MCI_SEQ_FILE: DWORD = 0x4002;
pub const MCI_SEQ_MIDI: DWORD = 0x4003;
pub const MCI_SEQ_SMPTE: DWORD = 0x4004;
pub const MCI_SEQ_NONE: DWORD = 65533;
pub const MCI_SEQ_MAPPER: DWORD = 65535;
pub const MCI_SEQ_STATUS_TEMPO: DWORD = 0x00004002;
pub const MCI_SEQ_STATUS_PORT: DWORD = 0x00004003;
pub const MCI_SEQ_STATUS_SLAVE: DWORD = 0x00004007;
pub const MCI_SEQ_STATUS_MASTER: DWORD = 0x00004008;
pub const MCI_SEQ_STATUS_OFFSET: DWORD = 0x00004009;
pub const MCI_SEQ_STATUS_DIVTYPE: DWORD = 0x0000400A;
pub const MCI_SEQ_STATUS_NAME: DWORD = 0x0000400B;
pub const MCI_SEQ_STATUS_COPYRIGHT: DWORD = 0x0000400C;
pub const MCI_SEQ_SET_TEMPO: DWORD = 0x00010000;
pub const MCI_SEQ_SET_PORT: DWORD = 0x00020000;
pub const MCI_SEQ_SET_SLAVE: DWORD = 0x00040000;
pub const MCI_SEQ_SET_MASTER: DWORD = 0x00080000;
pub const MCI_SEQ_SET_OFFSET: DWORD = 0x01000000;
STRUCT!{#[repr(packed)] struct MCI_SEQ_SET_PARMS {
    dwCallback: DWORD_PTR,
    dwTimeFormat: DWORD,
    dwAudio: DWORD,
    dwTempo: DWORD,
    dwPort: DWORD,
    dwSlave: DWORD,
    dwMaster: DWORD,
    dwOffset: DWORD,
}}
pub type PMCI_SEQ_SET_PARMS = *mut MCI_SEQ_SET_PARMS;
pub type LPMCI_SEQ_SET_PARMS = *mut MCI_SEQ_SET_PARMS;
pub const MCI_ANIM_OPEN_WS: DWORD = 0x00010000;
pub const MCI_ANIM_OPEN_PARENT: DWORD = 0x00020000;
pub const MCI_ANIM_OPEN_NOSTATIC: DWORD = 0x00040000;
pub const MCI_ANIM_PLAY_SPEED: DWORD = 0x00010000;
pub const MCI_ANIM_PLAY_REVERSE: DWORD = 0x00020000;
pub const MCI_ANIM_PLAY_FAST: DWORD = 0x00040000;
pub const MCI_ANIM_PLAY_SLOW: DWORD = 0x00080000;
pub const MCI_ANIM_PLAY_SCAN: DWORD = 0x00100000;
pub const MCI_ANIM_STEP_REVERSE: DWORD = 0x00010000;
pub const MCI_ANIM_STEP_FRAMES: DWORD = 0x00020000;
pub const MCI_ANIM_STATUS_SPEED: DWORD = 0x00004001;
pub const MCI_ANIM_STATUS_FORWARD: DWORD = 0x00004002;
pub const MCI_ANIM_STATUS_HWND: DWORD = 0x00004003;
pub const MCI_ANIM_STATUS_HPAL: DWORD = 0x00004004;
pub const MCI_ANIM_STATUS_STRETCH: DWORD = 0x00004005;
pub const MCI_ANIM_INFO_TEXT: DWORD = 0x00010000;
pub const MCI_ANIM_GETDEVCAPS_CAN_REVERSE: DWORD = 0x00004001;
pub const MCI_ANIM_GETDEVCAPS_FAST_RATE: DWORD = 0x00004002;
pub const MCI_ANIM_GETDEVCAPS_SLOW_RATE: DWORD = 0x00004003;
pub const MCI_ANIM_GETDEVCAPS_NORMAL_RATE: DWORD = 0x00004004;
pub const MCI_ANIM_GETDEVCAPS_PALETTES: DWORD = 0x00004006;
pub const MCI_ANIM_GETDEVCAPS_CAN_STRETCH: DWORD = 0x00004007;
pub const MCI_ANIM_GETDEVCAPS_MAX_WINDOWS: DWORD = 0x00004008;
pub const MCI_ANIM_REALIZE_NORM: DWORD = 0x00010000;
pub const MCI_ANIM_REALIZE_BKGD: DWORD = 0x00020000;
pub const MCI_ANIM_WINDOW_HWND: DWORD = 0x00010000;
pub const MCI_ANIM_WINDOW_STATE: DWORD = 0x00040000;
pub const MCI_ANIM_WINDOW_TEXT: DWORD = 0x00080000;
pub const MCI_ANIM_WINDOW_ENABLE_STRETCH: DWORD = 0x00100000;
pub const MCI_ANIM_WINDOW_DISABLE_STRETCH: DWORD = 0x00200000;
pub const MCI_ANIM_WINDOW_DEFAULT: DWORD = 0x00000000;
pub const MCI_ANIM_RECT: DWORD = 0x00010000;
pub const MCI_ANIM_PUT_SOURCE: DWORD = 0x00020000;
pub const MCI_ANIM_PUT_DESTINATION: DWORD = 0x00040000;
pub const MCI_ANIM_WHERE_SOURCE: DWORD = 0x00020000;
pub const MCI_ANIM_WHERE_DESTINATION: DWORD = 0x00040000;
pub const MCI_ANIM_UPDATE_HDC: DWORD = 0x00020000;
STRUCT!{#[repr(packed)] struct MCI_ANIM_OPEN_PARMSA {
    dwCallback: DWORD_PTR,
    wDeviceID: MCIDEVICEID,
    lpstrDeviceType: LPCSTR,
    lpstrElementName: LPCSTR,
    lpstrAlias: LPCSTR,
    dwStyle: DWORD,
    hWndParent: HWND,
}}
pub type PMCI_ANIM_OPEN_PARMSA = *mut MCI_ANIM_OPEN_PARMSA;
pub type LPMCI_ANIM_OPEN_PARMSA = *mut MCI_ANIM_OPEN_PARMSA;
STRUCT!{#[repr(packed)] struct MCI_ANIM_OPEN_PARMSW {
    dwCallback: DWORD_PTR,
    wDeviceID: MCIDEVICEID,
    lpstrDeviceType: LPCWSTR,
    lpstrElementName: LPCWSTR,
    lpstrAlias: LPCWSTR,
    dwStyle: DWORD,
    hWndParent: HWND,
}}
pub type PMCI_ANIM_OPEN_PARMSW = *mut MCI_ANIM_OPEN_PARMSW;
pub type LPMCI_ANIM_OPEN_PARMSW = *mut MCI_ANIM_OPEN_PARMSW;
STRUCT!{#[repr(packed)] struct MCI_ANIM_PLAY_PARMS {
    dwCallback: DWORD_PTR,
    dwFrom: DWORD,
    dwTo: DWORD,
    dwSpeed: DWORD,
}}
pub type PMCI_ANIM_PLAY_PARMS = *mut MCI_ANIM_PLAY_PARMS;
pub type LPMCI_ANIM_PLAY_PARMS = *mut MCI_ANIM_PLAY_PARMS;
STRUCT!{#[repr(packed)] struct MCI_ANIM_STEP_PARMS {
    dwCallback: DWORD_PTR,
    dwFrames: DWORD,
}}
pub type PMCI_ANIM_STEP_PARMS = *mut MCI_ANIM_STEP_PARMS;
pub type LPMCI_ANIM_STEP_PARMS = *mut MCI_ANIM_STEP_PARMS;
STRUCT!{#[repr(packed)] struct MCI_ANIM_WINDOW_PARMSA {
    dwCallback: DWORD_PTR,
    hWnd: HWND,
    nCmdShow: UINT,
    lpstrText: LPCSTR,
}}
pub type PMCI_ANIM_WINDOW_PARMSA = *mut MCI_ANIM_WINDOW_PARMSA;
pub type LPMCI_ANIM_WINDOW_PARMSA = *mut MCI_ANIM_WINDOW_PARMSA;
STRUCT!{#[repr(packed)] struct MCI_ANIM_WINDOW_PARMSW {
    dwCallback: DWORD_PTR,
    hWnd: HWND,
    nCmdShow: UINT,
    lpstrText: LPCWSTR,
}}
pub type PMCI_ANIM_WINDOW_PARMSW = *mut MCI_ANIM_WINDOW_PARMSW;
pub type LPMCI_ANIM_WINDOW_PARMSW = *mut MCI_ANIM_WINDOW_PARMSW;
STRUCT!{#[repr(packed)] struct MCI_ANIM_RECT_PARMS {
    dwCallback: DWORD_PTR,
    rc: RECT,
}}
pub type PMCI_ANIM_RECT_PARMS = *mut MCI_ANIM_RECT_PARMS;
pub type LPMCI_ANIM_RECT_PARMS = *mut MCI_ANIM_RECT_PARMS;
STRUCT!{#[repr(packed)] struct MCI_ANIM_UPDATE_PARMS {
    dwCallback: DWORD_PTR,
    rc: RECT,
    hDC: HDC,
}}
pub type PMCI_ANIM_UPDATE_PARMS = *mut MCI_ANIM_UPDATE_PARMS;
pub type LPMCI_ANIM_UPDATE_PARMS = *mut MCI_ANIM_UPDATE_PARMS;
pub const MCI_OVLY_OPEN_WS: DWORD = 0x00010000;
pub const MCI_OVLY_OPEN_PARENT: DWORD = 0x00020000;
pub const MCI_OVLY_STATUS_HWND: DWORD = 0x00004001;
pub const MCI_OVLY_STATUS_STRETCH: DWORD = 0x00004002;
pub const MCI_OVLY_INFO_TEXT: DWORD = 0x00010000;
pub const MCI_OVLY_GETDEVCAPS_CAN_STRETCH: DWORD = 0x00004001;
pub const MCI_OVLY_GETDEVCAPS_CAN_FREEZE: DWORD = 0x00004002;
pub const MCI_OVLY_GETDEVCAPS_MAX_WINDOWS: DWORD = 0x00004003;
pub const MCI_OVLY_WINDOW_HWND: DWORD = 0x00010000;
pub const MCI_OVLY_WINDOW_STATE: DWORD = 0x00040000;
pub const MCI_OVLY_WINDOW_TEXT: DWORD = 0x00080000;
pub const MCI_OVLY_WINDOW_ENABLE_STRETCH: DWORD = 0x00100000;
pub const MCI_OVLY_WINDOW_DISABLE_STRETCH: DWORD = 0x00200000;
pub const MCI_OVLY_WINDOW_DEFAULT: DWORD = 0x00000000;
pub const MCI_OVLY_RECT: DWORD = 0x00010000;
pub const MCI_OVLY_PUT_SOURCE: DWORD = 0x00020000;
pub const MCI_OVLY_PUT_DESTINATION: DWORD = 0x00040000;
pub const MCI_OVLY_PUT_FRAME: DWORD = 0x00080000;
pub const MCI_OVLY_PUT_VIDEO: DWORD = 0x00100000;
pub const MCI_OVLY_WHERE_SOURCE: DWORD = 0x00020000;
pub const MCI_OVLY_WHERE_DESTINATION: DWORD = 0x00040000;
pub const MCI_OVLY_WHERE_FRAME: DWORD = 0x00080000;
pub const MCI_OVLY_WHERE_VIDEO: DWORD = 0x00100000;
STRUCT!{#[repr(packed)] struct MCI_OVLY_OPEN_PARMSA {
    dwCallback: DWORD_PTR,
    wDeviceID: MCIDEVICEID,
    lpstrDeviceType: LPCSTR,
    lpstrElementName: LPCSTR,
    lpstrAlias: LPCSTR,
    dwStyle: DWORD,
    hWndParent: HWND,
}}
pub type PMCI_OVLY_OPEN_PARMSA = *mut MCI_OVLY_OPEN_PARMSA;
pub type LPMCI_OVLY_OPEN_PARMSA = *mut MCI_OVLY_OPEN_PARMSA;
STRUCT!{#[repr(packed)] struct MCI_OVLY_OPEN_PARMSW {
    dwCallback: DWORD_PTR,
    wDeviceID: MCIDEVICEID,
    lpstrDeviceType: LPCWSTR,
    lpstrElementName: LPCWSTR,
    lpstrAlias: LPCWSTR,
    dwStyle: DWORD,
    hWndParent: HWND,
}}
pub type PMCI_OVLY_OPEN_PARMSW = *mut MCI_OVLY_OPEN_PARMSW;
pub type LPMCI_OVLY_OPEN_PARMSW = *mut MCI_OVLY_OPEN_PARMSW;
STRUCT!{#[repr(packed)] struct MCI_OVLY_WINDOW_PARMSA {
    dwCallback: DWORD_PTR,
    hWnd: HWND,
    nCmdShow: UINT,
    lpstrText: LPCSTR,
}}
pub type PMCI_OVLY_WINDOW_PARMSA = *mut MCI_OVLY_WINDOW_PARMSA;
pub type LPMCI_OVLY_WINDOW_PARMSA = *mut MCI_OVLY_WINDOW_PARMSA;
STRUCT!{#[repr(packed)] struct MCI_OVLY_WINDOW_PARMSW {
    dwCallback: DWORD_PTR,
    hWnd: HWND,
    nCmdShow: UINT,
    lpstrText: LPCWSTR,
}}
pub type PMCI_OVLY_WINDOW_PARMSW = *mut MCI_OVLY_WINDOW_PARMSW;
pub type LpMCI_OVLY_WINDOW_PARMSW = *mut MCI_OVLY_WINDOW_PARMSW;
STRUCT!{#[repr(packed)] struct MCI_OVLY_RECT_PARMS {
    dwCallback: DWORD_PTR,
    rc: RECT,
}}
pub type PMCI_OVLY_RECT_PARMS = *mut MCI_OVLY_RECT_PARMS;
pub type LPMCI_OVLY_RECT_PARMS = *mut MCI_OVLY_RECT_PARMS;
STRUCT!{#[repr(packed)] struct MCI_OVLY_SAVE_PARMSA {
    dwCallback: DWORD_PTR,
    lpfilename: LPCSTR,
    rc: RECT,
}}
pub type PMCI_OVLY_SAVE_PARMSA = *mut MCI_OVLY_SAVE_PARMSA;
pub type LPMCI_OVLY_SAVE_PARMSA = *mut MCI_OVLY_SAVE_PARMSA;
STRUCT!{#[repr(packed)] struct MCI_OVLY_SAVE_PARMSW {
    dwCallback: DWORD_PTR,
    lpfilename: LPCWSTR,
    rc: RECT,
}}
pub type PMCI_OVLY_SAVE_PARMSW = *mut MCI_OVLY_SAVE_PARMSW;
pub type LPMCI_OVLY_SAVE_PARMSW = *mut MCI_OVLY_SAVE_PARMSW;
STRUCT!{#[repr(packed)] struct MCI_OVLY_LOAD_PARMSA {
    dwCallback: DWORD_PTR,
    lpfilename: LPCSTR,
    rc: RECT,
}}
pub type PMCI_OVLY_LOAD_PARMSA = *mut MCI_OVLY_LOAD_PARMSA;
pub type LPMCI_OVLY_LOAD_PARMSA = *mut MCI_OVLY_LOAD_PARMSA;
STRUCT!{#[repr(packed)] struct MCI_OVLY_LOAD_PARMSW {
    dwCallback: DWORD_PTR,
    lpfilename: LPCWSTR,
    rc: RECT,
}}
pub type PMCI_OVLY_LOAD_PARMSW = *mut MCI_OVLY_LOAD_PARMSW;
pub type LPMCI_OVLY_LOAD_PARMSW = *mut MCI_OVLY_LOAD_PARMSW;
extern "system" {
    pub fn mciGetDriverData(
        wDeviceID: MCIDEVICEID,
    ) -> DWORD_PTR;
    pub fn mciLoadCommandResource(
        hInstance: HANDLE,
        lpResName: LPCWSTR,
        wType: UINT,
    ) -> UINT;
    pub fn mciSetDriverData(
        wDeviceID: MCIDEVICEID,
        dwData: DWORD_PTR,
    ) -> BOOL;
    pub fn mciDriverYield(
        wDeviceID: MCIDEVICEID,
    ) -> UINT;
    pub fn mciDriverNotify(
        hwndCallback: HANDLE,
        wDeviceID: MCIDEVICEID,
        uStatus: UINT,
    ) -> BOOL;
    pub fn mciFreeCommandResource(
        wTable: UINT,
    ) -> BOOL;
}
