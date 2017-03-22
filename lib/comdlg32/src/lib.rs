// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to comdlg32.
#![cfg(windows)]

mod isolation_aware;

extern crate winapi;
#[cfg(test)]
#[cfg_attr(test, macro_use)]
extern crate kernel32;
use winapi::*;
extern "system" {
    pub fn ChooseColorA(lpcc: LPCHOOSECOLORA) -> BOOL;
    pub fn ChooseColorW(lpcc: LPCHOOSECOLORW) -> BOOL;
    pub fn ChooseFontA(lpcf: LPCHOOSEFONTA) -> BOOL;
    pub fn ChooseFontW(lpcf: LPCHOOSEFONTW) -> BOOL;
    pub fn CommDlgExtendedError() -> DWORD;
    pub fn FindTextA(lpfr: LPFINDREPLACEA) -> HWND;
    pub fn FindTextW(lpfr: LPFINDREPLACEW) -> HWND;
    pub fn GetFileTitleA(lpszFile: LPCSTR, Buf: LPSTR, cchSize: WORD) -> c_short;
    pub fn GetFileTitleW(lpszFile: LPCWSTR, Buf: LPWSTR, cchSize: WORD) -> c_short;
    pub fn GetOpenFileNameA(lpofn: LPOPENFILENAMEA) -> BOOL;
    pub fn GetOpenFileNameW(lpofn: LPOPENFILENAMEW) -> BOOL;
    pub fn GetSaveFileNameA(lpofn: LPOPENFILENAMEA) -> BOOL;
    pub fn GetSaveFileNameW(lpofn: LPOPENFILENAMEW) -> BOOL;
    pub fn PageSetupDlgA(lppsd: LPPAGESETUPDLGA) -> BOOL;
    pub fn PageSetupDlgW(lppsd: LPPAGESETUPDLGW) -> BOOL;
    pub fn PrintDlgA(pPD: LPPRINTDLGA) -> BOOL;
    pub fn PrintDlgExA(pPD: LPPRINTDLGEXA) -> HRESULT;
    pub fn PrintDlgExW(pPD: LPPRINTDLGEXW) -> HRESULT;
    pub fn PrintDlgW(pPD: LPPRINTDLGW) -> BOOL;
    pub fn ReplaceTextA(lpfr: LPFINDREPLACEA) -> HWND;
    pub fn ReplaceTextW(lpfr: LPFINDREPLACEW) -> HWND;
}
