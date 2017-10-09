// Copyright © 2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::{DWORD_PTR, UINT_PTR};
use shared::guiddef::GUID;
use shared::minwindef::{BYTE, DWORD, LPBYTE, LPDWORD, LPVOID, LPWORD, UINT, WORD};
use shared::windef::HWND;
use um::mmsyscom::{
    LPDRVCALLBACK, LPMMTIME, LPUINT, MAXPNAMELEN, MIDIERR_BASE, MIXERR_BASE, MMRESULT, MMVERSION,
    MM_MIM_CLOSE, MM_MIM_DATA, MM_MIM_ERROR, MM_MIM_LONGDATA, MM_MIM_LONGERROR, MM_MIM_MOREDATA,
    MM_MIM_OPEN, MM_MOM_CLOSE, MM_MOM_DONE, MM_MOM_OPEN, MM_MOM_POSITIONCB, MM_WIM_CLOSE,
    MM_WIM_DATA, MM_WIM_OPEN, MM_WOM_CLOSE, MM_WOM_DONE, MM_WOM_OPEN, WAVERR_BASE,
};
use um::winnt::{CHAR, LONG, LPSTR, LPWSTR, WCHAR};
pub const WAVERR_BADFORMAT: MMRESULT = WAVERR_BASE + 0;
pub const WAVERR_STILLPLAYING: MMRESULT = WAVERR_BASE + 1;
pub const WAVERR_UNPREPARED: MMRESULT = WAVERR_BASE + 2;
pub const WAVERR_SYNC: MMRESULT = WAVERR_BASE + 3;
pub const WAVERR_LASTERROR: MMRESULT = WAVERR_BASE + 3;
DECLARE_HANDLE!(HWAVE, HWAVE__);
DECLARE_HANDLE!(HWAVEIN, HWAVEIN__);
DECLARE_HANDLE!(HWAVEOUT, HWAVEOUT__);
pub type LPHWAVEIN = *mut HWAVEIN;
pub type LPHWAVEOUT = *mut HWAVEOUT;
pub type LPWAVECALLBACK = LPDRVCALLBACK;
pub const WOM_OPEN: UINT = MM_WOM_OPEN;
pub const WOM_CLOSE: UINT = MM_WOM_CLOSE;
pub const WOM_DONE: UINT = MM_WOM_DONE;
pub const WIM_OPEN: UINT = MM_WIM_OPEN;
pub const WIM_CLOSE: UINT = MM_WIM_CLOSE;
pub const WIM_DATA: UINT = MM_WIM_DATA;
pub const WAVE_MAPPER: UINT = -1i32 as u32;
pub const WAVE_FORMAT_QUERY: DWORD = 0x0001;
pub const WAVE_ALLOWSYNC: DWORD = 0x0002;
pub const WAVE_MAPPED: DWORD = 0x0004;
pub const WAVE_FORMAT_DIRECT: DWORD = 0x0008;
pub const WAVE_FORMAT_DIRECT_QUERY: DWORD = WAVE_FORMAT_QUERY | WAVE_FORMAT_DIRECT;
pub const WAVE_MAPPED_DEFAULT_COMMUNICATION_DEVICE: DWORD = 0x0010;
STRUCT!{#[repr(packed)] struct WAVEHDR {
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
pub const WHDR_DONE: DWORD = 0x00000001;
pub const WHDR_PREPARED: DWORD = 0x00000002;
pub const WHDR_BEGINLOOP: DWORD = 0x00000004;
pub const WHDR_ENDLOOP: DWORD = 0x00000008;
pub const WHDR_INQUEUE: DWORD = 0x00000010;
STRUCT!{#[repr(packed)] struct WAVEOUTCAPSA {
    wMid: WORD,
    wPid: WORD,
    vDriverVersion: MMVERSION,
    szPname: [CHAR; MAXPNAMELEN],
    dwFormats: DWORD,
    wChannels: WORD,
    wReserved1: WORD,
    dwSupport: DWORD,
}}
pub type PWAVEOUTCAPSA = *mut WAVEOUTCAPSA;
pub type NPWAVEOUTCAPSA = *mut WAVEOUTCAPSA;
pub type LPWAVEOUTCAPSA = *mut WAVEOUTCAPSA;
STRUCT!{#[repr(packed)] struct WAVEOUTCAPSW {
    wMid: WORD,
    wPid: WORD,
    vDriverVersion: MMVERSION,
    szPname: [WCHAR; MAXPNAMELEN],
    dwFormats: DWORD,
    wChannels: WORD,
    wReserved1: WORD,
    dwSupport: DWORD,
}}
pub type PWAVEOUTCAPSW = *mut WAVEOUTCAPSW;
pub type NPWAVEOUTCAPSW = *mut WAVEOUTCAPSW;
pub type LPWAVEOUTCAPSW = *mut WAVEOUTCAPSW;
STRUCT!{#[repr(packed)] struct WAVEOUTCAPS2A {
    wMid: WORD,
    wPid: WORD,
    vDriverVersion: MMVERSION,
    szPname: [CHAR; MAXPNAMELEN],
    dwFormats: DWORD,
    wChannels: WORD,
    wReserved1: WORD,
    dwSupport: DWORD,
    ManufacturerGuid: GUID,
    ProductGuid: GUID,
    NameGuid: GUID,
}}
pub type PWAVEOUTCAPS2A = *mut WAVEOUTCAPS2A;
pub type NPWAVEOUTCAPS2A = *mut WAVEOUTCAPS2A;
pub type LPWAVEOUTCAPS2A = *mut WAVEOUTCAPS2A;
STRUCT!{#[repr(packed)] struct WAVEOUTCAPS2W {
    wMid: WORD,
    wPid: WORD,
    vDriverVersion: MMVERSION,
    szPname: [WCHAR; MAXPNAMELEN],
    dwFormats: DWORD,
    wChannels: WORD,
    wReserved1: WORD,
    dwSupport: DWORD,
    ManufacturerGuid: GUID,
    ProductGuid: GUID,
    NameGuid: GUID,
}}
pub type PWAVEOUTCAPS2W = *mut WAVEOUTCAPS2W;
pub type NPWAVEOUTCAPS2W = *mut WAVEOUTCAPS2W;
pub type LPWAVEOUTCAPS2W = *mut WAVEOUTCAPS2W;
pub const WAVECAPS_PITCH: DWORD = 0x0001;
pub const WAVECAPS_PLAYBACKRATE: DWORD = 0x0002;
pub const WAVECAPS_VOLUME: DWORD = 0x0004;
pub const WAVECAPS_LRVOLUME: DWORD = 0x0008;
pub const WAVECAPS_SYNC: DWORD = 0x0010;
pub const WAVECAPS_SAMPLEACCURATE: DWORD = 0x0020;
STRUCT!{#[repr(packed)] struct WAVEINCAPSA {
    wMid: WORD,
    wPid: WORD,
    vDriverVersion: MMVERSION,
    szPname: [CHAR; MAXPNAMELEN],
    dwFormats: DWORD,
    wChannels: WORD,
    wReserved1: WORD,
}}
pub type PWAVEINCAPSA = *mut WAVEINCAPSA;
pub type NPWAVEINCAPSA = *mut WAVEINCAPSA;
pub type LPWAVEINCAPSA = *mut WAVEINCAPSA;
STRUCT!{#[repr(packed)] struct WAVEINCAPSW {
    wMid: WORD,
    wPid: WORD,
    vDriverVersion: MMVERSION,
    szPname: [WCHAR; MAXPNAMELEN],
    dwFormats: DWORD,
    wChannels: WORD,
    wReserved1: WORD,
}}
pub type PWAVEINCAPSW = *mut WAVEINCAPSW;
pub type NPWAVEINCAPSW = *mut WAVEINCAPSW;
pub type LPWAVEINCAPSW = *mut WAVEINCAPSW;
STRUCT!{#[repr(packed)] struct WAVEINCAPS2A {
    wMid: WORD,
    wPid: WORD,
    vDriverVersion: MMVERSION,
    szPname: [CHAR; MAXPNAMELEN],
    dwFormats: DWORD,
    wChannels: WORD,
    wReserved1: WORD,
    ManufacturerGuid: GUID,
    ProductGuid: GUID,
    NameGuid: GUID,
}}
pub type PWAVEINCAPS2A = *mut WAVEINCAPS2A;
pub type NPWAVEINCAPS2A = *mut WAVEINCAPS2A;
pub type LPWAVEINCAPS2A = *mut WAVEINCAPS2A;
STRUCT!{#[repr(packed)] struct WAVEINCAPS2W {
    wMid: WORD,
    wPid: WORD,
    vDriverVersion: MMVERSION,
    szPname: [WCHAR; MAXPNAMELEN],
    dwFormats: DWORD,
    wChannels: WORD,
    wReserved1: WORD,
    ManufacturerGuid: GUID,
    ProductGuid: GUID,
    NameGuid: GUID,
}}
pub type PWAVEINCAPS2W = *mut WAVEINCAPS2W;
pub type NPWAVEINCAPS2W = *mut WAVEINCAPS2W;
pub type LPWAVEINCAPS2W = *mut WAVEINCAPS2W;
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
STRUCT!{#[repr(packed)] struct WAVEFORMAT {
    wFormatTag: WORD,
    nChannels: WORD,
    nSamplesPerSec: DWORD,
    nAvgBytesPerSec: DWORD,
    nBlockAlign: WORD,
}}
pub type PWAVEFORMAT = *mut WAVEFORMAT;
pub type NPWAVEFORMAT = *mut WAVEFORMAT;
pub type LPWAVEFORMAT = *mut WAVEFORMAT;
pub const WAVE_FORMAT_PCM: WORD = 1;
STRUCT!{#[repr(packed)] struct PCMWAVEFORMAT {
    wf: WAVEFORMAT,
    wBitsPerSample: WORD,
}}
pub type PPCMWAVEFORMAT = *mut PCMWAVEFORMAT;
pub type NPPCMWAVEFORMAT = *mut PCMWAVEFORMAT;
pub type LPPCMWAVEFORMAT = *mut PCMWAVEFORMAT;
STRUCT!{#[repr(packed)] struct WAVEFORMATEX {
    wFormatTag: WORD,
    nChannels: WORD,
    nSamplesPerSec: DWORD,
    nAvgBytesPerSec: DWORD,
    nBlockAlign: WORD,
    wBitsPerSample: WORD,
    cbSize: WORD,
}}
pub type PWAVEFORMATEX = *mut WAVEFORMATEX;
pub type NPWAVEFORMATEX = *mut WAVEFORMATEX;
pub type LPWAVEFORMATEX = *mut WAVEFORMATEX;
pub type LPCWAVEFORMATEX = *const WAVEFORMATEX;
extern "system" {
    pub fn waveOutGetNumDevs() -> UINT;
    pub fn waveOutGetDevCapsA(
        uDeviceID: UINT_PTR,
        pwoc: LPWAVEOUTCAPSA,
        cbwoc: UINT,
    ) -> MMRESULT;
    pub fn waveOutGetDevCapsW(
        uDeviceID: UINT_PTR,
        pwoc: LPWAVEOUTCAPSW,
        cbwoc: UINT,
    ) -> MMRESULT;
    pub fn waveOutGetVolume(
        hwo: HWAVEOUT,
        pdwVolume: LPDWORD,
    ) -> MMRESULT;
    pub fn waveOutSetVolume(
        hwo: HWAVEOUT,
        dwVolume: DWORD,
    ) -> MMRESULT;
    pub fn waveOutGetErrorTextA(
        mmrError: MMRESULT,
        pszText: LPSTR,
        cchText: UINT,
    ) -> MMRESULT;
    pub fn waveOutGetErrorTextW(
        mmrError: MMRESULT,
        pszText: LPWSTR,
        cchText: UINT,
    ) -> MMRESULT;
    pub fn waveOutOpen(
        phwo: LPHWAVEOUT,
        uDeviceID: UINT,
        pwfx: LPCWAVEFORMATEX,
        dwCallback: DWORD_PTR,
        dwInstance: DWORD_PTR,
        fdwOpen: DWORD,
    ) -> MMRESULT;
    pub fn waveOutClose(
        hwo: HWAVEOUT,
    ) -> MMRESULT;
    pub fn waveOutPrepareHeader(
        hwo: HWAVEOUT,
        pwh: LPWAVEHDR,
        cbwh: UINT,
    ) -> MMRESULT;
    pub fn waveOutUnprepareHeader(
        hwo: HWAVEOUT,
        pwh: LPWAVEHDR,
        cbwh: UINT,
    ) -> MMRESULT;
    pub fn waveOutWrite(
        hwo: HWAVEOUT,
        pwh: LPWAVEHDR,
        cbwh: UINT,
    ) -> MMRESULT;
    pub fn waveOutPause(
        hwo: HWAVEOUT,
    ) -> MMRESULT;
    pub fn waveOutRestart(
        hwo: HWAVEOUT,
    ) -> MMRESULT;
    pub fn waveOutReset(
        hwo: HWAVEOUT,
    ) -> MMRESULT;
    pub fn waveOutBreakLoop(
        hwo: HWAVEOUT,
    ) -> MMRESULT;
    pub fn waveOutGetPosition(
        hwo: HWAVEOUT,
        pmmt: LPMMTIME,
        cbmmt: UINT,
    ) -> MMRESULT;
    pub fn waveOutGetPitch(
        hwo: HWAVEOUT,
        pdwPitch: LPDWORD,
    ) -> MMRESULT;
    pub fn waveOutSetPitch(
        hwo: HWAVEOUT,
        dwPitch: DWORD,
    ) -> MMRESULT;
    pub fn waveOutGetPlaybackRate(
        hwo: HWAVEOUT,
        pdwRate: LPDWORD,
    ) -> MMRESULT;
    pub fn waveOutSetPlaybackRate(
        hwo: HWAVEOUT,
        dwRate: DWORD,
    ) -> MMRESULT;
    pub fn waveOutGetID(
        hwo: HWAVEOUT,
        puDeviceID: LPUINT,
    ) -> MMRESULT;
    pub fn waveOutMessage(
        hwo: HWAVEOUT,
        uMsg: UINT,
        dw1: DWORD_PTR,
        dw2: DWORD_PTR,
    ) -> MMRESULT;
    pub fn waveInGetNumDevs() -> UINT;
    pub fn waveInGetDevCapsA(
        uDeviceID: UINT_PTR,
        pwic: LPWAVEINCAPSA,
        cbwic: UINT,
    ) -> MMRESULT;
    pub fn waveInGetDevCapsW(
        uDeviceID: UINT_PTR,
        pwic: LPWAVEINCAPSW,
        cbwic: UINT,
    ) -> MMRESULT;
    pub fn waveInGetErrorTextA(
        mmrError: MMRESULT,
        pszText: LPSTR,
        cchText: UINT,
    ) -> MMRESULT;
    pub fn waveInGetErrorTextW(
        mmrError: MMRESULT,
        pszText: LPWSTR,
        cchText: UINT,
    ) -> MMRESULT;
    pub fn waveInOpen(
        phwi: LPHWAVEIN,
        uDeviceID: UINT,
        pwfx: LPCWAVEFORMATEX,
        dwCallback: DWORD_PTR,
        dwInstance: DWORD_PTR,
        fdwOpen: DWORD,
    ) -> MMRESULT;
    pub fn waveInClose(
        hwi: HWAVEIN,
    ) -> MMRESULT;
    pub fn waveInPrepareHeader(
        hwi: HWAVEIN,
        pwh: LPWAVEHDR,
        cbwh: UINT,
    ) -> MMRESULT;
    pub fn waveInUnprepareHeader(
        hwi: HWAVEIN,
        pwh: LPWAVEHDR,
        cbwh: UINT,
    ) -> MMRESULT;
    pub fn waveInAddBuffer(
        hwi: HWAVEIN,
        pwh: LPWAVEHDR,
        cbwh: UINT,
    ) -> MMRESULT;
    pub fn waveInStart(
        hwi: HWAVEIN,
    ) -> MMRESULT;
    pub fn waveInStop(
        hwi: HWAVEIN,
    ) -> MMRESULT;
    pub fn waveInReset(
        hwi: HWAVEIN,
    ) -> MMRESULT;
    pub fn waveInGetPosition(
        hwi: HWAVEIN,
        pmmt: LPMMTIME,
        cbmmt: UINT,
    ) -> MMRESULT;
    pub fn waveInGetID(
        hwi: HWAVEIN,
        puDeviceID: LPUINT,
    ) -> MMRESULT;
    pub fn waveInMessage(
        hwi: HWAVEIN,
        uMsg: UINT,
        dw1: DWORD_PTR,
        dw2: DWORD_PTR,
    ) -> MMRESULT;
}
pub const MIDIERR_UNPREPARED: MMRESULT = MIDIERR_BASE + 0;
pub const MIDIERR_STILLPLAYING: MMRESULT = MIDIERR_BASE + 1;
pub const MIDIERR_NOMAP: MMRESULT = MIDIERR_BASE + 2;
pub const MIDIERR_NOTREADY: MMRESULT = MIDIERR_BASE + 3;
pub const MIDIERR_NODEVICE: MMRESULT = MIDIERR_BASE + 4;
pub const MIDIERR_INVALIDSETUP: MMRESULT = MIDIERR_BASE + 5;
pub const MIDIERR_BADOPENMODE: MMRESULT = MIDIERR_BASE + 6;
pub const MIDIERR_DONT_CONTINUE: MMRESULT = MIDIERR_BASE + 7;
pub const MIDIERR_LASTERROR: MMRESULT = MIDIERR_BASE + 7;
DECLARE_HANDLE!(HMIDI, HMIDI__);
DECLARE_HANDLE!(HMIDIIN, HMIDIIN__);
DECLARE_HANDLE!(HMIDIOUT, HMIDIOUT__);
DECLARE_HANDLE!(HMIDISTRM, HMIDISTRM__);
pub type LPHMIDI = *mut HMIDI;
pub type LPHMIDIIN = *mut HMIDIIN;
pub type LPHMIDIOUT = *mut HMIDIOUT;
pub type LPHMIDISTRM = *mut HMIDISTRM;
pub type LPMIDICALLBACK = LPDRVCALLBACK;
pub const MIDIPATCHSIZE: usize = 128;
pub type PATCHARRAY = [WORD; MIDIPATCHSIZE];
pub type LPPATCHARRAY = *mut WORD;
pub type KEYARRAY = [WORD; MIDIPATCHSIZE];
pub type LPKEYARRAY = *mut WORD;
pub const MIM_OPEN: UINT = MM_MIM_OPEN;
pub const MIM_CLOSE: UINT = MM_MIM_CLOSE;
pub const MIM_DATA: UINT = MM_MIM_DATA;
pub const MIM_LONGDATA: UINT = MM_MIM_LONGDATA;
pub const MIM_ERROR: UINT = MM_MIM_ERROR;
pub const MIM_LONGERROR: UINT = MM_MIM_LONGERROR;
pub const MOM_OPEN: UINT = MM_MOM_OPEN;
pub const MOM_CLOSE: UINT = MM_MOM_CLOSE;
pub const MOM_DONE: UINT = MM_MOM_DONE;
pub const MIM_MOREDATA: UINT = MM_MIM_MOREDATA;
pub const MOM_POSITIONCB: UINT = MM_MOM_POSITIONCB;
pub const MIDIMAPPER: UINT = -1i32 as u32;
pub const MIDI_MAPPER: UINT = -1i32 as u32;
pub const MIDI_IO_STATUS: DWORD = 0x00000020;
pub const MIDI_CACHE_ALL: UINT = 1;
pub const MIDI_CACHE_BESTFIT: UINT = 2;
pub const MIDI_CACHE_QUERY: UINT = 3;
pub const MIDI_UNCACHE: UINT = 4;
STRUCT!{#[repr(packed)] struct MIDIOUTCAPSA {
    wMid: WORD,
    wPid: WORD,
    vDriverVersion: MMVERSION,
    szPname: [CHAR; MAXPNAMELEN],
    wTechnology: WORD,
    wVoices: WORD,
    wNotes: WORD,
    wChannelMask: WORD,
    dwSupport: DWORD,
}}
pub type PMIDIOUTCAPSA = *mut MIDIOUTCAPSA;
pub type NPMIDIOUTCAPSA = *mut MIDIOUTCAPSA;
pub type LPMIDIOUTCAPSA = *mut MIDIOUTCAPSA;
STRUCT!{#[repr(packed)] struct MIDIOUTCAPSW {
    wMid: WORD,
    wPid: WORD,
    vDriverVersion: MMVERSION,
    szPname: [WCHAR; MAXPNAMELEN],
    wTechnology: WORD,
    wVoices: WORD,
    wNotes: WORD,
    wChannelMask: WORD,
    dwSupport: DWORD,
}}
pub type PMIDIOUTCAPSW = *mut MIDIOUTCAPSW;
pub type NPMIDIOUTCAPSW = *mut MIDIOUTCAPSW;
pub type LPMIDIOUTCAPSW = *mut MIDIOUTCAPSW;
STRUCT!{#[repr(packed)] struct MIDIOUTCAPS2A {
    wMid: WORD,
    wPid: WORD,
    vDriverVersion: MMVERSION,
    szPname: [CHAR; MAXPNAMELEN],
    wTechnology: WORD,
    wVoices: WORD,
    wNotes: WORD,
    wChannelMask: WORD,
    dwSupport: DWORD,
    ManufacturerGuid: GUID,
    ProductGuid: GUID,
    NameGuid: GUID,
}}
pub type PMIDIOUTCAPS2A = *mut MIDIOUTCAPS2A;
pub type NPMIDIOUTCAPS2A = *mut MIDIOUTCAPS2A;
pub type LPMIDIOUTCAPS2A = *mut MIDIOUTCAPS2A;
STRUCT!{#[repr(packed)] struct MIDIOUTCAPS2W {
    wMid: WORD,
    wPid: WORD,
    vDriverVersion: MMVERSION,
    szPname: [WCHAR; MAXPNAMELEN],
    wTechnology: WORD,
    wVoices: WORD,
    wNotes: WORD,
    wChannelMask: WORD,
    dwSupport: DWORD,
    ManufacturerGuid: GUID,
    ProductGuid: GUID,
    NameGuid: GUID,
}}
pub type PMIDIOUTCAPS2W = *mut MIDIOUTCAPS2W;
pub type NPMIDIOUTCAPS2W = *mut MIDIOUTCAPS2W;
pub type LPMIDIOUTCAPS2W = *mut MIDIOUTCAPS2W;
pub const MOD_MIDIPORT: WORD = 1;
pub const MOD_SYNTH: WORD = 2;
pub const MOD_SQSYNTH: WORD = 3;
pub const MOD_FMSYNTH: WORD = 4;
pub const MOD_MAPPER: WORD = 5;
pub const MOD_WAVETABLE: WORD = 6;
pub const MOD_SWSYNTH: WORD = 7;
pub const MIDICAPS_VOLUME: DWORD = 0x0001;
pub const MIDICAPS_LRVOLUME: DWORD = 0x0002;
pub const MIDICAPS_CACHE: DWORD = 0x0004;
pub const MIDICAPS_STREAM: DWORD = 0x0008;
STRUCT!{#[repr(packed)] struct MIDIINCAPSA {
    wMid: WORD,
    wPid: WORD,
    vDriverVersion: MMVERSION,
    szPname: [CHAR; MAXPNAMELEN],
    dwSupport: DWORD,
}}
pub type PMIDIINCAPSA = *mut MIDIINCAPSA;
pub type NPMIDIINCAPSA = *mut MIDIINCAPSA;
pub type LPMIDIINCAPSA = *mut MIDIINCAPSA;
STRUCT!{#[repr(packed)] struct MIDIINCAPSW {
    wMid: WORD,
    wPid: WORD,
    vDriverVersion: MMVERSION,
    szPname: [WCHAR; MAXPNAMELEN],
    dwSupport: DWORD,
}}
pub type PMIDIINCAPSW = *mut MIDIINCAPSW;
pub type NPMIDIINCAPSW = *mut MIDIINCAPSW;
pub type LPMIDIINCAPSW = *mut MIDIINCAPSW;
STRUCT!{#[repr(packed)] struct MIDIINCAPS2A {
    wMid: WORD,
    wPid: WORD,
    vDriverVersion: MMVERSION,
    szPname: [CHAR; MAXPNAMELEN],
    dwSupport: DWORD,
    ManufacturerGuid: GUID,
    ProductGuid: GUID,
    NameGuid: GUID,
}}
pub type PMIDIINCAPS2A = *mut MIDIINCAPS2A;
pub type NPMIDIINCAPS2A = *mut MIDIINCAPS2A;
pub type LPMIDIINCAPS2A = *mut MIDIINCAPS2A;
STRUCT!{#[repr(packed)] struct MIDIINCAPS2W {
    wMid: WORD,
    wPid: WORD,
    vDriverVersion: MMVERSION,
    szPname: [WCHAR; MAXPNAMELEN],
    dwSupport: DWORD,
    ManufacturerGuid: GUID,
    ProductGuid: GUID,
    NameGuid: GUID,
}}
pub type PMIDIINCAPS2W = *mut MIDIINCAPS2W;
pub type NPMIDIINCAPS2W = *mut MIDIINCAPS2W;
pub type LPMIDIINCAPS2W = *mut MIDIINCAPS2W;
STRUCT!{#[repr(packed)] struct MIDIHDR {
    lpData: LPSTR,
    dwBufferLength: DWORD,
    dwBytesRecorded: DWORD,
    dwUser: DWORD_PTR,
    dwFlags: DWORD,
    lpNext: *mut MIDIHDR,
    reserved: DWORD_PTR,
    dwOffset: DWORD,
    dwReserved: [DWORD_PTR; 8],
}}
pub type PMIDIHDR = *mut MIDIHDR;
pub type NPMIDIHDR = *mut MIDIHDR;
pub type LPMIDIHDR = *mut MIDIHDR;
STRUCT!{#[repr(packed)] struct MIDIEVENT {
    dwDeltaTime: DWORD,
    dwStreamID: DWORD,
    dwEvent: DWORD,
    dwParms: [DWORD; 1],
}}
STRUCT!{#[repr(packed)] struct MIDISTRMBUFFVER {
    dwVersion: DWORD,
    dwMid: DWORD,
    dwOEMVersion: DWORD,
}}
pub const MHDR_DONE: DWORD = 0x00000001;
pub const MHDR_PREPARED: DWORD = 0x00000002;
pub const MHDR_INQUEUE: DWORD = 0x00000004;
pub const MHDR_ISSTRM: DWORD = 0x00000008;
pub const MEVT_F_SHORT: DWORD = 0x00000000;
pub const MEVT_F_LONG: DWORD = 0x80000000;
pub const MEVT_F_CALLBACK: DWORD = 0x40000000;
#[inline]
pub fn MEVT_EVENTTYPE(x: DWORD) -> BYTE {
    ((x >> 24) & 0xff) as BYTE
}
#[inline]
pub fn MEVT_EVENTPARM(x: DWORD) -> DWORD {
    x & 0x00FFFFFF
}
pub const MEVT_SHORTMSG: BYTE = 0x00;
pub const MEVT_TEMPO: BYTE = 0x01;
pub const MEVT_NOP: BYTE = 0x02;
pub const MEVT_LONGMSG: BYTE = 0x80;
pub const MEVT_COMMENT: BYTE = 0x82;
pub const MEVT_VERSION: BYTE = 0x84;
pub const MIDISTRM_ERROR: MMRESULT = -2i32 as u32;
pub const MIDIPROP_SET: DWORD = 0x80000000;
pub const MIDIPROP_GET: DWORD = 0x40000000;
pub const MIDIPROP_TIMEDIV: DWORD = 0x00000001;
pub const MIDIPROP_TEMPO: DWORD = 0x00000002;
STRUCT!{#[repr(packed)] struct MIDIPROPTIMEDIV {
    cbStruct: DWORD,
    dwTimeDiv: DWORD,
}}
pub type LPMIDIPROPTIMEDIV = *mut MIDIPROPTIMEDIV;
STRUCT!{#[repr(packed)] struct MIDIPROPTEMPO {
    cbStruct: DWORD,
    dwTempo: DWORD,
}}
pub type LPMIDIPROPTEMPO = *mut MIDIPROPTEMPO;
extern "system" {
    pub fn midiOutGetNumDevs() -> UINT;
    pub fn midiStreamOpen(
        lphStream: LPHMIDISTRM,
        puDeviceID: LPUINT,
        cMidi: DWORD,
        dwCallback: DWORD_PTR,
        dwInstance: DWORD_PTR,
        fdwOpen: DWORD,
    ) -> MMRESULT;
    pub fn midiStreamClose(
        hStream: HMIDISTRM,
    ) -> MMRESULT;
    pub fn midiStreamProperty(
        hm: HMIDISTRM,
        lppropdata: LPBYTE,
        dwProperty: DWORD,
    ) -> MMRESULT;
    pub fn midiStreamPosition(
        hms: HMIDISTRM,
        pmmt: LPMMTIME,
        cbmmt: UINT,
    ) -> MMRESULT;
    pub fn midiStreamOut(
        hMidiStream: HMIDISTRM,
        lpMidiHdr: LPMIDIHDR,
        cbMidiHdr: UINT,
    ) -> MMRESULT;
    pub fn midiStreamPause(
        hms: HMIDISTRM,
    ) -> MMRESULT;
    pub fn midiStreamRestart(
        hms: HMIDISTRM,
    ) -> MMRESULT;
    pub fn midiStreamStop(
        hms: HMIDISTRM,
    ) -> MMRESULT;
    pub fn midiConnect(
        hMidi: HMIDI,
        hmo: HMIDIOUT,
        pReserved: LPVOID,
    ) -> MMRESULT;
    pub fn midiDisconnect(
        hMidi: HMIDI,
        hmo: HMIDIOUT,
        pReserved: LPVOID,
    ) -> MMRESULT;
    pub fn midiOutGetDevCapsA(
        uDeviceID: UINT_PTR,
        lpMidiOutCaps: LPMIDIOUTCAPSA,
        cbMidiOutCaps: UINT,
    ) -> MMRESULT;
    pub fn midiOutGetDevCapsW(
        uDeviceID: UINT_PTR,
        lpMidiOutCaps: LPMIDIOUTCAPSW,
        cbMidiOutCaps: UINT,
    ) -> MMRESULT;
    pub fn midiOutGetVolume(
        hmo: HMIDIOUT,
        lpdwVolume: LPDWORD,
    ) -> MMRESULT;
    pub fn midiOutSetVolume(
        hmo: HMIDIOUT,
        dwVolume: DWORD,
    ) -> MMRESULT;
    pub fn midiOutGetErrorTextA(
        mmrError: MMRESULT,
        lpText: LPSTR,
        cchText: UINT,
    ) -> MMRESULT;
    pub fn midiOutGetErrorTextW(
        mmrError: MMRESULT,
        lpText: LPWSTR,
        cchText: UINT,
    ) -> MMRESULT;
    pub fn midiOutOpen(
        lphmo: LPHMIDIOUT,
        uDeviceID: UINT,
        dwCallback: DWORD_PTR,
        dwCallbackInstance: DWORD_PTR,
        dwFlags: DWORD,
    ) -> MMRESULT;
    pub fn midiOutClose(
        hmo: HMIDIOUT,
    ) -> MMRESULT;
    pub fn midiOutPrepareHeader(
        hmo: HMIDIOUT,
        lpMidiOutHdr: LPMIDIHDR,
        cbMidiOutHdr: UINT,
    ) -> MMRESULT;
    pub fn midiOutUnprepareHeader(
        hmo: HMIDIOUT,
        lpMidiOutHdr: LPMIDIHDR,
        cbMidiOutHdr: UINT,
    ) -> MMRESULT;
    pub fn midiOutShortMsg(
        hmo: HMIDIOUT,
        dwMsg: DWORD,
    ) -> MMRESULT;
    pub fn midiOutLongMsg(
        hmo: HMIDIOUT,
        lpMidiOutHdr: LPMIDIHDR,
        cbmh: UINT,
    ) -> MMRESULT;
    pub fn midiOutReset(
        hmo: HMIDIOUT,
    ) -> MMRESULT;
    pub fn midiOutCachePatches(
        hmo: HMIDIOUT,
        uBank: UINT,
        pwpa: LPWORD,
        fuCache: UINT,
    ) -> MMRESULT;
    pub fn midiOutCacheDrumPatches(
        hmo: HMIDIOUT,
        uPatch: UINT,
        pwkya: LPWORD,
        fuCache: UINT,
    ) -> MMRESULT;
    pub fn midiOutGetID(
        hmo: HMIDIOUT,
        puDeviceID: LPUINT,
    ) -> MMRESULT;
    pub fn midiOutMessage(
        hmo: HMIDIOUT,
        uMsg: UINT,
        dw1: DWORD_PTR,
        dw2: DWORD_PTR,
    ) -> MMRESULT;
    pub fn midiInGetNumDevs() -> UINT;
    pub fn midiInGetDevCapsA(
        uDeviceID: UINT_PTR,
        pmic: LPMIDIINCAPSA,
        cbmic: UINT,
    ) -> MMRESULT;
    pub fn midiInGetDevCapsW(
        uDeviceID: UINT_PTR,
        pmic: LPMIDIINCAPSW,
        cbmic: UINT,
    ) -> MMRESULT;
    pub fn midiInGetErrorTextA(
        mmrError: MMRESULT,
        pszText: LPSTR,
        cchText: UINT,
    ) -> MMRESULT;
    pub fn midiInGetErrorTextW(
        mmrError: MMRESULT,
        pszText: LPWSTR,
        cchText: UINT,
    ) -> MMRESULT;
    pub fn midiInOpen(
        phmi: LPHMIDIIN,
        uDeviceID: UINT,
        dwCallback: DWORD_PTR,
        dwInstance: DWORD_PTR,
        fdwOpen: DWORD,
    ) -> MMRESULT;
    pub fn midiInClose(
        hmi: HMIDIIN,
    ) -> MMRESULT;
    pub fn midiInPrepareHeader(
        hmi: HMIDIIN,
        pmh: LPMIDIHDR,
        cbmh: UINT,
    ) -> MMRESULT;
    pub fn midiInUnprepareHeader(
        hmi: HMIDIIN,
        pmh: LPMIDIHDR,
        cbmh: UINT,
    ) -> MMRESULT;
    pub fn midiInAddBuffer(
        hmi: HMIDIIN,
        pmh: LPMIDIHDR,
        cbmh: UINT,
    ) -> MMRESULT;
    pub fn midiInStart(
        hmi: HMIDIIN,
    ) -> MMRESULT;
    pub fn midiInStop(
        hmi: HMIDIIN,
    ) -> MMRESULT;
    pub fn midiInReset(
        hmi: HMIDIIN,
    ) -> MMRESULT;
    pub fn midiInGetID(
        hmi: HMIDIIN,
        puDeviceID: LPUINT,
    ) -> MMRESULT;
    pub fn midiInMessage(
        hmi: HMIDIIN,
        uMsg: UINT,
        dw1: DWORD_PTR,
        dw2: DWORD_PTR,
    ) -> MMRESULT;
}
pub const AUX_MAPPER: UINT = -1i32 as u32;
STRUCT!{#[repr(packed)] struct AUXCAPSA {
    wMid: WORD,
    wPid: WORD,
    vDriverVersion: MMVERSION,
    szPname: [CHAR; MAXPNAMELEN],
    wTechnology: WORD,
    wReserved1: WORD,
    dwSupport: DWORD,
}}
pub type PAUXCAPSA = *mut AUXCAPSA;
pub type NPAUXCAPSA = *mut AUXCAPSA;
pub type LPAUXCAPSA = *mut AUXCAPSA;
STRUCT!{#[repr(packed)] struct AUXCAPSW {
    wMid: WORD,
    wPid: WORD,
    vDriverVersion: MMVERSION,
    szPname: [WCHAR; MAXPNAMELEN],
    wTechnology: WORD,
    wReserved1: WORD,
    dwSupport: DWORD,
}}
pub type PAUXCAPSW = *mut AUXCAPSW;
pub type NPAUXCAPSW = *mut AUXCAPSW;
pub type LPAUXCAPSW = *mut AUXCAPSW;
STRUCT!{#[repr(packed)] struct AUXCAPS2A {
    wMid: WORD,
    wPid: WORD,
    vDriverVersion: MMVERSION,
    szPname: [CHAR; MAXPNAMELEN],
    wTechnology: WORD,
    wReserved1: WORD,
    dwSupport: DWORD,
    ManufacturerGuid: GUID,
    ProductGuid: GUID,
    NameGuid: GUID,
}}
pub type PAUXCAPS2A = *mut AUXCAPS2A;
pub type NPAUXCAPS2A = *mut AUXCAPS2A;
pub type LPAUXCAPS2A = *mut AUXCAPS2A;
STRUCT!{#[repr(packed)] struct AUXCAPS2W {
    wMid: WORD,
    wPid: WORD,
    vDriverVersion: MMVERSION,
    szPname: [WCHAR; MAXPNAMELEN],
    wTechnology: WORD,
    wReserved1: WORD,
    dwSupport: DWORD,
    ManufacturerGuid: GUID,
    ProductGuid: GUID,
    NameGuid: GUID,
}}
pub type PAUXCAPS2W = *mut AUXCAPS2W;
pub type NPAUXCAPS2W = *mut AUXCAPS2W;
pub type LPAUXCAPS2W = *mut AUXCAPS2W;
pub const AUXCAPS_CDAUDIO: WORD = 1;
pub const AUXCAPS_AUXIN: WORD = 2;
pub const AUXCAPS_VOLUME: DWORD = 0x0001;
pub const AUXCAPS_LRVOLUME: DWORD = 0x0002;
extern "system" {
    pub fn auxGetNumDevs() -> UINT;
    pub fn auxGetDevCapsA(
        uDeviceID: UINT_PTR,
        pac: LPAUXCAPSA,
        cbac: UINT,
    ) -> MMRESULT;
    pub fn auxGetDevCapsW(
        uDeviceID: UINT_PTR,
        pac: LPAUXCAPSW,
        cbac: UINT,
    ) -> MMRESULT;
    pub fn auxSetVolume(
        uDeviceID: UINT,
        dwVolume: DWORD,
    ) -> MMRESULT;
    pub fn auxGetVolume(
        uDeviceID: UINT,
        pdwVolume: LPDWORD,
    ) -> MMRESULT;
    pub fn auxOutMessage(
        uDeviceID: UINT,
        uMsg: UINT,
        dw1: DWORD_PTR,
        dw2: DWORD_PTR,
    ) -> MMRESULT;
}
DECLARE_HANDLE!(HMIXEROBJ, HMIXEROBJ__);
pub type LPHMIXEROBJ = *mut HMIXEROBJ;
DECLARE_HANDLE!(HMIXER, HMIXER__);
pub type LPHMIXER = *mut HMIXER;
pub const MIXER_SHORT_NAME_CHARS: usize = 16;
pub const MIXER_LONG_NAME_CHARS: usize = 64;
pub const MIXERR_INVALLINE: MMRESULT = MIXERR_BASE + 0;
pub const MIXERR_INVALCONTROL: MMRESULT = MIXERR_BASE + 1;
pub const MIXERR_INVALVALUE: MMRESULT = MIXERR_BASE + 2;
pub const MIXERR_LASTERROR: MMRESULT = MIXERR_BASE + 2;
pub const MIXER_OBJECTF_HANDLE: DWORD = 0x80000000;
pub const MIXER_OBJECTF_MIXER: DWORD = 0x00000000;
pub const MIXER_OBJECTF_HMIXER: DWORD = MIXER_OBJECTF_HANDLE | MIXER_OBJECTF_MIXER;
pub const MIXER_OBJECTF_WAVEOUT: DWORD = 0x10000000;
pub const MIXER_OBJECTF_HWAVEOUT: DWORD = MIXER_OBJECTF_HANDLE | MIXER_OBJECTF_WAVEOUT;
pub const MIXER_OBJECTF_WAVEIN: DWORD = 0x20000000;
pub const MIXER_OBJECTF_HWAVEIN: DWORD = MIXER_OBJECTF_HANDLE | MIXER_OBJECTF_WAVEIN;
pub const MIXER_OBJECTF_MIDIOUT: DWORD = 0x30000000;
pub const MIXER_OBJECTF_HMIDIOUT: DWORD = MIXER_OBJECTF_HANDLE | MIXER_OBJECTF_MIDIOUT;
pub const MIXER_OBJECTF_MIDIIN: DWORD = 0x40000000;
pub const MIXER_OBJECTF_HMIDIIN: DWORD = MIXER_OBJECTF_HANDLE | MIXER_OBJECTF_MIDIIN;
pub const MIXER_OBJECTF_AUX: DWORD = 0x50000000;
extern "system" {
    pub fn mixerGetNumDevs() -> UINT;
}
STRUCT!{#[repr(packed)] struct MIXERCAPSA {
    wMid: WORD,
    wPid: WORD,
    vDriverVersion: MMVERSION,
    szPname: [CHAR; MAXPNAMELEN],
    fdwSupport: DWORD,
    cDestinations: DWORD,
}}
pub type PMIXERCAPSA = *mut MIXERCAPSA;
pub type LPMIXERCAPSA = *mut MIXERCAPSA;
STRUCT!{#[repr(packed)] struct MIXERCAPSW {
    wMid: WORD,
    wPid: WORD,
    vDriverVersion: MMVERSION,
    szPname: [WCHAR; MAXPNAMELEN],
    fdwSupport: DWORD,
    cDestinations: DWORD,
}}
pub type PMIXERCAPSW = *mut MIXERCAPSW;
pub type LPMIXERCAPSW = *mut MIXERCAPSW;
STRUCT!{#[repr(packed)] struct MIXERCAPS2A {
    wMid: WORD,
    wPid: WORD,
    vDriverVersion: MMVERSION,
    szPname: [CHAR; MAXPNAMELEN],
    fdwSupport: DWORD,
    cDestinations: DWORD,
    ManufacturerGuid: GUID,
    ProductGuid: GUID,
    NameGuid: GUID,
}}
pub type PMIXERCAPS2A = *mut MIXERCAPS2A;
pub type LPMIXERCAPS2A = *mut MIXERCAPS2A;
STRUCT!{#[repr(packed)] struct MIXERCAPS2W {
    wMid: WORD,
    wPid: WORD,
    vDriverVersion: MMVERSION,
    szPname: [WCHAR; MAXPNAMELEN],
    fdwSupport: DWORD,
    cDestinations: DWORD,
    ManufacturerGuid: GUID,
    ProductGuid: GUID,
    NameGuid: GUID,
}}
pub type PMIXERCAPS2W = *mut MIXERCAPS2W;
pub type LPMIXERCAPS2W = *mut MIXERCAPS2W;
extern "system" {
    pub fn mixerGetDevCapsA(
        uMxId: UINT_PTR,
        pmxcaps: LPMIXERCAPSA,
        cbmxcaps: UINT,
    ) -> MMRESULT;
    pub fn mixerGetDevCapsW(
        uMxId: UINT_PTR,
        pmxcaps: LPMIXERCAPSW,
        cbmxcaps: UINT,
    ) -> MMRESULT;
    pub fn mixerOpen(
        phmx: LPHMIXER,
        uMxId: UINT,
        dwCallback: DWORD_PTR,
        dwInstance: DWORD_PTR,
        fdwOpen: DWORD,
    ) -> MMRESULT;
    pub fn mixerClose(
        hmx: HMIXER,
    ) -> MMRESULT;
    pub fn mixerMessage(
        hmx: HMIXER,
        uMsg: UINT,
        dwParam1: DWORD_PTR,
        dwParam2: DWORD_PTR,
    ) -> DWORD;
}
STRUCT!{#[repr(packed)] struct MIXERLINEA_Target {
    dwType: DWORD,
    dwDeviceID: DWORD,
    wMid: WORD,
    wPid: WORD,
    vDriverVersion: MMVERSION,
    szPname: [CHAR; MAXPNAMELEN],
}}
STRUCT!{#[repr(packed)] struct MIXERLINEA {
    cbStruct: DWORD,
    dwDestination: DWORD,
    dwSource: DWORD,
    dwLineID: DWORD,
    fdwLine: DWORD,
    dwUser: DWORD_PTR,
    dwComponentType: DWORD,
    cChannels: DWORD,
    cConnections: DWORD,
    cControls: DWORD,
    szShortName: [CHAR; MIXER_SHORT_NAME_CHARS],
    szName: [CHAR; MIXER_LONG_NAME_CHARS],
    Target: MIXERLINEA_Target,
}}
pub type PMIXERLINEA = *mut MIXERLINEA;
pub type LPMIXERLINEA = *mut MIXERLINEA;
STRUCT!{#[repr(packed)] struct MIXERLINEW_Target {
    dwType: DWORD,
    dwDeviceID: DWORD,
    wMid: WORD,
    wPid: WORD,
    vDriverVersion: MMVERSION,
    szPname: [WCHAR; MAXPNAMELEN],
}}
STRUCT!{#[repr(packed)] struct MIXERLINEW {
    cbStruct: DWORD,
    dwDestination: DWORD,
    dwSource: DWORD,
    dwLineID: DWORD,
    fdwLine: DWORD,
    dwUser: DWORD_PTR,
    dwComponentType: DWORD,
    cChannels: DWORD,
    cConnections: DWORD,
    cControls: DWORD,
    szShortName: [WCHAR; MIXER_SHORT_NAME_CHARS],
    szName: [WCHAR; MIXER_LONG_NAME_CHARS],
    Target: MIXERLINEW_Target,
}}
pub type PMIXERLINEW = *mut MIXERLINEW;
pub type LPMIXERLINEW = *mut MIXERLINEW;
pub const MIXERLINE_LINEF_ACTIVE: DWORD = 0x00000001;
pub const MIXERLINE_LINEF_DISCONNECTED: DWORD = 0x00008000;
pub const MIXERLINE_LINEF_SOURCE: DWORD = 0x80000000;
pub const MIXERLINE_COMPONENTTYPE_DST_FIRST: DWORD = 0x00000000;
pub const MIXERLINE_COMPONENTTYPE_DST_UNDEFINED: DWORD = MIXERLINE_COMPONENTTYPE_DST_FIRST + 0;
pub const MIXERLINE_COMPONENTTYPE_DST_DIGITAL: DWORD = MIXERLINE_COMPONENTTYPE_DST_FIRST + 1;
pub const MIXERLINE_COMPONENTTYPE_DST_LINE: DWORD = MIXERLINE_COMPONENTTYPE_DST_FIRST + 2;
pub const MIXERLINE_COMPONENTTYPE_DST_MONITOR: DWORD = MIXERLINE_COMPONENTTYPE_DST_FIRST + 3;
pub const MIXERLINE_COMPONENTTYPE_DST_SPEAKERS: DWORD = MIXERLINE_COMPONENTTYPE_DST_FIRST + 4;
pub const MIXERLINE_COMPONENTTYPE_DST_HEADPHONES: DWORD = MIXERLINE_COMPONENTTYPE_DST_FIRST + 5;
pub const MIXERLINE_COMPONENTTYPE_DST_TELEPHONE: DWORD = MIXERLINE_COMPONENTTYPE_DST_FIRST + 6;
pub const MIXERLINE_COMPONENTTYPE_DST_WAVEIN: DWORD = MIXERLINE_COMPONENTTYPE_DST_FIRST + 7;
pub const MIXERLINE_COMPONENTTYPE_DST_VOICEIN: DWORD = MIXERLINE_COMPONENTTYPE_DST_FIRST + 8;
pub const MIXERLINE_COMPONENTTYPE_DST_LAST: DWORD = MIXERLINE_COMPONENTTYPE_DST_FIRST + 8;
pub const MIXERLINE_COMPONENTTYPE_SRC_FIRST: DWORD = 0x00001000;
pub const MIXERLINE_COMPONENTTYPE_SRC_UNDEFINED: DWORD = MIXERLINE_COMPONENTTYPE_SRC_FIRST + 0;
pub const MIXERLINE_COMPONENTTYPE_SRC_DIGITAL: DWORD = MIXERLINE_COMPONENTTYPE_SRC_FIRST + 1;
pub const MIXERLINE_COMPONENTTYPE_SRC_LINE: DWORD = MIXERLINE_COMPONENTTYPE_SRC_FIRST + 2;
pub const MIXERLINE_COMPONENTTYPE_SRC_MICROPHONE: DWORD = MIXERLINE_COMPONENTTYPE_SRC_FIRST + 3;
pub const MIXERLINE_COMPONENTTYPE_SRC_SYNTHESIZER: DWORD = MIXERLINE_COMPONENTTYPE_SRC_FIRST + 4;
pub const MIXERLINE_COMPONENTTYPE_SRC_COMPACTDISC: DWORD = MIXERLINE_COMPONENTTYPE_SRC_FIRST + 5;
pub const MIXERLINE_COMPONENTTYPE_SRC_TELEPHONE: DWORD = MIXERLINE_COMPONENTTYPE_SRC_FIRST + 6;
pub const MIXERLINE_COMPONENTTYPE_SRC_PCSPEAKER: DWORD = MIXERLINE_COMPONENTTYPE_SRC_FIRST + 7;
pub const MIXERLINE_COMPONENTTYPE_SRC_WAVEOUT: DWORD = MIXERLINE_COMPONENTTYPE_SRC_FIRST + 8;
pub const MIXERLINE_COMPONENTTYPE_SRC_AUXILIARY: DWORD = MIXERLINE_COMPONENTTYPE_SRC_FIRST + 9;
pub const MIXERLINE_COMPONENTTYPE_SRC_ANALOG: DWORD = MIXERLINE_COMPONENTTYPE_SRC_FIRST + 10;
pub const MIXERLINE_COMPONENTTYPE_SRC_LAST: DWORD = MIXERLINE_COMPONENTTYPE_SRC_FIRST + 10;
pub const MIXERLINE_TARGETTYPE_UNDEFINED: DWORD = 0;
pub const MIXERLINE_TARGETTYPE_WAVEOUT: DWORD = 1;
pub const MIXERLINE_TARGETTYPE_WAVEIN: DWORD = 2;
pub const MIXERLINE_TARGETTYPE_MIDIOUT: DWORD = 3;
pub const MIXERLINE_TARGETTYPE_MIDIIN: DWORD = 4;
pub const MIXERLINE_TARGETTYPE_AUX: DWORD = 5;
extern "system" {
    pub fn mixerGetLineInfoA(
        hmxobj: HMIXEROBJ,
        pmxl: LPMIXERLINEA,
        fdwInfo: DWORD,
    ) -> MMRESULT;
    pub fn mixerGetLineInfoW(
        hmxobj: HMIXEROBJ,
        pmxl: LPMIXERLINEW,
        fdwInfo: DWORD,
    ) -> MMRESULT;
}
pub const MIXER_GETLINEINFOF_DESTINATION: DWORD = 0x00000000;
pub const MIXER_GETLINEINFOF_SOURCE: DWORD = 0x00000001;
pub const MIXER_GETLINEINFOF_LINEID: DWORD = 0x00000002;
pub const MIXER_GETLINEINFOF_COMPONENTTYPE: DWORD = 0x00000003;
pub const MIXER_GETLINEINFOF_TARGETTYPE: DWORD = 0x00000004;
pub const MIXER_GETLINEINFOF_QUERYMASK: DWORD = 0x0000000F;
extern "system" {
    pub fn mixerGetID(
        hmxobj: HMIXEROBJ,
        puMxId: *mut UINT,
        fdwId: DWORD,
    ) -> MMRESULT;
}
STRUCT!{#[repr(packed)] struct MIXERCONTROLA_Bounds_s1 {
    lMinimum: LONG,
    lMaximum: LONG,
}}
STRUCT!{#[repr(packed)] struct MIXERCONTROLA_Bounds_s2 {
    dwMinimum: DWORD,
    dwMaximum: DWORD,
}}
UNION2!{union MIXERCONTROLA_Bounds {
    [u8; 24],
    s1 s1_mut: MIXERCONTROLA_Bounds_s1,
    s2 s2_mut: MIXERCONTROLA_Bounds_s2,
    dwReserved dwReserved_mut: [DWORD; 6],
}}
UNION2!{union MIXERCONTROLA_Metrics {
    [u8; 24],
    cSteps cSteps_mut: DWORD,
    cbCustomData cbCustomData_mut: DWORD,
    dwReserved dwReserved_mut: [DWORD; 6],
}}
STRUCT!{#[repr(packed)] struct MIXERCONTROLA {
    cbStruct: DWORD,
    dwControlID: DWORD,
    dwControlType: DWORD,
    fdwControl: DWORD,
    cMultipleItems: DWORD,
    szShortName: [CHAR; MIXER_SHORT_NAME_CHARS],
    szName: [CHAR; MIXER_LONG_NAME_CHARS],
    Bounds: MIXERCONTROLA_Bounds,
    Metrics: MIXERCONTROLA_Metrics,
}}
pub type PMIXERCONTROLA = *mut MIXERCONTROLA;
pub type LPMIXERCONTROLA = *mut MIXERCONTROLA;
STRUCT!{#[repr(packed)] struct MIXERCONTROLW_Bounds_s1 {
    lMinimum: LONG,
    lMaximum: LONG,
}}
STRUCT!{#[repr(packed)] struct MIXERCONTROLW_Bounds_s2 {
    dwMinimum: DWORD,
    dwMaximum: DWORD,
}}
UNION2!{union MIXERCONTROLW_Bounds {
    [u8; 24],
    s1 s1_mut: MIXERCONTROLA_Bounds_s1,
    s2 s2_mut: MIXERCONTROLA_Bounds_s2,
    dwReserved dwReserved_mut: [DWORD; 6],
}}
UNION2!{union MIXERCONTROLW_Metrics {
    [u8; 24],
    cSteps cSteps_mut: DWORD,
    cbCustomData cbCustomData_mut: DWORD,
    dwReserved dwReserved_mut: [DWORD; 6],
}}
STRUCT!{#[repr(packed)] struct MIXERCONTROLW {
    cbStruct: DWORD,
    dwControlID: DWORD,
    dwControlType: DWORD,
    fdwControl: DWORD,
    cMultipleItems: DWORD,
    szShortName: [WCHAR; MIXER_SHORT_NAME_CHARS],
    szName: [WCHAR; MIXER_LONG_NAME_CHARS],
    Bounds: MIXERCONTROLW_Bounds,
    Metrics: MIXERCONTROLW_Metrics,
}}
pub type PMIXERCONTROLW = *mut MIXERCONTROLW;
pub type LPMIXERCONTROLW = *mut MIXERCONTROLW;
pub const MIXERCONTROL_CONTROLF_UNIFORM: DWORD = 0x00000001;
pub const MIXERCONTROL_CONTROLF_MULTIPLE: DWORD = 0x00000002;
pub const MIXERCONTROL_CONTROLF_DISABLED: DWORD = 0x80000000;
pub const MIXERCONTROL_CT_CLASS_MASK: DWORD = 0xF0000000;
pub const MIXERCONTROL_CT_CLASS_CUSTOM: DWORD = 0x00000000;
pub const MIXERCONTROL_CT_CLASS_METER: DWORD = 0x10000000;
pub const MIXERCONTROL_CT_CLASS_SWITCH: DWORD = 0x20000000;
pub const MIXERCONTROL_CT_CLASS_NUMBER: DWORD = 0x30000000;
pub const MIXERCONTROL_CT_CLASS_SLIDER: DWORD = 0x40000000;
pub const MIXERCONTROL_CT_CLASS_FADER: DWORD = 0x50000000;
pub const MIXERCONTROL_CT_CLASS_TIME: DWORD = 0x60000000;
pub const MIXERCONTROL_CT_CLASS_LIST: DWORD = 0x70000000;
pub const MIXERCONTROL_CT_SUBCLASS_MASK: DWORD = 0x0F000000;
pub const MIXERCONTROL_CT_SC_SWITCH_BOOLEAN: DWORD = 0x00000000;
pub const MIXERCONTROL_CT_SC_SWITCH_BUTTON: DWORD = 0x01000000;
pub const MIXERCONTROL_CT_SC_METER_POLLED: DWORD = 0x00000000;
pub const MIXERCONTROL_CT_SC_TIME_MICROSECS: DWORD = 0x00000000;
pub const MIXERCONTROL_CT_SC_TIME_MILLISECS: DWORD = 0x01000000;
pub const MIXERCONTROL_CT_SC_LIST_SINGLE: DWORD = 0x00000000;
pub const MIXERCONTROL_CT_SC_LIST_MULTIPLE: DWORD = 0x01000000;
pub const MIXERCONTROL_CT_UNITS_MASK: DWORD = 0x00FF0000;
pub const MIXERCONTROL_CT_UNITS_CUSTOM: DWORD = 0x00000000;
pub const MIXERCONTROL_CT_UNITS_BOOLEAN: DWORD = 0x00010000;
pub const MIXERCONTROL_CT_UNITS_SIGNED: DWORD = 0x00020000;
pub const MIXERCONTROL_CT_UNITS_UNSIGNED: DWORD = 0x00030000;
pub const MIXERCONTROL_CT_UNITS_DECIBELS: DWORD = 0x00040000;
pub const MIXERCONTROL_CT_UNITS_PERCENT: DWORD = 0x00050000;
pub const MIXERCONTROL_CONTROLTYPE_CUSTOM: DWORD = MIXERCONTROL_CT_CLASS_CUSTOM |
    MIXERCONTROL_CT_UNITS_CUSTOM;
