// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to mpr.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn MultinetGetConnectionPerformanceA(
        lpNetResource: LPNETRESOURCEA, lpNetConnectInfoStruct: LPNETCONNECTINFOSTRUCT,
    ) -> DWORD;
    pub fn MultinetGetConnectionPerformanceW(
        lpNetResource: LPNETRESOURCEW, lpNetConnectInfoStruct: LPNETCONNECTINFOSTRUCT,
    ) -> DWORD;
    pub fn WNetAddConnection2A(
        lpNetResource: LPNETRESOURCEA, lpPassword: LPCSTR, lpUsername: LPCSTR, dwFlags: DWORD,
    ) -> DWORD;
    pub fn WNetAddConnection2W(
        lpNetResource: LPNETRESOURCEW, lpPassword: LPCWSTR, lpUsername: LPCWSTR, dwFlags: DWORD,
    ) -> DWORD;
    pub fn WNetAddConnection3A(
        hwndOwner: HWND, lpNetResource: LPNETRESOURCEA, lpPassword: LPCSTR, lpUsername: LPCSTR,
        dwFlags: DWORD,
    ) -> DWORD;
    pub fn WNetAddConnection3W(
        hwndOwner: HWND, lpNetResource: LPNETRESOURCEW, lpPassword: LPCWSTR, lpUsername: LPCWSTR,
        dwFlags: DWORD,
    ) -> DWORD;
    pub fn WNetCancelConnectionA(lpName: LPCSTR, fForce: BOOL) -> DWORD;
    pub fn WNetCancelConnectionW(lpName: LPCWSTR, fForce: BOOL) -> DWORD;
    pub fn WNetCancelConnection2A(lpName: LPCSTR, dwFlags: DWORD, fForce: BOOL) -> DWORD;
    pub fn WNetCancelConnection2W(lpName: LPCWSTR, dwFlags: DWORD, fForce: BOOL) -> DWORD;
    pub fn WNetCloseEnum(hEnum: HANDLE) -> DWORD;
    pub fn WNetConnectionDialog(hwnd: HWND, dwType: DWORD) -> DWORD;
    pub fn WNetConnectionDialog1A(lpConnDlgStruct: LPCONNECTDLGSTRUCTA) -> DWORD;
    pub fn WNetConnectionDialog1W(lpConnDlgStruct: LPCONNECTDLGSTRUCTW) -> DWORD;
    pub fn WNetDisconnectDialog(hwnd: HWND, dwType: DWORD) -> DWORD;
    pub fn WNetDisconnectDialog1A(lpConnDlgStruct: LPDISCDLGSTRUCTA) -> DWORD;
    pub fn WNetDisconnectDialog1W(lpConnDlgStruct: LPDISCDLGSTRUCTW) -> DWORD;
    pub fn WNetEnumResourceA(
        hEnum: HANDLE, lpcCount: LPDWORD, lpBuffer: LPVOID, lpBufferSize: LPDWORD,
    ) -> DWORD;
    pub fn WNetEnumResourceW(
        hEnum: HANDLE, lpcCount: LPDWORD, lpBuffer: LPVOID, lpBufferSize: LPDWORD,
    ) -> DWORD;
    pub fn WNetGetConnectionA(
        lpLocalName: LPCSTR, lpRemoteName: LPSTR, lpnLength: LPDWORD,
    ) -> DWORD;
    pub fn WNetGetConnectionW(
        lpLocalName: LPCWSTR, lpRemoteName: LPWSTR, lpnLength: LPDWORD,
    ) -> DWORD;
    pub fn WNetGetLastErrorA(
        lpError: LPDWORD, lpErrorBuf: LPSTR, nErrorBufSize: DWORD, lpNameBuf: LPSTR,
        nNameBufSize: DWORD,
    ) -> DWORD;
    pub fn WNetGetLastErrorW(
        lpError: LPDWORD, lpErrorBuf: LPWSTR, nErrorBufSize: DWORD, lpNameBuf: LPWSTR,
        nNameBufSize: DWORD,
    ) -> DWORD;
    pub fn WNetGetNetworkInformationA(
        lpProvider: LPCSTR, lpNetInfoStruct: LPNETINFOSTRUCT,
    ) -> DWORD;
    pub fn WNetGetNetworkInformationW(
        lpProvider: LPCWSTR, lpNetInfoStruct: LPNETINFOSTRUCT,
    ) -> DWORD;
    pub fn WNetGetProviderNameA(
        dwNetType: DWORD, lpProviderName: LPSTR, lpBufferSize: LPDWORD,
    ) -> DWORD;
    pub fn WNetGetProviderNameW(
        dwNetType: DWORD, lpProviderName: LPWSTR, lpBufferSize: LPDWORD,
    ) -> DWORD;
    pub fn WNetGetResourceInformationA(
        lpNetResource: LPNETRESOURCEA, lpBuffer: LPVOID, lpcbBuffer: LPDWORD,
        lplpSystem: *mut LPSTR,
    ) -> DWORD;
    pub fn WNetGetResourceInformationW(
        lpNetResource: LPNETRESOURCEW, lpBuffer: LPVOID, lpcbBuffer: LPDWORD,
        lplpSystem: *mut LPWSTR,
    ) -> DWORD;
    pub fn WNetGetResourceParentA(
        lpNetResource: LPNETRESOURCEA, lpBuffer: LPVOID, lpcbBuffer: LPDWORD,
    ) -> DWORD;
    pub fn WNetGetResourceParentW(
        lpNetResource: LPNETRESOURCEW, lpBuffer: LPVOID, lpcbBuffer: LPDWORD,
    ) -> DWORD;
    pub fn WNetGetUniversalNameA(
        lpLocalPath: LPCSTR, dwInfoLevel: DWORD, lpBuffer: LPVOID, lpBufferSize: LPDWORD,
    ) -> DWORD;
    pub fn WNetGetUniversalNameW(
        lpLocalPath: LPCWSTR, dwInfoLevel: DWORD, lpBuffer: LPVOID, lpBufferSize: LPDWORD,
    ) -> DWORD;
    pub fn WNetGetUserA(lpName: LPCSTR, lpUserName: LPSTR, lpnLength: LPDWORD) -> DWORD;
    pub fn WNetGetUserW(lpName: LPCWSTR, lpUserName: LPWSTR, lpnLength: LPDWORD) -> DWORD;
    pub fn WNetOpenEnumA(
        dwScope: DWORD, dwType: DWORD, dwUsage: DWORD, lpNetResource: LPNETRESOURCEA,
        lphEnum: LPHANDLE,
    ) -> DWORD;
    pub fn WNetOpenEnumW(
        dwScope: DWORD, dwType: DWORD, dwUsage: DWORD, lpNetResource: LPNETRESOURCEW,
        lphEnum: LPHANDLE,
    ) -> DWORD;
    pub fn WNetUseConnectionA(
        hwndOwner: HWND, lpNetResource: LPNETRESOURCEA, lpPassword: LPCSTR, lpUserId: LPCSTR,
        dwFlags: DWORD, lpAccessName: LPSTR, lpBufferSize: LPDWORD, lpResult: LPDWORD
    ) -> DWORD;
    pub fn WNetUseConnectionW(
        hwndOwner: HWND, lpNetResource: LPNETRESOURCEW, lpPassword: LPCWSTR, lpUserId: LPCWSTR,
        dwFlags: DWORD, lpAccessName: LPWSTR, lpBufferSize: LPDWORD, lpResult: LPDWORD,
    ) -> DWORD;
}
