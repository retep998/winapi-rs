// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to advapi32.
#![no_std]
#![experimental]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn AdjustTokenPrivileges(
        TokenHandle: HANDLE, DisableAllPrivileges: BOOL, NewState: PTOKEN_PRIVILEGES,
        BufferLength: DWORD, PreviousState: PTOKEN_PRIVILEGES, ReturnLength: PDWORD,
    ) -> BOOL;
    pub fn CryptAcquireContextA(
        phProv: *mut HCRYPTPROV, szContainer: LPCSTR, szProvider: LPCSTR, dwProvType: DWORD,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptAcquireContextW(
        phProv: *mut HCRYPTPROV, szContainer: LPCWSTR, szProvider: LPCWSTR, dwProvType: DWORD,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptCreateHash(
        hProv: HCRYPTPROV, Algid: ALG_ID, hKey: HCRYPTKEY, dwFlags: DWORD, phHash: *mut HCRYPTHASH,
    ) -> BOOL;
    pub fn CryptDestroyHash(hHash: HCRYPTHASH) -> BOOL;
    pub fn CryptGetHashParam(
        hHash: HCRYPTHASH, dwParam: DWORD, pbData: *mut BYTE, pdwDataLen: *mut DWORD,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptHashData(
        hHash: HCRYPTHASH, pbData: *const BYTE, dwDataLen: DWORD, dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptReleaseContext(hProv: HCRYPTPROV, dwFlags: DWORD) -> BOOL;
    pub fn OpenProcessToken(
        ProcessHandle: HANDLE, DesiredAccess: DWORD, TokenHandle: PHANDLE,
    ) -> BOOL;
    pub fn RegCloseKey(hKey: HKEY) -> LONG;
    pub fn RegConnectRegistryA(lpMachineName: LPCSTR, hKey: HKEY, phkResult: PHKEY) -> LONG;
    pub fn RegConnectRegistryW(lpMachineName: LPCWSTR, hKey: HKEY, phkResult: PHKEY) -> LONG;
    pub fn RegCopyTreeA(hKeySrc: HKEY, lpSubKey: LPCSTR, hKeyDest: HKEY) -> LONG;
    pub fn RegCopyTreeW(hKeySrc: HKEY, lpSubKey: LPCWSTR, hKeyDest: HKEY) -> LONG;
    pub fn RegCreateKeyExA(
        hKey: HKEY, lpSubKey: LPCSTR, Reserved: DWORD, lpClass: LPCSTR, dwOptions: DWORD,
        samDesired: REGSAM, lpSecurityAttributes: LPSECURITY_ATTRIBUTES, phkResult: PHKEY,
        lpdwDisposition: LPDWORD,
    ) -> LONG;
    pub fn RegCreateKeyExW(
        hKey: HKEY, lpSubKey: LPCWSTR, Reserved: DWORD, lpClass: LPCWSTR, dwOptions: DWORD,
        samDesired: REGSAM, lpSecurityAttributes: LPSECURITY_ATTRIBUTES, phkResult: PHKEY,
        lpdwDisposition: LPDWORD,
    ) -> LONG;
    pub fn RegDeleteKeyA(hKey: HKEY, lpSubKey: LPCSTR) -> LONG;
    pub fn RegDeleteKeyW(hKey: HKEY, lpSubKey: LPCWSTR) -> LONG;
    pub fn RegDeleteKeyExA(
        hKey: HKEY, lpSubKey: LPCSTR, samDesired: REGSAM, Reserved: DWORD,
    ) -> LONG;
    pub fn RegDeleteKeyExW(
        hKey: HKEY, lpSubKey: LPCWSTR, samDesired: REGSAM, Reserved: DWORD,
    ) -> LONG;
    pub fn RegDeleteKeyValueA(hKey: HKEY, lpSubKey: LPCSTR, lpValueName: LPCSTR) -> LONG;
    pub fn RegDeleteKeyValueW(hKey: HKEY, lpSubKey: LPCWSTR, lpValueName: LPCWSTR) -> LONG;
    pub fn RegDeleteTreeA(hKey: HKEY, lpSubKey: LPCSTR) -> LONG;
    pub fn RegDeleteTreeW(hKey: HKEY, lpSubKey: LPCWSTR) -> LONG;
    pub fn RegDeleteValueA(hKey: HKEY, lpValueName: LPCSTR) -> LONG;
    pub fn RegDeleteValueW(hKey: HKEY, lpValueName: LPCWSTR) -> LONG;
    pub fn RegDisablePredefinedCache() -> LONG;
    pub fn RegDisablePredefinedCacheEx() -> LONG;
    pub fn RegDisableReflectionKey(hBase: HKEY) -> LONG;
    pub fn RegEnableReflectionKey(hBase: HKEY) -> LONG;
    pub fn RegEnumKeyExA(
        hKey: HKEY, dwIndex: DWORD, lpName: LPSTR, lpcName: LPDWORD, lpReserved: LPDWORD,
        lpClass: LPSTR, lpcClass: LPDWORD, lpftLastWriteTime: PFILETIME,
    ) -> LONG;
    pub fn RegEnumKeyExW(
        hKey: HKEY, dwIndex: DWORD, lpName: LPWSTR, lpcName: LPDWORD, lpReserved: LPDWORD,
        lpClass: LPWSTR, lpcClass: LPDWORD, lpftLastWriteTime: PFILETIME,
    ) -> LONG;
    pub fn RegEnumValueA(
        hKey: HKEY, dwIndex: DWORD, lpValueName: LPSTR, lpcchValueName: LPDWORD,
        lpReserved: LPDWORD, lpType: LPDWORD, lpData: LPBYTE, lpcbData: LPDWORD,
    ) -> LONG;
    pub fn RegEnumValueW(
        hKey: HKEY, dwIndex: DWORD, lpValueName: LPWSTR, lpcchValueName: LPDWORD,
        lpReserved: LPDWORD, lpType: LPDWORD, lpData: LPBYTE, lpcbData: LPDWORD,
    ) -> LONG;
    pub fn RegFlushKey(hKey: HKEY) -> LONG;
    pub fn RegGetValueA(
        hkey: HKEY, lpSubKey: LPCSTR, lpValue: LPCSTR, dwFlags: DWORD, pdwType: LPDWORD,
        pvData: PVOID, pcbData: LPDWORD,
    ) -> LONG;
    pub fn RegGetValueW(
        hkey: HKEY, lpSubKey: LPCWSTR, lpValue: LPCWSTR, dwFlags: DWORD, pdwType: LPDWORD,
        pvData: PVOID, pcbData: LPDWORD,
    ) -> LONG;
    pub fn RegLoadMUIStringW(
        hKey: HKEY, pszValue: LPCWSTR, pszOutBuf: LPWSTR, cbOutBuf: DWORD, pcbData: LPDWORD,
        Flags: DWORD, pszDirectory: LPCWSTR,
    ) -> LONG;
    pub fn RegNotifyChangeKeyValue(
        hKey: HKEY, bWatchSubtree: BOOL, dwNotifyFilter: DWORD, hEvent: HANDLE,
        fAsynchronous: BOOL,
    ) -> LONG;
    pub fn RegOpenCurrentUser(samDesired: REGSAM, phkResult: PHKEY) -> LONG;
    pub fn RegOpenKeyExA(
        hKey: HKEY, lpSubKey: LPCSTR, ulOptions: DWORD, samDesired: REGSAM, phkResult: PHKEY,
    ) -> LONG;
    pub fn RegOpenKeyExW(
        hKey: HKEY, lpSubKey: LPCWSTR, ulOptions: DWORD, samDesired: REGSAM, phkResult: PHKEY,
    ) -> LONG;
    pub fn RegOpenUserClassesRoot(
        hToken: HANDLE, dwOptions: DWORD, samDesired: REGSAM, phkResult: PHKEY,
    ) -> LONG;
    pub fn RegOverridePredefKey(hKey: HKEY, hNewHKey: HKEY) -> LONG;
    pub fn RegQueryInfoKeyA(
        hKey: HKEY, lpClass: LPSTR, lpcClass: LPDWORD, lpReserved: LPDWORD, lpcSubKeys: LPDWORD,
        lpcMaxSubKeyLen: LPDWORD, lpcMaxClassLen: LPDWORD, lpcValues: LPDWORD,
        lpcMaxValueNameLen: LPDWORD, lpcMaxValueLen: LPDWORD, lpcbSecurityDescriptor: LPDWORD,
        lpftLastWriteTime: PFILETIME,
    ) -> LONG;
    pub fn RegQueryInfoKeyW(
        hKey: HKEY, lpClass: LPWSTR, lpcClass: LPDWORD, lpReserved: LPDWORD, lpcSubKeys: LPDWORD,
        lpcMaxSubKeyLen: LPDWORD, lpcMaxClassLen: LPDWORD, lpcValues: LPDWORD,
        lpcMaxValueNameLen: LPDWORD, lpcMaxValueLen: LPDWORD, lpcbSecurityDescriptor: LPDWORD,
        lpftLastWriteTime: PFILETIME,
    ) -> LONG;
    pub fn RegQueryMultipleValuesA(
        hKey: HKEY, val_list: PVALENTA, num_vals: DWORD, lpValueBuf: LPSTR, ldwTotsize: LPDWORD,
    ) -> LONG;
    pub fn RegQueryMultipleValuesW(
        hKey: HKEY, val_list: PVALENTW, num_vals: DWORD, lpValueBuf: LPWSTR, ldwTotsize: LPDWORD,
    ) -> LONG;
    pub fn RegQueryReflectionKey(hBase: HKEY, bIsReflectionDisabled: PBOOL) -> LONG;
    pub fn RegQueryValueExA(
        hKey: HKEY, lpValueName: LPCSTR, lpReserved: LPDWORD, lpType: LPDWORD, lpData: LPBYTE,
        lpcbData: LPDWORD,
    ) -> LONG;
    pub fn RegQueryValueExW(
        hKey: HKEY, lpValueName: LPCWSTR, lpReserved: LPDWORD, lpType: LPDWORD, lpData: LPBYTE,
        lpcbData: LPDWORD,
    ) -> LONG;
    pub fn RegSetKeyValueA(
        hKey: HKEY, lpSubKey: LPCSTR, lpValueName: LPCSTR, dwType: DWORD, lpData: LPCVOID,
        cbData: DWORD,
    ) -> LONG;
    pub fn RegSetKeyValueW(
        hKey: HKEY, lpSubKey: LPCWSTR, lpValueName: LPCWSTR, dwType: DWORD, lpData: LPCVOID,
        cbData: DWORD,
    ) -> LONG;
    pub fn RegSetValueExA(
        hKey: HKEY, lpValueName: LPCSTR, Reserved: DWORD, dwType: DWORD, lpData: *const BYTE,
        cbData: DWORD,
    ) -> LONG;
    pub fn RegSetValueExW(
        hKey: HKEY, lpValueName: LPCWSTR, Reserved: DWORD, dwType: DWORD, lpData: *const BYTE,
        cbData: DWORD,
    ) -> LONG;
}