pub const MIXERCONTROL_CONTROLTYPE_BOOLEANMETER: DWORD = MIXERCONTROL_CT_CLASS_METER |
    MIXERCONTROL_CT_SC_METER_POLLED | MIXERCONTROL_CT_UNITS_BOOLEAN;
pub const MIXERCONTROL_CONTROLTYPE_SIGNEDMETER: DWORD = MIXERCONTROL_CT_CLASS_METER |
    MIXERCONTROL_CT_SC_METER_POLLED | MIXERCONTROL_CT_UNITS_SIGNED;
pub const MIXERCONTROL_CONTROLTYPE_PEAKMETER: DWORD = MIXERCONTROL_CONTROLTYPE_SIGNEDMETER + 1;
pub const MIXERCONTROL_CONTROLTYPE_UNSIGNEDMETER: DWORD = MIXERCONTROL_CT_CLASS_METER |
    MIXERCONTROL_CT_SC_METER_POLLED | MIXERCONTROL_CT_UNITS_UNSIGNED;
pub const MIXERCONTROL_CONTROLTYPE_BOOLEAN: DWORD = MIXERCONTROL_CT_CLASS_SWITCH |
    MIXERCONTROL_CT_SC_SWITCH_BOOLEAN | MIXERCONTROL_CT_UNITS_BOOLEAN;
