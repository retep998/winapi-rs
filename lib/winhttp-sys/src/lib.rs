// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to winhttp.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn WinHttpAddRequestHeaders(
        hRequest: HINTERNET, lpszHeaders: LPCWSTR, dwHeadersLength: DWORD, dwModifiers: DWORD,
    ) -> BOOL;
    // pub fn WinHttpAutoProxySvcMain();
    pub fn WinHttpCheckPlatform() -> BOOL;
    pub fn WinHttpCloseHandle(hInternet: HINTERNET) -> BOOL;
    pub fn WinHttpConnect(
        hSession: HINTERNET, pswzServerName: LPCWSTR, nServerPort: INTERNET_PORT, dwReserved: DWORD,
    ) -> HINTERNET;
    pub fn WinHttpCrackUrl(
        pwszUrl: LPCWSTR, dwUrlLength: DWORD, dwFlags: DWORD, lpUrlComponents: LPURL_COMPONENTS,
    ) -> BOOL;
    pub fn WinHttpCreateProxyResolver(hSession: HINTERNET, phResolver: *mut HINTERNET) -> DWORD;
    pub fn WinHttpCreateUrl(
        lpUrlComponents: LPURL_COMPONENTS, dwFlags: DWORD, pwszUrl: LPWSTR, pdwUrlLength: LPDWORD,
    ) -> BOOL;
    pub fn WinHttpDetectAutoProxyConfigUrl(
        dwAutoDetectFlags: DWORD, ppwstrAutoConfigUrl: *mut LPWSTR,
    ) -> BOOL;
    pub fn WinHttpFreeProxyResult(pProxyResult: *mut WINHTTP_PROXY_RESULT);
    pub fn WinHttpGetDefaultProxyConfiguration(pProxyInfo: *mut WINHTTP_PROXY_INFO) -> BOOL;
    pub fn WinHttpGetIEProxyConfigForCurrentUser(
        pProxyConfig: *mut WINHTTP_CURRENT_USER_IE_PROXY_CONFIG,
    ) -> BOOL;
    pub fn WinHttpGetProxyForUrl(
        hSession: HINTERNET, lpcwszUrl: LPCWSTR, pAutoProxyOptions: *mut WINHTTP_AUTOPROXY_OPTIONS,
        pProxyInfo: *mut WINHTTP_PROXY_INFO,
    ) -> BOOL;
    pub fn WinHttpGetProxyForUrlEx(
        hResolver: HINTERNET, pcwszUrl: PCWSTR, pAutoProxyOptions: *mut WINHTTP_AUTOPROXY_OPTIONS,
        pContext: DWORD_PTR,
    ) -> DWORD;
    pub fn WinHttpGetProxyResult(
        hResolver: HINTERNET, pProxyResult: *mut WINHTTP_PROXY_RESULT,
    ) -> DWORD;
    pub fn WinHttpOpen(
        pszAgentW: LPCWSTR, dwAccessType: DWORD, pszProxyW: LPCWSTR, pszProxyBypassW: LPCWSTR,
        dwFlags: DWORD,
    ) -> HINTERNET;
    pub fn WinHttpOpenRequest(
        hConnect: HINTERNET, pwszVerb: LPCWSTR, pwszObjectName: LPCWSTR, pwszVersion: LPCWSTR,
        pwszReferrer: LPCWSTR, ppwszAcceptTypes: *mut LPCWSTR, dwFlags: DWORD,
    ) -> HINTERNET;
    // pub fn WinHttpProbeConnectivity();
    pub fn WinHttpQueryAuthSchemes(
        hRequest: HINTERNET, lpdwSupportedSchemes: LPDWORD, lpdwFirstScheme: LPDWORD,
        pdwAuthTarget: LPDWORD,
    ) -> BOOL;
    pub fn WinHttpQueryDataAvailable(
        hRequest: HINTERNET, lpdwNumberOfBytesAvailable: LPDWORD,
    ) -> BOOL;
    pub fn WinHttpQueryHeaders(
        hRequest: HINTERNET, dwInfoLevel: DWORD, pwszName: LPCWSTR, lpBuffer: LPVOID,
        lpdwBufferLength: LPDWORD, lpdwIndex: LPDWORD,
    ) -> BOOL;
    pub fn WinHttpQueryOption(
        hInternet: HINTERNET, dwOption: DWORD, lpBuffer: LPVOID, lpdwBufferLength: LPDWORD,
    ) -> BOOL;
    pub fn WinHttpReadData(
        hRequest: HINTERNET, lpBuffer: LPVOID, dwNumberOfBytesToRead: DWORD,
        lpdwNumberOfBytesRead: LPDWORD,
    ) -> BOOL;
    pub fn WinHttpReceiveResponse(hRequest: HINTERNET, lpReserved: LPVOID) -> BOOL;
    pub fn WinHttpResetAutoProxy(hSession: HINTERNET, dwFlags: DWORD) -> DWORD;
    // pub fn WinHttpSaveProxyCredentials();
    pub fn WinHttpSendRequest(
        hRequest: HINTERNET, lpszHeaders: LPCWSTR, dwHeadersLength: DWORD, lpOptional: LPVOID,
        dwOptionalLength: DWORD, dwTotalLength: DWORD, dwContext: DWORD_PTR,
    ) -> BOOL;
    pub fn WinHttpSetCredentials(
        hRequest: HINTERNET, AuthTargets: DWORD, AuthScheme: DWORD, pwszUserName: LPCWSTR,
        pwszPassword: LPCWSTR, pAuthParams: LPVOID,
    ) -> BOOL;
    pub fn WinHttpSetDefaultProxyConfiguration(pProxyInfo: *mut WINHTTP_PROXY_INFO) -> BOOL;
    pub fn WinHttpSetOption(
        hInternet: HINTERNET, dwOption: DWORD, lpBuffer: LPVOID, dwBufferLength: DWORD,
    ) -> BOOL;
    pub fn WinHttpSetStatusCallback(
        hInternet: HINTERNET, lpfnInternetCallback: WINHTTP_STATUS_CALLBACK,
        dwNotificationFlags: DWORD, dwReserved: DWORD_PTR,
    ) -> WINHTTP_STATUS_CALLBACK;
    pub fn WinHttpSetTimeouts(
        hInternet: HINTERNET, nResolveTimeout: c_int, nConnectTimeout: c_int, nSendTimeout: c_int,
        nReceiveTimeout: c_int,
    ) -> BOOL;
    pub fn WinHttpTimeFromSystemTime(pst: *const SYSTEMTIME, pwszTime: LPWSTR) -> BOOL;
    pub fn WinHttpTimeToSystemTime(pwszTime: LPCWSTR, pst: *mut SYSTEMTIME) -> BOOL;
    pub fn WinHttpWebSocketClose(
        hWebSocket: HINTERNET, usStatus: USHORT, pvReason: PVOID, dwReasonLength: DWORD,
    ) -> DWORD;
    pub fn WinHttpWebSocketCompleteUpgrade(hRequest: HINTERNET, pContext: DWORD_PTR) -> HINTERNET;
    pub fn WinHttpWebSocketQueryCloseStatus(
        hWebSocket: HINTERNET, pusStatus: *mut USHORT, pvReason: PVOID, dwReasonLength: DWORD,
        pdwReasonLengthConsumed: *mut DWORD,
    ) -> DWORD;
    pub fn WinHttpWebSocketReceive(
        hWebSocket: HINTERNET, pvBuffer: PVOID, dwBufferLength: DWORD, pdwBytesRead: *mut DWORD,
        peBufferType: *mut WINHTTP_WEB_SOCKET_BUFFER_TYPE,
    ) -> DWORD;
    pub fn WinHttpWebSocketSend(
        hWebSocket: HINTERNET, eBufferType: WINHTTP_WEB_SOCKET_BUFFER_TYPE, pvBuffer: PVOID,
        dwBufferLength: DWORD,
    ) -> DWORD;
    pub fn WinHttpWebSocketShutdown(
        hWebSocket: HINTERNET, usStatus: USHORT, pvReason: PVOID, dwReasonLength: DWORD,
    ) -> DWORD;
    pub fn WinHttpWriteData(
        hRequest: HINTERNET, lpBuffer: LPCVOID, dwNumberOfBytesToWrite: DWORD,
        lpdwNumberOfBytesWritten: LPDWORD,
    ) -> BOOL;
}
