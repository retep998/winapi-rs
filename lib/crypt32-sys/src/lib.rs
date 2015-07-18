// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to crypt32.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn CertAddCRLContextToStore(
        hCertStore: HCERTSTORE, pCrlContext: PCCRL_CONTEXT, dwAddDisposition: DWORD,
        ppStoreContext: *mut PCCRL_CONTEXT,
    ) -> BOOL;
    pub fn CertAddCRLLinkToStore(
        hCertStore: HCERTSTORE, pCrlContext: PCCRL_CONTEXT, dwAddDisposition: DWORD,
        ppStoreContext: *mut PCCRL_CONTEXT,
    ) -> BOOL;
    pub fn CertAddCTLContextToStore(
        hCertStore: HCERTSTORE, pCtlContext: PCCTL_CONTEXT, dwAddDisposition: DWORD,
        ppStoreContext: *mut PCCTL_CONTEXT,
    ) -> BOOL;
    pub fn CertAddCTLLinkToStore(
        hCertStore: HCERTSTORE, pCtlContext: PCCTL_CONTEXT, dwAddDisposition: DWORD,
        ppStoreContext: *mut PCCTL_CONTEXT,
    ) -> BOOL;
    pub fn CertAddCertificateContextToStore(
        hCertStore: HCERTSTORE, pCertContext: PCCERT_CONTEXT, dwAddDisposition: DWORD,
        ppStoreContext: *mut PCCERT_CONTEXT,
    ) -> BOOL;
    pub fn CertAddCertificateLinkToStore(
        hCertStore: HCERTSTORE, pCertContext: PCCERT_CONTEXT, dwAddDisposition: DWORD,
        ppStoreContext: *mut PCCERT_CONTEXT,
    ) -> BOOL;
    pub fn CertAddEncodedCRLToStore(
        hCertStore: HCERTSTORE, dwCertEncodingType: DWORD, pbCrlEncoded: *const BYTE,
        cbCrlEncoded: DWORD, dwAddDisposition: DWORD, ppCrlContext: *mut PCCRL_CONTEXT,
    ) -> BOOL;
    pub fn CertAddEncodedCTLToStore(
        hCertStore: HCERTSTORE, dwMsgAndCertEncodingType: DWORD, pbCtlEncoded: *const BYTE,
        cbCtlEncoded: DWORD, dwAddDisposition: DWORD, ppCtlContext: *mut PCCTL_CONTEXT,
    ) -> BOOL;
    pub fn CertAddEncodedCertificateToStore(
        hCertStore: HCERTSTORE, dwCertEncodingType: DWORD, pbCertEncoded: *const BYTE,
        cbCertEncoded: DWORD, dwAddDisposition: DWORD, ppCertContext: *mut PCCERT_CONTEXT,
    ) -> BOOL;
    pub fn CertAddEncodedCertificateToSystemStoreA(
        szCertStoreName: LPCSTR, pbCertEncoded: *const BYTE, cbCertEncoded: DWORD,
    ) -> BOOL;
    pub fn CertAddEncodedCertificateToSystemStoreW(
        szCertStoreName: LPCWSTR, pbCertEncoded: *const BYTE, cbCertEncoded: DWORD,
    ) -> BOOL;
    pub fn CertAddEnhancedKeyUsageIdentifier(
        pCertContext: PCCERT_CONTEXT, pszUsageIdentifier: LPCSTR,
    ) -> BOOL;
    pub fn CertAddRefServerOcspResponse(hServerOcspResponse: HCERT_SERVER_OCSP_RESPONSE);
    pub fn CertAddRefServerOcspResponseContext(
        pServerOcspResponseContext: PCCERT_SERVER_OCSP_RESPONSE_CONTEXT,
    );
    pub fn CertAddSerializedElementToStore(
        hCertStore: HCERTSTORE, pbElement: *const BYTE, cbElement: DWORD, dwAddDisposition: DWORD,
        dwFlags: DWORD, dwContextTypeFlags: DWORD, pdwContextType: *mut DWORD,
        ppvContext: *mut *const c_void,
    ) -> BOOL;
    pub fn CertAddStoreToCollection(
        hCollectionStore: HCERTSTORE, hSiblingStore: HCERTSTORE, dwUpdateFlags: DWORD,
        dwPriority: DWORD,
    ) -> BOOL;
    pub fn CertAlgIdToOID(dwAlgId: DWORD) -> LPCSTR;
    pub fn CertCloseServerOcspResponse(
        hServerOcspResponse: HCERT_SERVER_OCSP_RESPONSE, dwFlags: DWORD,
    );
    pub fn CertCloseStore(hCertStore: HCERTSTORE, dwFlags: DWORD) -> BOOL;
    pub fn CertCompareCertificate(
        dwCertEncodingType: DWORD, pCertId1: PCERT_INFO, pCertId2: PCERT_INFO,
    ) -> BOOL;
    pub fn CertCompareCertificateName(
        dwCertEncodingType: DWORD, pCertName1: PCERT_NAME_BLOB, pCertName2: PCERT_NAME_BLOB,
    ) -> BOOL;
    pub fn CertCompareIntegerBlob(pInt1: PCRYPT_INTEGER_BLOB, pInt2: PCRYPT_INTEGER_BLOB) -> BOOL;
    pub fn CertComparePublicKeyInfo(
        dwCertEncodingType: DWORD, pPublicKey1: PCERT_PUBLIC_KEY_INFO,
        pPublicKey2: PCERT_PUBLIC_KEY_INFO,
    ) -> BOOL;
    pub fn CertControlStore(
        hCertStore: HCERTSTORE, dwFlags: DWORD, dwCtrlType: DWORD, pvCtrlPara: *const c_void,
    ) -> BOOL;
    pub fn CertCreateCRLContext(
        dwCertEncodingType: DWORD, pbCrlEncoded: *const BYTE, cbCrlEncoded: DWORD,
    ) -> PCCRL_CONTEXT;
    pub fn CertCreateCTLContext(
        dwMsgAndCertEncodingType: DWORD, pbCtlEncoded: *const BYTE, cbCtlEncoded: DWORD,
    ) -> PCCTL_CONTEXT;
    pub fn CertCreateCTLEntryFromCertificateContextProperties(
        pCertContext: PCCERT_CONTEXT, cOptAttr: DWORD, rgOptAttr: PCRYPT_ATTRIBUTE, dwFlags: DWORD,
        pvReserved: *mut c_void, pCtlEntry: PCTL_ENTRY, pcbCtlEntry: *mut DWORD,
    ) -> BOOL;
    pub fn CertCreateCertificateChainEngine(
        pConfig: PCERT_CHAIN_ENGINE_CONFIG, phChainEngine: *mut HCERTCHAINENGINE,
    ) -> BOOL;
    pub fn CertCreateCertificateContext(
        dwCertEncodingType: DWORD, pbCertEncoded: *const BYTE, cbCertEncoded: DWORD,
    ) -> PCCERT_CONTEXT;
    pub fn CertCreateContext(
        dwContextType: DWORD, dwEncodingType: DWORD, pbEncoded: *const BYTE, cbEncoded: DWORD,
        dwFlags: DWORD, pCreatePara: PCERT_CREATE_CONTEXT_PARA,
    ) -> *const c_void;
    pub fn CertCreateSelfSignCertificate(
        hCryptProvOrNCryptKey: HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, pSubjectIssuerBlob: PCERT_NAME_BLOB,
        dwFlags: DWORD, pKeyProvInfo: PCRYPT_KEY_PROV_INFO,
        pSignatureAlgorithm: PCRYPT_ALGORITHM_IDENTIFIER, pStartTime: PSYSTEMTIME,
        pEndTime: PSYSTEMTIME, pExtensions: PCERT_EXTENSIONS,
    ) -> PCCERT_CONTEXT;
    pub fn CertDeleteCRLFromStore(pCrlContext: PCCRL_CONTEXT) -> BOOL;
    pub fn CertDeleteCTLFromStore(pCtlContext: PCCTL_CONTEXT) -> BOOL;
    pub fn CertDeleteCertificateFromStore(pCertContext: PCCERT_CONTEXT) -> BOOL;
    pub fn CertDuplicateCRLContext(pCrlContext: PCCRL_CONTEXT) -> PCCRL_CONTEXT;
    pub fn CertDuplicateCTLContext(pCtlContext: PCCTL_CONTEXT) -> PCCTL_CONTEXT;
    pub fn CertDuplicateCertificateChain(
        pChainContext: PCCERT_CHAIN_CONTEXT,
    ) -> PCCERT_CHAIN_CONTEXT;
    pub fn CertDuplicateCertificateContext(pCertContext: PCCERT_CONTEXT) -> PCCERT_CONTEXT;
    pub fn CertDuplicateStore(hCertStore: HCERTSTORE) -> HCERTSTORE;
    pub fn CertEnumCRLContextProperties(pCrlContext: PCCRL_CONTEXT, dwPropId: DWORD) -> DWORD;
    pub fn CertEnumCRLsInStore(
        hCertStore: HCERTSTORE, pPrevCrlContext: PCCRL_CONTEXT,
    ) -> PCCRL_CONTEXT;
    pub fn CertEnumCTLContextProperties(pCtlContext: PCCTL_CONTEXT, dwPropId: DWORD) -> DWORD;
    pub fn CertEnumCTLsInStore(
        hCertStore: HCERTSTORE, pPrevCtlContext: PCCTL_CONTEXT,
    ) -> PCCTL_CONTEXT;
    pub fn CertEnumCertificateContextProperties(
        pCertContext: PCCERT_CONTEXT, dwPropId: DWORD,
    ) -> DWORD;
    pub fn CertEnumCertificatesInStore(
        hCertStore: HCERTSTORE, pPrevCertContext: PCCERT_CONTEXT,
    ) -> PCCERT_CONTEXT;
    pub fn CertEnumPhysicalStore(
        pvSystemStore: *const c_void, dwFlags: DWORD, pvArg: *mut c_void,
        pfnEnum: PFN_CERT_ENUM_PHYSICAL_STORE,
    ) -> BOOL;
    pub fn CertEnumSubjectInSortedCTL(
        pCtlContext: PCCTL_CONTEXT, ppvNextSubject: *mut *mut c_void,
        pSubjectIdentifier: PCRYPT_DER_BLOB, pEncodedAttributes: PCRYPT_DER_BLOB,
    ) -> BOOL;
    pub fn CertEnumSystemStore(
        dwFlags: DWORD, pvSystemStoreLocationPara: *mut c_void, pvArg: *mut c_void,
        pfnEnum: PFN_CERT_ENUM_SYSTEM_STORE,
    ) -> BOOL;
    pub fn CertEnumSystemStoreLocation(
        dwFlags: DWORD, pvArg: *mut c_void, pfnEnum: PFN_CERT_ENUM_SYSTEM_STORE_LOCATION,
    ) -> BOOL;
    pub fn CertFindAttribute(
        pszObjId: LPCSTR, cAttr: DWORD, rgAttr: *mut CRYPT_ATTRIBUTE,
    ) -> PCRYPT_ATTRIBUTE;
    pub fn CertFindCRLInStore(
        hCertStore: HCERTSTORE, dwCertEncodingType: DWORD, dwFindFlags: DWORD, dwFindType: DWORD,
        pvFindPara: *const c_void, pPrevCrlContext: PCCRL_CONTEXT,
    ) -> PCCRL_CONTEXT;
    pub fn CertFindCTLInStore(
        hCertStore: HCERTSTORE, dwMsgAndCertEncodingType: DWORD, dwFindFlags: DWORD,
        dwFindType: DWORD, pvFindPara: *const c_void, pPrevCtlContext: PCCTL_CONTEXT,
    ) -> PCCTL_CONTEXT;
    pub fn CertFindCertificateInCRL(
        pCert: PCCERT_CONTEXT, pCrlContext: PCCRL_CONTEXT, dwFlags: DWORD, pvReserved: *mut c_void,
        ppCrlEntry: *mut PCRL_ENTRY,
    ) -> BOOL;
    pub fn CertFindCertificateInStore(
        hCertStore: HCERTSTORE, dwCertEncodingType: DWORD, dwFindFlags: DWORD, dwFindType: DWORD,
        pvFindPara: *const c_void, pPrevCertContext: PCCERT_CONTEXT,
    ) -> PCCERT_CONTEXT;
    pub fn CertFindChainInStore(
        hCertStore: HCERTSTORE, dwCertEncodingType: DWORD, dwFindFlags: DWORD, dwFindType: DWORD,
        pvFindPara: *const c_void, pPrevChainContext: PCCERT_CHAIN_CONTEXT,
    ) -> PCCERT_CHAIN_CONTEXT;
    pub fn CertFindExtension(
        pszObjId: LPCSTR, cExtensions: DWORD, rgExtensions: *mut CERT_EXTENSION,
    ) -> PCERT_EXTENSION;
    pub fn CertFindRDNAttr(pszObjId: LPCSTR, pName: PCERT_NAME_INFO) -> PCERT_RDN_ATTR;
    pub fn CertFindSubjectInCTL(
        dwEncodingType: DWORD, dwSubjectType: DWORD, pvSubject: *mut c_void,
        pCtlContext: PCCTL_CONTEXT, dwFlags: DWORD,
    ) -> PCTL_ENTRY;
    pub fn CertFindSubjectInSortedCTL(
        pSubjectIdentifier: PCRYPT_DATA_BLOB, pCtlContext: PCCTL_CONTEXT, dwFlags: DWORD,
        pvReserved: *mut c_void, pEncodedAttributes: PCRYPT_DER_BLOB,
    ) -> BOOL;
    pub fn CertFreeCRLContext(pCrlContext: PCCRL_CONTEXT) -> BOOL;
    pub fn CertFreeCTLContext(pCtlContext: PCCTL_CONTEXT) -> BOOL;
    pub fn CertFreeCertificateChain(pChainContext: PCCERT_CHAIN_CONTEXT);
    pub fn CertFreeCertificateChainEngine(hChainEngine: HCERTCHAINENGINE);
    pub fn CertFreeCertificateChainList(prgpSelection: *mut PCCERT_CHAIN_CONTEXT);
    pub fn CertFreeCertificateContext(pCertContext: PCCERT_CONTEXT) -> BOOL;
    pub fn CertFreeServerOcspResponseContext(
        pServerOcspResponseContext: PCCERT_SERVER_OCSP_RESPONSE_CONTEXT,
    );
    pub fn CertGetCRLContextProperty(
        pCrlContext: PCCRL_CONTEXT, dwPropId: DWORD, pvData: *mut c_void, pcbData: *mut DWORD,
    ) -> BOOL;
    pub fn CertGetCRLFromStore(
        hCertStore: HCERTSTORE, pIssuerContext: PCCERT_CONTEXT, pPrevCrlContext: PCCRL_CONTEXT,
        pdwFlags: *mut DWORD,
    ) -> PCCRL_CONTEXT;
    pub fn CertGetCTLContextProperty(
        pCtlContext: PCCTL_CONTEXT, dwPropId: DWORD, pvData: *mut c_void, pcbData: *mut DWORD,
    ) -> BOOL;
    pub fn CertGetCertificateChain(
        hChainEngine: HCERTCHAINENGINE, pCertContext: PCCERT_CONTEXT, pTime: LPFILETIME,
        hAdditionalStore: HCERTSTORE, pChainPara: PCERT_CHAIN_PARA, dwFlags: DWORD,
        pvReserved: LPVOID, ppChainContext: *mut PCCERT_CHAIN_CONTEXT,
    ) -> BOOL;
    pub fn CertGetCertificateContextProperty(
        pCertContext: PCCERT_CONTEXT, dwPropId: DWORD, pvData: *mut c_void, pcbData: *mut DWORD,
    ) -> BOOL;
    pub fn CertGetEnhancedKeyUsage(
        pCertContext: PCCERT_CONTEXT, dwFlags: DWORD, pUsage: PCERT_ENHKEY_USAGE,
        pcbUsage: *mut DWORD,
    ) -> BOOL;
    pub fn CertGetIntendedKeyUsage(
        dwCertEncodingType: DWORD, pCertInfo: PCERT_INFO, pbKeyUsage: *mut BYTE, cbKeyUsage: DWORD,
    ) -> BOOL;
    pub fn CertGetIssuerCertificateFromStore(
        hCertStore: HCERTSTORE, pSubjectContext: PCCERT_CONTEXT, pPrevIssuerContext: PCCERT_CONTEXT,
        pdwFlags: *mut DWORD,
    ) -> PCCERT_CONTEXT;
    pub fn CertGetNameStringA(
        pCertContext: PCCERT_CONTEXT, dwType: DWORD, dwFlags: DWORD, pvTypePara: *mut c_void,
        pszNameString: LPSTR, cchNameString: DWORD,
    ) -> DWORD;
    pub fn CertGetNameStringW(
        pCertContext: PCCERT_CONTEXT, dwType: DWORD, dwFlags: DWORD, pvTypePara: *mut c_void,
        pszNameString: LPWSTR, cchNameString: DWORD,
    ) -> DWORD;
    pub fn CertGetPublicKeyLength(
        dwCertEncodingType: DWORD, pPublicKey: PCERT_PUBLIC_KEY_INFO,
    ) -> DWORD;
    pub fn CertGetServerOcspResponseContext(
        hServerOcspResponse: HCERT_SERVER_OCSP_RESPONSE, dwFlags: DWORD, pvReserved: LPVOID,
    ) -> PCCERT_SERVER_OCSP_RESPONSE_CONTEXT;
    pub fn CertGetStoreProperty(
        hCertStore: HCERTSTORE, dwPropId: DWORD, pvData: *mut c_void, pcbData: *mut DWORD,
    ) -> BOOL;
    pub fn CertGetSubjectCertificateFromStore(
        hCertStore: HCERTSTORE, dwCertEncodingType: DWORD, pCertId: PCERT_INFO,
    ) -> PCCERT_CONTEXT;
    pub fn CertGetValidUsages(
        cCerts: DWORD, rghCerts: *mut PCCERT_CONTEXT, cNumOIDs: *mut c_int, rghOIDs: *mut LPSTR,
        pcbOIDs: *mut DWORD,
    ) -> BOOL;
    pub fn CertIsRDNAttrsInCertificateName(
        dwCertEncodingType: DWORD, dwFlags: DWORD, pCertName: PCERT_NAME_BLOB, pRDN: PCERT_RDN,
    ) -> BOOL;
    pub fn CertIsStrongHashToSign(
        pStrongSignPara: PCCERT_STRONG_SIGN_PARA, pwszCNGHashAlgid: LPCWSTR,
        pSigningCert: PCCERT_CONTEXT,
    ) -> BOOL;
    pub fn CertIsValidCRLForCertificate(
        pCert: PCCERT_CONTEXT, pCrl: PCCRL_CONTEXT, dwFlags: DWORD, pvReserved: *mut c_void,
    ) -> BOOL;
    pub fn CertNameToStrA(
        dwCertEncodingType: DWORD, pName: PCERT_NAME_BLOB, dwStrType: DWORD, psz: LPSTR, csz: DWORD,
    ) -> DWORD;
    pub fn CertNameToStrW(
        dwCertEncodingType: DWORD, pName: PCERT_NAME_BLOB, dwStrType: DWORD, psz: LPWSTR,
        csz: DWORD,
    ) -> DWORD;
    pub fn CertOIDToAlgId(pszObjId: LPCSTR) -> DWORD;
    pub fn CertOpenServerOcspResponse(
        pChainContext: PCCERT_CHAIN_CONTEXT, dwFlags: DWORD, pvReserved: LPVOID,
    ) -> HCERT_SERVER_OCSP_RESPONSE;
    pub fn CertOpenStore(
        lpszStoreProvider: LPCSTR, dwEncodingType: DWORD, hCryptProv: HCRYPTPROV_LEGACY,
        dwFlags: DWORD, pvPara: *const c_void,
    ) -> HCERTSTORE;
    pub fn CertOpenSystemStoreA(
        hProv: HCRYPTPROV_LEGACY, szSubsystemProtocol: LPCSTR,
    ) -> HCERTSTORE;
    pub fn CertOpenSystemStoreW(
        hProv: HCRYPTPROV_LEGACY, szSubsystemProtocol: LPCWSTR,
    ) -> HCERTSTORE;
    pub fn CertRDNValueToStrA(
        dwValueType: DWORD, pValue: PCERT_RDN_VALUE_BLOB, psz: LPSTR, csz: DWORD,
    ) -> DWORD;
    pub fn CertRDNValueToStrW(
        dwValueType: DWORD, pValue: PCERT_RDN_VALUE_BLOB, psz: LPWSTR, csz: DWORD,
    ) -> DWORD;
    pub fn CertRegisterPhysicalStore(
        pvSystemStore: *const c_void, dwFlags: DWORD, pwszStoreName: LPCWSTR,
        pStoreInfo: PCERT_PHYSICAL_STORE_INFO, pvReserved: *mut c_void,
    ) -> BOOL;
    pub fn CertRegisterSystemStore(
        pvSystemStore: *const c_void, dwFlags: DWORD, pStoreInfo: PCERT_SYSTEM_STORE_INFO,
        pvReserved: *mut c_void,
    ) -> BOOL;
    pub fn CertRemoveEnhancedKeyUsageIdentifier(
        pCertContext: PCCERT_CONTEXT, pszUsageIdentifier: LPCSTR,
    ) -> BOOL;
    pub fn CertRemoveStoreFromCollection(hCollectionStore: HCERTSTORE, hSiblingStore: HCERTSTORE);
    pub fn CertResyncCertificateChainEngine(hChainEngine: HCERTCHAINENGINE) -> BOOL;
    pub fn CertRetrieveLogoOrBiometricInfo(
        pCertContext: PCCERT_CONTEXT, lpszLogoOrBiometricType: LPCSTR, dwRetrievalFlags: DWORD,
        dwTimeout: DWORD, dwFlags: DWORD, pvReserved: *mut c_void, ppbData: *mut *mut BYTE,
        pcbData: *mut DWORD, ppwszMimeType: *mut LPWSTR,
    ) -> BOOL;
    pub fn CertSaveStore(
        hCertStore: HCERTSTORE, dwEncodingType: DWORD, dwSaveAs: DWORD, dwSaveTo: DWORD,
        pvSaveToPara: *mut c_void, dwFlags: DWORD,
    ) -> BOOL;
    pub fn CertSelectCertificateChains(
        pSelectionContext: LPCGUID, dwFlags: DWORD, pChainParameters: PCCERT_SELECT_CHAIN_PARA,
        cCriteria: DWORD, rgpCriteria: PCCERT_SELECT_CRITERIA, hStore: HCERTSTORE,
        pcSelection: PDWORD, pprgpSelection: *mut *mut PCCERT_CHAIN_CONTEXT,
    ) -> BOOL;
    pub fn CertSerializeCRLStoreElement(
        pCrlContext: PCCRL_CONTEXT, dwFlags: DWORD, pbElement: *mut BYTE, pcbElement: *mut DWORD,
    ) -> BOOL;
    pub fn CertSerializeCTLStoreElement(
        pCtlContext: PCCTL_CONTEXT, dwFlags: DWORD, pbElement: *mut BYTE, pcbElement: *mut DWORD,
    ) -> BOOL;
    pub fn CertSerializeCertificateStoreElement(
        pCertContext: PCCERT_CONTEXT, dwFlags: DWORD, pbElement: *mut BYTE, pcbElement: *mut DWORD,
    ) -> BOOL;
    pub fn CertSetCRLContextProperty(
        pCrlContext: PCCRL_CONTEXT, dwPropId: DWORD, dwFlags: DWORD, pvData: *const c_void,
    ) -> BOOL;
    pub fn CertSetCTLContextProperty(
        pCtlContext: PCCTL_CONTEXT, dwPropId: DWORD, dwFlags: DWORD, pvData: *const c_void,
    ) -> BOOL;
    pub fn CertSetCertificateContextPropertiesFromCTLEntry(
        pCertContext: PCCERT_CONTEXT, pCtlEntry: PCTL_ENTRY, dwFlags: DWORD,
    ) -> BOOL;
    pub fn CertSetCertificateContextProperty(
        pCertContext: PCCERT_CONTEXT, dwPropId: DWORD, dwFlags: DWORD, pvData: *const c_void,
    ) -> BOOL;
    pub fn CertSetEnhancedKeyUsage(
        pCertContext: PCCERT_CONTEXT, pUsage: PCERT_ENHKEY_USAGE,
    ) -> BOOL;
    pub fn CertSetStoreProperty(
        hCertStore: HCERTSTORE, dwPropId: DWORD, dwFlags: DWORD, pvData: *const c_void,
    ) -> BOOL;
    pub fn CertStrToNameA(
        dwCertEncodingType: DWORD, pszX500: LPCSTR, dwStrType: DWORD, pvReserved: *mut c_void,
        pbEncoded: *mut BYTE, pcbEncoded: *mut DWORD, ppszError: *mut LPCSTR,
    ) -> BOOL;
    pub fn CertStrToNameW(
        dwCertEncodingType: DWORD, pszX500: LPCWSTR, dwStrType: DWORD, pvReserved: *mut c_void,
        pbEncoded: *mut BYTE, pcbEncoded: *mut DWORD, ppszError: *mut LPCWSTR,
    ) -> BOOL;
    pub fn CertUnregisterPhysicalStore(
        pvSystemStore: *const c_void, dwFlags: DWORD, pwszStoreName: LPCWSTR,
    ) -> BOOL;
    pub fn CertUnregisterSystemStore(pvSystemStore: *const c_void, dwFlags: DWORD) -> BOOL;
    pub fn CertVerifyCRLRevocation(
        dwCertEncodingType: DWORD, pCertId: PCERT_INFO, cCrlInfo: DWORD, rgpCrlInfo: *mut PCRL_INFO,
    ) -> BOOL;
    pub fn CertVerifyCRLTimeValidity(pTimeToVerify: LPFILETIME, pCrlInfo: PCRL_INFO) -> LONG;
    pub fn CertVerifyCTLUsage(
        dwEncodingType: DWORD, dwSubjectType: DWORD, pvSubject: *mut c_void,
        pSubjectUsage: PCTL_USAGE, dwFlags: DWORD, pVerifyUsagePara: PCTL_VERIFY_USAGE_PARA,
        pVerifyUsageStatus: PCTL_VERIFY_USAGE_STATUS,
    ) -> BOOL;
    pub fn CertVerifyCertificateChainPolicy(
        pszPolicyOID: LPCSTR, pChainContext: PCCERT_CHAIN_CONTEXT,
        pPolicyPara: PCERT_CHAIN_POLICY_PARA, pPolicyStatus: PCERT_CHAIN_POLICY_STATUS,
    ) -> BOOL;
    pub fn CertVerifyRevocation(
        dwEncodingType: DWORD, dwRevType: DWORD, cContext: DWORD, rgpvContext: *mut PVOID,
        dwFlags: DWORD, pRevPara: PCERT_REVOCATION_PARA, pRevStatus: PCERT_REVOCATION_STATUS,
    ) -> BOOL;
    pub fn CertVerifySubjectCertificateContext(
        pSubject: PCCERT_CONTEXT, pIssuer: PCCERT_CONTEXT, pdwFlags: *mut DWORD,
    ) -> BOOL;
    pub fn CertVerifyTimeValidity(pTimeToVerify: LPFILETIME, pCertInfo: PCERT_INFO) -> LONG;
    pub fn CertVerifyValidityNesting(pSubjectInfo: PCERT_INFO, pIssuerInfo: PCERT_INFO) -> BOOL;
    pub fn CryptAcquireCertificatePrivateKey(
        pCert: PCCERT_CONTEXT, dwFlags: DWORD, pvParameters: *mut c_void,
        phCryptProvOrNCryptKey: *mut HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, pdwKeySpec: *mut DWORD,
        pfCallerFreeProvOrNCryptKey: *mut BOOL,
    ) -> BOOL;
    pub fn CryptBinaryToStringA(
        pbBinary: *const BYTE, cbBinary: DWORD, dwFlags: DWORD, pszString: LPSTR,
        pcchString: *mut DWORD,
    ) -> BOOL;
    pub fn CryptBinaryToStringW(
        pbBinary: *const BYTE, cbBinary: DWORD, dwFlags: DWORD, pszString: LPWSTR,
        pcchString: *mut DWORD,
    ) -> BOOL;
    pub fn CryptCloseAsyncHandle(hAsync: HCRYPTASYNC) -> BOOL;
    pub fn CryptCreateAsyncHandle(dwFlags: DWORD, phAsync: PHCRYPTASYNC) -> BOOL;
    pub fn CryptCreateKeyIdentifierFromCSP(
        dwCertEncodingType: DWORD, pszPubKeyOID: LPCSTR, pPubKeyStruc: *const PUBLICKEYSTRUC,
        cbPubKeyStruc: DWORD, dwFlags: DWORD, pvReserved: *mut c_void, pbHash: *mut BYTE,
        pcbHash: *mut DWORD,
    ) -> BOOL;
    pub fn CryptDecodeMessage(
        dwMsgTypeFlags: DWORD, pDecryptPara: PCRYPT_DECRYPT_MESSAGE_PARA,
        pVerifyPara: PCRYPT_VERIFY_MESSAGE_PARA, dwSignerIndex: DWORD, pbEncodedBlob: *const BYTE,
        cbEncodedBlob: DWORD, dwPrevInnerContentType: DWORD, pdwMsgType: *mut DWORD,
        pdwInnerContentType: *mut DWORD, pbDecoded: *mut BYTE, pcbDecoded: *mut DWORD,
        ppXchgCert: *mut PCCERT_CONTEXT, ppSignerCert: *mut PCCERT_CONTEXT,
    ) -> BOOL;
    pub fn CryptDecodeObject(
        dwCertEncodingType: DWORD, lpszStructType: LPCSTR, pbEncoded: *const BYTE,
        cbEncoded: DWORD, dwFlags: DWORD, pvStructInfo: *mut c_void, pcbStructInfo: *mut DWORD,
    ) -> BOOL;
    pub fn CryptDecodeObjectEx(
        dwCertEncodingType: DWORD, lpszStructType: LPCSTR, pbEncoded: *const BYTE,
        cbEncoded: DWORD, dwFlags: DWORD, pDecodePara: PCRYPT_DECODE_PARA,
        pvStructInfo: *mut c_void, pcbStructInfo: *mut DWORD,
    ) -> BOOL;
    pub fn CryptDecryptAndVerifyMessageSignature(
        pDecryptPara: PCRYPT_DECRYPT_MESSAGE_PARA, pVerifyPara: PCRYPT_VERIFY_MESSAGE_PARA,
        dwSignerIndex: DWORD, pbEncryptedBlob: *const BYTE, cbEncryptedBlob: DWORD,
        pbDecrypted: *mut BYTE, pcbDecrypted: *mut DWORD, ppXchgCert: *mut PCCERT_CONTEXT,
        ppSignerCert: *mut PCCERT_CONTEXT,
    ) -> BOOL;
    pub fn CryptDecryptMessage(
        pDecryptPara: PCRYPT_DECRYPT_MESSAGE_PARA, pbEncryptedBlob: *const BYTE,
        cbEncryptedBlob: DWORD, pbDecrypted: *mut BYTE, pcbDecrypted: *mut DWORD,
        ppXchgCert: *mut PCCERT_CONTEXT,
    ) -> BOOL;
    pub fn CryptEncodeObject(
        dwCertEncodingType: DWORD, lpszStructType: LPCSTR, pvStructInfo: *const c_void,
        pbEncoded: *mut BYTE, pcbEncoded: *mut DWORD,
    ) -> BOOL;
    pub fn CryptEncodeObjectEx(
        dwCertEncodingType: DWORD, lpszStructType: LPCSTR, pvStructInfo: *const c_void,
        dwFlags: DWORD, pEncodePara: PCRYPT_ENCODE_PARA, pvEncoded: *mut c_void,
        pcbEncoded: *mut DWORD,
    ) -> BOOL;
    pub fn CryptEncryptMessage(
        pEncryptPara: PCRYPT_ENCRYPT_MESSAGE_PARA, cRecipientCert: DWORD,
        rgpRecipientCert: *mut PCCERT_CONTEXT, pbToBeEncrypted: *const BYTE,
        cbToBeEncrypted: DWORD, pbEncryptedBlob: *mut BYTE, pcbEncryptedBlob: *mut DWORD,
    ) -> BOOL;
    pub fn CryptEnumKeyIdentifierProperties(
        pKeyIdentifier: *const CRYPT_HASH_BLOB, dwPropId: DWORD, dwFlags: DWORD,
        pwszComputerName: LPCWSTR, pvReserved: *mut c_void, pvArg: *mut c_void,
        pfnEnum: PFN_CRYPT_ENUM_KEYID_PROP,
    ) -> BOOL;
    pub fn CryptEnumOIDFunction(
        dwEncodingType: DWORD, pszFuncName: LPCSTR, pszOID: LPCSTR, dwFlags: DWORD,
        pvArg: *mut c_void, pfnEnumOIDFunc: PFN_CRYPT_ENUM_OID_FUNC,
    ) -> BOOL;
    pub fn CryptEnumOIDInfo(
        dwGroupId: DWORD, dwFlags: DWORD, pvArg: *mut c_void,
        pfnEnumOIDInfo: PFN_CRYPT_ENUM_OID_INFO,
    ) -> BOOL;
    pub fn CryptExportPKCS8(
        hCryptProv: HCRYPTPROV, dwKeySpec: DWORD, pszPrivateKeyObjId: LPSTR, dwFlags: DWORD,
        pvAuxInfo: *mut c_void, pbPrivateKeyBlob: *mut BYTE, pcbPrivateKeyBlob: *mut DWORD,
    ) -> BOOL;
    pub fn CryptExportPublicKeyInfo(
        hCryptProvOrNCryptKey: HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, dwKeySpec: DWORD,
        dwCertEncodingType: DWORD, pInfo: PCERT_PUBLIC_KEY_INFO, pcbInfo: *mut DWORD,
    ) -> BOOL;
    pub fn CryptExportPublicKeyInfoEx(
        hCryptProvOrNCryptKey: HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, dwKeySpec: DWORD,
        dwCertEncodingType: DWORD, pszPublicKeyObjId: LPSTR, dwFlags: DWORD,
        pvAuxInfo: *mut c_void, pInfo: PCERT_PUBLIC_KEY_INFO, pcbInfo: *mut DWORD,
    ) -> BOOL;
    pub fn CryptExportPublicKeyInfoFromBCryptKeyHandle(
        hBCryptKey: BCRYPT_KEY_HANDLE, dwCertEncodingType: DWORD, pszPublicKeyObjId: LPSTR,
        dwFlags: DWORD, pvAuxInfo: *mut c_void, pInfo: PCERT_PUBLIC_KEY_INFO, pcbInfo: *mut DWORD,
    ) -> BOOL;
    pub fn CryptFindCertificateKeyProvInfo(
        pCert: PCCERT_CONTEXT, dwFlags: DWORD, pvReserved: *mut c_void,
    ) -> BOOL;
    pub fn CryptFindLocalizedName(pwszCryptName: LPCWSTR) -> LPCWSTR;
    pub fn CryptFindOIDInfo(
        dwKeyType: DWORD, pvKey: *mut c_void, dwGroupId: DWORD,
    ) -> PCCRYPT_OID_INFO;
    pub fn CryptFormatObject(
        dwCertEncodingType: DWORD, dwFormatType: DWORD, dwFormatStrType: DWORD,
        pFormatStruct: *mut c_void, lpszStructType: LPCSTR, pbEncoded: *const BYTE,
        cbEncoded: DWORD, pbFormat: *mut c_void, pcbFormat: *mut DWORD,
    ) -> BOOL;
    pub fn CryptFreeOIDFunctionAddress(hFuncAddr: HCRYPTOIDFUNCADDR, dwFlags: DWORD) -> BOOL;
    pub fn CryptGetAsyncParam(
        hAsync: HCRYPTASYNC, pszParamOid: LPSTR, ppvParam: *mut LPVOID,
        ppfnFree: *mut PFN_CRYPT_ASYNC_PARAM_FREE_FUNC,
    ) -> BOOL;
    pub fn CryptGetDefaultOIDDllList(
        hFuncSet: HCRYPTOIDFUNCSET, dwEncodingType: DWORD, pwszDllList: *mut WCHAR,
        pcchDllList: *mut DWORD,
    ) -> BOOL;
    pub fn CryptGetDefaultOIDFunctionAddress(
        hFuncSet: HCRYPTOIDFUNCSET, dwEncodingType: DWORD, pwszDll: LPCWSTR, dwFlags: DWORD,
        ppvFuncAddr: *mut *mut c_void, phFuncAddr: *mut HCRYPTOIDFUNCADDR,
    ) -> BOOL;
    pub fn CryptGetKeyIdentifierProperty(
        pKeyIdentifier: *const CRYPT_HASH_BLOB, dwPropId: DWORD, dwFlags: DWORD,
        pwszComputerName: LPCWSTR, pvReserved: *mut c_void, pvData: *mut c_void,
        pcbData: *mut DWORD,
    ) -> BOOL;
    pub fn CryptGetMessageCertificates(
        dwMsgAndCertEncodingType: DWORD, hCryptProv: HCRYPTPROV_LEGACY, dwFlags: DWORD,
        pbSignedBlob: *const BYTE, cbSignedBlob: DWORD,
    ) -> HCERTSTORE;
    pub fn CryptGetMessageSignerCount(
        dwMsgEncodingType: DWORD, pbSignedBlob: *const BYTE, cbSignedBlob: DWORD,
    ) -> LONG;
    pub fn CryptGetOIDFunctionAddress(
        hFuncSet: HCRYPTOIDFUNCSET, dwEncodingType: DWORD, pszOID: LPCSTR, dwFlags: DWORD,
        ppvFuncAddr: *mut *mut c_void, phFuncAddr: *mut HCRYPTOIDFUNCADDR,
    ) -> BOOL;
    pub fn CryptGetOIDFunctionValue(
        dwEncodingType: DWORD, pszFuncName: LPCSTR, pszOID: LPCSTR, pwszValueName: LPCWSTR,
        pdwValueType: *mut DWORD, pbValueData: *mut BYTE, pcbValueData: *mut DWORD,
    ) -> BOOL;
    pub fn CryptHashCertificate(
        hCryptProv: HCRYPTPROV_LEGACY, Algid: ALG_ID, dwFlags: DWORD, pbEncoded: *const BYTE,
        cbEncoded: DWORD, pbComputedHash: *mut BYTE, pcbComputedHash: *mut DWORD,
    ) -> BOOL;
    pub fn CryptHashCertificate2(
        pwszCNGHashAlgid: LPCWSTR, dwFlags: DWORD, pvReserved: *mut c_void, pbEncoded: *const BYTE,
        cbEncoded: DWORD, pbComputedHash: *mut BYTE, pcbComputedHash: *mut DWORD,
    ) -> BOOL;
    pub fn CryptHashMessage(
        pHashPara: PCRYPT_HASH_MESSAGE_PARA, fDetachedHash: BOOL, cToBeHashed: DWORD,
        rgpbToBeHashed: *mut *const BYTE, rgcbToBeHashed: *mut DWORD, pbHashedBlob: *mut BYTE,
        pcbHashedBlob: *mut DWORD, pbComputedHash: *mut BYTE, pcbComputedHash: *mut DWORD,
    ) -> BOOL;
    pub fn CryptHashPublicKeyInfo(
        hCryptProv: HCRYPTPROV_LEGACY, Algid: ALG_ID, dwFlags: DWORD, dwCertEncodingType: DWORD,
        pInfo: PCERT_PUBLIC_KEY_INFO, pbComputedHash: *mut BYTE, pcbComputedHash: *mut DWORD,
    ) -> BOOL;
    pub fn CryptHashToBeSigned(
        hCryptProv: HCRYPTPROV_LEGACY, dwCertEncodingType: DWORD, pbEncoded: *const BYTE,
        cbEncoded: DWORD, pbComputedHash: *mut BYTE, pcbComputedHash: *mut DWORD,
    ) -> BOOL;
    pub fn CryptImportPKCS8(
        sPrivateKeyAndParams: CRYPT_PKCS8_IMPORT_PARAMS, dwFlags: DWORD,
        phCryptProv: *mut HCRYPTPROV, pvAuxInfo: *mut c_void,
    ) -> BOOL;
    pub fn CryptImportPublicKeyInfo(
        hCryptProv: HCRYPTPROV, dwCertEncodingType: DWORD, pInfo: PCERT_PUBLIC_KEY_INFO,
        phKey: *mut HCRYPTKEY,
    ) -> BOOL;
    pub fn CryptImportPublicKeyInfoEx(
        hCryptProv: HCRYPTPROV, dwCertEncodingType: DWORD, pInfo: PCERT_PUBLIC_KEY_INFO,
        aiKeyAlg: ALG_ID, dwFlags: DWORD, pvAuxInfo: *mut c_void, phKey: *mut HCRYPTKEY,
    ) -> BOOL;
    pub fn CryptImportPublicKeyInfoEx2(
        dwCertEncodingType: DWORD, pInfo: PCERT_PUBLIC_KEY_INFO, dwFlags: DWORD,
        pvAuxInfo: *mut c_void, phKey: *mut BCRYPT_KEY_HANDLE,
    ) -> BOOL;
    pub fn CryptInitOIDFunctionSet(pszFuncName: LPCSTR, dwFlags: DWORD) -> HCRYPTOIDFUNCSET;
    pub fn CryptInstallDefaultContext(
        hCryptProv: HCRYPTPROV, dwDefaultType: DWORD, pvDefaultPara: *const c_void, dwFlags: DWORD,
        pvReserved: *mut c_void, phDefaultContext: *mut HCRYPTDEFAULTCONTEXT,
    ) -> BOOL;
    pub fn CryptInstallOIDFunctionAddress(
        hModule: HMODULE, dwEncodingType: DWORD, pszFuncName: LPCSTR, cFuncEntry: DWORD,
        rgFuncEntry: *const CRYPT_OID_FUNC_ENTRY, dwFlags: DWORD,
    ) -> BOOL;
    // pub fn CryptLoadSip();
    pub fn CryptMemAlloc(cbSize: ULONG) -> LPVOID;
    pub fn CryptMemFree(pv: LPVOID);
    pub fn CryptMemRealloc(pv: LPVOID, cbSize: ULONG) -> LPVOID;
    pub fn CryptMsgCalculateEncodedLength(
        dwMsgEncodingType: DWORD, dwFlags: DWORD, dwMsgType: DWORD, pvMsgEncodeInfo: *const c_void,
        pszInnerContentObjID: LPSTR, cbData: DWORD,
    ) -> DWORD;
    pub fn CryptMsgClose(hCryptMsg: HCRYPTMSG) -> BOOL;
    pub fn CryptMsgControl(
        hCryptMsg: HCRYPTMSG, dwFlags: DWORD, dwCtrlType: DWORD, pvCtrlPara: *const c_void,
    ) -> BOOL;
    pub fn CryptMsgCountersign(
        hCryptMsg: HCRYPTMSG, dwIndex: DWORD, cCountersigners: DWORD,
        rgCountersigners: PCMSG_SIGNER_ENCODE_INFO,
    ) -> BOOL;
    pub fn CryptMsgCountersignEncoded(
        dwEncodingType: DWORD, pbSignerInfo: PBYTE, cbSignerInfo: DWORD, cCountersigners: DWORD,
        rgCountersigners: PCMSG_SIGNER_ENCODE_INFO, pbCountersignature: PBYTE,
        pcbCountersignature: PDWORD,
    ) -> BOOL;
    pub fn CryptMsgDuplicate(hCryptMsg: HCRYPTMSG) -> HCRYPTMSG;
    pub fn CryptMsgEncodeAndSignCTL(
        dwMsgEncodingType: DWORD, pCtlInfo: PCTL_INFO, pSignInfo: PCMSG_SIGNED_ENCODE_INFO,
        dwFlags: DWORD, pbEncoded: *mut BYTE, pcbEncoded: *mut DWORD,
    ) -> BOOL;
    pub fn CryptMsgGetAndVerifySigner(
        hCryptMsg: HCRYPTMSG, cSignerStore: DWORD, rghSignerStore: *mut HCERTSTORE, dwFlags: DWORD,
        ppSigner: *mut PCCERT_CONTEXT, pdwSignerIndex: *mut DWORD,
    ) -> BOOL;
    pub fn CryptMsgGetParam(
        hCryptMsg: HCRYPTMSG, dwParamType: DWORD, dwIndex: DWORD, pvData: *mut c_void,
        pcbData: *mut DWORD,
    ) -> BOOL;
    pub fn CryptMsgOpenToDecode(
        dwMsgEncodingType: DWORD, dwFlags: DWORD, dwMsgType: DWORD, hCryptProv: HCRYPTPROV_LEGACY,
        pRecipientInfo: PCERT_INFO, pStreamInfo: PCMSG_STREAM_INFO,
    ) -> HCRYPTMSG;
    pub fn CryptMsgOpenToEncode(
        dwMsgEncodingType: DWORD, dwFlags: DWORD, dwMsgType: DWORD, pvMsgEncodeInfo: *mut c_void,
        pszInnerContentObjID: LPSTR, pStreamInfo: PCMSG_STREAM_INFO,
    ) -> HCRYPTMSG;
    pub fn CryptMsgSignCTL(
        dwMsgEncodingType: DWORD, pbCtlContent: *mut BYTE, cbCtlContent: DWORD,
        pSignInfo: PCMSG_SIGNED_ENCODE_INFO, dwFlags: DWORD, pbEncoded: *mut BYTE,
        pcbEncoded: *mut DWORD,
    ) -> BOOL;
    pub fn CryptMsgUpdate(
        hCryptMsg: HCRYPTMSG, pbData: *const BYTE, cbData: DWORD, fFinal: BOOL,
    ) -> BOOL;
    pub fn CryptMsgVerifyCountersignatureEncoded(
        hCryptProv: HCRYPTPROV_LEGACY, dwEncodingType: DWORD, pbSignerInfo: PBYTE,
        cbSignerInfo: DWORD, pbSignerInfoCountersignature: PBYTE,
        cbSignerInfoCountersignature: DWORD, pciCountersigner: PCERT_INFO,
    ) -> BOOL;
    pub fn CryptMsgVerifyCountersignatureEncodedEx(
        hCryptProv: HCRYPTPROV_LEGACY, dwEncodingType: DWORD, pbSignerInfo: PBYTE,
        cbSignerInfo: DWORD, pbSignerInfoCountersignature: PBYTE,
        cbSignerInfoCountersignature: DWORD, dwSignerType: DWORD, pvSigner: *mut c_void,
        dwFlags: DWORD, pvExtra: *mut c_void,
    ) -> BOOL;
    pub fn CryptProtectData(
        pDataIn: *mut DATA_BLOB, szDataDescr: LPCWSTR, pOptionalEntropy: *mut DATA_BLOB,
        pvReserved: PVOID, pPromptStruct: *mut CRYPTPROTECT_PROMPTSTRUCT, dwFlags: DWORD,
        pDataOut: *mut DATA_BLOB,
    ) -> BOOL;
    pub fn CryptProtectMemory(pDataIn: LPVOID, cbDataIn: DWORD, dwFlags: DWORD) -> BOOL;
    pub fn CryptQueryObject(
        dwObjectType: DWORD, pvObject: *const c_void, dwExpectedContentTypeFlags: DWORD,
        dwExpectedFormatTypeFlags: DWORD, dwFlags: DWORD, pdwMsgAndCertEncodingType: *mut DWORD,
        pdwContentType: *mut DWORD, pdwFormatType: *mut DWORD, phCertStore: *mut HCERTSTORE,
        phMsg: *mut HCRYPTMSG, ppvContext: *mut *const c_void,
    ) -> BOOL;
    pub fn CryptRegisterDefaultOIDFunction(
        dwEncodingType: DWORD, pszFuncName: LPCSTR, dwIndex: DWORD, pwszDll: LPCWSTR,
    ) -> BOOL;
    pub fn CryptRegisterOIDFunction(
        dwEncodingType: DWORD, pszFuncName: LPCSTR, pszOID: LPCSTR, pwszDll: LPCWSTR,
        pszOverrideFuncName: LPCSTR,
    ) -> BOOL;
    pub fn CryptRegisterOIDInfo(pInfo: PCCRYPT_OID_INFO, dwFlags: DWORD) -> BOOL;
    pub fn CryptRetrieveTimeStamp(
        wszUrl: LPCWSTR, dwRetrievalFlags: DWORD, dwTimeout: DWORD, pszHashId: LPCSTR,
        pPara: *const CRYPT_TIMESTAMP_PARA, pbData: *const BYTE, cbData: DWORD,
        ppTsContext: *mut PCRYPT_TIMESTAMP_CONTEXT, ppTsSigner: *mut PCCERT_CONTEXT,
        phStore: *mut HCERTSTORE,
    ) -> BOOL;
    pub fn CryptSIPAddProvider(psNewProv: *mut SIP_ADD_NEWPROVIDER) -> BOOL;
    pub fn CryptSIPCreateIndirectData(
        pSubjectInfo: *mut SIP_SUBJECTINFO, pcbIndirectData: *mut DWORD,
        pIndirectData: *mut SIP_INDIRECT_DATA,
    ) -> BOOL;
    pub fn CryptSIPGetCaps(pSubjInfo: *mut SIP_SUBJECTINFO, pCaps: *mut SIP_CAP_SET) -> BOOL;
    pub fn CryptSIPGetSealedDigest(
        pSubjectInfo: *mut SIP_SUBJECTINFO, pSig: *const BYTE, dwSig: DWORD, pbDigest: *mut BYTE,
        pcbDigest: *mut DWORD,
    ) -> BOOL;
    pub fn CryptSIPGetSignedDataMsg(
        pSubjectInfo: *mut SIP_SUBJECTINFO, pdwEncodingType: *mut DWORD, dwIndex: DWORD,
        pcbSignedDataMsg: *mut DWORD, pbSignedDataMsg: *mut BYTE,
    ) -> BOOL;
    pub fn CryptSIPLoad(
        pgSubject: *const GUID, dwFlags: DWORD, pSipDispatch: *mut SIP_DISPATCH_INFO,
    ) -> BOOL;
    pub fn CryptSIPPutSignedDataMsg(
        pSubjectInfo: *mut SIP_SUBJECTINFO, dwEncodingType: DWORD, pdwIndex: *mut DWORD,
        cbSignedDataMsg: DWORD, pbSignedDataMsg: *mut BYTE,
    ) -> BOOL;
    pub fn CryptSIPRemoveProvider(pgProv: *mut GUID) -> BOOL;
    pub fn CryptSIPRemoveSignedDataMsg(pSubjectInfo: *mut SIP_SUBJECTINFO, dwIndex: DWORD) -> BOOL;
    pub fn CryptSIPRetrieveSubjectGuid(
        FileName: LPCWSTR, hFileIn: HANDLE, pgSubject: *mut GUID,
    ) -> BOOL;
    pub fn CryptSIPRetrieveSubjectGuidForCatalogFile(
        FileName: LPCWSTR, hFileIn: HANDLE, pgSubject: *mut GUID,
    ) -> BOOL;
    pub fn CryptSIPVerifyIndirectData(
        pSubjectInfo: *mut SIP_SUBJECTINFO, pIndirectData: *mut SIP_INDIRECT_DATA,
    ) -> BOOL;
    pub fn CryptSetAsyncParam(
        hAsync: HCRYPTASYNC, pszParamOid: LPSTR, pvParam: LPVOID,
        pfnFree: PFN_CRYPT_ASYNC_PARAM_FREE_FUNC,
    ) -> BOOL;
    pub fn CryptSetKeyIdentifierProperty(
        pKeyIdentifier: *const CRYPT_HASH_BLOB, dwPropId: DWORD, dwFlags: DWORD,
        pwszComputerName: LPCWSTR, pvReserved: *mut c_void, pvData: *const c_void,
    ) -> BOOL;
    pub fn CryptSetOIDFunctionValue(
        dwEncodingType: DWORD, pszFuncName: LPCSTR, pszOID: LPCSTR, pwszValueName: LPCWSTR,
        dwValueType: DWORD, pbValueData: *const BYTE, cbValueData: DWORD,
    ) -> BOOL;
    pub fn CryptSignAndEncodeCertificate(
        hCryptProvOrNCryptKey: HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, dwKeySpec: DWORD,
        dwCertEncodingType: DWORD, lpszStructType: LPCSTR, pvStructInfo: *const c_void,
        pSignatureAlgorithm: PCRYPT_ALGORITHM_IDENTIFIER, pvHashAuxInfo: *const c_void,
        pbEncoded: *mut BYTE, pcbEncoded: *mut DWORD,
    ) -> BOOL;
    pub fn CryptSignAndEncryptMessage(
        pSignPara: PCRYPT_SIGN_MESSAGE_PARA, pEncryptPara: PCRYPT_ENCRYPT_MESSAGE_PARA,
        cRecipientCert: DWORD, rgpRecipientCert: *mut PCCERT_CONTEXT,
        pbToBeSignedAndEncrypted: *const BYTE, cbToBeSignedAndEncrypted: DWORD,
        pbSignedAndEncryptedBlob: *mut BYTE, pcbSignedAndEncryptedBlob: *mut DWORD,
    ) -> BOOL;
    pub fn CryptSignCertificate(
        hCryptProvOrNCryptKey: HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, dwKeySpec: DWORD,
        dwCertEncodingType: DWORD, pbEncodedToBeSigned: *const BYTE, cbEncodedToBeSigned: DWORD,
        pSignatureAlgorithm: PCRYPT_ALGORITHM_IDENTIFIER, pvHashAuxInfo: *const c_void,
        pbSignature: *mut BYTE, pcbSignature: *mut DWORD,
    ) -> BOOL;
    pub fn CryptSignMessage(
        pSignPara: PCRYPT_SIGN_MESSAGE_PARA, fDetachedSignature: BOOL, cToBeSigned: DWORD,
         rgpbToBeSigned: *mut *const BYTE, rgcbToBeSigned: *mut DWORD, pbSignedBlob: *mut BYTE,
         pcbSignedBlob: *mut DWORD,
    ) -> BOOL;
    pub fn CryptSignMessageWithKey(
        pSignPara: PCRYPT_KEY_SIGN_MESSAGE_PARA, pbToBeSigned: *const BYTE, cbToBeSigned: DWORD,
        pbSignedBlob: *mut BYTE, pcbSignedBlob: *mut DWORD,
    ) -> BOOL;
    pub fn CryptStringToBinaryA(
        pszString: LPCSTR, cchString: DWORD, dwFlags: DWORD, pbBinary: *mut BYTE,
        pcbBinary: *mut DWORD, pdwSkip: *mut DWORD, pdwFlags: *mut DWORD,
    ) -> BOOL;
    pub fn CryptStringToBinaryW(
        pszString: LPCWSTR, cchString: DWORD, dwFlags: DWORD, pbBinary: *mut BYTE,
        pcbBinary: *mut DWORD, pdwSkip: *mut DWORD, pdwFlags: *mut DWORD,
    ) -> BOOL;
    pub fn CryptUninstallDefaultContext(
        hDefaultContext: HCRYPTDEFAULTCONTEXT, dwFlags: DWORD, pvReserved: *mut c_void,
    ) -> BOOL;
    pub fn CryptUnprotectData(
        pDataIn: *mut DATA_BLOB, ppszDataDescr: *mut LPWSTR, pOptionalEntropy: *mut DATA_BLOB,
        pvReserved: PVOID, pPromptStruct: *mut CRYPTPROTECT_PROMPTSTRUCT, dwFlags: DWORD,
        pDataOut: *mut DATA_BLOB,
    ) -> BOOL;
    pub fn CryptUnprotectMemory(pDataIn: LPVOID, cbDataIn: DWORD, dwFlags: DWORD) -> BOOL;
    pub fn CryptUnregisterDefaultOIDFunction(
        dwEncodingType: DWORD, pszFuncName: LPCSTR, pwszDll: LPCWSTR,
    ) -> BOOL;
    pub fn CryptUnregisterOIDFunction(
        dwEncodingType: DWORD, pszFuncName: LPCSTR, pszOID: LPCSTR,
    ) -> BOOL;
    pub fn CryptUnregisterOIDInfo(pInfo: PCCRYPT_OID_INFO) -> BOOL;
    pub fn CryptUpdateProtectedState(
        pOldSid: PSID, pwszOldPassword: LPCWSTR, dwFlags: DWORD, pdwSuccessCount: *mut DWORD,
        pdwFailureCount: *mut DWORD,
    ) -> BOOL;
    pub fn CryptVerifyCertificateSignature(
        hCryptProv: HCRYPTPROV_LEGACY, dwCertEncodingType: DWORD, pbEncoded: *const BYTE,
        cbEncoded: DWORD, pPublicKey: PCERT_PUBLIC_KEY_INFO,
    ) -> BOOL;
    pub fn CryptVerifyCertificateSignatureEx(
        hCryptProv: HCRYPTPROV_LEGACY, dwCertEncodingType: DWORD, dwSubjectType: DWORD,
        pvSubject: *mut c_void, dwIssuerType: DWORD, pvIssuer: *mut c_void, dwFlags: DWORD,
        pvExtra: *mut c_void,
    ) -> BOOL;
    pub fn CryptVerifyDetachedMessageHash(
        pHashPara: PCRYPT_HASH_MESSAGE_PARA, pbDetachedHashBlob: *mut BYTE,
        cbDetachedHashBlob: DWORD, cToBeHashed: DWORD, rgpbToBeHashed: *mut *const BYTE,
        rgcbToBeHashed: *mut DWORD, pbComputedHash: *mut BYTE, pcbComputedHash: *mut DWORD,
    ) -> BOOL;
    pub fn CryptVerifyDetachedMessageSignature(
        pVerifyPara: PCRYPT_VERIFY_MESSAGE_PARA, dwSignerIndex: DWORD,
        pbDetachedSignBlob: *const BYTE, cbDetachedSignBlob: DWORD, cToBeSigned: DWORD,
        rgpbToBeSigned: *mut *const BYTE, rgcbToBeSigned: *mut DWORD,
        ppSignerCert: *mut PCCERT_CONTEXT,
    ) -> BOOL;
    pub fn CryptVerifyMessageHash(
        pHashPara: PCRYPT_HASH_MESSAGE_PARA, pbHashedBlob: *mut BYTE, cbHashedBlob: DWORD,
        pbToBeHashed: *mut BYTE, pcbToBeHashed: *mut DWORD, pbComputedHash: *mut BYTE,
        pcbComputedHash: *mut DWORD,
    ) -> BOOL;
    pub fn CryptVerifyMessageSignature(
        pVerifyPara: PCRYPT_VERIFY_MESSAGE_PARA, dwSignerIndex: DWORD, pbSignedBlob: *const BYTE,
        cbSignedBlob: DWORD, pbDecoded: *mut BYTE, pcbDecoded: *mut DWORD,
        ppSignerCert: *mut PCCERT_CONTEXT,
    ) -> BOOL;
    pub fn CryptVerifyMessageSignatureWithKey(
        pVerifyPara: PCRYPT_KEY_VERIFY_MESSAGE_PARA, pPublicKeyInfo: PCERT_PUBLIC_KEY_INFO,
        pbSignedBlob: *const BYTE, cbSignedBlob: DWORD, pbDecoded: *mut BYTE,
        pcbDecoded: *mut DWORD,
    ) -> BOOL;
    pub fn CryptVerifyTimeStampSignature(
        pbTSContentInfo: *const BYTE, cbTSContentInfo: DWORD, pbData: *const BYTE, cbData: DWORD,
        hAdditionalStore: HCERTSTORE, ppTsContext: *mut PCRYPT_TIMESTAMP_CONTEXT,
        ppTsSigner: *mut PCCERT_CONTEXT, phStore: *mut HCERTSTORE,
    ) -> BOOL;
    // pub fn DbgInitOSS();
    pub fn PFXExportCertStore(
        hStore: HCERTSTORE, pPFX: *mut CRYPT_DATA_BLOB, szPassword: LPCWSTR, dwFlags: DWORD,
    ) -> BOOL;
    // pub fn PFXExportCertStore2();
    pub fn PFXExportCertStoreEx(
        hStore: HCERTSTORE, pPFX: *mut CRYPT_DATA_BLOB, szPassword: LPCWSTR, pvPara: *mut c_void,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn PFXImportCertStore(
        pPFX: *mut CRYPT_DATA_BLOB, szPassword: LPCWSTR, dwFlags: DWORD,
    ) -> HCERTSTORE;
    pub fn PFXIsPFXBlob(pPFX: *mut CRYPT_DATA_BLOB) -> BOOL;
    pub fn PFXVerifyPassword(
        pPFX: *mut CRYPT_DATA_BLOB, szPassword: LPCWSTR, dwFlags: DWORD,
    ) -> BOOL;
}
