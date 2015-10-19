// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to netapi32.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn DavAddConnection(
        ConnectionHandle: *mut HANDLE, RemoteName: LPCWSTR, UserName: LPCWSTR, Password: LPCWSTR,
        ClientCert: PBYTE, CertSize: DWORD,
    ) -> DWORD;
    pub fn DavDeleteConnection(ConnectionHandle: HANDLE) -> DWORD;
    pub fn DavFlushFile(hFile: HANDLE) -> DWORD;
    pub fn DavGetExtendedError(
        hFile: HANDLE, ExtError: *mut DWORD, ExtErrorString: LPWSTR, cChSize: *mut DWORD,
    ) -> DWORD;
    pub fn DavGetHTTPFromUNCPath(UncPath: LPCWSTR, HttpPath: LPWSTR, lpSize: LPDWORD) -> DWORD;
    pub fn DavGetUNCFromHTTPPath(HttpPath: LPCWSTR, UncPath: LPWSTR, lpSize: LPDWORD) -> DWORD;
    pub fn DsAddressToSiteNamesA(
        ComputerName: LPCSTR, EntryCount: DWORD, SocketAddresses: PSOCKET_ADDRESS,
        SiteNames: *mut *mut LPSTR,
    ) -> DWORD;
    pub fn DsAddressToSiteNamesExA(
        ComputerName: LPCSTR, EntryCount: DWORD, SocketAddresses: PSOCKET_ADDRESS,
        SiteNames: *mut *mut LPSTR, SubnetNames: *mut *mut LPSTR,
    ) -> DWORD;
    pub fn DsAddressToSiteNamesExW(
        ComputerName: LPCWSTR, EntryCount: DWORD, SocketAddresses: PSOCKET_ADDRESS,
        SiteNames: *mut *mut LPWSTR, SubnetNames: *mut *mut LPWSTR,
    ) -> DWORD;
    pub fn DsAddressToSiteNamesW(
        ComputerName: LPCWSTR, EntryCount: DWORD, SocketAddresses: PSOCKET_ADDRESS,
        SiteNames: *mut *mut LPWSTR,
    ) -> DWORD;
    pub fn DsDeregisterDnsHostRecordsA(
        ServerName: LPSTR, DnsDomainName: LPSTR, DomainGuid: *mut GUID, DsaGuid: *mut GUID,
        DnsHostName: LPSTR,
    ) -> DWORD;
    pub fn DsDeregisterDnsHostRecordsW(
        ServerName: LPWSTR, DnsDomainName: LPWSTR, DomainGuid: *mut GUID, DsaGuid: *mut GUID,
        DnsHostName: LPWSTR,
    ) -> DWORD;
    pub fn DsEnumerateDomainTrustsA(
        ServerName: LPSTR, Flags: ULONG, Domains: *mut PDS_DOMAIN_TRUSTSA, DomainCount: PULONG,
    ) -> DWORD;
    pub fn DsEnumerateDomainTrustsW(
        ServerName: LPWSTR, Flags: ULONG, Domains: *mut PDS_DOMAIN_TRUSTSW, DomainCount: PULONG,
    ) -> DWORD;
    pub fn DsGetDcCloseW(GetDcContextHandle: HANDLE);
    pub fn DsGetDcNameA(
        ComputerName: LPCSTR, DomainName: LPCSTR, DomainGuid: *mut GUID, SiteName: LPCSTR,
        Flags: ULONG, DomainControllerInfo: *mut PDOMAIN_CONTROLLER_INFOA,
    ) -> DWORD;
    pub fn DsGetDcNameW(
        ComputerName: LPCWSTR, DomainName: LPCWSTR, DomainGuid: *mut GUID, SiteName: LPCWSTR,
        Flags: ULONG, DomainControllerInfo: *mut PDOMAIN_CONTROLLER_INFOW,
    ) -> DWORD;
    // pub fn DsGetDcNameWithAccountA();
    // pub fn DsGetDcNameWithAccountW();
    pub fn DsGetDcNextA(
        GetDcContextHandle: HANDLE, SockAddressCount: PULONG, SockAddresses: *mut LPSOCKET_ADDRESS,
        DnsHostName: *mut LPSTR,
    ) -> DWORD;
    pub fn DsGetDcNextW(
        GetDcContextHandle: HANDLE, SockAddressCount: PULONG, SockAddresses: *mut LPSOCKET_ADDRESS,
        DnsHostName: *mut LPWSTR,
    ) -> DWORD;
    pub fn DsGetDcOpenA(
        DnsName: LPCSTR, OptionFlags: ULONG, SiteName: LPCSTR, DomainGuid: *mut GUID,
        DnsForestName: LPCSTR, DcFlags: ULONG, RetGetDcContext: PHANDLE,
    ) -> DWORD;
    pub fn DsGetDcOpenW(
        DnsName: LPCWSTR, OptionFlags: ULONG, SiteName: LPCWSTR, DomainGuid: *mut GUID,
        DnsForestName: LPCWSTR, DcFlags: ULONG, RetGetDcContext: PHANDLE,
    ) -> DWORD;
    pub fn DsGetDcSiteCoverageA(
        ServerName: LPCSTR, EntryCount: PULONG, SiteNames: *mut *mut LPSTR,
    ) -> DWORD;
    pub fn DsGetDcSiteCoverageW(
        ServerName: LPCWSTR, EntryCount: PULONG, SiteNames: *mut *mut LPWSTR,
    ) -> DWORD;
    pub fn DsGetForestTrustInformationW(
        ServerName: LPCWSTR, TrustedDomainName: LPCWSTR, Flags: DWORD,
        ForestTrustInfo: *mut PLSA_FOREST_TRUST_INFORMATION,
    ) -> DWORD;
    pub fn DsGetSiteNameA(ComputerName: LPCSTR, SiteName: *mut LPSTR) -> DWORD;
    pub fn DsGetSiteNameW(ComputerName: LPCWSTR, SiteName: *mut LPWSTR) -> DWORD;
    pub fn DsMergeForestTrustInformationW(
        DomainName: LPCWSTR, NewForestTrustInfo: PLSA_FOREST_TRUST_INFORMATION,
        OldForestTrustInfo: PLSA_FOREST_TRUST_INFORMATION,
        MergedForestTrustInfo: *mut PLSA_FOREST_TRUST_INFORMATION,
    ) -> DWORD;
    pub fn DsRoleFreeMemory(Buffer: PVOID);
    pub fn DsRoleGetPrimaryDomainInformation(
        lpServer: LPCWSTR, InfoLevel: DSROLE_PRIMARY_DOMAIN_INFO_LEVEL, Buffer: *mut PBYTE,
    ) -> DWORD;
    pub fn DsValidateSubnetNameA(SubnetName: LPCSTR) -> DWORD;
    pub fn DsValidateSubnetNameW(SubnetName: LPCWSTR) -> DWORD;
    // pub fn I_BrowserDebugTrace();
    // pub fn I_BrowserQueryOtherDomains();
    // pub fn I_BrowserQueryStatistics();
    // pub fn I_BrowserResetNetlogonState();
    // pub fn I_BrowserResetStatistics();
    // pub fn I_BrowserServerEnum();
    pub fn I_NetLogonControl2(
        ServerName: LPCWSTR, FunctionCode: DWORD, QueryLevel: DWORD, Data: LPBYTE,
        Buffer: *mut LPBYTE,
    ) -> NET_API_STATUS;
    // pub fn I_NetServerPasswordGet();
    // pub fn I_NetServerPasswordSet2();
    pub fn NetAccessAdd(
        servername: LPCWSTR, level: DWORD, buf: LPBYTE, parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetAccessDel(servername: LPCWSTR, resource: LPCWSTR) -> NET_API_STATUS;
    pub fn NetAccessEnum(
        servername: LPCWSTR, BasePath: LPCWSTR, Recursive: DWORD, level: DWORD,
        bufptr: *mut LPBYTE, prefmaxlen: DWORD, entriesread: LPDWORD, totalentries: LPDWORD,
        resume_handle: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetAccessGetInfo(
        servername: LPCWSTR, resource: LPCWSTR, level: DWORD, bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetAccessGetUserPerms(
        servername: LPCWSTR, UGname: LPCWSTR, resource: LPCWSTR, Perms: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetAccessSetInfo(
        servername: LPCWSTR, resource: LPCWSTR, level: DWORD, buf: LPBYTE, parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetAddAlternateComputerName(
        Server: LPCWSTR, AlternateName: LPCWSTR, DomainAccount: LPCWSTR,
        DomainAccountPassword: LPCWSTR, Reserved: ULONG,
    ) -> NET_API_STATUS;
    pub fn NetAddServiceAccount(
        ServerName: LPWSTR, AccountName: LPWSTR, Password: LPWSTR, Flags: DWORD,
    ) -> NTSTATUS;
    pub fn NetAlertRaise(AlertType: LPCWSTR, Buffer: LPVOID, BufferSize: DWORD) -> NET_API_STATUS;
    pub fn NetAlertRaiseEx(
        AlertType: LPCWSTR, VariableInfo: LPVOID, VariableInfoSize: DWORD, ServiceName: LPCWSTR,
    ) -> NET_API_STATUS;
    pub fn NetApiBufferAllocate(ByteCount: DWORD, Buffer: *mut LPVOID) -> NET_API_STATUS;
    pub fn NetApiBufferFree(Buffer: LPVOID) -> NET_API_STATUS;
    pub fn NetApiBufferReallocate(
        OldBuffer: LPVOID, NewByteCount: DWORD, NewBuffer: *mut LPVOID,
    ) -> NET_API_STATUS;
    pub fn NetApiBufferSize(Buffer: LPVOID, ByteCount: LPDWORD) -> NET_API_STATUS;
    pub fn NetAuditClear(server: LPCWSTR, backupfile: LPCWSTR, service: LPCWSTR) -> NET_API_STATUS;
    pub fn NetAuditRead(
        server: LPCWSTR, service: LPCWSTR, auditloghandle: LPHLOG, offset: DWORD,
        reserved1: LPDWORD, reserved2: DWORD, offsetflag: DWORD, bufptr: *mut LPBYTE,
        prefmaxlen: DWORD, bytesread: LPDWORD, totalavailable: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetAuditWrite(
        _type: DWORD, buf: LPBYTE, numbytes: DWORD, service: LPCWSTR, reserved: LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetConfigGet(
        server: LPCWSTR, component: LPCWSTR, parameter: LPCWSTR, bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetConfigGetAll(
        server: LPCWSTR, component: LPCWSTR, bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetConfigSet(
        server: LPCWSTR, reserved1: LPCWSTR, component: LPCWSTR, level: DWORD, reserved2: DWORD,
        buf: LPBYTE, reserved3: DWORD,
    ) -> NET_API_STATUS;
    pub fn NetConnectionEnum(
        servername: LMSTR, qualifier: LMSTR, level: DWORD, bufptr: *mut LPBYTE, prefmaxlen: DWORD,
        entriesread: LPDWORD, totalentries: LPDWORD, resume_handle: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetCreateProvisioningPackage(
        pProvisioningParams: PNETSETUP_PROVISIONING_PARAMS, ppPackageBinData: *mut PBYTE,
        pdwPackageBinDataSize: *mut DWORD, ppPackageTextData: *mut LPWSTR,
    ) -> NET_API_STATUS;
    pub fn NetDfsAdd(
        DfsEntryPath: LPWSTR, ServerName: LPWSTR, ShareName: LPWSTR, Comment: LPWSTR, Flags: DWORD,
    ) -> NET_API_STATUS;
    pub fn NetDfsAddFtRoot(
        ServerName: LPWSTR, RootShare: LPWSTR, FtDfsName: LPWSTR, Comment: LPWSTR, Flags: DWORD,
    ) -> NET_API_STATUS;
    pub fn NetDfsAddRootTarget(
        pDfsPath: LPWSTR, pTargetPath: LPWSTR, MajorVersion: ULONG, pComment: LPWSTR, Flags: ULONG,
    ) -> NET_API_STATUS;
    pub fn NetDfsAddStdRoot(
        ServerName: LPWSTR, RootShare: LPWSTR, Comment: LPWSTR, Flags: DWORD,
    ) -> NET_API_STATUS;
    pub fn NetDfsEnum(
        DfsName: LPWSTR, Level: DWORD, PrefMaxLen: DWORD, Buffer: *mut LPBYTE,
        EntriesRead: LPDWORD, ResumeHandle: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetDfsGetClientInfo(
        DfsEntryPath: LPWSTR, ServerName: LPWSTR, ShareName: LPWSTR, Level: DWORD,
        Buffer: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetDfsGetFtContainerSecurity(
        DomainName: LPWSTR, SecurityInformation: SECURITY_INFORMATION,
        ppSecurityDescriptor: *mut PSECURITY_DESCRIPTOR, lpcbSecurityDescriptor: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetDfsGetInfo(
        DfsEntryPath: LPWSTR, ServerName: LPWSTR, ShareName: LPWSTR, Level: DWORD,
        Buffer: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetDfsGetSecurity(
        DfsEntryPath: LPWSTR, SecurityInformation: SECURITY_INFORMATION,
        ppSecurityDescriptor: *mut PSECURITY_DESCRIPTOR, lpcbSecurityDescriptor: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetDfsGetStdContainerSecurity(
        MachineName: LPWSTR, SecurityInformation: SECURITY_INFORMATION,
        ppSecurityDescriptor: *mut PSECURITY_DESCRIPTOR, lpcbSecurityDescriptor: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetDfsGetSupportedNamespaceVersion(
        Origin: DFS_NAMESPACE_VERSION_ORIGIN, pName: PWSTR,
        ppVersionInfo: *mut PDFS_SUPPORTED_NAMESPACE_VERSION_INFO,
    ) -> NET_API_STATUS;
    pub fn NetDfsMove(
        OldDfsEntryPath: LPWSTR, NewDfsEntryPath: LPWSTR, Flags: ULONG,
    ) -> NET_API_STATUS;
    pub fn NetDfsRemove(
        DfsEntryPath: LPWSTR, ServerName: LPWSTR, ShareName: LPWSTR,
    ) -> NET_API_STATUS;
    pub fn NetDfsRemoveFtRoot(
        ServerName: LPWSTR, RootShare: LPWSTR, FtDfsName: LPWSTR, Flags: DWORD,
    ) -> NET_API_STATUS;
    pub fn NetDfsRemoveFtRootForced(
        DomainName: LPWSTR, ServerName: LPWSTR, RootShare: LPWSTR, FtDfsName: LPWSTR, Flags: DWORD,
    ) -> NET_API_STATUS;
    pub fn NetDfsRemoveRootTarget(
        pDfsPath: LPWSTR, pTargetPath: LPWSTR, Flags: DWORD,
    ) -> NET_API_STATUS;
    pub fn NetDfsRemoveStdRoot(
        ServerName: LPWSTR, RootShare: LPWSTR, Flags: DWORD,
    ) -> NET_API_STATUS;
    pub fn NetDfsSetClientInfo(
        DfsEntryPath: LPWSTR, ServerName: LPWSTR, ShareName: LPWSTR, Level: DWORD, Buffer: LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetDfsSetFtContainerSecurity(
        DomainName: LPWSTR, SecurityInformation: SECURITY_INFORMATION,
        pSecurityDescriptor: PSECURITY_DESCRIPTOR,
    ) -> NET_API_STATUS;
    pub fn NetDfsSetInfo(
        DfsEntryPath: LPWSTR, ServerName: LPWSTR, ShareName: LPWSTR, Level: DWORD, Buffer: LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetDfsSetSecurity(
        DfsEntryPath: LPWSTR, SecurityInformation: SECURITY_INFORMATION,
        pSecurityDescriptor: PSECURITY_DESCRIPTOR,
    ) -> NET_API_STATUS;
    pub fn NetDfsSetStdContainerSecurity(
        MachineName: LPWSTR, SecurityInformation: SECURITY_INFORMATION,
        pSecurityDescriptor: PSECURITY_DESCRIPTOR,
    ) -> NET_API_STATUS;
    pub fn NetEnumerateComputerNames(
        Server: LPCWSTR, NameType: NET_COMPUTER_NAME_TYPE, Reserved: ULONG, EntryCount: PDWORD,
        ComputerNames: *mut *mut LPWSTR,
    ) -> NET_API_STATUS;
    pub fn NetEnumerateServiceAccounts(
        ServerName: LPWSTR, Flags: DWORD, AccountsCount: *mut DWORD, Accounts: *mut PZPWSTR,
    ) -> NTSTATUS;
    pub fn NetErrorLogClear(
        UncServerName: LPCWSTR, BackupFile: LPCWSTR, Reserved: LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetErrorLogRead(
        UncServerName: LPCWSTR, Reserved1: LPWSTR, ErrorLogHandle: LPHLOG, Offset: DWORD,
        Reserved2: LPDWORD, Reserved3: DWORD, OffsetFlag: DWORD, BufPtr: *mut LPBYTE,
        PrefMaxSize: DWORD, BytesRead: LPDWORD, TotalAvailable: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetErrorLogWrite(
        Reserved1: LPBYTE, Code: DWORD, Component: LPCWSTR, Buffer: LPBYTE, NumBytes: DWORD,
        MsgBuf: LPBYTE, StrCount: DWORD, Reserved2: LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetFileClose(servername: LMSTR, fileid: DWORD) -> NET_API_STATUS;
    pub fn NetFileEnum(
        servername: LMSTR, basepath: LMSTR, username: LMSTR, level: DWORD, bufptr: *mut LPBYTE,
        prefmaxlen: DWORD, entriesread: LPDWORD, totalentries: LPDWORD, resume_handle: PDWORD_PTR,
    ) -> NET_API_STATUS;
    pub fn NetFileGetInfo(
        servername: LMSTR, fileid: DWORD, level: DWORD, bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetGetAnyDCName(
        servername: LPCWSTR, domainname: LPCWSTR, bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetGetDCName(
        servername: LPCWSTR, domainname: LPCWSTR, bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetGetDisplayInformationIndex(
        ServerName: LPCWSTR, Level: DWORD, Prefix: LPCWSTR, Index: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetGetJoinInformation(
        lpServer: LPCWSTR, lpNameBuffer: *mut LPWSTR, BufferType: PNETSETUP_JOIN_STATUS,
    ) -> NET_API_STATUS;
    pub fn NetGetJoinableOUs(
        lpServer: LPCWSTR, lpDomain: LPCWSTR, lpAccount: LPCWSTR, lpPassword: LPCWSTR,
        OUCount: *mut DWORD, OUs: *mut *mut LPWSTR,
    ) -> NET_API_STATUS;
    pub fn NetGroupAdd(
        servername: LPCWSTR, level: DWORD, buf: LPBYTE, parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetGroupAddUser(
        servername: LPCWSTR, GroupName: LPCWSTR, username: LPCWSTR,
    ) -> NET_API_STATUS;
    pub fn NetGroupDel(servername: LPCWSTR, groupname: LPCWSTR) -> NET_API_STATUS;
    pub fn NetGroupDelUser(
        servername: LPCWSTR, GroupName: LPCWSTR, Username: LPCWSTR,
    ) -> NET_API_STATUS;
    pub fn NetGroupEnum(
        servername: LPCWSTR, level: DWORD, bufptr: *mut LPBYTE, prefmaxlen: DWORD,
        entriesread: LPDWORD, totalentries: LPDWORD, resume_handle: PDWORD_PTR,
    ) -> NET_API_STATUS;
    pub fn NetGroupGetInfo(
        servername: LPCWSTR, groupname: LPCWSTR, level: DWORD, bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetGroupGetUsers(
        servername: LPCWSTR, groupname: LPCWSTR, level: DWORD, bufptr: *mut LPBYTE,
        prefmaxlen: DWORD, entriesread: LPDWORD, totalentries: LPDWORD, ResumeHandle: PDWORD_PTR,
    ) -> NET_API_STATUS;
    pub fn NetGroupSetInfo(
        servername: LPCWSTR, groupname: LPCWSTR, level: DWORD, buf: LPBYTE, parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetGroupSetUsers(
        servername: LPCWSTR, groupname: LPCWSTR, level: DWORD, buf: LPBYTE, totalentries: DWORD,
    ) -> NET_API_STATUS;
    pub fn NetIsServiceAccount(
        ServerName: LPWSTR, AccountName: LPWSTR, IsService: *mut BOOL,
    ) -> NTSTATUS;
    pub fn NetJoinDomain(
        lpServer: LPCWSTR, lpDomain: LPCWSTR, lpMachineAccountOU: LPCWSTR, lpAccount: LPCWSTR,
        lpPassword: LPCWSTR, fJoinOptions: DWORD,
    ) -> NET_API_STATUS;
    pub fn NetLocalGroupAdd(
        servername: LPCWSTR, level: DWORD, buf: LPBYTE, parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetLocalGroupAddMember(
        servername: LPCWSTR, groupname: LPCWSTR, membersid: PSID,
    ) -> NET_API_STATUS;
    pub fn NetLocalGroupAddMembers(
        servername: LPCWSTR, groupname: LPCWSTR, level: DWORD, buf: LPBYTE, totalentries: DWORD,
    ) -> NET_API_STATUS;
    pub fn NetLocalGroupDel(servername: LPCWSTR, groupname: LPCWSTR) -> NET_API_STATUS;
    pub fn NetLocalGroupDelMember(
        servername: LPCWSTR, groupname: LPCWSTR, membersid: PSID,
    ) -> NET_API_STATUS;
    pub fn NetLocalGroupDelMembers(
        servername: LPCWSTR, groupname: LPCWSTR, level: DWORD, buf: LPBYTE, totalentries: DWORD,
    ) -> NET_API_STATUS;
    pub fn NetLocalGroupEnum(
        servername: LPCWSTR, level: DWORD, bufptr: *mut LPBYTE, prefmaxlen: DWORD,
        entriesread: LPDWORD, totalentries: LPDWORD, resumehandle: PDWORD_PTR,
    ) -> NET_API_STATUS;
    pub fn NetLocalGroupGetInfo(
        servername: LPCWSTR, groupname: LPCWSTR, level: DWORD, bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetLocalGroupGetMembers(
        servername: LPCWSTR, localgroupname: LPCWSTR, level: DWORD, bufptr: *mut LPBYTE,
        prefmaxlen: DWORD, entriesread: LPDWORD, totalentries: LPDWORD, resumehandle: PDWORD_PTR,
    ) -> NET_API_STATUS;
    pub fn NetLocalGroupSetInfo(
        servername: LPCWSTR, groupname: LPCWSTR, level: DWORD, buf: LPBYTE, parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetLocalGroupSetMembers(
        servername: LPCWSTR, groupname: LPCWSTR, level: DWORD, buf: LPBYTE, totalentries: DWORD,
    ) -> NET_API_STATUS;
    // pub fn NetLogonGetTimeServiceParentDomain();
    pub fn NetMessageBufferSend(
        servername: LPCWSTR, msgname: LPCWSTR, fromname: LPCWSTR, buf: LPBYTE, buflen: DWORD,
    ) -> NET_API_STATUS;
    pub fn NetMessageNameAdd(servername: LPCWSTR, msgname: LPCWSTR) -> NET_API_STATUS;
    pub fn NetMessageNameDel(servername: LPCWSTR, msgname: LPCWSTR) -> NET_API_STATUS;
    pub fn NetMessageNameEnum(
        servername: LPCWSTR, level: DWORD, bufptr: *mut LPBYTE, prefmaxlen: DWORD,
        entriesread: LPDWORD, totalentries: LPDWORD, resumehandle: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetMessageNameGetInfo(
        servername: LPCWSTR, msgname: LPCWSTR, level: DWORD, bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetProvisionComputerAccount(
        lpDomain: LPCWSTR, lpMachineName: LPCWSTR, lpMachineAccountOU: LPCWSTR, lpDcName: LPCWSTR,
        dwOptions: DWORD, pProvisionBinData: *mut PBYTE, pdwProvisionBinDataSize: *mut DWORD,
        pProvisionTextData: *mut LPWSTR,
    ) -> NET_API_STATUS;
    pub fn NetQueryDisplayInformation(
        ServerName: LPCWSTR, Level: DWORD, Index: DWORD, EntriesRequested: DWORD,
        PreferredMaximumLength: DWORD, ReturnedEntryCount: LPDWORD, SortedBuffer: *mut PVOID,
    ) -> NET_API_STATUS;
    pub fn NetQueryServiceAccount(
        ServerName: LPWSTR, AccountName: LPWSTR, InfoLevel: DWORD, Buffer: *mut PBYTE,
    ) -> NTSTATUS;
    pub fn NetRemoteComputerSupports(
        UncServerName: LPCWSTR, OptionsWanted: DWORD, OptionsSupported: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetRemoteTOD(UncServerName: LPCWSTR, BufferPtr: *mut LPBYTE) -> NET_API_STATUS;
    pub fn NetRemoveAlternateComputerName(
        Server: LPCWSTR, AlternateName: LPCWSTR, DomainAccount: LPCWSTR,
        DomainAccountPassword: LPCWSTR, Reserved: ULONG,
    ) -> NET_API_STATUS;
    pub fn NetRemoveServiceAccount(
        ServerName: LPWSTR, AccountName: LPWSTR, Flags: DWORD,
    ) -> NTSTATUS;
    pub fn NetRenameMachineInDomain(
        lpServer: LPCWSTR, lpNewMachineName: LPCWSTR, lpAccount: LPCWSTR, lpPassword: LPCWSTR,
        fRenameOptions: DWORD,
    ) -> NET_API_STATUS;
    pub fn NetReplExportDirAdd(
        servername: LPCWSTR, level: DWORD, buf: LPBYTE, parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetReplExportDirDel(servername: LPCWSTR, dirname: LPCWSTR) -> NET_API_STATUS;
    pub fn NetReplExportDirEnum(
        servername: LPCWSTR, level: DWORD, bufptr: *mut LPBYTE, prefmaxlen: DWORD,
        entriesread: LPDWORD, totalentries: LPDWORD, resumehandle: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetReplExportDirGetInfo(
        servername: LPCWSTR, dirname: LPCWSTR, level: DWORD, bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetReplExportDirLock(servername: LPCWSTR, dirname: LPCWSTR) -> NET_API_STATUS;
    pub fn NetReplExportDirSetInfo(
        servername: LPCWSTR, dirname: LPCWSTR, level: DWORD, buf: LPBYTE, parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetReplExportDirUnlock(
        servername: LPCWSTR, dirname: LPCWSTR, unlockforce: DWORD,
    ) -> NET_API_STATUS;
    pub fn NetReplGetInfo(
        servername: LPCWSTR, level: DWORD, bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetReplImportDirAdd(
        servername: LPCWSTR, level: DWORD, buf: LPBYTE, parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetReplImportDirDel(servername: LPCWSTR, dirname: LPCWSTR) -> NET_API_STATUS;
    pub fn NetReplImportDirEnum(
        servername: LPCWSTR, level: DWORD, bufptr: *mut LPBYTE, prefmaxlen: DWORD,
        entriesread: LPDWORD, totalentries: LPDWORD, resumehandle: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetReplImportDirGetInfo(
        servername: LPCWSTR, dirname: LPCWSTR, level: DWORD, bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetReplImportDirLock(servername: LPCWSTR, dirname: LPCWSTR) -> NET_API_STATUS;
    pub fn NetReplImportDirUnlock(
        servername: LPCWSTR, dirname: LPCWSTR, unlockforce: DWORD,
    ) -> NET_API_STATUS;
    pub fn NetReplSetInfo(
        servername: LPCWSTR, level: DWORD, buf: LPBYTE, parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetRequestOfflineDomainJoin(
        pProvisionBinData: *mut BYTE, cbProvisionBinDataSize: DWORD, dwOptions: DWORD,
        lpWindowsPath: LPCWSTR,
    ) -> NET_API_STATUS;
    pub fn NetRequestProvisioningPackageInstall(
        pPackageBinData: *mut BYTE, dwPackageBinDataSize: DWORD, dwProvisionOptions: DWORD,
        lpWindowsPath: LPCWSTR, pvReserved: PVOID,
    ) -> NET_API_STATUS;
    pub fn NetScheduleJobAdd(
        Servername: LPCWSTR, Buffer: LPBYTE, JobId: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetScheduleJobDel(
        Servername: LPCWSTR, MinJobId: DWORD, MaxJobId: DWORD,
    ) -> NET_API_STATUS;
    pub fn NetScheduleJobEnum(
        Servername: LPCWSTR, PointerToBuffer: *mut LPBYTE, PointerToBuffer: DWORD,
        EntriesRead: LPDWORD, TotalEntries: LPDWORD, ResumeHandle: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetScheduleJobGetInfo(
        Servername: LPCWSTR, JobId: DWORD, PointerToBuffer: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetServerAliasAdd(servername: LMSTR, level: DWORD, buf: LPBYTE) -> NET_API_STATUS;
    pub fn NetServerAliasDel(servername: LMSTR, level: DWORD, buf: LPBYTE) -> NET_API_STATUS;
    pub fn NetServerAliasEnum(
        servername: LMSTR, level: DWORD, bufptr: *mut LPBYTE, prefmaxlen: DWORD,
        entriesread: LPDWORD, totalentries: LPDWORD, resumehandle: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetServerComputerNameAdd(
        ServerName: LMSTR, EmulatedDomainName: LMSTR, EmulatedServerName: LMSTR,
    ) -> NET_API_STATUS;
    pub fn NetServerComputerNameDel(
        ServerName: LMSTR, EmulatedServerName: LMSTR,
    ) -> NET_API_STATUS;
    pub fn NetServerDiskEnum(
        servername: LMSTR, level: DWORD, bufptr: *mut LPBYTE, prefmaxlen: DWORD,
        entriesread: LPDWORD, totalentries: LPDWORD, resumehandle: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetServerEnum(
        servername: LMSTR, level: DWORD, bufptr: *mut LPBYTE, prefmaxlen: DWORD,
        entriesread: LPDWORD, totalentries: LPDWORD, servertype: DWORD, domain: LMCSTR,
        resumehandle: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetServerGetInfo(
        servername: LMSTR, level: DWORD, bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetServerSetInfo(
        servername: LMSTR, level: DWORD, buf: LPBYTE, ParmError: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetServerTransportAdd(
        servername: LMSTR, level: DWORD, bufptr: LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetServerTransportAddEx(
        servername: LMSTR, level: DWORD, bufptr: LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetServerTransportDel(
        servername: LMSTR, level: DWORD, bufptr: LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetServerTransportEnum(
        servername: LMSTR, level: DWORD, bufptr: *mut LPBYTE, prefmaxlen: DWORD,
        entriesread: LPDWORD, totalentries: LPDWORD, resumehandle: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetServiceControl(
        servername: LPCWSTR, service: LPCWSTR, opcode: DWORD, arg: DWORD, bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetServiceEnum(
        servername: LPCWSTR, level: DWORD, bufptr: *mut LPBYTE, prefmaxlen: DWORD,
        entriesread: LPDWORD, totalentries: LPDWORD, resumehandle: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetServiceGetInfo(
        servername: LPCWSTR, service: LPCWSTR, level: DWORD, bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetServiceInstall(
        servername: LPCWSTR, service: LPCWSTR, argc: DWORD, argv: *mut LPCWSTR,
        bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetSessionDel(
        servername: LMSTR, UncClientName: LMSTR, username: LMSTR,
    ) -> NET_API_STATUS;
    pub fn NetSessionEnum(
        servername: LMSTR, UncClientName: LMSTR, username: LMSTR, level: DWORD,
        bufptr: *mut LPBYTE, prefmaxlen: DWORD, entriesread: LPDWORD, totalentries: LPDWORD,
        resumehandle: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetSessionGetInfo(
        servername: LMSTR, UncClientName: LMSTR, username: LMSTR, level: DWORD,
        bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetSetPrimaryComputerName(
        Server: LPCWSTR, PrimaryName: LPCWSTR, DomainAccount: LPCWSTR,
        DomainAccountPassword: LPCWSTR, Reserved: ULONG,
    ) -> NET_API_STATUS;
    pub fn NetShareAdd(
        servername: LMSTR, level: DWORD, buf: LPBYTE, parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetShareCheck(servername: LMSTR, device: LMSTR, _type: LPDWORD) -> NET_API_STATUS;
    pub fn NetShareDel(servername: LMSTR, netname: LMSTR, reserved: DWORD) -> NET_API_STATUS;
    pub fn NetShareDelEx(servername: LMSTR, level: DWORD, buf: LPBYTE) -> NET_API_STATUS;
    pub fn NetShareDelSticky(servername: LMSTR, netname: LMSTR, reserved: DWORD) -> NET_API_STATUS;
    pub fn NetShareEnum(
        servername: LMSTR, level: DWORD, bufptr: *mut LPBYTE, prefmaxlen: DWORD,
        entriesread: LPDWORD, totalentries: LPDWORD, resumehandle: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetShareEnumSticky(
        servername: LMSTR, level: DWORD, bufptr: *mut LPBYTE, prefmaxlen: DWORD,
        entriesread: LPDWORD, totalentries: LPDWORD, resumehandle: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetShareGetInfo(
        servername: LMSTR, netname: LMSTR, level: DWORD, bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetShareSetInfo(
        servername: LMSTR, netname: LMSTR, level: DWORD, buf: LPBYTE, parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetStatisticsGet(
        ServerName: LPCWSTR, Service: LPCWSTR, Level: DWORD, Options: DWORD, Buffer: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetUnjoinDomain(
        lpServer: LPCWSTR, lpAccount: LPCWSTR, lpPassword: LPCWSTR, fUnjoinOptions: DWORD,
    ) -> NET_API_STATUS;
    pub fn NetUseAdd(
        servername: LPWSTR, level: DWORD, buf: LPBYTE, parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetUseDel(UncServerName: LMSTR, UseName: LMSTR, ForceCond: DWORD) -> NET_API_STATUS;
    pub fn NetUseEnum(
        UncServerName: LMSTR, Level: DWORD, BufPtr: *mut LPBYTE, PreferedMaximumSize: DWORD,
        EntriesRead: LPDWORD, TotalEntries: LPDWORD, ResumeHandle: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetUseGetInfo(
        UncServerName: LMSTR, UseName: LMSTR, level: DWORD, bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetUserAdd(
        servername: LPCWSTR, level: DWORD, buf: LPBYTE, parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetUserChangePassword(
        domainname: LPCWSTR, username: LPCWSTR, oldpassword: LPCWSTR, newpassword: LPCWSTR,
    ) -> NET_API_STATUS;
    pub fn NetUserDel(servername: LPCWSTR, username: LPCWSTR) -> NET_API_STATUS;
    pub fn NetUserEnum(
        servername: LPCWSTR, level: DWORD, filter: DWORD, bufptr: *mut LPBYTE, prefmaxlen: DWORD,
        entriesread: LPDWORD, totalentries: LPDWORD, resumehandle: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetUserGetGroups(
        servername: LPCWSTR, username: LPCWSTR, level: DWORD, bufptr: *mut LPBYTE,
        prefmaxlen: DWORD, entriesread: LPDWORD, totalentries: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetUserGetInfo(
        servername: LPCWSTR, username: LPCWSTR, level: DWORD, bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetUserGetLocalGroups(
        servername: LPCWSTR, level: DWORD, flags: DWORD, bufptr: *mut LPBYTE, prefmaxlen: DWORD,
        entriesread: LPDWORD, totalentries: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetUserModalsGet(
        servername: LPCWSTR, level: DWORD, bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetUserModalsSet(
        servername: LPCWSTR, level: DWORD, buf: LPBYTE, parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetUserSetGroups(
        servername: LPCWSTR, username: LPCWSTR, level: DWORD, buf: LPBYTE, num_entries: DWORD,
    ) -> NET_API_STATUS;
    pub fn NetUserSetInfo(
        servername: LPCWSTR, username: LPCWSTR, level: DWORD, buf: LPBYTE, parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetValidateName(
        lpServer: LPCWSTR, lpName: LPCWSTR, lpAccount: LPCWSTR, lpPassword: LPCWSTR,
        NameType: NETSETUP_NAME_TYPE,
    ) -> NET_API_STATUS;
    pub fn NetValidatePasswordPolicy(
        ServerName: LPCWSTR, Qualifier: LPVOID, ValidationType: NET_VALIDATE_PASSWORD_TYPE,
        InputArg: LPVOID, OutputArg: *mut LPVOID,
    ) -> NET_API_STATUS;
    pub fn NetValidatePasswordPolicyFree(OutputArg: *mut LPVOID) -> NET_API_STATUS;
    pub fn NetWkstaGetInfo(servername: LMSTR, level: DWORD, bufptr: *mut LPBYTE) -> NET_API_STATUS;
    pub fn NetWkstaSetInfo(
        servername: LMSTR, level: DWORD, buffer: LPBYTE, parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetWkstaTransportAdd(
        servername: LPCWSTR, level: DWORD, buf: LPBYTE, parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetWkstaTransportDel(
        servername: LMSTR, transportname: LMSTR, ucond: DWORD,
    ) -> NET_API_STATUS;
    pub fn NetWkstaTransportEnum(
        servername: LPCWSTR, level: DWORD, bufptr: *mut LPBYTE, prefmaxlen: DWORD,
        entriesread: LPDWORD, totalentries: LPDWORD, resumehandle: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetWkstaUserEnum(
        servername: LMSTR, level: DWORD, bufptr: *mut LPBYTE, prefmaxlen: DWORD,
        entriesread: LPDWORD, totalentries: LPDWORD, resumehandle: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetWkstaUserGetInfo(
        reserved: LMSTR, level: DWORD, bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetWkstaUserSetInfo(
        reserved: LMSTR, level: DWORD, buf: LPBYTE, parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn Netbios(pncb: PNCB) -> UCHAR;
    // pub fn NlBindingSetAuthInfo();
}
