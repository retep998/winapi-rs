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
    // pub fn GetHostNameW();
    // pub fn GetNameInfoW();
    // pub fn InetNtopW();
    // pub fn InetPtonW();
    // pub fn SetAddrInfoExA();
    // pub fn SetAddrInfoExW();
    // pub fn WEP();
    // pub fn WPUCompleteOverlappedRequest();
    // pub fn WPUGetProviderPathEx();
    // pub fn WSAAccept();
    // pub fn WSAAddressToStringA();
    // pub fn WSAAddressToStringW();
    // pub fn WSAAdvertiseProvider();
    // pub fn WSAAsyncGetHostByAddr();
    // pub fn WSAAsyncGetHostByName();
    // pub fn WSAAsyncGetProtoByName();
    // pub fn WSAAsyncGetProtoByNumber();
    // pub fn WSAAsyncGetServByName();
    // pub fn WSAAsyncGetServByPort();
    // pub fn WSAAsyncSelect();
    // pub fn WSACancelAsyncRequest();
    // pub fn WSACancelBlockingCall();
    // pub fn WSACleanup();
    // pub fn WSACloseEvent();
    // pub fn WSAConnect();
    // pub fn WSAConnectByList();
    // pub fn WSAConnectByNameA();
    // pub fn WSAConnectByNameW();
    // pub fn WSACreateEvent();
    // pub fn WSADuplicateSocketA();
    // pub fn WSADuplicateSocketW();
    // pub fn WSAEnumNameSpaceProvidersA();
    // pub fn WSAEnumNameSpaceProvidersExA();
    // pub fn WSAEnumNameSpaceProvidersExW();
    // pub fn WSAEnumNameSpaceProvidersW();
    // pub fn WSAEnumNetworkEvents();
    // pub fn WSAEnumProtocolsA();
    // pub fn WSAEnumProtocolsW();
    // pub fn WSAEventSelect();
    // pub fn WSAGetLastError();
    // pub fn WSAGetOverlappedResult();
    // pub fn WSAGetQOSByName();
    // pub fn WSAGetServiceClassInfoA();
    // pub fn WSAGetServiceClassInfoW();
    // pub fn WSAGetServiceClassNameByClassIdA();
    // pub fn WSAGetServiceClassNameByClassIdW();
    // pub fn WSAHtonl();
    // pub fn WSAHtons();
    // pub fn WSAInstallServiceClassA();
    // pub fn WSAInstallServiceClassW();
    // pub fn WSAIoctl();
    // pub fn WSAIsBlocking();
    // pub fn WSAJoinLeaf();
    // pub fn WSALookupServiceBeginA();
    // pub fn WSALookupServiceBeginW();
    // pub fn WSALookupServiceEnd();
    // pub fn WSALookupServiceNextA();
    // pub fn WSALookupServiceNextW();
    // pub fn WSANSPIoctl();
    // pub fn WSANtohl();
    // pub fn WSANtohs();
    // pub fn WSAPoll();
    // pub fn WSAProviderCompleteAsyncCall();
    // pub fn WSAProviderConfigChange();
    // pub fn WSARecv();
    // pub fn WSARecvDisconnect();
    // pub fn WSARecvFrom();
    // pub fn WSARemoveServiceClass();
    // pub fn WSAResetEvent();
    // pub fn WSASend();
    // pub fn WSASendDisconnect();
    // pub fn WSASendMsg();
    // pub fn WSASendTo();
    // pub fn WSASetBlockingHook();
    // pub fn WSASetEvent();
    // pub fn WSASetLastError();
    // pub fn WSASetServiceA();
    // pub fn WSASetServiceW();
    // pub fn WSASocketA();
    // pub fn WSASocketW();
    // pub fn WSAStartup();
    // pub fn WSAStringToAddressA();
    // pub fn WSAStringToAddressW();
    // pub fn WSAUnadvertiseProvider();
    // pub fn WSAUnhookBlockingHook();
    // pub fn WSAWaitForMultipleEvents();
    // pub fn WSCDeinstallProvider();
    // pub fn WSCDeinstallProviderEx();
    // pub fn WSCEnableNSProvider();
    // pub fn WSCEnumProtocols();
    // pub fn WSCEnumProtocolsEx();
    // pub fn WSCGetApplicationCategory();
    // pub fn WSCGetApplicationCategoryEx();
    // pub fn WSCGetProviderInfo();
    // pub fn WSCGetProviderPath();
    // pub fn WSCInstallNameSpace();
    // pub fn WSCInstallNameSpaceEx();
    // pub fn WSCInstallNameSpaceEx2();
    // pub fn WSCInstallProvider();
    // pub fn WSCInstallProviderEx();
    // pub fn WSCSetApplicationCategory();
    // pub fn WSCSetApplicationCategoryEx();
    // pub fn WSCSetProviderInfo();
    // pub fn WSCUnInstallNameSpace();
    // pub fn WSCUnInstallNameSpaceEx2();
    // pub fn WSCUpdateProvider();
    // pub fn WSCUpdateProviderEx();
    // pub fn WSCWriteNameSpaceOrder();
    // pub fn WSCWriteProviderOrder();
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
    pub fn __WSAFDIsSet(fd: SOCKET, _: *mut fd_set) -> c_int;
    // pub fn accept();
    // pub fn bind();
    // pub fn closesocket();
    // pub fn connect();
    pub fn freeaddrinfo(pAddrInfo: PADDRINFOA);
    pub fn getaddrinfo(
        pNodeName: PCSTR, pServiceName: PCSTR, pHints: *const ADDRINFOA, ppResult: *mut PADDRINFOA,
    ) -> INT;
    // pub fn gethostbyaddr();
    // pub fn gethostbyname();
    // pub fn gethostname();
    // pub fn getnameinfo();
    // pub fn getpeername();
    // pub fn getprotobyname();
    // pub fn getprotobynumber();
    // pub fn getservbyname();
    // pub fn getservbyport();
    // pub fn getsockname();
    // pub fn getsockopt();
    // pub fn htonl();
    // pub fn htons();
    // pub fn inet_addr();
    // pub fn inet_ntoa();
    // pub fn inet_ntop();
    // pub fn inet_pton();
    // pub fn ioctlsocket();
    // pub fn listen();
    // pub fn ntohl();
    // pub fn ntohs();
    // pub fn recv();
    // pub fn recvfrom();
    // pub fn select();
    // pub fn send();
    // pub fn sendto();
    // pub fn setsockopt();
    // pub fn shutdown();
    // pub fn socket();
}
#[cfg(any(target_arch = "x86", target_arch = "arm"))]
extern "system" {
    // pub fn WSCInstallProviderAndChains();
}
#[cfg(target_arch = "x86_64")]
extern "system" {
    // pub fn WSCDeinstallProvider32();
    // pub fn WSCEnableNSProvider32();
    // pub fn WSCEnumNameSpaceProviders32();
    // pub fn WSCEnumNameSpaceProvidersEx32();
    // pub fn WSCEnumProtocols32();
    // pub fn WSCGetProviderInfo32();
    // pub fn WSCGetProviderPath32();
    // pub fn WSCInstallNameSpace32();
    // pub fn WSCInstallNameSpaceEx32();
    // pub fn WSCInstallProvider64_32();
    // pub fn WSCInstallProviderAndChains64_32();
    // pub fn WSCSetProviderInfo32();
    // pub fn WSCUnInstallNameSpace32();
    // pub fn WSCUpdateProvider32();
    // pub fn WSCWriteNameSpaceOrder32();
    // pub fn WSCWriteProviderOrder32();
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
