// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to ws2_32.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn FreeAddrInfoEx(pAddrInfoEx: PADDRINFOEXA);
    pub fn FreeAddrInfoExW(pAddrInfoEx: PADDRINFOEXW);
    pub fn FreeAddrInfoW(pAddrInfo: PADDRINFOW);
    pub fn GetAddrInfoExA(
        pName: PCSTR, pServiceName: PCSTR, dwNameSpace: DWORD, lpNspId: LPGUID,
        hints: *const ADDRINFOEXA, ppResult: *mut PADDRINFOEXA, timeout: *mut timeval,
        lpOverlapped: LPOVERLAPPED, lpCompletionRoutine: LPLOOKUPSERVICE_COMPLETION_ROUTINE,
        lpNameHandle: LPHANDLE,
    ) -> INT;
    pub fn GetAddrInfoExCancel(lpHandle: LPHANDLE) -> INT;
    pub fn GetAddrInfoExOverlappedResult(lpOverlapped: LPOVERLAPPED) -> INT;
    pub fn GetAddrInfoExW(
        pName: PCWSTR, pServiceName: PCWSTR, dwNameSpace: DWORD, lpNspId: LPGUID,
        hints: *const ADDRINFOEXW, ppResult: *mut PADDRINFOEXW, timeout: *mut timeval,
        lpOverlapped: LPOVERLAPPED, lpCompletionRoutine: LPLOOKUPSERVICE_COMPLETION_ROUTINE,
        lpNameHandle: LPHANDLE,
    ) -> INT;
    pub fn GetAddrInfoW(
        pNodeName: PCWSTR, pServiceName: PCWSTR, pHints: *const ADDRINFOW,
        ppResult: *mut PADDRINFOW,
    ) -> INT;
    pub fn GetNameInfoW(
        pSockaddr: *const SOCKADDR, SockaddrLength: socklen_t, pNodeBuffer: PWCHAR,
        NodeBufferSize: DWORD, pServiceBuffer: PWCHAR, ServiceBufferSize: DWORD, Flags: INT,
    ) -> INT;
    pub fn InetNtopW(Family: INT, pAddr: PVOID, pStringBuf: PWSTR, StringBufSize: size_t) -> PCWSTR;
    pub fn InetPtonW(Family: INT, pszAddrString: PCWSTR, pAddrBuf: PVOID) -> INT;
    pub fn SetAddrInfoExA(
        pName: PCSTR, pServiceName: PCSTR, pAddresses: *mut SOCKET_ADDRESS, dwAddressCount: DWORD,
        lpBlob: LPBLOB, dwFlags: DWORD, dwNameSpace: DWORD, lpNspId: LPGUID, timeout: *mut timeval,
        lpOverlapped: LPOVERLAPPED, lpCompletionRoutine: LPLOOKUPSERVICE_COMPLETION_ROUTINE,
        lpNameHandle: LPHANDLE,
    ) -> INT;
    pub fn SetAddrInfoExW(
        pName: PCWSTR, pServiceName: PCWSTR, pAddresses: *mut SOCKET_ADDRESS, dwAddressCount: DWORD,
        lpBlob: LPBLOB, dwFlags: DWORD, dwNameSpace: DWORD, lpNspId: LPGUID, timeout: *mut timeval,
        lpOverlapped: LPOVERLAPPED, lpCompletionRoutine: LPLOOKUPSERVICE_COMPLETION_ROUTINE,
        lpNameHandle: LPHANDLE,
    ) -> INT;
    // pub fn WEP();
    pub fn WPUCompleteOverlappedRequest(
        s: SOCKET, lpOverlapped: LPWSAOVERLAPPED, dwError: DWORD, cbTransferred: DWORD,
        lpErrno: LPINT,
    ) -> c_int;
    // pub fn WPUGetProviderPathEx();
    // pub fn WSCDeinstallProviderEx();
    // pub fn WSCEnumProtocolsEx();
    // pub fn WSCGetApplicationCategoryEx();
    // pub fn WSCInstallNameSpaceEx2();
    // pub fn WSCInstallProviderEx();
    // pub fn WSCSetApplicationCategoryEx();
    // pub fn WSCUnInstallNameSpaceEx2();
    // pub fn WSCUpdateProviderEx();
    pub fn WSCWriteNameSpaceOrder(lpProviderId: LPGUID, dwNumberOfEntries: DWORD) -> c_int;
    pub fn WSCWriteProviderOrder(lpwdCatalogEntryId: LPDWORD, dwNumberOfEntries: DWORD) -> c_int;
    // pub fn WSCWriteProviderOrderEx();
    // pub fn WahCloseApcHelper();
    // pub fn WahCloseHandleHelper();
    // pub fn WahCloseNotificationHandleHelper();
    // pub fn WahCloseSocketHandle();
    // pub fn WahCloseThread();
    // pub fn WahCompleteRequest();
    // pub fn WahCreateHandleContextTable();
    // pub fn WahCreateNotificationHandle();
    // pub fn WahCreateSocketHandle();
    // pub fn WahDestroyHandleContextTable();
    // pub fn WahDisableNonIFSHandleSupport();
    // pub fn WahEnableNonIFSHandleSupport();
    // pub fn WahEnumerateHandleContexts();
    // pub fn WahInsertHandleContext();
    // pub fn WahNotifyAllProcesses();
    // pub fn WahOpenApcHelper();
    // pub fn WahOpenCurrentThread();
    // pub fn WahOpenHandleHelper();
    // pub fn WahOpenNotificationHandleHelper();
    // pub fn WahQueueUserApc();
    // pub fn WahReferenceContextByHandle();
    // pub fn WahRemoveHandleContext();
    // pub fn WahWaitForNotification();
    // pub fn WahWriteLSPEvent();
    pub fn freeaddrinfo(pAddrInfo: PADDRINFOA);
    pub fn getaddrinfo(
        pNodeName: PCSTR, pServiceName: PCSTR, pHints: *const ADDRINFOA, ppResult: *mut PADDRINFOA,
    ) -> INT;
    pub fn getnameinfo(
        pSockaddr: *const SOCKADDR, SockaddrLength: socklen_t, pNodeBuffer: PCHAR,
        NodeBufferSize: DWORD, pServiceBuffer: PCHAR, ServiceBufferSize: DWORD, Flags: INT,
    ) -> INT;
    pub fn inet_ntop(Family: INT, pAddr: PVOID, pStringBuf: PSTR, StringBufSize: size_t) -> PCSTR;
    pub fn inet_pton(Family: INT, pszAddrString: PCSTR, pAddrBuf: PVOID) -> INT;
}
#[cfg(target_arch = "x86_64")]
extern "system" {
    pub fn WSCWriteNameSpaceOrder32(lpProviderId: LPGUID, dwNumberOfEntries: DWORD) -> c_int;
    pub fn WSCWriteProviderOrder32(lpwdCatalogEntryId: LPDWORD, dwNumberOfEntries: DWORD) -> c_int;
}
extern {
    // pub static AddressFamilyInformation;
    // pub static eui48_broadcast;
    // pub static in4addr_alligmpv3routersonlink;
    // pub static in4addr_allnodesonlink;
    // pub static in4addr_allroutersonlink;
    // pub static in4addr_allteredohostsonlink;
    // pub static in4addr_any;
    // pub static in4addr_broadcast;
    // pub static in4addr_linklocalprefix;
    // pub static in4addr_loopback;
    // pub static in4addr_multicastprefix;
    // pub static in6addr_6to4prefix;
    // pub static in6addr_allmldv2routersonlink;
    // pub static in6addr_allnodesonlink;
    // pub static in6addr_allnodesonnode;
    // pub static in6addr_allroutersonlink;
    // pub static in6addr_any;
    // pub static in6addr_linklocalprefix;
    // pub static in6addr_loopback;
    // pub static in6addr_multicastprefix;
    // pub static in6addr_solicitednodemulticastprefix;
    // pub static in6addr_teredoinitiallinklocaladdress;
    // pub static in6addr_teredoprefix;
    // pub static in6addr_teredoprefix_old;
    // pub static in6addr_v4mappedprefix;
    // pub static scopeid_unspecified;
    // pub static sockaddr_size;
}
