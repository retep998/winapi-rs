// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to secur32.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    // pub fn AcceptSecurityContext();
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
    // pub fn ApplyControlToken();
    // pub fn ChangeAccountPasswordA();
    // pub fn ChangeAccountPasswordW();
    // pub fn CompleteAuthToken();
    // pub fn DecryptMessage();
    // pub fn DeleteSecurityContext();
    // pub fn DeleteSecurityPackageA();
    // pub fn DeleteSecurityPackageW();
    // pub fn EncryptMessage();
    // pub fn EnumerateSecurityPackagesA();
    // pub fn EnumerateSecurityPackagesW();
    // pub fn ExportSecurityContext();
    // pub fn FreeContextBuffer();
    pub fn FreeCredentialsHandle(phCredential: PCredHandle) -> SECURITY_STATUS;
    // pub fn GetComputerObjectNameA();
    // pub fn GetComputerObjectNameW();
    // pub fn GetSecurityUserInfo();
    // pub fn GetUserNameExA();
    // pub fn GetUserNameExW();
    // pub fn ImpersonateSecurityContext();
    // pub fn ImportSecurityContextA();
    // pub fn ImportSecurityContextW();
    // pub fn InitSecurityInterfaceA();
    // pub fn InitSecurityInterfaceW();
    // pub fn InitializeSecurityContextA();
    // pub fn InitializeSecurityContextW();
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
    // pub fn MakeSignature();
    // pub fn QueryContextAttributesA();
    // pub fn QueryContextAttributesW();
    // pub fn QueryCredentialsAttributesA();
    // pub fn QueryCredentialsAttributesW();
    // pub fn QuerySecurityContextToken();
    // pub fn QuerySecurityPackageInfoA();
    // pub fn QuerySecurityPackageInfoW();
    // pub fn RevertSecurityContext();
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
    // pub fn SetContextAttributesA();
    // pub fn SetContextAttributesW();
    // pub fn SetCredentialsAttributesA();
    // pub fn SetCredentialsAttributesW();
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
    // pub fn VerifySignature();
}
