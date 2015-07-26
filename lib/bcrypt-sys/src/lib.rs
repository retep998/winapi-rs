// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to bcrypt.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn BCryptAddContextFunction(
        dwTable: ULONG, pszContext: LPCWSTR, dwInterface: ULONG, pszFunction: LPCWSTR,
        dwPosition: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptCloseAlgorithmProvider(hAlgorithm: BCRYPT_ALG_HANDLE, dwFlags: ULONG) -> NTSTATUS;
    pub fn BCryptConfigureContext(
        dwTable: ULONG, pszContext: LPCWSTR, pConfig: PCRYPT_CONTEXT_CONFIG,
    ) -> NTSTATUS;
    pub fn BCryptConfigureContextFunction(
        dwTable: ULONG, pszContext: LPCWSTR, dwInterface: ULONG, pszFunction: LPCWSTR,
        pConfig: PCRYPT_CONTEXT_FUNCTION_CONFIG,
    ) -> NTSTATUS;
    pub fn BCryptCreateContext(
        dwTable: ULONG, pszContext: LPCWSTR, pConfig: PCRYPT_CONTEXT_CONFIG,
    ) -> NTSTATUS;
    pub fn BCryptCreateHash(
        hAlgorithm: BCRYPT_ALG_HANDLE, phHash: *mut BCRYPT_HASH_HANDLE, pbHashObject: PUCHAR,
        cbHashObject: ULONG, pbSecret: PUCHAR, cbSecret: ULONG, dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptCreateMultiHash(
        hAlgorithm: BCRYPT_ALG_HANDLE, phHash: *mut BCRYPT_HASH_HANDLE, nHashes: ULONG,
        pbHashObject: PUCHAR, cbHashObject: ULONG, pbSecret: PUCHAR, cbSecret: ULONG,
        dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptDecrypt(
        hKey: BCRYPT_KEY_HANDLE, pbInput: PUCHAR, cbInput: ULONG, pPaddingInfo: *mut VOID,
        pbIV: PUCHAR, cbIV: ULONG, pbOutput: PUCHAR, cbOutput: ULONG, pcbResult: *mut ULONG,
        dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptDeleteContext(dwTable: ULONG, pszContext: LPCWSTR) -> NTSTATUS;
    pub fn BCryptDeriveKey(
        hSharedSecret: BCRYPT_SECRET_HANDLE, pwszKDF: LPCWSTR,
        pParameterList: *mut BCryptBufferDesc, pbDerivedKey: PUCHAR, cbDerivedKey: ULONG,
        pcbResult: *mut ULONG, dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptDeriveKeyCapi(
        hHash: BCRYPT_HASH_HANDLE, hTargetAlg: BCRYPT_ALG_HANDLE, pbDerivedKey: PUCHAR,
        cbDerivedKey: ULONG, dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptDeriveKeyPBKDF2(
        hPrf: BCRYPT_ALG_HANDLE, pbPassword: PUCHAR, cbPassword: ULONG, pbSalt: PUCHAR,
        cbSalt: ULONG, cIterations: ULONGLONG, pbDerivedKey: PUCHAR, cbDerivedKey: ULONG,
        dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptDestroyHash(hHash: BCRYPT_HASH_HANDLE) -> NTSTATUS;
    pub fn BCryptDestroyKey(hKey: BCRYPT_KEY_HANDLE) -> NTSTATUS;
    pub fn BCryptDestroySecret(hSecret: BCRYPT_SECRET_HANDLE) -> NTSTATUS;
    pub fn BCryptDuplicateHash(
        hHash: BCRYPT_HASH_HANDLE, phNewHash: *mut BCRYPT_HASH_HANDLE, pbHashObject: PUCHAR,
        cbHashObject: ULONG, dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptDuplicateKey(
        hKey: BCRYPT_KEY_HANDLE, phNewKey: *mut BCRYPT_KEY_HANDLE, pbKeyObject: PUCHAR,
        cbKeyObject: ULONG, dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptEncrypt(
        hKey: BCRYPT_KEY_HANDLE, pbInput: PUCHAR, cbInput: ULONG, pPaddingInfo: *mut VOID,
        pbIV: PUCHAR, cbIV: ULONG, pbOutput: PUCHAR, cbOutput: ULONG, pcbResult: *mut ULONG,
        dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptEnumAlgorithms(
        dwAlgOperations: ULONG, pAlgCount: *mut ULONG,
        ppAlgList: *mut *mut BCRYPT_ALGORITHM_IDENTIFIER, dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptEnumContextFunctionProviders(
        dwTable: ULONG, pszContext: LPCWSTR, dwInterface: ULONG, pszFunction: LPCWSTR,
        pcbBuffer: *mut ULONG, ppBuffer: *mut PCRYPT_CONTEXT_FUNCTION_PROVIDERS,
    ) -> NTSTATUS;
    pub fn BCryptEnumContextFunctions(
        dwTable: ULONG, pszContext: LPCWSTR, dwInterface: ULONG, pcbBuffer: *mut ULONG,
        ppBuffer: *mut PCRYPT_CONTEXT_FUNCTIONS,
    ) -> NTSTATUS;
    pub fn BCryptEnumContexts(
        dwTable: ULONG, pcbBuffer: *mut ULONG, ppBuffer: *mut PCRYPT_CONTEXTS,
    ) -> NTSTATUS;
    pub fn BCryptEnumProviders(
        pszAlgId: LPCWSTR, pImplCount: *mut ULONG, ppImplList: *mut *mut BCRYPT_PROVIDER_NAME,
        dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptEnumRegisteredProviders(
        pcbBuffer: *mut ULONG, ppBuffer: *mut PCRYPT_PROVIDERS,
    ) -> NTSTATUS;
    pub fn BCryptExportKey(
        hKey: BCRYPT_KEY_HANDLE, hExportKey: BCRYPT_KEY_HANDLE, pszBlobType: LPCWSTR,
        pbOutput: PUCHAR, cbOutput: ULONG, pcbResult: *mut ULONG, dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptFinalizeKeyPair(hKey: BCRYPT_KEY_HANDLE, dwFlags: ULONG) -> NTSTATUS;
    pub fn BCryptFinishHash(
        hHash: BCRYPT_HASH_HANDLE, pbOutput: PUCHAR, cbOutput: ULONG, dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptFreeBuffer(pvBuffer: PVOID);
    pub fn BCryptGenRandom(
        hAlgorithm: BCRYPT_ALG_HANDLE, pbBuffer: PUCHAR, cbBuffer: ULONG, dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptGenerateKeyPair(
        hAlgorithm: BCRYPT_ALG_HANDLE, phKey: *mut BCRYPT_KEY_HANDLE, dwLength: ULONG,
        dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptGenerateSymmetricKey(
        hAlgorithm: BCRYPT_ALG_HANDLE, phKey: *mut BCRYPT_KEY_HANDLE, pbKeyObject: PUCHAR,
        cbKeyObject: ULONG, pbSecret: PUCHAR, cbSecret: ULONG, dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptGetFipsAlgorithmMode(pfEnabled: *mut BOOLEAN) -> NTSTATUS;
    pub fn BCryptGetProperty(
        hObject: BCRYPT_HANDLE, pszProperty: LPCWSTR, pbOutput: PUCHAR, cbOutput: ULONG,
        pcbResult: *mut ULONG, dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptHashData(
        hHash: BCRYPT_HASH_HANDLE, pbInput: PUCHAR, cbInput: ULONG, dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptImportKey(
        hAlgorithm: BCRYPT_ALG_HANDLE, hImportKey: BCRYPT_KEY_HANDLE, pszBlobType: LPCWSTR,
        phKey: *mut BCRYPT_KEY_HANDLE, pbKeyObject: PUCHAR, cbKeyObject: ULONG, pbInput: PUCHAR,
        cbInput: ULONG, dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptImportKeyPair(
        hAlgorithm: BCRYPT_ALG_HANDLE, hImportKey: BCRYPT_KEY_HANDLE, pszBlobType: LPCWSTR,
        phKey: *mut BCRYPT_KEY_HANDLE, pbInput: PUCHAR, cbInput: ULONG, dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptKeyDerivation(
        hKey: BCRYPT_KEY_HANDLE, pParameterList: *mut BCryptBufferDesc, pbDerivedKey: PUCHAR,
        cbDerivedKey: ULONG, pcbResult: *mut ULONG, dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptOpenAlgorithmProvider(
        phAlgorithm: *mut BCRYPT_ALG_HANDLE, pszAlgId: LPCWSTR, pszImplementation: LPCWSTR,
        dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptProcessMultiOperations(
        hObject: BCRYPT_HANDLE, operationType: BCRYPT_MULTI_OPERATION_TYPE, pOperations: PVOID,
        cbOperations: ULONG, dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptQueryContextConfiguration(
        dwTable: ULONG, pszContext: LPCWSTR, pcbBuffer: *mut ULONG,
        ppBuffer: *mut PCRYPT_CONTEXT_CONFIG,
    ) -> NTSTATUS;
    pub fn BCryptQueryContextFunctionConfiguration(
        dwTable: ULONG, pszContext: LPCWSTR, dwInterface: ULONG, pszFunction: LPCWSTR,
        pcbBuffer: *mut ULONG, ppBuffer: *mut PCRYPT_CONTEXT_FUNCTION_CONFIG,
    ) -> NTSTATUS;
    pub fn BCryptQueryContextFunctionProperty(
        dwTable: ULONG, pszContext: LPCWSTR, dwInterface: ULONG, pszFunction: LPCWSTR,
        pszProperty: LPCWSTR, pcbValue: *mut ULONG, ppbValue: *mut PUCHAR,
    ) -> NTSTATUS;
    pub fn BCryptQueryProviderRegistration(
        pszProvider: LPCWSTR, dwMode: ULONG, dwInterface: ULONG, pcbBuffer: *mut ULONG,
        ppBuffer: *mut PCRYPT_PROVIDER_REG,
    ) -> NTSTATUS;
    pub fn BCryptRegisterConfigChangeNotify(phEvent: *mut HANDLE) -> NTSTATUS;
    pub fn BCryptRemoveContextFunction(
        dwTable: ULONG, pszContext: LPCWSTR, dwInterface: ULONG, pszFunction: LPCWSTR,
    ) -> NTSTATUS;
    pub fn BCryptResolveProviders(
        pszContext: LPCWSTR, dwInterface: ULONG, pszFunction: LPCWSTR, pszProvider: LPCWSTR,
        dwMode: ULONG, dwFlags: ULONG, pcbBuffer: *mut ULONG, ppBuffer: *mut PCRYPT_PROVIDER_REFS,
    ) -> NTSTATUS;
    pub fn BCryptSecretAgreement(
        hPrivKey: BCRYPT_KEY_HANDLE, hPubKey: BCRYPT_KEY_HANDLE,
        phAgreedSecret: *mut BCRYPT_SECRET_HANDLE, dwFlags: ULONG,
    ) -> NTSTATUS;
    // pub fn BCryptSetAuditingInterface();
    pub fn BCryptSetContextFunctionProperty(
        dwTable: ULONG, pszContext: LPCWSTR, dwInterface: ULONG, pszFunction: LPCWSTR,
        pszProperty: LPCWSTR, cbValue: ULONG, pbValue: PUCHAR,
    ) -> NTSTATUS;
    pub fn BCryptSetProperty(
        hObject: BCRYPT_HANDLE, pszProperty: LPCWSTR, pbInput: PUCHAR, cbInput: ULONG,
        dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptSignHash(
        hKey: BCRYPT_KEY_HANDLE, pPaddingInfo: *mut VOID, pbInput: PUCHAR, cbInput: ULONG,
        pbOutput: PUCHAR, cbOutput: ULONG, pcbResult: *mut ULONG, dwFlags: ULONG,
    ) -> NTSTATUS;
    pub fn BCryptUnregisterConfigChangeNotify(hEvent: HANDLE) -> NTSTATUS;
    pub fn BCryptVerifySignature(
        hKey: BCRYPT_KEY_HANDLE, pPaddingInfo: *mut VOID, pbHash: PUCHAR, cbHash: ULONG,
        pbSignature: PUCHAR, cbSignature: ULONG, dwFlags: ULONG,
    ) -> NTSTATUS;
}
