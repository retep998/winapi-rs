// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to advapi32.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    // pub fn AddUsersToEncryptedFile();
    // pub fn AddUsersToEncryptedFileEx();
    // pub fn BaseRegCloseKey();
    // pub fn BaseRegCreateKey();
    // pub fn BaseRegDeleteKeyEx();
    // pub fn BaseRegDeleteValue();
    // pub fn BaseRegFlushKey();
    // pub fn BaseRegGetVersion();
    // pub fn BaseRegLoadKey();
    // pub fn BaseRegOpenKey();
    // pub fn BaseRegRestoreKey();
    // pub fn BaseRegSaveKeyEx();
    // pub fn BaseRegSetKeySecurity();
    // pub fn BaseRegSetValue();
    // pub fn BaseRegUnLoadKey();
    // pub fn BuildExplicitAccessWithNameA();
    // pub fn BuildExplicitAccessWithNameW();
    // pub fn BuildImpersonateExplicitAccessWithNameA();
    // pub fn BuildImpersonateExplicitAccessWithNameW();
    // pub fn BuildImpersonateTrusteeA();
    // pub fn BuildImpersonateTrusteeW();
    // pub fn BuildSecurityDescriptorA();
    // pub fn BuildSecurityDescriptorW();
    // pub fn BuildTrusteeWithNameA();
    // pub fn BuildTrusteeWithNameW();
    // pub fn BuildTrusteeWithObjectsAndNameA();
    // pub fn BuildTrusteeWithObjectsAndNameW();
    // pub fn BuildTrusteeWithObjectsAndSidA();
    // pub fn BuildTrusteeWithObjectsAndSidW();
    // pub fn BuildTrusteeWithSidA();
    // pub fn BuildTrusteeWithSidW();
    // pub fn CancelOverlappedAccess();
    // pub fn CloseCodeAuthzLevel();
    // pub fn CloseThreadWaitChainSession();
    // pub fn CloseTrace();
    // pub fn CommandLineFromMsiDescriptor();
    // pub fn ComputeAccessTokenFromCodeAuthzLevel();
    // pub fn ControlTraceA();
    // pub fn ControlTraceW();
    // pub fn ConvertAccessToSecurityDescriptorA();
    // pub fn ConvertAccessToSecurityDescriptorW();
    // pub fn ConvertSDToStringSDDomainW();
    // pub fn ConvertSDToStringSDRootDomainA();
    // pub fn ConvertSDToStringSDRootDomainW();
    // pub fn ConvertSecurityDescriptorToAccessA();
    // pub fn ConvertSecurityDescriptorToAccessNamedA();
    // pub fn ConvertSecurityDescriptorToAccessNamedW();
    // pub fn ConvertSecurityDescriptorToAccessW();
    // pub fn ConvertSecurityDescriptorToStringSecurityDescriptorA();
    // pub fn ConvertSecurityDescriptorToStringSecurityDescriptorW();
    // pub fn ConvertSidToStringSidA();
    // pub fn ConvertSidToStringSidW();
    // pub fn ConvertStringSDToSDDomainA();
    // pub fn ConvertStringSDToSDDomainW();
    // pub fn ConvertStringSDToSDRootDomainA();
    // pub fn ConvertStringSDToSDRootDomainW();
    // pub fn ConvertStringSecurityDescriptorToSecurityDescriptorA();
    // pub fn ConvertStringSecurityDescriptorToSecurityDescriptorW();
    // pub fn ConvertStringSidToSidA();
    // pub fn ConvertStringSidToSidW();
    // pub fn CreateCodeAuthzLevel();
    // pub fn CreateTraceInstanceId();
    pub fn CredDeleteA(TargetName: LPCSTR, Type: DWORD, Flags: DWORD) -> BOOL;
    pub fn CredDeleteW(TargetName: LPCWSTR, Type: DWORD, Flags: DWORD) -> BOOL;
    // pub fn CredEnumerateA();
    // pub fn CredEnumerateW();
    // pub fn CredFindBestCredentialA();
    // pub fn CredFindBestCredentialW();
    pub fn CredFree(Buffer: PVOID);
    // pub fn CredGetSessionTypes();
    // pub fn CredGetTargetInfoA();
    // pub fn CredGetTargetInfoW();
    // pub fn CredIsMarshaledCredentialA();
    // pub fn CredIsMarshaledCredentialW();
    // pub fn CredIsProtectedA();
    // pub fn CredIsProtectedW();
    // pub fn CredMarshalCredentialA();
    // pub fn CredMarshalCredentialW();
    // pub fn CredProtectA();
    // pub fn CredProtectW();
    pub fn CredReadA(
        TargetName: LPCSTR, Type: DWORD, Flags: DWORD, Credential: *mut PCREDENTIALA,
    ) -> BOOL;
    // pub fn CredReadDomainCredentialsA();
    // pub fn CredReadDomainCredentialsW();
    pub fn CredReadW(
        TargetName: LPCWSTR, Type: DWORD, Flags: DWORD, Credential: *mut PCREDENTIALW,
    ) -> BOOL;
    // pub fn CredRenameA();
    // pub fn CredRenameW();
    // pub fn CredUnmarshalCredentialA();
    // pub fn CredUnmarshalCredentialW();
    // pub fn CredUnprotectA();
    // pub fn CredUnprotectW();
    pub fn CredWriteA(Credential: PCREDENTIALA, Flags: DWORD) -> BOOL;
    // pub fn CredWriteDomainCredentialsA();
    // pub fn CredWriteDomainCredentialsW();
    pub fn CredWriteW(Credential: PCREDENTIALW, Flags: DWORD) -> BOOL;
    pub fn CryptAcquireContextA(
        phProv: *mut HCRYPTPROV, szContainer: LPCSTR, szProvider: LPCSTR, dwProvType: DWORD,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptAcquireContextW(
        phProv: *mut HCRYPTPROV, szContainer: LPCWSTR, szProvider: LPCWSTR, dwProvType: DWORD,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptContextAddRef(hProv: HCRYPTPROV, pdwReserved: *mut DWORD, dwFlags: DWORD) -> BOOL;
    pub fn CryptCreateHash(
        hProv: HCRYPTPROV, Algid: ALG_ID, hKey: HCRYPTKEY, dwFlags: DWORD, phHash: *mut HCRYPTHASH,
    ) -> BOOL;
    pub fn CryptDecrypt(
        hKey: HCRYPTKEY, hHash: HCRYPTHASH, Final: BOOL, dwFlags: DWORD, pbData: *mut BYTE,
        pdwDataLen: *mut DWORD,
    ) -> BOOL;
    pub fn CryptDeriveKey(
        hProv: HCRYPTPROV, Algid: ALG_ID, hBaseData: HCRYPTHASH, dwFlags: DWORD,
        phKey: *mut HCRYPTKEY,
    ) -> BOOL;
    pub fn CryptDestroyHash(hHash: HCRYPTHASH) -> BOOL;
    pub fn CryptDestroyKey(hKey: HCRYPTKEY) -> BOOL;
    pub fn CryptDuplicateHash(
        hHash: HCRYPTHASH, pdwReserved: *mut DWORD, dwFlags: DWORD, phHash: *mut HCRYPTHASH,
    ) -> BOOL;
    pub fn CryptDuplicateKey(
        hKey: HCRYPTKEY, pdwReserved: *mut DWORD, dwFlags: DWORD, phKey: *mut HCRYPTKEY,
    ) -> BOOL;
    pub fn CryptEncrypt(
        hKey: HCRYPTKEY, hHash: HCRYPTHASH, Final: BOOL, dwFlags: DWORD, pbData: *mut BYTE,
        pdwDataLen: *mut DWORD, dwBufLen: DWORD,
    ) -> BOOL;
    pub fn CryptEnumProviderTypesA(
        dwIndex: DWORD, pdwReserved: *mut DWORD, dwFlags: DWORD, pdwProvType: *mut DWORD,
        szTypeName: LPSTR, pcbTypeName: *mut DWORD,
    ) -> BOOL;
    pub fn CryptEnumProviderTypesW(
        dwIndex: DWORD, pdwReserved: *mut DWORD, dwFlags: DWORD, pdwProvType: *mut DWORD,
        szTypeName: LPWSTR, pcbTypeName: *mut DWORD,
    ) -> BOOL;
    pub fn CryptEnumProvidersA(
        dwIndex: DWORD, pdwReserved: *mut DWORD, dwFlags: DWORD, pdwProvType: *mut DWORD,
        szProvName: LPSTR, pcbProvName: *mut DWORD,
    ) -> BOOL;
    pub fn CryptEnumProvidersW(
        dwIndex: DWORD, pdwReserved: *mut DWORD, dwFlags: DWORD, pdwProvType: *mut DWORD,
        szProvName: LPWSTR, pcbProvName: *mut DWORD,
    ) -> BOOL;
    pub fn CryptExportKey(
        hKey: HCRYPTKEY, hExpKey: HCRYPTKEY, dwBlobType: DWORD, dwFlags: DWORD, pbData: *mut BYTE,
        pdwDataLen: *mut DWORD,
    ) -> BOOL;
    pub fn CryptGenKey(
        hProv: HCRYPTPROV, Algid: ALG_ID, dwFlags: DWORD, phKey: *mut HCRYPTKEY,
    ) -> BOOL;
    pub fn CryptGenRandom(hProv: HCRYPTPROV, dwLen: DWORD, pbBuffer: *mut BYTE) -> BOOL;
    pub fn CryptGetDefaultProviderA(
        dwProvType: DWORD, pdwReserved: *mut DWORD, dwFlags: DWORD, pszProvName: LPSTR,
        pcbProvName: *mut DWORD,
    ) -> BOOL;
    pub fn CryptGetDefaultProviderW(
        dwProvType: DWORD, pdwReserved: *mut DWORD, dwFlags: DWORD, pszProvName: LPWSTR,
        pcbProvName: *mut DWORD,
    ) -> BOOL;
    pub fn CryptGetHashParam(
        hHash: HCRYPTHASH, dwParam: DWORD, pbData: *mut BYTE, pdwDataLen: *mut DWORD,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptGetKeyParam(
        hKey: HCRYPTKEY, dwParam: DWORD, pbData: *mut BYTE, pdwDataLen: *mut DWORD, dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptGetProvParam(
        hProv: HCRYPTPROV, dwParam: DWORD, pbData: *mut BYTE, pdwDataLen: *mut DWORD,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptGetUserKey(hProv: HCRYPTPROV, dwKeySpec: DWORD, phUserKey: *mut HCRYPTKEY) -> BOOL;
    pub fn CryptHashData(
        hHash: HCRYPTHASH, pbData: *const BYTE, dwDataLen: DWORD, dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptHashSessionKey(hHash: HCRYPTHASH, hKey: HCRYPTKEY, dwFlags: DWORD) -> BOOL;
    pub fn CryptImportKey(
        hProv: HCRYPTPROV, pbData: *const BYTE, dwDataLen: DWORD, hPubKey: HCRYPTKEY,
        dwFlags: DWORD, phKey: *mut HCRYPTKEY,
    ) -> BOOL;
    pub fn CryptReleaseContext(hProv: HCRYPTPROV, dwFlags: DWORD) -> BOOL;
    pub fn CryptSetHashParam(
        hHash: HCRYPTHASH, dwParam: DWORD, pbData: *const BYTE, dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptSetKeyParam(
        hKey: HCRYPTKEY, dwParam: DWORD, pbData: *const BYTE, dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptSetProvParam(
        hProv: HCRYPTPROV, dwParam: DWORD, pbData: *const BYTE, dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptSetProviderA(pszProvName: LPCSTR, dwProvType: DWORD) -> BOOL;
    pub fn CryptSetProviderExA(
        pszProvName: LPCSTR, dwProvType: DWORD, pdwReserved: *mut DWORD, dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptSetProviderExW(
        pszProvName: LPCWSTR, dwProvType: DWORD, pdwReserved: *mut DWORD, dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptSetProviderW(pszProvName: LPCWSTR, dwProvType: DWORD) -> BOOL;
    pub fn CryptSignHashA(
        hHash: HCRYPTHASH, dwKeySpec: DWORD, szDescription: LPCSTR, dwFlags: DWORD,
        pbSignature: *mut BYTE, pdwSigLen: *mut DWORD,
    ) -> BOOL;
    pub fn CryptSignHashW(
        hHash: HCRYPTHASH, dwKeySpec: DWORD, szDescription: LPCWSTR, dwFlags: DWORD,
        pbSignature: *mut BYTE, pdwSigLen: *mut DWORD,
    ) -> BOOL;
    pub fn CryptVerifySignatureA(
        hHash: HCRYPTHASH, pbSignature: *const BYTE, dwSigLen: DWORD, hPubKey: HCRYPTKEY,
        szDescription: LPCSTR, dwFlags: DWORD,
    ) -> BOOL;
    pub fn CryptVerifySignatureW(
        hHash: HCRYPTHASH, pbSignature: *const BYTE, dwSigLen: DWORD, hPubKey: HCRYPTKEY,
        szDescription: LPCWSTR, dwFlags: DWORD,
    ) -> BOOL;
    // pub fn DuplicateEncryptionInfoFile();
    // pub fn ElfBackupEventLogFileA();
    // pub fn ElfBackupEventLogFileW();
    // pub fn ElfChangeNotify();
    // pub fn ElfClearEventLogFileA();
    // pub fn ElfClearEventLogFileW();
    // pub fn ElfCloseEventLog();
    // pub fn ElfDeregisterEventSource();
    // pub fn ElfFlushEventLog();
    // pub fn ElfNumberOfRecords();
    // pub fn ElfOldestRecord();
    // pub fn ElfOpenBackupEventLogA();
    // pub fn ElfOpenBackupEventLogW();
    // pub fn ElfOpenEventLogA();
    // pub fn ElfOpenEventLogW();
    // pub fn ElfReadEventLogA();
    // pub fn ElfReadEventLogW();
    // pub fn ElfRegisterEventSourceA();
    // pub fn ElfRegisterEventSourceW();
    // pub fn ElfReportEventA();
    // pub fn ElfReportEventAndSourceW();
    // pub fn ElfReportEventW();
    // pub fn EnableTrace();
    // pub fn EnableTraceEx();
    // pub fn EnableTraceEx2();
    // pub fn EncryptedFileKeyInfo();
    // pub fn EncryptionDisable();
    // pub fn EnumDynamicTimeZoneInformation();
    // pub fn EnumServiceGroupW();
    // pub fn EnumerateTraceGuids();
    // pub fn EnumerateTraceGuidsEx();
    // pub fn EtwLogSysConfigExtension();
    // pub fn EventAccessControl();
    // pub fn EventAccessQuery();
    // pub fn EventAccessRemove();
    // pub fn EventActivityIdControl();
    // pub fn EventEnabled();
    // pub fn EventProviderEnabled();
    // pub fn EventRegister();
    // pub fn EventSetInformation();
    // pub fn EventUnregister();
    // pub fn EventWrite();
    // pub fn EventWriteEndScenario();
    // pub fn EventWriteEx();
    // pub fn EventWriteStartScenario();
    // pub fn EventWriteString();
    // pub fn EventWriteTransfer();
    // pub fn FlushEfsCache();
    // pub fn FlushTraceA();
    // pub fn FlushTraceW();
    // pub fn FreeEncryptedFileKeyInfo();
    // pub fn FreeEncryptedFileMetadata();
    // pub fn FreeEncryptionCertificateHashList();
    // pub fn FreeInheritedFromArray();
    // pub fn GetAccessPermissionsForObjectA();
    // pub fn GetAccessPermissionsForObjectW();
    // pub fn GetAuditedPermissionsFromAclA();
    // pub fn GetAuditedPermissionsFromAclW();
    // pub fn GetDynamicTimeZoneInformationEffectiveYears();
    // pub fn GetEffectiveRightsFromAclA();
    // pub fn GetEffectiveRightsFromAclW();
    // pub fn GetEncryptedFileMetadata();
    // pub fn GetExplicitEntriesFromAclA();
    // pub fn GetExplicitEntriesFromAclW();
    // pub fn GetInformationCodeAuthzLevelW();
    // pub fn GetInformationCodeAuthzPolicyW();
    // pub fn GetInheritanceSourceA();
    // pub fn GetInheritanceSourceW();
    // pub fn GetLocalManagedApplicationData();
    // pub fn GetLocalManagedApplications();
    // pub fn GetManagedApplicationCategories();
    // pub fn GetManagedApplications();
    // pub fn GetMultipleTrusteeA();
    // pub fn GetMultipleTrusteeOperationA();
    // pub fn GetMultipleTrusteeOperationW();
    // pub fn GetMultipleTrusteeW();
    // pub fn GetNamedSecurityInfoA();
    // pub fn GetNamedSecurityInfoExA();
    // pub fn GetNamedSecurityInfoExW();
    // pub fn GetNamedSecurityInfoW();
    // pub fn GetOverlappedAccessResults();
    // pub fn GetSecurityInfo();
    // pub fn GetSecurityInfoExA();
    // pub fn GetSecurityInfoExW();
    // pub fn GetStringConditionFromBinary();
    // pub fn GetThreadWaitChain();
    // pub fn GetTraceEnableFlags();
    // pub fn GetTraceEnableLevel();
    // pub fn GetTraceLoggerHandle();
    // pub fn GetTrusteeFormA();
    // pub fn GetTrusteeFormW();
    // pub fn GetTrusteeNameA();
    // pub fn GetTrusteeNameW();
    // pub fn GetTrusteeTypeA();
    // pub fn GetTrusteeTypeW();
    // pub fn I_ScSetServiceBitsA();
    // pub fn I_ScSetServiceBitsW();
    // pub fn IdentifyCodeAuthzLevelW();
    // pub fn ImpersonateNamedPipeClient();
    // pub fn InstallApplication();
    // pub fn IsValidRelativeSecurityDescriptor();
    // pub fn LookupSecurityDescriptorPartsA();
    // pub fn LookupSecurityDescriptorPartsW();
    // pub fn LsaAddAccountRights();
    // pub fn LsaAddPrivilegesToAccount();
    // pub fn LsaClearAuditLog();
    // pub fn LsaClose();
    // pub fn LsaCreateAccount();
    // pub fn LsaCreateSecret();
    // pub fn LsaCreateTrustedDomain();
    // pub fn LsaCreateTrustedDomainEx();
    // pub fn LsaDelete();
    // pub fn LsaDeleteTrustedDomain();
    // pub fn LsaEnumerateAccountRights();
    // pub fn LsaEnumerateAccounts();
    // pub fn LsaEnumerateAccountsWithUserRight();
    // pub fn LsaEnumeratePrivileges();
    // pub fn LsaEnumeratePrivilegesOfAccount();
    // pub fn LsaEnumerateTrustedDomains();
    // pub fn LsaEnumerateTrustedDomainsEx();
    // pub fn LsaFreeMemory();
    // pub fn LsaGetAppliedCAPIDs();
    // pub fn LsaGetQuotasForAccount();
    // pub fn LsaGetRemoteUserName();
    // pub fn LsaGetSystemAccessAccount();
    // pub fn LsaGetUserName();
    // pub fn LsaICLookupNames();
    // pub fn LsaICLookupNamesWithCreds();
    // pub fn LsaICLookupSids();
    // pub fn LsaICLookupSidsWithCreds();
    // pub fn LsaLookupNames();
    // pub fn LsaLookupNames2();
    // pub fn LsaLookupPrivilegeDisplayName();
    // pub fn LsaLookupPrivilegeName();
    // pub fn LsaLookupPrivilegeValue();
    // pub fn LsaLookupSids();
    // pub fn LsaLookupSids2();
    // pub fn LsaManageSidNameMapping();
    // pub fn LsaNtStatusToWinError();
    // pub fn LsaOpenAccount();
    // pub fn LsaOpenPolicy();
    // pub fn LsaOpenPolicySce();
    // pub fn LsaOpenSecret();
    // pub fn LsaOpenTrustedDomain();
    // pub fn LsaOpenTrustedDomainByName();
    // pub fn LsaQueryCAPs();
    // pub fn LsaQueryDomainInformationPolicy();
    // pub fn LsaQueryForestTrustInformation();
    // pub fn LsaQueryInfoTrustedDomain();
    // pub fn LsaQueryInformationPolicy();
    // pub fn LsaQuerySecret();
    // pub fn LsaQuerySecurityObject();
    // pub fn LsaQueryTrustedDomainInfo();
    // pub fn LsaQueryTrustedDomainInfoByName();
    // pub fn LsaRemoveAccountRights();
    // pub fn LsaRemovePrivilegesFromAccount();
    // pub fn LsaRetrievePrivateData();
    // pub fn LsaSetCAPs();
    // pub fn LsaSetDomainInformationPolicy();
    // pub fn LsaSetForestTrustInformation();
    // pub fn LsaSetInformationPolicy();
    // pub fn LsaSetInformationTrustedDomain();
    // pub fn LsaSetQuotasForAccount();
    // pub fn LsaSetSecret();
    // pub fn LsaSetSecurityObject();
    // pub fn LsaSetSystemAccessAccount();
    // pub fn LsaSetTrustedDomainInfoByName();
    // pub fn LsaSetTrustedDomainInformation();
    // pub fn LsaStorePrivateData();
    // pub fn MIDL_user_free_Ext();
    // pub fn MSChapSrvChangePassword();
    // pub fn MSChapSrvChangePassword2();
    // pub fn MakeAbsoluteSD2();
    // pub fn NotifyServiceStatusChange();
    // pub fn OpenThreadWaitChainSession();
    // pub fn OpenTraceA();
    // pub fn OpenTraceW();
    // pub fn PerfAddCounters();
    // pub fn PerfCloseQueryHandle();
    // pub fn PerfCreateInstance();
    // pub fn PerfDecrementULongCounterValue();
    // pub fn PerfDecrementULongLongCounterValue();
    // pub fn PerfDeleteCounters();
    // pub fn PerfDeleteInstance();
    // pub fn PerfEnumerateCounterSet();
    // pub fn PerfEnumerateCounterSetInstances();
    // pub fn PerfIncrementULongCounterValue();
    // pub fn PerfIncrementULongLongCounterValue();
    // pub fn PerfOpenQueryHandle();
    // pub fn PerfQueryCounterData();
    // pub fn PerfQueryCounterInfo();
    // pub fn PerfQueryCounterSetRegistrationInfo();
    // pub fn PerfQueryInstance();
    // pub fn PerfRegCloseKey();
    // pub fn PerfRegEnumKey();
    // pub fn PerfRegEnumValue();
    // pub fn PerfRegQueryInfoKey();
    // pub fn PerfRegQueryValue();
    // pub fn PerfRegSetValue();
    // pub fn PerfSetCounterRefValue();
    // pub fn PerfSetCounterSetInfo();
    // pub fn PerfSetULongCounterValue();
    // pub fn PerfSetULongLongCounterValue();
    // pub fn PerfStartProvider();
    // pub fn PerfStartProviderEx();
    // pub fn PerfStopProvider();
    // pub fn ProcessTrace();
    // pub fn QueryAllTracesA();
    // pub fn QueryAllTracesW();
    // pub fn QueryRecoveryAgentsOnEncryptedFile();
    // pub fn QueryTraceA();
    // pub fn QueryTraceW();
    // pub fn QueryUsersOnEncryptedFile();
    // pub fn RegisterTraceGuidsA();
    // pub fn RegisterTraceGuidsW();
    // pub fn RegisterWaitChainCOMCallback();
    // pub fn RemoteRegEnumKeyWrapper();
    // pub fn RemoteRegEnumValueWrapper();
    // pub fn RemoteRegQueryInfoKeyWrapper();
    // pub fn RemoteRegQueryValueWrapper();
    // pub fn RemoveTraceCallback();
    // pub fn RemoveUsersFromEncryptedFile();
    // pub fn SafeBaseRegGetKeySecurity();
    // pub fn SaferCloseLevel();
    // pub fn SaferComputeTokenFromLevel();
    // pub fn SaferCreateLevel();
    // pub fn SaferGetLevelInformation();
    // pub fn SaferGetPolicyInformation();
    // pub fn SaferIdentifyLevel();
    // pub fn SaferRecordEventLogEntry();
    // pub fn SaferSetLevelInformation();
    // pub fn SaferSetPolicyInformation();
    // pub fn SaferiIsExecutableFileType();
    // pub fn SetEncryptedFileMetadata();
    // pub fn SetEntriesInAccessListA();
    // pub fn SetEntriesInAccessListW();
    // pub fn SetEntriesInAclA();
    // pub fn SetEntriesInAclW();
    // pub fn SetEntriesInAuditListA();
    // pub fn SetEntriesInAuditListW();
    // pub fn SetInformationCodeAuthzLevelW();
    // pub fn SetInformationCodeAuthzPolicyW();
    // pub fn SetNamedSecurityInfoA();
    // pub fn SetNamedSecurityInfoExA();
    // pub fn SetNamedSecurityInfoExW();
    // pub fn SetNamedSecurityInfoW();
    // pub fn SetSecurityInfo();
    // pub fn SetSecurityInfoExA();
    // pub fn SetSecurityInfoExW();
    // pub fn SetServiceBits();
    // pub fn SetTraceCallback();
    // pub fn SetUserFileEncryptionKey();
    // pub fn SetUserFileEncryptionKeyEx();
    // pub fn StartTraceA();
    // pub fn StartTraceW();
    // pub fn StopTraceA();
    // pub fn StopTraceW();
    // pub fn SystemFunction001();
    // pub fn SystemFunction002();
    // pub fn SystemFunction003();
    // pub fn SystemFunction004();
    // pub fn SystemFunction005();
    // pub fn SystemFunction006();
    // pub fn SystemFunction007();
    // pub fn SystemFunction008();
    // pub fn SystemFunction009();
    // pub fn SystemFunction010();
    // pub fn SystemFunction011();
    // pub fn SystemFunction012();
    // pub fn SystemFunction013();
    // pub fn SystemFunction014();
    // pub fn SystemFunction015();
    // pub fn SystemFunction016();
    // pub fn SystemFunction017();
    // pub fn SystemFunction018();
    // pub fn SystemFunction019();
    // pub fn SystemFunction020();
    // pub fn SystemFunction021();
    // pub fn SystemFunction022();
    // pub fn SystemFunction023();
    // pub fn SystemFunction024();
    // pub fn SystemFunction025();
    // pub fn SystemFunction026();
    // pub fn SystemFunction027();
    // pub fn SystemFunction028();
    // pub fn SystemFunction029();
    // pub fn SystemFunction030();
    // pub fn SystemFunction031();
    // pub fn SystemFunction032();
    // pub fn SystemFunction033();
    // pub fn SystemFunction034();
    // pub fn SystemFunction036();
    // pub fn SystemFunction040();
    // pub fn SystemFunction041();
    // pub fn TraceEvent();
    // pub fn TraceEventInstance();
    // pub fn TraceMessage();
    // pub fn TraceMessageVa();
    // pub fn TraceQueryInformation();
    // pub fn TraceSetInformation();
    // pub fn TreeResetNamedSecurityInfoA();
    // pub fn TreeResetNamedSecurityInfoW();
    // pub fn TreeSetNamedSecurityInfoA();
    // pub fn TreeSetNamedSecurityInfoW();
    // pub fn TrusteeAccessToObjectA();
    // pub fn TrusteeAccessToObjectW();
    // pub fn UninstallApplication();
    // pub fn UnregisterTraceGuids();
    // pub fn UpdateTraceA();
    // pub fn UpdateTraceW();
    // pub fn UsePinForEncryptedFilesA();
    // pub fn UsePinForEncryptedFilesW();
    // pub fn WaitServiceState();
}
