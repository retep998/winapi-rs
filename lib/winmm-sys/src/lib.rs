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
}
