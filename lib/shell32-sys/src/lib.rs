// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to shell32.
#![no_std]
#![experimental]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn SHCloneSpecialIDList(hwnd: HWND, csidl: c_int, fCreate: BOOL) -> PIDLIST_ABSOLUTE;
    pub fn SHCreateDirectory(hwnd: HWND, pszPath: PCWSTR) -> c_int;
    pub fn SHCreateDirectoryExA(
        hwnd: HWND, pszPath: LPCSTR, psa: *const SECURITY_ATTRIBUTES,
    ) -> c_int;
    pub fn SHCreateDirectoryExW(
        hwnd: HWND, pszPath: LPCWSTR, psa: *const SECURITY_ATTRIBUTES,
    ) -> c_int;
    pub fn SHCreateShellItem(
        pidlParent: PCIDLIST_ABSOLUTE, psfParent: *mut IShellFolder, pidl: PCUITEMID_CHILD,
        ppsi: *mut *mut IShellItem,
    ) -> HRESULT;
    pub fn SHFlushSFCache();
    pub fn SHGetFolderLocation(
        hwnd: HWND, csidl: c_int, hToken: HANDLE, dwFlags: DWORD, ppidl: *mut PIDLIST_ABSOLUTE,
    ) -> HRESULT;
    pub fn SHGetFolderPathA(
        hwnd: HWND, csidl: c_int, hToken: HANDLE, dwFlags: DWORD, pszPath: LPSTR,
    ) -> HRESULT;
    pub fn SHGetFolderPathAndSubDirA(
        hwnd: HWND, csidl: c_int, hToken: HANDLE, dwFlags: DWORD, pszSubDir: LPCSTR,
        pszPath: LPSTR,
    ) -> HRESULT;
    pub fn SHGetFolderPathAndSubDirW(
        hwnd: HWND, csidl: c_int, hToken: HANDLE, dwFlags: DWORD, pszSubDir: LPCWSTR,
        pszPath: LPWSTR,
    ) -> HRESULT;
    pub fn SHGetFolderPathW(
        hwnd: HWND, csidl: c_int, hToken: HANDLE, dwFlags: DWORD, pszPath: LPWSTR,
    ) -> HRESULT;
    pub fn SHGetIconOverlayIndexA(pszIconPath: LPCSTR, iIconIndex: c_int) -> c_int;
    pub fn SHGetIconOverlayIndexW(pszIconPath: LPCWSTR, iIconIndex: c_int) -> c_int;
    pub fn SHGetKnownFolderIDList(
        rfid: REFKNOWNFOLDERID, dwFlags: DWORD, hToken: HANDLE, ppidl: *mut PIDLIST_ABSOLUTE,
    ) -> HRESULT;
    pub fn SHGetKnownFolderItem(
        rfid: REFKNOWNFOLDERID, flags: KNOWN_FOLDER_FLAG, hToken: HANDLE, riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
    pub fn SHGetKnownFolderPath(
        rfid: REFKNOWNFOLDERID, dwFlags: DWORD, hToken: HANDLE, pszPath: *mut PWSTR,
    ) -> HRESULT;
    pub fn SHGetPathFromIDListA(pidl: PCIDLIST_ABSOLUTE, pszPath: LPSTR) -> BOOL;
    pub fn SHGetPathFromIDListEx(
        pidl: PCIDLIST_ABSOLUTE, pszPath: PWSTR, cchPath: DWORD, uOpts: GPFIDL_FLAGS,
    ) -> BOOL;
    pub fn SHGetPathFromIDListW(pidl: PCIDLIST_ABSOLUTE, pszPath: LPWSTR) -> BOOL;
    pub fn SHGetSpecialFolderLocation(
        hwnd: HWND, csidl: c_int, ppidl: *mut PIDLIST_ABSOLUTE,
    ) -> HRESULT;
    pub fn SHGetSpecialFolderPathA(
        hwnd: HWND, pszPath: LPSTR, csidl: c_int, fCreate: BOOL,
    ) -> BOOL;
    pub fn SHGetSpecialFolderPathW(
        hwnd: HWND, pszPath: LPWSTR, csidl: c_int, fCreate: BOOL,
    ) -> BOOL;
    pub fn SHOpenFolderAndSelectItems(
        pidlFolder: PCIDLIST_ABSOLUTE, cidl: UINT, apidl: PCUITEMID_CHILD_ARRAY, dwFlags: DWORD,
    ) -> HRESULT;
    pub fn SHSetFolderPathA(
        csidl: c_int, hToken: HANDLE, dwFlags: DWORD, pszPath: LPCSTR,
    ) -> HRESULT;
    pub fn SHSetFolderPathW(
        csidl: c_int, hToken: HANDLE, dwFlags: DWORD, pszPath: LPCWSTR,
    ) -> HRESULT;
    pub fn SHSetKnownFolderPath(
        rfid: REFKNOWNFOLDERID, dwFlags: DWORD, hToken: HANDLE, pszPath: PCWSTR,
    ) -> HRESULT;
}