pub const MIXERCONTROL_CONTROLTYPE_ONOFF: DWORD = MIXERCONTROL_CONTROLTYPE_BOOLEAN + 1;
pub const MIXERCONTROL_CONTROLTYPE_MUTE: DWORD = MIXERCONTROL_CONTROLTYPE_BOOLEAN + 2;
pub const MIXERCONTROL_CONTROLTYPE_MONO: DWORD = MIXERCONTROL_CONTROLTYPE_BOOLEAN + 3;
pub const MIXERCONTROL_CONTROLTYPE_LOUDNESS: DWORD = MIXERCONTROL_CONTROLTYPE_BOOLEAN + 4;
pub const MIXERCONTROL_CONTROLTYPE_STEREOENH: DWORD = MIXERCONTROL_CONTROLTYPE_BOOLEAN + 5;
pub const MIXERCONTROL_CONTROLTYPE_BASS_BOOST: DWORD = MIXERCONTROL_CONTROLTYPE_BOOLEAN +
    0x00002277;
pub const MIXERCONTROL_CONTROLTYPE_BUTTON: DWORD = MIXERCONTROL_CT_CLASS_SWITCH |
    MIXERCONTROL_CT_SC_SWITCH_BUTTON | MIXERCONTROL_CT_UNITS_BOOLEAN;
