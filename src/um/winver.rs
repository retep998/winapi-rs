// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms
//! Version management functions, types, and definitions
use shared::minwindef::{BOOL, DWORD, LPCVOID, LPDWORD, LPVOID, PUINT};
use um::winnt::{LPCSTR, LPCWSTR, LPSTR, LPWSTR};
extern "system" {
    pub fn VerFindFileA(
        uFlags: DWORD,
        szFileName: LPCSTR,
        szWinDir: LPCSTR,
        szAppDir: LPCSTR,
        szCurDir: LPSTR,
        puCurDirLen: PUINT,
        szDestDir: LPSTR,
        puDestDirLen: PUINT,
    ) -> DWORD;
    pub fn VerFindFileW(
        uFlags: DWORD,
        szFileName: LPCWSTR,
        szWinDir: LPCWSTR,
        szAppDir: LPCWSTR,
        szCurDir: LPWSTR,
        puCurDirLen: PUINT,
        szDestDir: LPWSTR,
        puDestDirLen: PUINT,
    ) -> DWORD;
    pub fn VerInstallFileA(
        uFlags: DWORD,
        szSrcFileName: LPCSTR,
        szDestFileName: LPCSTR,
        szSrcDir: LPCSTR,
        szDestDir: LPCSTR,
        szCurDir: LPCSTR,
        szTmpFile: LPSTR,
        puTmpFileLen: PUINT,
    ) -> DWORD;
    pub fn VerInstallFileW(
        uFlags: DWORD,
        szSrcFileName: LPCWSTR,
        szDestFileName: LPCWSTR,
        szSrcDir: LPCWSTR,
        szDestDir: LPCWSTR,
        szCurDir: LPCWSTR,
        szTmpFile: LPWSTR,
        puTmpFileLen: PUINT,
    ) -> DWORD;
    pub fn GetFileVersionInfoSizeA(
        lptstrFilename: LPCSTR,
        lpdwHandle: LPDWORD,
    ) -> DWORD;
    pub fn GetFileVersionInfoSizeW(
        lptstrFilename: LPCWSTR,
        lpdwHandle: LPDWORD,
    ) -> DWORD;
    pub fn GetFileVersionInfoA(
        lptstrFilename: LPCSTR,
        dwHandle: DWORD,
        dwLen: DWORD,
        lpData: LPVOID,
    ) -> BOOL;
    pub fn GetFileVersionInfoW(
        lptstrFilename: LPCWSTR,
        dwHandle: DWORD,
        dwLen: DWORD,
        lpData: LPVOID,
    ) -> BOOL;
    pub fn GetFileVersionInfoSizeExA(
        dwFlags: DWORD,
        lpwstrFilename: LPCSTR,
        lpdwHandle: LPDWORD,
    ) -> DWORD;
    pub fn GetFileVersionInfoSizeExW(
        dwFlags: DWORD,
        lpwstrFilename: LPCWSTR,
        lpdwHandle: LPDWORD,
    ) -> DWORD;
    pub fn GetFileVersionInfoExA(
        dwFlags: DWORD,
        lpwstrFilename: LPCSTR,
        dwHandle: DWORD,
        dwLen: DWORD,
        lpData: LPVOID,
    ) -> BOOL;
    pub fn GetFileVersionInfoExW(
        dwFlags: DWORD,
        lpwstrFilename: LPCWSTR,
        dwHandle: DWORD,
        dwLen: DWORD,
        lpData: LPVOID,
    ) -> BOOL;
    pub fn VerLanguageNameA(
        wLang: DWORD,
        szLang: LPSTR,
        cchLang: DWORD,
    ) -> DWORD;
    pub fn VerLanguageNameW(
        wLang: DWORD,
        szLang: LPWSTR,
        cchLang: DWORD,
    ) -> DWORD;
    pub fn VerQueryValueA(
        pBlock: LPCVOID,
        lpSubBlock: LPCSTR,
        lplpBuffer: &mut LPVOID,
        puLen: PUINT,
    ) -> BOOL;
    pub fn VerQueryValueW(
        pBlock: LPCVOID,
        lpSubBlock: LPCWSTR,
        lplpBuffer: &mut LPVOID,
        puLen: PUINT,
    ) -> BOOL;
}
