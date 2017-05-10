// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! MM procedure declarations, constant definitions and macros
use shared::basetsd::{DWORD_PTR, UINT_PTR};
use shared::minwindef::{BOOL, BYTE, DWORD, LPBYTE, LPDWORD, LPWORD, UINT, WORD};
use shared::mmreg::WAVEFORMATEX;
use shared::ntdef::{LPCTSTR, LPTSTR};
use um::imm::LPUINT;
use um::winnt::{LPCSTR, LPCWSTR, LPSTR, LPWSTR, PVOID, WCHAR};
//109 (Win 7 SDK)
pub type MMVERSION = UINT;
pub type MMRESULT = UINT;
STRUCT!{struct MMTIME {
    wType: UINT,
    u: MMTIME_u,
}}
pub type PMMTIME = *mut MMTIME;
pub type NPMMTIME = *mut MMTIME;
pub type LPMMTIME = *mut MMTIME;
STRUCT!{struct MMTIME_u {
    data: [u8; 8],
}}
UNION!(MMTIME_u, data, ms, ms_mut, DWORD);
UNION!(MMTIME_u, data, sample, sample_mut, DWORD);
UNION!(MMTIME_u, data, cb, cb_mut, DWORD);
UNION!(MMTIME_u, data, ticks, ticks_mut, DWORD);
UNION!(MMTIME_u, data, smpte, smpte_mut, MMTIME_smpte);
UNION!(MMTIME_u, data, midi, midi_mut, MMTIME_midi);
STRUCT!{struct MMTIME_smpte {
    hour: BYTE,
    min: BYTE,
    sec: BYTE,
    frame: BYTE,
    fps: BYTE,
    dummy: BYTE,
    pad: [BYTE; 2],
}}
STRUCT!{struct MMTIME_midi {
    songptrpos: DWORD,
}}
pub const TIME_MS: UINT = 0x0001;
pub const TIME_SAMPLES: UINT = 0x0002;
pub const TIME_BYTES: UINT = 0x0004;
pub const TIME_SMPTE: UINT = 0x0008;
pub const TIME_MIDI: UINT = 0x0010;
pub const TIME_TICKS: UINT = 0x0020;
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
pub const MMSYSERR_BASE: MMRESULT = 0;
pub const WAVERR_BASE: MMRESULT = 32;
pub const MIDIERR_BASE: MMRESULT = 64;
pub const TIMERR_BASE: MMRESULT = 96;
pub const JOYERR_BASE: MMRESULT = 160;
pub const MCIERR_BASE: MMRESULT = 256;
pub const MIXERR_BASE: MMRESULT = 1024;
pub const MMSYSERR_NOERROR: MMRESULT = 0;
pub const MMSYSERR_ERROR: MMRESULT = MMSYSERR_BASE + 1;
pub const MMSYSERR_BADDEVICEID: MMRESULT = MMSYSERR_BASE + 2;
pub const MMSYSERR_NOTENABLED: MMRESULT = MMSYSERR_BASE + 3;
pub const MMSYSERR_ALLOCATED: MMRESULT = MMSYSERR_BASE + 4;
pub const MMSYSERR_INVALHANDLE: MMRESULT = MMSYSERR_BASE + 5;
pub const MMSYSERR_NODRIVER: MMRESULT = MMSYSERR_BASE + 6;
pub const MMSYSERR_NOMEM: MMRESULT = MMSYSERR_BASE + 7;
pub const MMSYSERR_NOTSUPPORTED: MMRESULT = MMSYSERR_BASE + 8;
pub const MMSYSERR_BADERRNUM: MMRESULT = MMSYSERR_BASE + 9;
pub const MMSYSERR_INVALFLAG: MMRESULT = MMSYSERR_BASE + 10;
pub const MMSYSERR_INVALPARAM: MMRESULT = MMSYSERR_BASE + 11;
pub const MMSYSERR_HANDLEBUSY: MMRESULT = MMSYSERR_BASE + 12;
pub const MMSYSERR_INVALIDALIAS: MMRESULT = MMSYSERR_BASE + 13;
pub const MMSYSERR_BADDB: MMRESULT = MMSYSERR_BASE + 14;
pub const MMSYSERR_KEYNOTFOUND: MMRESULT = MMSYSERR_BASE + 15;
pub const MMSYSERR_READERROR: MMRESULT = MMSYSERR_BASE + 16;
pub const MMSYSERR_WRITEERROR: MMRESULT = MMSYSERR_BASE + 17;
pub const MMSYSERR_DELETEERROR: MMRESULT = MMSYSERR_BASE + 18;
pub const MMSYSERR_VALNOTFOUND: MMRESULT = MMSYSERR_BASE + 19;
pub const MMSYSERR_NODRIVERCB: MMRESULT = MMSYSERR_BASE + 20;
pub const MMSYSERR_MOREDATA: MMRESULT = MMSYSERR_BASE + 21;
pub const MMSYSERR_LASTERROR: MMRESULT = MMSYSERR_BASE + 21;
pub const MIDIERR_UNPREPARED: MMRESULT = MIDIERR_BASE + 0;
pub const MIDIERR_STILLPLAYING: MMRESULT = MIDIERR_BASE + 1;
pub const MIDIERR_NOMAP: MMRESULT = MIDIERR_BASE + 2;
pub const MIDIERR_NOTREADY: MMRESULT = MIDIERR_BASE + 3;
pub const MIDIERR_NODEVICE: MMRESULT = MIDIERR_BASE + 4;
pub const MIDIERR_INVALIDSETUP: MMRESULT = MIDIERR_BASE + 5;
pub const MIDIERR_BADOPENMODE: MMRESULT = MIDIERR_BASE + 6;
pub const MIDIERR_DONT_CONTINUE: MMRESULT = MIDIERR_BASE + 7;
pub const MIDIERR_LASTERROR: MMRESULT = MIDIERR_BASE + 7;
pub const CALLBACK_TYPEMASK: DWORD = 0x00070000;
pub const CALLBACK_NULL: DWORD = 0x00000000;
pub const CALLBACK_WINDOW: DWORD = 0x00010000;
pub const CALLBACK_TASK: DWORD = 0x00020000;
pub const CALLBACK_FUNCTION: DWORD = 0x00030000;
pub const CALLBACK_THREAD: DWORD = CALLBACK_TASK;
pub const CALLBACK_EVENT: DWORD = 0x00050000;
//497 (Win 7 SDK)
pub const WAVERR_BADFORMAT: MMRESULT = WAVERR_BASE + 0;
pub const WAVERR_STILLPLAYING: MMRESULT = WAVERR_BASE + 1;
pub const WAVERR_UNPREPARED: MMRESULT = WAVERR_BASE + 2;
pub const WAVERR_SYNC: MMRESULT = WAVERR_BASE + 3;
pub const WAVERR_LASTERROR: MMRESULT = WAVERR_BASE + 3;
DECLARE_HANDLE!(HWAVEIN, HWAVEIN__);
DECLARE_HANDLE!(HWAVEOUT, HWAVEOUT__);
pub type LPHWAVEIN = *mut HWAVEIN;
pub type LPHWAVEOUT = *mut HWAVEOUT;
pub const WOM_OPEN: UINT = MM_WOM_OPEN;
pub const WOM_CLOSE: UINT = MM_WOM_CLOSE;
pub const WOM_DONE: UINT = MM_WOM_DONE;
pub const WIM_OPEN: UINT = MM_WIM_OPEN;
pub const WIM_CLOSE: UINT = MM_WIM_CLOSE;
pub const WIM_DATA: UINT = MM_WIM_DATA;
pub const WAVE_MAPPER: UINT = 0xFFFFFFFF;
pub const WAVE_FORMAT_QUERY: DWORD = 0x0001;
pub const WAVE_ALLOWSYNC: DWORD = 0x0002;
pub const WAVE_MAPPED: DWORD = 0x0004;
pub const WAVE_FORMAT_DIRECT: DWORD = 0x0008;
pub const WAVE_FORMAT_DIRECT_QUERY: DWORD = WAVE_FORMAT_QUERY | WAVE_FORMAT_DIRECT;
pub const WAVE_MAPPED_DEFAULT_COMMUNICATION_DEVICE: DWORD = 0x0010;
STRUCT!{struct WAVEHDR {
    lpData: LPSTR,
    dwBufferLength: DWORD,
    dwBytesRecorded: DWORD,
    dwUser: DWORD_PTR,
    dwFlags: DWORD,
    dwLoops: DWORD,
    lpNext: *mut WAVEHDR,
    reserved: DWORD_PTR,
}}
pub type PWAVEHDR = *mut WAVEHDR;
pub type NPWAVEHDR = *mut WAVEHDR;
pub type LPWAVEHDR = *mut WAVEHDR;
STRUCT!{struct WAVEOUTCAPSW {
    wMid: WORD,
    wPid: WORD,
    vDriverVersion: MMVERSION,
    szPname: [WCHAR; 32],
    dwFormats: DWORD,
    wChannels: WORD,
    wReserved1: WORD,
    dwSupport: DWORD,
}}
pub type PWAVEOUTCAPSW = *mut WAVEOUTCAPSW;
pub type NPWAVEOUTCAPSW = *mut WAVEOUTCAPSW;
pub type LPWAVEOUTCAPSW = *mut WAVEOUTCAPSW;
STRUCT!{struct WAVEINCAPSW {
    wMid: WORD,
    wPid: WORD,
    vDriverVersion: MMVERSION,
    szPname: [WCHAR; 32],
    dwFormats: DWORD,
    wChannels: WORD,
    wReserved1: WORD,
}}
pub type PWAVEINCAPSW = *mut WAVEINCAPSW;
pub type NPWAVEINCAPSW = *mut WAVEINCAPSW;
pub type LPWAVEINCAPSW = *mut WAVEINCAPSW;
pub const WAVE_INVALIDFORMAT: DWORD = 0x00000000;
pub const WAVE_FORMAT_1M08: DWORD = 0x00000001;
pub const WAVE_FORMAT_1S08: DWORD = 0x00000002;
pub const WAVE_FORMAT_1M16: DWORD = 0x00000004;
pub const WAVE_FORMAT_1S16: DWORD = 0x00000008;
pub const WAVE_FORMAT_2M08: DWORD = 0x00000010;
pub const WAVE_FORMAT_2S08: DWORD = 0x00000020;
pub const WAVE_FORMAT_2M16: DWORD = 0x00000040;
pub const WAVE_FORMAT_2S16: DWORD = 0x00000080;
pub const WAVE_FORMAT_4M08: DWORD = 0x00000100;
pub const WAVE_FORMAT_4S08: DWORD = 0x00000200;
pub const WAVE_FORMAT_4M16: DWORD = 0x00000400;
pub const WAVE_FORMAT_4S16: DWORD = 0x00000800;
pub const WAVE_FORMAT_44M08: DWORD = 0x00000100;
pub const WAVE_FORMAT_44S08: DWORD = 0x00000200;
pub const WAVE_FORMAT_44M16: DWORD = 0x00000400;
pub const WAVE_FORMAT_44S16: DWORD = 0x00000800;
pub const WAVE_FORMAT_48M08: DWORD = 0x00001000;
pub const WAVE_FORMAT_48S08: DWORD = 0x00002000;
pub const WAVE_FORMAT_48M16: DWORD = 0x00004000;
pub const WAVE_FORMAT_48S16: DWORD = 0x00008000;
pub const WAVE_FORMAT_96M08: DWORD = 0x00010000;
pub const WAVE_FORMAT_96S08: DWORD = 0x00020000;
pub const WAVE_FORMAT_96M16: DWORD = 0x00040000;
pub const WAVE_FORMAT_96S16: DWORD = 0x00080000;
//782 (Win 7 SDK)
pub type PWAVEFORMATEX = *mut WAVEFORMATEX;
pub type NPWAVEFORMATEX = *mut WAVEFORMATEX;
pub type LPWAVEFORMATEX = *mut WAVEFORMATEX;
pub type LPCWAVEFORMATEX = *const WAVEFORMATEX;
//2170 (Win 7 SDK)
pub const TIMERR_NOERROR: MMRESULT = 0;
pub const TIMERR_NOCANDO: MMRESULT = TIMERR_BASE + 1;
pub const TIMERR_STRUCT: MMRESULT = TIMERR_BASE + 33;
//2198 (Win 7 SDK)
STRUCT!{struct TIMECAPS {
    wPeriodMin: UINT,
    wPeriodMax: UINT,
}}
pub type PTIMECAPS = *mut TIMECAPS;
pub type NPTIMECAPS = *mut TIMECAPS;
pub type LPTIMECAPS = *mut TIMECAPS;
STRUCT!{struct MIDIHDR {
    lpData: LPSTR,
    dwBufferLength: DWORD,
    dwBytesRecorded: DWORD,
    dwUser: DWORD_PTR,
    dwFlags: DWORD,
    lpNext: *mut MIDIHDR,
    reserved: DWORD_PTR,
    dwOffset: DWORD,
    dwReserved: [DWORD_PTR; 4],
}}
pub type PMIDIHDR = *mut MIDIHDR;
pub type NPMIDIHDR = *mut MIDIHDR;
pub type LPMIDIHDR = *mut MIDIHDR;
STRUCT!{struct MIDIINCAPSW {
    wMid: WORD,
    wPid: WORD,
    vDriverVersion: MMVERSION,
    szPname: [WCHAR; 32],
    dwSupport: DWORD,
}}
pub type PMIDIINCAPSW = *mut MIDIINCAPSW;
pub type NPMIDIINCAPSW = *mut MIDIINCAPSW;
pub type LPMIDIINCAPSW = *mut MIDIINCAPSW;
STRUCT!{struct MIDIOUTCAPSW {
    wMid: WORD,
    wPid: WORD,
    vDriverVersion: MMVERSION,
    szPname: [WCHAR; 32],
    wTechnology: WORD,
    wVoices: WORD,
    wNotes: WORD,
    wChannelMask: WORD,
    dwSupport: DWORD,
}}
pub type PMIDIOUTCAPSW = *mut MIDIOUTCAPSW;
pub type NPMIDIOUTCAPSW = *mut MIDIOUTCAPSW;
pub type LPMIDIOUTCAPSW = *mut MIDIOUTCAPSW;
DECLARE_HANDLE!(HMIDIIN, HMIDIIN__);
DECLARE_HANDLE!(HMIDIOUT, HMIDIOUT__);
pub type LPHMIDIIN = *mut HMIDIIN;
pub type LPHMIDIOUT = *mut HMIDIOUT;
DECLARE_HANDLE!(HMIDISTRM, HMIDISTRM__);
DECLARE_HANDLE!(HMIDI, HMIDI__);
pub type LPHMIDISTRM = *mut HMIDISTRM;
pub type LPHMIDI = *mut HMIDI;