pub const MIXERCONTROL_CONTROLTYPE_DECIBELS: DWORD = MIXERCONTROL_CT_CLASS_NUMBER |
    MIXERCONTROL_CT_UNITS_DECIBELS;
pub const MIXERCONTROL_CONTROLTYPE_SIGNED: DWORD = MIXERCONTROL_CT_CLASS_NUMBER |
    MIXERCONTROL_CT_UNITS_SIGNED;
pub const MIXERCONTROL_CONTROLTYPE_UNSIGNED: DWORD = MIXERCONTROL_CT_CLASS_NUMBER |
    MIXERCONTROL_CT_UNITS_UNSIGNED;
pub const MIXERCONTROL_CONTROLTYPE_PERCENT: DWORD = MIXERCONTROL_CT_CLASS_NUMBER |
    MIXERCONTROL_CT_UNITS_PERCENT;
pub const MIXERCONTROL_CONTROLTYPE_SLIDER: DWORD = MIXERCONTROL_CT_CLASS_SLIDER |
    MIXERCONTROL_CT_UNITS_SIGNED;
pub const MIXERCONTROL_CONTROLTYPE_PAN: DWORD = MIXERCONTROL_CONTROLTYPE_SLIDER + 1;
pub const MIXERCONTROL_CONTROLTYPE_QSOUNDPAN: DWORD = MIXERCONTROL_CONTROLTYPE_SLIDER + 2;
pub const MIXERCONTROL_CONTROLTYPE_FADER: DWORD = MIXERCONTROL_CT_CLASS_FADER |
    MIXERCONTROL_CT_UNITS_UNSIGNED;
