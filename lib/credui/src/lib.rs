// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to credui.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
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