pub type MCIDEVICEID = UINT;
pub type MCIERROR = DWORD;
pub type YIELDPROC = DWORD;

pub const MCIERR_NO_ERROR: DWORD = 0;
pub const MCIERR_UNRECOGNIZED_KEYWORD: DWORD = 259;
pub const MCIERR_UNRECOGNIZED_COMMAND: DWORD = 261;
pub const MCIERR_HARDWARE: DWORD = 262;
pub const MCIERR_INVALID_DEVICE_NAME: DWORD = 263;
pub const MCIERR_OUT_OF_MEMORY: DWORD = 264;
pub const MCIERR_DEVICE_OPEN: DWORD = 265;
pub const MCIERR_CANNOT_LOAD_DRIVER: DWORD = 266;
pub const MCIERR_MISSING_COMMAND_STRING: DWORD = 267;
pub const MCIERR_PARAM_OVERFLOW: DWORD = 268;
pub const MCIERR_MISSING_STRING_ARGUMENT: DWORD = 269;
pub const MCIERR_BAD_INTEGER: DWORD = 270;
pub const MCIERR_PARSER_INTERNAL: DWORD = 271;
pub const MCIERR_DRIVER_INTERNAL: DWORD = 272;
pub const MCIERR_MISSING_PARAMETER: DWORD = 273;
pub const MCIERR_UNSUPPORTED_FUNCTION: DWORD = 274;
pub const MCIERR_FILE_NOT_FOUND: DWORD = 275;
pub const MCIERR_DEVICE_NOT_READY: DWORD = 276;
pub const MCIERR_INTERNAL: DWORD = 277;
pub const MCIERR_DRIVER: DWORD = 278;
pub const MCIERR_CANNOT_USE_ALL: DWORD = 279;
pub const MCIERR_MULTIPLE: DWORD = 280;
pub const MCIERR_EXTENSION_NOT_FOUND: DWORD = 281;
pub const MCIERR_OUTOFRANGE: DWORD = 282;
pub const MCIERR_FLAGS_NOT_COMPATIBLE: DWORD = 284;
pub const MCIERR_FILE_NOT_SAVED: DWORD = 286;
pub const MCIERR_DEVICE_TYPE_REQUIRED: DWORD = 287;
pub const MCIERR_DEVICE_LOCKED: DWORD = 288;
pub const MCIERR_DUPLICATE_ALIAS: DWORD = 289;
pub const MCIERR_BAD_CONSTANT: DWORD = 290;
pub const MCIERR_MUST_USE_SHAREABLE: DWORD = 291;
pub const MCIERR_MISSING_DEVICE_NAME: DWORD = 292;
pub const MCIERR_BAD_TIME_FORMAT: DWORD = 293;
pub const MCIERR_NO_CLOSING_QUOTE: DWORD = 294;
pub const MCIERR_DUPLICATE_FLAGS: DWORD = 295;
pub const MCIERR_INVALID_FILE: DWORD = 296;
pub const MCIERR_NULL_PARAMETER_BLOCK: DWORD = 297;
pub const MCIERR_UNNAMED_RESOURCE: DWORD = 298;
pub const MCIERR_NEW_REQUIRES_ALIAS: DWORD = 299;
pub const MCIERR_NOTIFY_ON_AUTO_OPEN: DWORD = 300;
pub const MCIERR_NO_ELEMENT_ALLOWED: DWORD = 301;
pub const MCIERR_NONAPPLICABLE_FUNCTION: DWORD = 302;
pub const MCIERR_ILLEGAL_FOR_AUTO_OPEN: DWORD = 303;
pub const MCIERR_FILENAME_REQUIRED: DWORD = 304;
pub const MCIERR_EXTRA_CHARACTERS: DWORD = 305;
pub const MCIERR_DEVICE_NOT_INSTALLED: DWORD = 306;
pub const MCIERR_GET_CD: DWORD = 307;
pub const MCIERR_SET_CD: DWORD = 308;
pub const MCIERR_SET_DRIVE: DWORD = 309;
pub const MCIERR_DEVICE_LENGTH: DWORD = 310;
pub const MCIERR_DEVICE_ORD_LENGTH: DWORD = 311;
pub const MCIERR_NO_INTEGER: DWORD = 312;
pub const MCIERR_WAVE_OUTPUTSINUSE: DWORD = 320;
pub const MCIERR_WAVE_SETOUTPUTINUSE: DWORD = 321;
pub const MCIERR_WAVE_INPUTSINUSE: DWORD = 322;
pub const MCIERR_WAVE_SETINPUTINUSE: DWORD = 323;
pub const MCIERR_WAVE_OUTPUTUNSPECIFIED: DWORD = 324;
pub const MCIERR_WAVE_INPUTUNSPECIFIED: DWORD = 325;
pub const MCIERR_WAVE_OUTPUTSUNSUITABLE: DWORD = 326;
pub const MCIERR_WAVE_SETOUTPUTUNSUITABLE: DWORD = 327;
pub const MCIERR_WAVE_INPUTSUNSUITABLE: DWORD = 328;
pub const MCIERR_WAVE_SETINPUTUNSUITABLE: DWORD = 329;
pub const MCIERR_SEQ_DIV_INCOMPATIBLE: DWORD = 336;
pub const MCIERR_SEQ_PORT_INUSE: DWORD = 337;
pub const MCIERR_SEQ_PORT_NONEXISTENT: DWORD = 338;
pub const MCIERR_SEQ_PORT_MAPNODEVICE: DWORD = 339;
pub const MCIERR_SEQ_PORT_MISCERROR: DWORD = 340;
pub const MCIERR_SEQ_TIMER: DWORD = 341;
pub const MCIERR_SEQ_PORTUNSPECIFIED: DWORD = 342;
pub const MCIERR_SEQ_NOMIDIPRESENT: DWORD = 343;
pub const MCIERR_NO_WINDOW: DWORD = 346;
pub const MCIERR_CREATEWINDOW: DWORD = 347;
pub const MCIERR_FILE_READ: DWORD = 348;
pub const MCIERR_FILE_WRITE: DWORD = 349;
pub const MCIERR_NO_IDENTITY: DWORD = 350;
pub const MCIERR_CUSTOM_DRIVER_BASE: DWORD = 512;