pub const MIXERCONTROL_CONTROLTYPE_VOLUME: DWORD = MIXERCONTROL_CONTROLTYPE_FADER + 1;
pub const MIXERCONTROL_CONTROLTYPE_BASS: DWORD = MIXERCONTROL_CONTROLTYPE_FADER + 2;
pub const MIXERCONTROL_CONTROLTYPE_TREBLE: DWORD = MIXERCONTROL_CONTROLTYPE_FADER + 3;
pub const MIXERCONTROL_CONTROLTYPE_EQUALIZER: DWORD = MIXERCONTROL_CONTROLTYPE_FADER + 4;
pub const MIXERCONTROL_CONTROLTYPE_SINGLESELECT: DWORD = MIXERCONTROL_CT_CLASS_LIST |
    MIXERCONTROL_CT_SC_LIST_SINGLE | MIXERCONTROL_CT_UNITS_BOOLEAN;
pub const MIXERCONTROL_CONTROLTYPE_MUX: DWORD = MIXERCONTROL_CONTROLTYPE_SINGLESELECT + 1;
pub const MIXERCONTROL_CONTROLTYPE_MULTIPLESELECT: DWORD = MIXERCONTROL_CT_CLASS_LIST |
    MIXERCONTROL_CT_SC_LIST_MULTIPLE | MIXERCONTROL_CT_UNITS_BOOLEAN;
