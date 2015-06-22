// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to secur32.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn AcceptSecurityContext(
        phCredential: PCredHandle, phContext: PCtxtHandle, pInput: PSecBufferDesc,
        fContextReq: c_ulong, TargetDataRep: c_ulong, phNewContext: PCtxtHandle,
        pOutput: PSecBufferDesc, pfContextAttr: *mut c_ulong, ptsExpiry: PTimeStamp,
    ) -> SECURITY_STATUS;
    pub fn AcquireCredentialsHandleA(
        pszPrincipal: LPSTR, pszPackage: LPSTR, fCredentialUse: c_ulong, pvLogonId: *mut c_void,
        pAuthData: *mut c_void, pGetKeyFn: SEC_GET_KEY_FN, pvGetKeyArgument: *mut c_void,
        phCredential: PCredHandle, ptsExpiry: PTimeStamp,
    ) -> SECURITY_STATUS;
    pub fn AcquireCredentialsHandleW(
        pszPrincipal: LPWSTR, pszPackage: LPWSTR, fCredentialUse: c_ulong, pvLogonId: *mut c_void,
        pAuthData: *mut c_void, pGetKeyFn: SEC_GET_KEY_FN, pvGetKeyArgument: *mut c_void,
        phCredential: PCredHandle, ptsExpiry: PTimeStamp,
    ) -> SECURITY_STATUS;
    pub fn AddCredentialsA(
        hCredentials: PCredHandle, pszPrincipal: LPSTR, pszPackage: LPSTR, fCredentialUse: c_ulong,
        pAuthData: *mut c_void, pGetKeyFn: SEC_GET_KEY_FN, pvGetKeyArgument: *mut c_void,
        ptsExpiry: PTimeStamp,
    ) -> SECURITY_STATUS;
    pub fn AddCredentialsW(
        hCredentials: PCredHandle, pszPrincipal: LPWSTR, pszPackage: LPWSTR,
        fCredentialUse: c_ulong, pAuthData: *mut c_void, pGetKeyFn: SEC_GET_KEY_FN,
        pvGetKeyArgument: *mut c_void, ptsExpiry: PTimeStamp,
    ) -> SECURITY_STATUS;
    // pub fn AddSecurityPackageA();
    // pub fn AddSecurityPackageW();
    pub fn ApplyControlToken(phContext: PCtxtHandle, pInput: PSecBufferDesc) -> SECURITY_STATUS;
    pub fn ChangeAccountPasswordA(
        pszPackageName: *mut SEC_CHAR, pszDomainName: *mut SEC_CHAR, pszAccountName: *mut SEC_CHAR, pszOldPassword: *mut SEC_CHAR, pszNewPassword: *mut SEC_CHAR, bImpersonating: BOOLEAN,
        dwReserved: c_ulong, pOutput: PSecBufferDesc,
    ) -> SECURITY_STATUS;
    pub fn ChangeAccountPasswordW(
        pszPackageName: *mut SEC_WCHAR, pszDomainName: *mut SEC_WCHAR,
        pszAccountName: *mut SEC_WCHAR, pszOldPassword: *mut SEC_WCHAR,
        pszNewPassword: *mut SEC_WCHAR, bImpersonating: BOOLEAN, dwReserved: c_ulong,
        pOutput: PSecBufferDesc,
    ) -> SECURITY_STATUS;
    pub fn CompleteAuthToken(phContext: PCtxtHandle, pToken: PSecBufferDesc) -> SECURITY_STATUS;
    pub fn DecryptMessage(
        phContext: PCtxtHandle, pMessage: PSecBufferDesc, MessageSeqNo: c_ulong,
        pfQOP: *mut c_ulong,
    ) -> SECURITY_STATUS;
    pub fn DeleteSecurityContext(phContext: PCtxtHandle) -> SECURITY_STATUS;
    // pub fn DeleteSecurityPackageA();
    // pub fn DeleteSecurityPackageW();
    pub fn EncryptMessage(
        phContext: PCtxtHandle, fQOP: c_ulong, pMessage: PSecBufferDesc, MessageSeqNo: c_ulong,
    ) -> SECURITY_STATUS;
    pub fn EnumerateSecurityPackagesA(
        pcPackages: *mut c_ulong, ppPackageInfo: *mut PSecPkgInfoA,
    ) -> SECURITY_STATUS;
    pub fn EnumerateSecurityPackagesW(
        pcPackages: *mut c_ulong, ppPackageInfo: *mut PSecPkgInfoW,
    ) -> SECURITY_STATUS;
    pub fn ExportSecurityContext(
        phContext: PCtxtHandle, fFlags: ULONG, pPackedContext: PSecBuffer,
        pToken: *mut *mut c_void,
    ) -> SECURITY_STATUS;
    pub fn FreeContextBuffer(pvContextBuffer: PVOID) -> SECURITY_STATUS;
    pub fn FreeCredentialsHandle(phCredential: PCredHandle) -> SECURITY_STATUS;
    // pub fn GetComputerObjectNameA();
    // pub fn GetComputerObjectNameW();
    // pub fn GetSecurityUserInfo();
    // pub fn GetUserNameExA();
    // pub fn GetUserNameExW();
    pub fn ImpersonateSecurityContext(phContext: PCtxtHandle) -> SECURITY_STATUS;
    pub fn ImportSecurityContextA(
        pszPackage: LPSTR, pPackedContext: PSecBuffer, Token: *mut c_void, phContext: PCtxtHandle,
    ) -> SECURITY_STATUS;
    pub fn ImportSecurityContextW(
        pszPackage: LPWSTR, pPackedContext: PSecBuffer, Token: *mut c_void, phContext: PCtxtHandle,
    ) -> SECURITY_STATUS;
    // pub fn InitSecurityInterfaceA();
    // pub fn InitSecurityInterfaceW();
    pub fn InitializeSecurityContextA(
        phCredential: PCredHandle, phContext: PCtxtHandle, pszTargetName: *mut SEC_CHAR,
        fContextReq: c_ulong, Reserved1: c_ulong, TargetDataRep: c_ulong, pInput: PSecBufferDesc,
        Reserved2: c_ulong, phNewContext: PCtxtHandle, pOutput: PSecBufferDesc,
        pfContextAttr: *mut c_ulong, ptsExpiry: PTimeStamp,
    ) -> SECURITY_STATUS;
    pub fn InitializeSecurityContextW(
        phCredential: PCredHandle, phContext: PCtxtHandle, pszTargetName: *mut SEC_WCHAR,
        fContextReq: c_ulong, Reserved1: c_ulong, TargetDataRep: c_ulong, pInput: PSecBufferDesc,
        Reserved2: c_ulong, phNewContext: PCtxtHandle, pOutput: PSecBufferDesc,
        pfContextAttr: *mut c_ulong, ptsExpiry: PTimeStamp,
    ) -> SECURITY_STATUS;
    // pub fn LsaCallAuthenticationPackage();
    // pub fn LsaConnectUntrusted();
    // pub fn LsaDeregisterLogonProcess();
    // pub fn LsaEnumerateLogonSessions();
    // pub fn LsaFreeReturnBuffer();
    // pub fn LsaGetLogonSessionData();
    // pub fn LsaLogonUser();
    // pub fn LsaLookupAuthenticationPackage();
    // pub fn LsaRegisterLogonProcess();
    // pub fn LsaRegisterPolicyChangeNotification();
    // pub fn LsaUnregisterPolicyChangeNotification();
    pub fn MakeSignature(
        phContext: PCtxtHandle, fQOP: c_ulong, pMessage: PSecBufferDesc, MessageSeqNo: c_ulong,
    ) -> SECURITY_STATUS;
    pub fn QueryContextAttributesA(
        phContext: PCtxtHandle, ulAttribute: c_ulong, pBuffer: *mut c_void,
    -> SECURITY_STATUS;
    pub fn QueryContextAttributesW(
        phContext: PCtxtHandle, ulAttribute: c_ulong, pBuffer: *mut c_void,
    -> SECURITY_STATUS;
    pub fn QueryCredentialsAttributesA(
        phCredential: PCredHandle, ulAttribute: c_ulong, pBuffer: *mut c_void,
    ) -> SECURITY_STATUS;
    pub fn QueryCredentialsAttributesW(
        phCredential: PCredHandle, ulAttribute: c_ulong, pBuffer: *mut c_void,
    ) -> SECURITY_STATUS;
    pub fn QuerySecurityContextToken(
        phContext: PCtxtHandle, Token: *mut *mut c_void,
    ) -> SECURITY_STATUS;
    pub fn QuerySecurityPackageInfoA(
        pszPackageName: LPSTR, ppPackageInfo: *mut PSecPkgInfoA,
    ) -> SECURITY_STATUS;
    pub fn QuerySecurityPackageInfoW(
        pszPackageName: LPWSTR, ppPackageInfo: *mut PSecPkgInfoW,
    ) -> SECURITY_STATUS;
    pub fn RevertSecurityContext(phContext: PCtxtHandle) -> SECURITY_STATUS;
    // pub fn SaslAcceptSecurityContext();
    // pub fn SaslEnumerateProfilesA();
    // pub fn SaslEnumerateProfilesW();
    // pub fn SaslGetContextOption();
    // pub fn SaslGetProfilePackageA();
    // pub fn SaslGetProfilePackageW();
    // pub fn SaslIdentifyPackageA();
    // pub fn SaslIdentifyPackageW();
    // pub fn SaslInitializeSecurityContextA();
    // pub fn SaslInitializeSecurityContextW();
    // pub fn SaslSetContextOption();
    // pub fn SealMessage();
    // pub fn SeciAllocateAndSetCallFlags();
    // pub fn SeciAllocateAndSetIPAddress();
    // pub fn SeciFreeCallContext();
    pub fn SetContextAttributesA(
        phContext: PCtxtHandle, ulAttribute: c_ulong, pBuffer: *mut c_void, cbBuffer: c_ulong,
    ) -> SECURITY_STATUS;
    pub fn SetContextAttributesW(
        phContext: PCtxtHandle, ulAttribute: c_ulong, pBuffer: *mut c_void, cbBuffer: c_ulong,
    ) -> SECURITY_STATUS;
    pub fn SetCredentialsAttributesA(
        phCredential: PCredHandle, ulAttribute: c_ulong, pBuffer: *mut c_void, cbBuffer: c_ulong,
    ) -> SECURITY_STATUS;
    pub fn SetCredentialsAttributesW(
        phCredential: PCredHandle, ulAttribute: c_ulong, pBuffer: *mut c_void, cbBuffer: c_ulong,
    ) -> SECURITY_STATUS;
    // pub fn SspiCompareAuthIdentities();
    // pub fn SspiCopyAuthIdentity();
    // pub fn SspiDecryptAuthIdentity();
    // pub fn SspiEncodeAuthIdentityAsStrings();
    // pub fn SspiEncodeStringsAsAuthIdentity();
    // pub fn SspiEncryptAuthIdentity();
    // pub fn SspiExcludePackage();
    // pub fn SspiFreeAuthIdentity();
    // pub fn SspiGetTargetHostName();
    // pub fn SspiIsAuthIdentityEncrypted();
    // pub fn SspiLocalFree();
    // pub fn SspiMarshalAuthIdentity();
    // pub fn SspiPrepareForCredRead();
    // pub fn SspiPrepareForCredWrite();
    // pub fn SspiUnmarshalAuthIdentity();
    // pub fn SspiValidateAuthIdentity();
    // pub fn SspiZeroAuthIdentity();
    // pub fn TranslateNameA();
    // pub fn TranslateNameW();
    // pub fn UnsealMessage();
    pub fn VerifySignature(
        phContext: PCtxtHandle, pMessage: PSecBufferDesc, MessageSeqNo: c_ulong,
        pfQOP: *mut c_ulong,
    ) -> SECURITY_STATUS;
}