extern "system" {
    pub fn PlaySoundA(
        pszSound: LPCSTR,
        hmod: HMODULE,
        fdwSound: DWORD
    ) -> BOOL;
    pub fn PlaySoundW(
        pszSound: LPCWSTR,
        hmod: HMODULE,
        fdwSound: DWORD
    ) -> BOOL;
    pub fn sndPlaySoundA(
        pszSound: LPCSTR,
        fuSound: UINT
    ) -> BOOL;
    pub fn sndPlaySoundW(
        pszSound: LPCWSTR,
        fuSound: UINT
    ) -> BOOL;
    pub fn timeBeginPeriod(
        uPeriod: UINT
    ) -> MMRESULT;
    pub fn timeEndPeriod(
        uPeriod: UINT
    ) -> MMRESULT;
    pub fn timeGetDevCaps(
        ptc: LPTIMECAPS,
        cbtc: UINT
    ) -> MMRESULT;
    pub fn timeGetTime(
    ) -> DWORD;
    pub fn waveInAddBuffer(
        hwi: HWAVEIN,
        pwh: LPWAVEHDR,
        cbwh: UINT
    ) -> MMRESULT;
    pub fn waveInClose(
        hwi: HWAVEIN
    ) -> MMRESULT;
    // pub fn waveInGetDevCapsA();
    pub fn waveInGetDevCapsW(
        uDeviceID: UINT_PTR,
        pwic: LPWAVEINCAPSW,
        cbwic: UINT
    ) -> MMRESULT;
    // pub fn waveInGetErrorTextA();
    pub fn waveInGetErrorTextW(
        mmrError: MMRESULT,
        pszText: LPWSTR,
        cchText: UINT
    ) -> MMRESULT;
    // pub fn waveInGetID();
    pub fn waveInGetNumDevs(
    ) -> UINT;
    pub fn waveInGetPosition(
        hwi: HWAVEIN,
        pmmt: LPMMTIME,
        cbmmt: UINT
    ) -> MMRESULT;
    pub fn waveInMessage(
        hwi: HWAVEIN,
        uMsg: UINT,
        dw1: DWORD_PTR,
        dw2: DWORD_PTR
    ) -> MMRESULT;
    pub fn waveInOpen(
        phwi: LPHWAVEIN,
        uDeviceID: UINT,
        pwfx: LPCWAVEFORMATEX,
        dwCallback: DWORD_PTR,
        dwInstance: DWORD_PTR,
        fdwOpen: DWORD,
    ) -> MMRESULT;
    pub fn waveInPrepareHeader(
        hwi: HWAVEIN,
        pwh: LPWAVEHDR,
        cbwh: UINT
    ) -> MMRESULT;
    pub fn waveInReset(
        hwi: HWAVEIN
    ) -> MMRESULT;
    pub fn waveInStart(
        hwi: HWAVEIN
    ) -> MMRESULT;
    pub fn waveInStop(
        hwi: HWAVEIN
    ) -> MMRESULT;
    pub fn waveInUnprepareHeader(
        hwi: HWAVEIN,
        pwh: LPWAVEHDR,
        cbwh: UINT
    ) -> MMRESULT;
    pub fn waveOutBreakLoop(
        hwo: HWAVEOUT
    ) -> MMRESULT;
    pub fn waveOutClose(
        hwo: HWAVEOUT
    ) -> MMRESULT;
    // pub fn waveOutGetDevCapsA();
    pub fn waveOutGetDevCapsW(
        uDeviceID: UINT_PTR,
        pwoc: LPWAVEOUTCAPSW,
        cbwoc: UINT
    ) -> MMRESULT;
    // pub fn waveOutGetErrorTextA();
    pub fn waveOutGetErrorTextW(
        mmrError: MMRESULT,
        pszText: LPWSTR,
        cchText: UINT
    ) -> MMRESULT;
    // pub fn waveOutGetID();
    pub fn waveOutGetNumDevs(
    ) -> UINT;
    pub fn waveOutGetPitch(
        hwo: HWAVEOUT,
        pdwPitch: LPDWORD
    ) -> MMRESULT;
    pub fn waveOutGetPlaybackRate(
        hwo: HWAVEOUT,
        pdwRate: LPDWORD
    ) -> MMRESULT;
    pub fn waveOutGetPosition(
        hwo: HWAVEOUT,
        pmmt: LPMMTIME,
        cbmmt: UINT
    ) -> MMRESULT;
    pub fn waveOutGetVolume(
        hwo: HWAVEOUT,
        pdwVolume: LPDWORD
    ) -> MMRESULT;
    pub fn waveOutMessage(
        hwo: HWAVEOUT,
        uMsg: UINT,
        dw1: DWORD_PTR,
        dw2: DWORD_PTR
    ) -> MMRESULT;
    pub fn waveOutOpen(
        phwo: LPHWAVEOUT,
        uDeviceID: UINT,
        pwfx: LPCWAVEFORMATEX,
        dwCallback: DWORD_PTR,
        dwInstance: DWORD_PTR,
        fdwOpen: DWORD,
    ) -> MMRESULT;
    pub fn waveOutPause(
        hwo: HWAVEOUT
    ) -> MMRESULT;
    pub fn waveOutPrepareHeader(
        hwo: HWAVEOUT,
        pwh: LPWAVEHDR,
        cbwh: UINT
    ) -> MMRESULT;
    pub fn waveOutReset(
        hwo: HWAVEOUT
    ) -> MMRESULT;
    pub fn waveOutRestart(
        hwo: HWAVEOUT
    ) -> MMRESULT;
    pub fn waveOutSetPitch(
        hwo: HWAVEOUT,
        dwPitch: DWORD
    ) -> MMRESULT;
    pub fn waveOutSetPlaybackRate(
        hwo: HWAVEOUT,
        dwRate: DWORD
    ) -> MMRESULT;
    pub fn waveOutSetVolume(
        hwo: HWAVEOUT,
        dwVolume: DWORD
    ) -> MMRESULT;
    pub fn waveOutUnprepareHeader(
        hwo: HWAVEOUT,
        pwh: LPWAVEHDR,
        cbwh: UINT
    ) -> MMRESULT;
    pub fn waveOutWrite(
        hwo: HWAVEOUT,
        pwh: LPWAVEHDR,
        cbwh: UINT
    ) -> MMRESULT;
    pub fn midiStreamOpen(
        lphStream: LPHMIDISTRM,
        puDeviceID: LPUINT,
        cMidi: DWORD,
        dwCallback: DWORD_PTR,
        dwInstance: DWORD_PTR,
        fdwOpen: DWORD
    ) -> MMRESULT;
    pub fn midiStreamClose(
        hStream: HMIDISTRM
    ) -> MMRESULT;
    pub fn midiStreamProperty(
        hm: HMIDISTRM,
        lppropdata: LPBYTE,
        dwProperty: DWORD
    ) -> MMRESULT;
    pub fn midiStreamPosition(
        hms: HMIDISTRM,
        pmmt: LPMMTIME,
        cbmmt: UINT
    ) -> MMRESULT;
    pub fn midiStreamOut(
        hMidiStream: HMIDISTRM,
        lpMidiHdr: LPMIDIHDR,
        cbMidiHdr: UINT
    ) -> MMRESULT;
    pub fn midiStreamPause(
        hms: HMIDISTRM
    ) -> MMRESULT;
    pub fn midiStreamRestart(
        hms: HMIDISTRM
    ) -> MMRESULT;
    pub fn midiStreamStop(
        hms: HMIDISTRM
    ) -> MMRESULT;
    pub fn midiConnect(
        hMidi: HMIDI,
        hmo: HMIDIOUT,
        pReserved: PVOID
    ) -> MMRESULT;
    pub fn midiDisconnect(
        hMidi: HMIDI,
        hmo: HMIDIOUT,
        pReserved: PVOID
    ) -> MMRESULT;
    pub fn midiOutGetNumDevs(
    ) -> UINT;
    pub fn midiOutGetDevCapsW(
        uDeviceID: UINT_PTR,
        lpMidiOutCaps: LPMIDIOUTCAPSW,
        cbMidiOutCaps: UINT
    ) -> MMRESULT;
    pub fn midiOutGetVolume(
        hmo: HMIDIOUT,
        lpdwVolume: PDWORD
    ) -> MMRESULT;
    pub fn midiOutSetVolume(
        hmo: HMIDIOUT,
        dwVolume: DWORD
    ) -> MMRESULT;
    pub fn midiOutGetErrorTextW(
        mmrError: MMRESULT,
        lpText: LPWSTR,
        cchText: UINT
    ) -> MMRESULT;
    pub fn midiOutOpen(
        lphmo: LPHMIDIOUT,
        uDeviceID: UINT,
        dwCallback: DWORD_PTR,
        dwCallbackInstance: DWORD_PTR,
        dwFlags: DWORD
    ) -> MMRESULT;
    pub fn midiOutClose(
        hmo: HMIDIOUT
    ) -> MMRESULT;
    pub fn midiOutPrepareHeader(
        hmo: HMIDIOUT,
        lpMidiOutHdr: LPMIDIHDR,
        cbMidiOutHdr: UINT
    ) -> MMRESULT;
    pub fn midiOutUnprepareHeader(
        hmo: HMIDIOUT,
        lpMidiOutHdr: LPMIDIHDR,
        cbMidiOutHdr: UINT
    ) -> MMRESULT;
    pub fn midiOutShortMsg(
        hmo: HMIDIOUT,
        dwMsg: DWORD
    ) -> MMRESULT;
    pub fn midiOutLongMsg(
        hmo: HMIDIOUT,
        lpMidiOutHdr: LPMIDIHDR,
        cbMidiOutHdr: UINT
    ) -> MMRESULT;
    pub fn midiOutReset(
        hmo: HMIDIOUT
    ) -> MMRESULT;
    pub fn midiOutCachePatches(
        hmo: HMIDIOUT,
        wBank: UINT,
        lpPatchArray: LPWORD,
        wFlags: UINT
    ) -> MMRESULT;
    pub fn midiOutCacheDrumPatches(
        hmo: HMIDIOUT,
        wPatch: UINT,
        lpKeyArray: LPWORD,
        wFlags: UINT
    ) -> MMRESULT;
    pub fn midiOutGetID(
        hmo: HMIDIOUT,
        puDeviceID: LPUINT
    ) -> MMRESULT;
    pub fn midiOutMessage(
        deviceID: HMIDIOUT,
        msg: UINT,
        dw1: DWORD_PTR,
        dw2: DWORD_PTR
    ) -> MMRESULT;
    pub fn midiInGetNumDevs(
    ) -> UINT;
    pub fn midiInGetDevCapsW(
        uDeviceID: UINT_PTR,
        lpMidiInCaps: LPMIDIINCAPSW,
        cbMidiInCaps: UINT
    ) -> MMRESULT;
    pub fn midiInGetErrorTextW(
        wError: MMRESULT,
        lpText: LPWSTR,
        cchText: UINT
    ) -> MMRESULT;
    pub fn midiInOpen(
        lphMidiIn: LPHMIDIIN,
        uDeviceID: UINT,
        dwCallback: DWORD_PTR,
        dwCallbackInstance: DWORD_PTR,
        dwFlags: DWORD
    ) -> MMRESULT;
    pub fn midiInClose(
        hMidiIn: HMIDIIN
    ) -> MMRESULT;
    pub fn midiInPrepareHeader(
        hMidiIn: HMIDIIN,
        lpMidiInHdr: LPMIDIHDR,
        cbMidiInHdr: UINT
    ) -> MMRESULT;
    pub fn midiInUnprepareHeader(
        hMidiIn: HMIDIIN,
        lpMidiInHdr: LPMIDIHDR,
        cbMidiInHdr: UINT
    ) -> MMRESULT;
    pub fn midiInAddBuffer(
        hMidiIn: HMIDIIN,
        lpMidiInHdr: LPMIDIHDR,
        cbMidiInHdr: UINT
    ) -> MMRESULT;
    pub fn midiInStart(
        hMidiIn: HMIDIIN
    ) -> MMRESULT;
    pub fn midiInStop(
        hMidiIn: HMIDIIN
    ) -> MMRESULT;
    pub fn midiInReset(
        hMidiIn: HMIDIIN
    ) -> MMRESULT;
    pub fn midiInGetID(
        hmi: HMIDIIN,
        puDeviceID: LPUINT
    ) -> MMRESULT;
    pub fn midiInMessage(
        deviceID: HMIDIIN,
        msg: UINT,
        dw1: DWORD_PTR,
        dw2: DWORD_PTR
    ) -> MMRESULT;
    pub fn mciExecute(
        pszCommand: LPCSTR
    ) -> BOOL;
    pub fn mciGetCreatorTask(
        IDDevice: MCIDEVICEID
    ) -> MMRESULT;
    pub fn mciGetDeviceID(
        lpszDevice: LPCTSTR
    ) -> MCIDEVICEID;
    pub fn mciGetDeviceIDFromElementID(
        dwElementID: DWORD,
        lpstrType: LPCTSTR
    ) -> MCIDEVICEID;
    pub fn mciGetErrorString(
        fdwError: DWORD,
        lpszErrorText: LPTSTR,
        cchErrorText: UINT
    ) -> BOOL;
    pub fn mciGetYieldProc(
        IDDevice: MCIDEVICEID,
        lpdwYieldData: LPDWORD
    ) -> YIELDPROC;
    pub fn mciSendCommand(
        IDDevice: MCIDEVICEID,
        uMsg: UINT,
        fdwCommand: DWORD_PTR,
        dwParam: DWORD_PTR
    ) -> MCIERROR;
    pub fn mciSendString(
        lpszCommand: LPCTSTR,
        lpszReturnString: LPTSTR,
        cchReturn: UINT,
        hwndCallback: HANDLE
    ) -> MCIERROR;
    pub fn mciSetYieldProc(
        IDDevice: MCIDEVICEID,
        yp: YIELDPROC,
        dwYieldData: DWORD
    ) -> UINT;
}
