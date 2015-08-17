// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
#![cfg(windows)]
extern crate bcrypt;
use bcrypt::*;
#[inline(never)] fn bb<T>(_: T) {}
#[test] #[cfg(target_env = "msvc")]
fn functions_msvc() {
    bb(BCryptCreateMultiHash);
    bb(BCryptDeriveKeyCapi);
    bb(BCryptDeriveKeyPBKDF2);
    bb(BCryptKeyDerivation);
    bb(BCryptProcessMultiOperations);
}
#[test]
fn functions() {
    bb(BCryptAddContextFunction);
    bb(BCryptCloseAlgorithmProvider);
    bb(BCryptConfigureContext);
    bb(BCryptConfigureContextFunction);
    bb(BCryptCreateContext);
    bb(BCryptCreateHash);
    bb(BCryptDecrypt);
    bb(BCryptDeleteContext);
    bb(BCryptDeriveKey);
    bb(BCryptDestroyHash);
    bb(BCryptDestroyKey);
    bb(BCryptDestroySecret);
    bb(BCryptDuplicateHash);
    bb(BCryptDuplicateKey);
    bb(BCryptEncrypt);
    bb(BCryptEnumAlgorithms);
    bb(BCryptEnumContextFunctionProviders);
    bb(BCryptEnumContextFunctions);
    bb(BCryptEnumContexts);
    bb(BCryptEnumProviders);
    bb(BCryptEnumRegisteredProviders);
    bb(BCryptExportKey);
    bb(BCryptFinalizeKeyPair);
    bb(BCryptFinishHash);
    bb(BCryptFreeBuffer);
    bb(BCryptGenRandom);
    bb(BCryptGenerateKeyPair);
    bb(BCryptGenerateSymmetricKey);
    bb(BCryptGetFipsAlgorithmMode);
    bb(BCryptGetProperty);
    bb(BCryptHashData);
    bb(BCryptImportKey);
    bb(BCryptImportKeyPair);
    bb(BCryptOpenAlgorithmProvider);
    bb(BCryptQueryContextConfiguration);
    bb(BCryptQueryContextFunctionConfiguration);
    bb(BCryptQueryContextFunctionProperty);
    bb(BCryptQueryProviderRegistration);
    bb(BCryptRegisterConfigChangeNotify);
    bb(BCryptRemoveContextFunction);
    bb(BCryptResolveProviders);
    bb(BCryptSecretAgreement);
    bb(BCryptSetContextFunctionProperty);
    bb(BCryptSetProperty);
    bb(BCryptSignHash);
    bb(BCryptUnregisterConfigChangeNotify);
    bb(BCryptVerifySignature);
}
