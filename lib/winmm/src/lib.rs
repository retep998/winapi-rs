// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to winmm.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn PlaySoundA(pszSound: LPCSTR, hmod: HMODULE, fdwSound: DWORD) -> BOOL;
    pub fn PlaySoundW(pszSound: LPCWSTR, hmod: HMODULE, fdwSound: DWORD) -> BOOL;
    pub fn sndPlaySoundA(pszSound: LPCSTR, fuSound: UINT) -> BOOL;
    pub fn sndPlaySoundW(pszSound: LPCWSTR, fuSound: UINT) -> BOOL;
    pub fn timeBeginPeriod(uPeriod: UINT) -> MMRESULT;
    pub fn timeEndPeriod(uPeriod: UINT) -> MMRESULT;
    pub fn timeGetDevCaps(ptc: LPTIMECAPS, cbtc: UINT) -> MMRESULT;
    pub fn timeGetTime() -> DWORD;
    pub fn waveInAddBuffer(hwi: HWAVEIN, pwh: LPWAVEHDR, cbwh: UINT) -> MMRESULT;
    pub fn waveInClose(hwi: HWAVEIN) -> MMRESULT;
    // pub fn waveInGetDevCapsA();
    pub fn waveInGetDevCapsW(uDeviceID: UINT_PTR, pwic: LPWAVEINCAPSW, cbwic: UINT) -> MMRESULT;
    // pub fn waveInGetErrorTextA();
    pub fn waveInGetErrorTextW(mmrError: MMRESULT, pszText: LPWSTR, cchText: UINT) -> MMRESULT;
    // pub fn waveInGetID();
    pub fn waveInGetNumDevs() -> UINT;
    pub fn waveInGetPosition(hwi: HWAVEIN, pmmt: LPMMTIME, cbmmt: UINT) -> MMRESULT;
    pub fn waveInMessage(hwi: HWAVEIN, uMsg: UINT, dw1: DWORD_PTR, dw2: DWORD_PTR) -> MMRESULT;
    pub fn waveInOpen(
        phwi: LPHWAVEIN, uDeviceID: UINT, pwfx: LPCWAVEFORMATEX, dwCallback: DWORD_PTR,
        dwInstance: DWORD_PTR, fdwOpen: DWORD,
    ) -> MMRESULT;
    pub fn waveInPrepareHeader(hwi: HWAVEIN, pwh: LPWAVEHDR, cbwh: UINT) -> MMRESULT;
    pub fn waveInReset(hwi: HWAVEIN) -> MMRESULT;
    pub fn waveInStart(hwi: HWAVEIN) -> MMRESULT;
    pub fn waveInStop(hwi: HWAVEIN) -> MMRESULT;
    pub fn waveInUnprepareHeader(hwi: HWAVEIN, pwh: LPWAVEHDR, cbwh: UINT) -> MMRESULT;
    pub fn waveOutBreakLoop(hwo: HWAVEOUT) -> MMRESULT;
    pub fn waveOutClose(hwo: HWAVEOUT) -> MMRESULT;
    // pub fn waveOutGetDevCapsA();
    pub fn waveOutGetDevCapsW(uDeviceID: UINT_PTR, pwoc: LPWAVEOUTCAPSW, cbwoc: UINT) -> MMRESULT;
    // pub fn waveOutGetErrorTextA();
    pub fn waveOutGetErrorTextW(mmrError: MMRESULT, pszText: LPWSTR, cchText: UINT) -> MMRESULT;
    // pub fn waveOutGetID();
    pub fn waveOutGetNumDevs() -> UINT;
    pub fn waveOutGetPitch(hwo: HWAVEOUT, pdwPitch: LPDWORD) -> MMRESULT;
    pub fn waveOutGetPlaybackRate(hwo: HWAVEOUT, pdwRate: LPDWORD) -> MMRESULT;
    pub fn waveOutGetPosition(hwo: HWAVEOUT, pmmt: LPMMTIME, cbmmt: UINT) -> MMRESULT;
    pub fn waveOutGetVolume(hwo: HWAVEOUT, pdwVolume: LPDWORD) -> MMRESULT;
    pub fn waveOutMessage(hwo: HWAVEOUT, uMsg: UINT, dw1: DWORD_PTR, dw2: DWORD_PTR) -> MMRESULT;
    pub fn waveOutOpen(
        phwo: LPHWAVEOUT, uDeviceID: UINT, pwfx: LPCWAVEFORMATEX, dwCallback: DWORD_PTR,
        dwInstance: DWORD_PTR, fdwOpen: DWORD,
    ) -> MMRESULT;
    pub fn waveOutPause(hwo: HWAVEOUT) -> MMRESULT;
    pub fn waveOutPrepareHeader(hwo: HWAVEOUT, pwh: LPWAVEHDR, cbwh: UINT) -> MMRESULT;
    pub fn waveOutReset(hwo: HWAVEOUT) -> MMRESULT;
    pub fn waveOutRestart(hwo: HWAVEOUT) -> MMRESULT;
    pub fn waveOutSetPitch(hwo: HWAVEOUT, dwPitch: DWORD) -> MMRESULT;
    pub fn waveOutSetPlaybackRate(hwo: HWAVEOUT, dwRate: DWORD) -> MMRESULT;
    pub fn waveOutSetVolume(hwo: HWAVEOUT, dwVolume: DWORD) -> MMRESULT;
    pub fn waveOutUnprepareHeader(hwo: HWAVEOUT, pwh: LPWAVEHDR, cbwh: UINT) -> MMRESULT;
    pub fn waveOutWrite(hwo: HWAVEOUT, pwh: LPWAVEHDR, cbwh: UINT) -> MMRESULT;
    pub fn midiStreamOpen(
        lphStream: LPHMIDISTRM, puDeviceID: LPUINT, cMidi: DWORD, dwCallback: DWORD_PTR,
        dwInstance: DWORD_PTR, fdwOpen: DWORD
    ) -> MMRESULT;
    pub fn midiStreamClose(hStream: HMIDISTRM) -> MMRESULT;
    pub fn midiStreamProperty(hm: HMIDISTRM, lppropdata: LPBYTE, dwProperty: DWORD) -> MMRESULT;
    pub fn midiStreamPosition(hms: HMIDISTRM, pmmt: LPMMTIME, cbmmt: UINT) -> MMRESULT;
    pub fn midiStreamOut(hMidiStream: HMIDISTRM, lpMidiHdr: LPMIDIHDR, cbMidiHdr: UINT) -> MMRESULT;
    pub fn midiStreamPause(hms: HMIDISTRM) -> MMRESULT;
    pub fn midiStreamRestart(hms: HMIDISTRM) -> MMRESULT;
    pub fn midiStreamStop(hms: HMIDISTRM) -> MMRESULT;
    pub fn midiConnect(hMidi: HMIDI, hmo: HMIDIOUT, pReserved: PVOID) -> MMRESULT;
    pub fn midiDisconnect(hMidi: HMIDI, hmo: HMIDIOUT, pReserved: PVOID) -> MMRESULT;
    pub fn midiOutGetNumDevs() -> UINT;
    pub fn midiOutGetDevCapsW(
        uDeviceID: UINT_PTR, lpMidiOutCaps: LPMIDIOUTCAPSW, cbMidiOutCaps: UINT
    ) -> MMRESULT;
    pub fn midiOutGetVolume(hmo: HMIDIOUT, lpdwVolume: PDWORD) -> MMRESULT;
    pub fn midiOutSetVolume(hmo: HMIDIOUT, dwVolume: DWORD) -> MMRESULT;
    pub fn midiOutGetErrorTextW(mmrError: MMRESULT, lpText: LPWSTR, cchText: UINT) -> MMRESULT;
    pub fn midiOutOpen(
        lphmo: LPHMIDIOUT, uDeviceID: UINT, dwCallback: DWORD_PTR, dwCallbackInstance: DWORD_PTR,
        dwFlags: DWORD
    ) -> MMRESULT;
    pub fn midiOutClose(hmo: HMIDIOUT) -> MMRESULT;
    pub fn midiOutPrepareHeader(
        hmo: HMIDIOUT, lpMidiOutHdr: LPMIDIHDR, cbMidiOutHdr: UINT
    ) -> MMRESULT;
    pub fn midiOutUnprepareHeader(
        hmo: HMIDIOUT, lpMidiOutHdr: LPMIDIHDR, cbMidiOutHdr: UINT
    ) -> MMRESULT;
    pub fn midiOutShortMsg(hmo: HMIDIOUT, dwMsg: DWORD) -> MMRESULT;
    pub fn midiOutLongMsg(
        hmo: HMIDIOUT, lpMidiOutHdr: LPMIDIHDR, cbMidiOutHdr: UINT
    ) -> MMRESULT;
    pub fn midiOutReset(hmo: HMIDIOUT) -> MMRESULT;
    pub fn midiOutCachePatches(hmo: HMIDIOUT, wBank: UINT, lpPatchArray: LPWORD, wFlags: UINT) -> MMRESULT;
    pub fn midiOutCacheDrumPatches(hmo: HMIDIOUT, wPatch: UINT, lpKeyArray: LPWORD, wFlags: UINT) -> MMRESULT;
    pub fn midiOutGetID(hmo: HMIDIOUT, puDeviceID: LPUINT) -> MMRESULT;
    pub fn midiOutMessage(deviceID: HMIDIOUT, msg: UINT, dw1: DWORD_PTR, dw2: DWORD_PTR) -> MMRESULT;
    pub fn midiInGetNumDevs() -> UINT;
    pub fn midiInGetDevCapsW(
        uDeviceID: UINT_PTR, lpMidiInCaps: LPMIDIINCAPSW, cbMidiInCaps: UINT
    ) -> MMRESULT;
    pub fn midiInGetErrorTextW(wError: MMRESULT, lpText: LPWSTR, cchText: UINT) -> MMRESULT;
    pub fn midiInOpen(
        lphMidiIn: LPHMIDIIN, uDeviceID: UINT, dwCallback: DWORD_PTR,
        dwCallbackInstance: DWORD_PTR, dwFlags: DWORD
    ) -> MMRESULT;
    pub fn midiInClose(hMidiIn: HMIDIIN) -> MMRESULT;
    pub fn midiInPrepareHeader(
        hMidiIn: HMIDIIN, lpMidiInHdr: LPMIDIHDR, cbMidiInHdr: UINT
    ) -> MMRESULT;
    pub fn midiInUnprepareHeader(
        hMidiIn: HMIDIIN, lpMidiInHdr: LPMIDIHDR, cbMidiInHdr: UINT
    ) -> MMRESULT;
    pub fn midiInAddBuffer(
        hMidiIn: HMIDIIN, lpMidiInHdr: LPMIDIHDR, cbMidiInHdr: UINT
    ) -> MMRESULT;
    pub fn midiInStart(hMidiIn: HMIDIIN) -> MMRESULT;
    pub fn midiInStop(hMidiIn: HMIDIIN) -> MMRESULT;
    pub fn midiInReset(hMidiIn: HMIDIIN) -> MMRESULT;
    pub fn midiInGetID(hmi: HMIDIIN, puDeviceID: LPUINT) -> MMRESULT;
    pub fn midiInMessage(deviceID: HMIDIIN, msg: UINT, dw1: DWORD_PTR, dw2: DWORD_PTR) -> MMRESULT;
    pub fn mciExecute(pszCommand: LPCSTR) -> BOOL;
    pub fn mciGetCreatorTask(IDDevice: MCIDEVICEID) -> MMRESULT;
    pub fn mciGetDeviceID(lpszDevice: LPCTSTR) -> MCIDEVICEID;
    pub fn mciGetDeviceIDFromElementID(dwElementID: DWORD, lpstrType: LPCTSTR) -> MCIDEVICEID;
    pub fn mciGetErrorString(fdwError: DWORD, lpszErrorText: LPTSTR, cchErrorText: UINT) -> BOOL;
    pub fn mciGetYieldProc(IDDevice: MCIDEVICEID, lpdwYieldData: LPDWORD) -> YIELDPROC;
    pub fn mciSendCommand(IDDevice: MCIDEVICEID, uMsg: UINT, fdwCommand: DWORD_PTR, dwParam: DWORD_PTR) -> MCIERROR;
    pub fn mciSendString(lpszCommand: LPCTSTR, lpszReturnString: LPTSTR, cchReturn: UINT, hwndCallback: HANDLE) -> MCIERROR;
    pub fn mciSetYieldProc(IDDevice: MCIDEVICEID, yp: YIELDPROC, dwYieldData: DWORD) -> UINT;
}
