// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to credui.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn CredPackAuthenticationBufferA(
        dwFlags: DWORD, pszUserName: LPSTR, pszPassword: LPSTR, pPackedCredentials: PBYTE,
        pcbPackedCredentials: *mut DWORD,
    ) -> BOOL;
    pub fn CredPackAuthenticationBufferW(
        dwFlags: DWORD, pszUserName: LPWSTR, pszPassword: LPWSTR, pPackedCredentials: PBYTE,
        pcbPackedCredentials: *mut DWORD,
    ) -> BOOL;
    pub fn CredUICmdLinePromptForCredentialsA(
        pszTargetName: PCSTR, pContext: PCtxtHandle, dwAuthError: DWORD, UserName: PSTR,
        ulUserBufferSize: ULONG, pszPassword: PSTR, ulPasswordBufferSize: ULONG, pfSave: PBOOL,
        dwFlags: DWORD,
    ) -> DWORD;
    pub fn CredUICmdLinePromptForCredentialsW(
        pszTargetName: PCWSTR, pContext: PCtxtHandle, dwAuthError: DWORD, UserName: PWSTR,
        ulUserBufferSize: ULONG, pszPassword: PWSTR, ulPasswordBufferSize: ULONG, pfSave: PBOOL,
        dwFlags: DWORD,
    ) -> DWORD;
    pub fn CredUIConfirmCredentialsA(pszTargetName: PCSTR, bConfirm: BOOL) -> DWORD;
    pub fn CredUIConfirmCredentialsW(pszTargetName: PCWSTR, bConfirm: BOOL) -> DWORD;
    pub fn CredUIParseUserNameA(
        userName: PCSTR, user: *mut CHAR, userBufferSize: ULONG, domain: *mut CHAR,
        domainBufferSize: ULONG,
    ) -> DWORD;
    pub fn CredUIParseUserNameW(
        userName: PCWSTR, user: *mut WCHAR, userBufferSize: ULONG, domain: *mut WCHAR,
        domainBufferSize: ULONG,
    ) -> DWORD;
    pub fn CredUIPromptForCredentialsA(
        pUiInfo: PCREDUI_INFOA, pszTargetName: PCSTR, pContext: PCtxtHandle, dwAuthError: DWORD,
        pszUserName: PSTR, ulUserNameBufferSize: ULONG, pszPassword: PSTR,
        ulPasswordBufferSize: ULONG, save: *mut BOOL, dwFlags: DWORD,
    ) -> DWORD;
    pub fn CredUIPromptForCredentialsW(
        pUiInfo: PCREDUI_INFOW, pszTargetName: PCWSTR, pContext: PCtxtHandle, dwAuthError: DWORD,
        pszUserName: PWSTR, ulUserNameBufferSize: ULONG, pszPassword: PWSTR,
        ulPasswordBufferSize: ULONG, save: *mut BOOL, dwFlags: DWORD,
    ) -> DWORD;
    pub fn CredUIPromptForWindowsCredentialsA(
        pUiInfo: PCREDUI_INFOA, dwAuthError: DWORD, pulAuthPackage: *mut ULONG,
        pvInAuthBuffer: LPCVOID, ulInAuthBufferSize: ULONG, ppvOutAuthBuffer: *mut LPVOID,
        pulOutAuthBufferSize: *mut ULONG, pfSave: *mut BOOL, dwFlags: DWORD,
    ) -> DWORD;
    pub fn CredUIPromptForWindowsCredentialsW(
        pUiInfo: PCREDUI_INFOW, dwAuthError: DWORD, pulAuthPackage: *mut ULONG,
        pvInAuthBuffer: LPCVOID, ulInAuthBufferSize: ULONG, ppvOutAuthBuffer: *mut LPVOID,
        pulOutAuthBufferSize: *mut ULONG, pfSave: *mut BOOL, dwFlags: DWORD,
    ) -> DWORD;
    pub fn CredUIReadSSOCredW(pszRealm: PCWSTR, ppszUsername: *mut PWSTR) -> DWORD;
    pub fn CredUIStoreSSOCredW(
        pszRealm: PCWSTR, pszUsername: PCWSTR, pszPassword: PCWSTR, bPersist: BOOL,
    ) -> DWORD;
    pub fn CredUnPackAuthenticationBufferA(
        dwFlags: DWORD, pAuthBuffer: PVOID, cbAuthBuffer: DWORD, pszUserName: LPSTR,
        pcchlMaxUserName: *mut DWORD, pszDomainName: LPSTR, pcchMaxDomainName: *mut DWORD,
        pszPassword: LPSTR, pcchMaxPassword: *mut DWORD,
    ) -> BOOL;
    pub fn CredUnPackAuthenticationBufferW(
        dwFlags: DWORD, pAuthBuffer: PVOID, cbAuthBuffer: DWORD, pszUserName: LPWSTR,
        pcchlMaxUserName: *mut DWORD, pszDomainName: LPWSTR, pcchMaxDomainName: *mut DWORD,
        pszPassword: LPWSTR, pcchMaxPassword: *mut DWORD,
    ) -> BOOL;
    pub fn SspiGetCredUIContext(
        ContextHandle: HANDLE, CredType: *mut GUID, LogonId: *mut LUID,
        CredUIContexts: *mut PSEC_WINNT_CREDUI_CONTEXT_VECTOR, TokenHandle: *mut HANDLE,
    ) -> SECURITY_STATUS;
    pub fn SspiIsPromptingNeeded(ErrorOrNtStatus: c_ulong) -> BOOLEAN;
    pub fn SspiPromptForCredentialsA(
        pszTargetName: PCSTR, pUiInfo: PCREDUI_INFOA, dwAuthError: c_ulong, pszPackage: PCSTR,
        pInputAuthIdentity: PSEC_WINNT_AUTH_IDENTITY_OPAQUE,
        ppAuthIdentity: *mut PSEC_WINNT_AUTH_IDENTITY_OPAQUE, pfSave: *mut c_int, dwFlags: c_ulong,
    ) -> c_ulong;
    pub fn SspiPromptForCredentialsW(
        pszTargetName: PCWSTR, pUiInfo: PCREDUI_INFOW, dwAuthError: c_ulong, pszPackage: PCWSTR,
        pInputAuthIdentity: PSEC_WINNT_AUTH_IDENTITY_OPAQUE,
        ppAuthIdentity: *mut PSEC_WINNT_AUTH_IDENTITY_OPAQUE, pfSave: *mut c_int, dwFlags: c_ulong,
    ) -> c_ulong;
    pub fn SspiUnmarshalCredUIContext(
        MarshaledCredUIContext: PUCHAR, MarshaledCredUIContextLength: ULONG,
        CredUIContext: *mut PSEC_WINNT_CREDUI_CONTEXT,
    ) -> SECURITY_STATUS;
    pub fn SspiUpdateCredentials(
        ContextHandle: HANDLE, CredType: *mut GUID, FlatCredUIContextLength: ULONG,
        FlatCredUIContext: PUCHAR,
    ) -> SECURITY_STATUS;
}