pub const MIXERCONTROL_CONTROLTYPE_MIXER: DWORD = MIXERCONTROL_CONTROLTYPE_MULTIPLESELECT + 1;
pub const MIXERCONTROL_CONTROLTYPE_MICROTIME: DWORD = MIXERCONTROL_CT_CLASS_TIME |
    MIXERCONTROL_CT_SC_TIME_MICROSECS | MIXERCONTROL_CT_UNITS_UNSIGNED;
pub const MIXERCONTROL_CONTROLTYPE_MILLITIME: DWORD = MIXERCONTROL_CT_CLASS_TIME |
    MIXERCONTROL_CT_SC_TIME_MILLISECS | MIXERCONTROL_CT_UNITS_UNSIGNED;
UNION2!{union MIXERLINECONTROLSA_u {
    [u8; 4],
    dwControlID dwControlID_mut: DWORD,
    dwControlType dwControlType_mut: DWORD,
}}
STRUCT!{#[repr(packed)] struct MIXERLINECONTROLSA {
    cbStruct: DWORD,
    dwLineID: DWORD,
    u: MIXERLINECONTROLSA_u,
    cControls: DWORD,
    cbmxctrl: DWORD,
    pamxctrl: LPMIXERCONTROLA,
}}
pub type PMIXERLINECONTROLSA = *mut MIXERLINECONTROLSA;
pub type LPMIXERLINECONTROLSA = *mut MIXERLINECONTROLSA;
UNION2!{union MIXERLINECONTROLSW_u {
    [u8; 4],
    dwControlID dwControlID_mut: DWORD,
    dwControlType dwControlType_mut: DWORD,
}}
STRUCT!{#[repr(packed)] struct MIXERLINECONTROLSW {
    cbStruct: DWORD,
    dwLineID: DWORD,
    u: MIXERLINECONTROLSW_u,
    cControls: DWORD,
    cbmxctrl: DWORD,
    pamxctrl: LPMIXERCONTROLW,
}}
pub type PMIXERLINECONTROLSW = *mut MIXERLINECONTROLSW;
pub type LPMIXERLINECONTROLSW = *mut MIXERLINECONTROLSW;
extern "system" {
    pub fn mixerGetLineControlsA(
        hmxobj: HMIXEROBJ,
        pmxlc: LPMIXERLINECONTROLSA,
        fdwControls: DWORD,
    ) -> MMRESULT;
    pub fn mixerGetLineControlsW(
        hmxobj: HMIXEROBJ,
        pmxlc: LPMIXERLINECONTROLSW,
        fdwControls: DWORD,
    ) -> MMRESULT;
}
pub const MIXER_GETLINECONTROLSF_ALL: DWORD = 0x00000000;
pub const MIXER_GETLINECONTROLSF_ONEBYID: DWORD = 0x00000001;
pub const MIXER_GETLINECONTROLSF_ONEBYTYPE: DWORD = 0x00000002;
pub const MIXER_GETLINECONTROLSF_QUERYMASK: DWORD = 0x0000000F;
UNION2!{union MIXERCONTROLDETAILS_u {
    [u8; 4] [u8; 8],
    hwndOwner hwndOwner_mut: HWND,
    cMultipleItems cMultipleItems_mut: DWORD,
}}
STRUCT!{#[repr(packed)] struct MIXERCONTROLDETAILS {
    cbStruct: DWORD,
    dwControlID: DWORD,
    cChannels: DWORD,
    u: MIXERCONTROLDETAILS_u,
    cbDetails: DWORD,
    paDetails: LPVOID,
}}
pub type PMIXERCONTROLDETAILS = *mut MIXERCONTROLDETAILS;
pub type LPMIXERCONTROLDETAILS = *mut MIXERCONTROLDETAILS;
STRUCT!{#[repr(packed)] struct MIXERCONTROLDETAILS_LISTTEXTA {
    dwParam1: DWORD,
    dwParam2: DWORD,
    szName: [CHAR; MIXER_LONG_NAME_CHARS],
}}
pub type PMIXERCONTROLDETAILS_LISTTEXTA = *mut MIXERCONTROLDETAILS_LISTTEXTA;
pub type LPMIXERCONTROLDETAILS_LISTTEXTA = *mut MIXERCONTROLDETAILS_LISTTEXTA;
STRUCT!{#[repr(packed)] struct MIXERCONTROLDETAILS_LISTTEXTW {
    dwParam1: DWORD,
    dwParam2: DWORD,
    szName: [WCHAR; MIXER_LONG_NAME_CHARS],
}}
pub type PMIXERCONTROLDETAILS_LISTTEXTW = *mut MIXERCONTROLDETAILS_LISTTEXTW;
pub type LPMIXERCONTROLDETAILS_LISTTEXTW = *mut MIXERCONTROLDETAILS_LISTTEXTW;
STRUCT!{#[repr(packed)] struct MIXERCONTROLDETAILS_BOOLEAN {
    fValue: LONG,
}}
pub type PMIXERCONTROLDETAILS_BOOLEAN = *mut MIXERCONTROLDETAILS_BOOLEAN;
pub type LPMIXERCONTROLDETAILS_BOOLEAN = *mut MIXERCONTROLDETAILS_BOOLEAN;
STRUCT!{#[repr(packed)] struct MIXERCONTROLDETAILS_SIGNED {
    lValue: LONG,
}}
pub type PMIXERCONTROLDETAILS_SIGNED = *mut MIXERCONTROLDETAILS_SIGNED;
pub type LPMIXERCONTROLDETAILS_SIGNED = *mut MIXERCONTROLDETAILS_SIGNED;
STRUCT!{#[repr(packed)] struct MIXERCONTROLDETAILS_UNSIGNED {
    dwValue: DWORD,
}}
pub type PMIXERCONTROLDETAILS_UNSIGNED = *mut MIXERCONTROLDETAILS_UNSIGNED;
pub type LPMIXERCONTROLDETAILS_UNSIGNED = *mut MIXERCONTROLDETAILS_UNSIGNED;
extern "system" {
    pub fn mixerGetControlDetailsA(
        hmxobj: HMIXEROBJ,
        pmxcd: LPMIXERCONTROLDETAILS,
        fdwDetails: DWORD,
    ) -> MMRESULT;
    pub fn mixerGetControlDetailsW(
        hmxobj: HMIXEROBJ,
        pmxcd: LPMIXERCONTROLDETAILS,
        fdwDetails: DWORD,
    ) -> MMRESULT;
}
pub const MIXER_GETCONTROLDETAILSF_VALUE: DWORD = 0x00000000;
pub const MIXER_GETCONTROLDETAILSF_LISTTEXT: DWORD = 0x00000001;
pub const MIXER_GETCONTROLDETAILSF_QUERYMASK: DWORD = 0x0000000F;
extern "system" {
    pub fn mixerSetControlDetails(
        hmxobj: HMIXEROBJ,
        pmxcd: LPMIXERCONTROLDETAILS,
        fdwDetails: DWORD,
    ) -> MMRESULT;
}
pub const MIXER_SETCONTROLDETAILSF_VALUE: DWORD = 0x00000000;
pub const MIXER_SETCONTROLDETAILSF_CUSTOM: DWORD = 0x00000001;
pub const MIXER_SETCONTROLDETAILSF_QUERYMASK: DWORD = 0x0000000F;
