// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to httpapi.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn HttpAddFragmentToCache(
        ReqQueueHandle: HANDLE, pUrlPrefix: PCWSTR, pDataChunk: PHTTP_DATA_CHUNK,
        pCachePolicy: PHTTP_CACHE_POLICY, pOverlapped: LPOVERLAPPED,
    ) -> ULONG;
    pub fn HttpAddUrl(
        ReqQueueHandle: HANDLE, pFullyQualifiedUrl: PCWSTR, pReserved: PVOID,
    ) -> ULONG;
    pub fn HttpAddUrlToUrlGroup(
        UrlGroupId: HTTP_URL_GROUP_ID, pFullyQualifiedUrl: PCWSTR, UrlContext: HTTP_URL_CONTEXT,
        Reserved: ULONG,
    ) -> ULONG;
    pub fn HttpCancelHttpRequest(
        ReqQueueHandle: HANDLE, RequestId: HTTP_REQUEST_ID, pOverlapped: LPOVERLAPPED,
    ) -> ULONG;
    pub fn HttpCloseRequestQueue(ReqQueueHandle: HANDLE) -> ULONG;
    pub fn HttpCloseServerSession(ServerSessionId: HTTP_SERVER_SESSION_ID) -> ULONG;
    pub fn HttpCloseUrlGroup(UrlGroupId: HTTP_URL_GROUP_ID) -> ULONG;
    // pub fn HttpControlService();
    pub fn HttpCreateHttpHandle(pReqQueueHandle: HANDLE, Reserved: ULONG) -> ULONG;
    pub fn HttpCreateRequestQueue(
        Version: HTTPAPI_VERSION, pName: PCWSTR, pSecurityAttributes: PSECURITY_ATTRIBUTES,
        Flags: ULONG, pReqQueueHandle: PHANDLE,
    ) -> ULONG;
    pub fn HttpCreateServerSession(
        Version: HTTPAPI_VERSION, pServerSessionId: PHTTP_SERVER_SESSION_ID, Reserved: ULONG,
    ) -> ULONG;
    pub fn HttpCreateUrlGroup(
        ServerSessionId: HTTP_SERVER_SESSION_ID, pUrlGroupId: PHTTP_URL_GROUP_ID, Reserved: ULONG,
    ) -> ULONG;
    pub fn HttpDeleteServiceConfiguration(
        ServiceHandle: HANDLE, ConfigId: HTTP_SERVICE_CONFIG_ID, pConfigInformation: PVOID,
        ConfigInformationLength: ULONG, pOverlapped: LPOVERLAPPED,
    ) -> ULONG;
    // pub fn HttpEvaluateRequest();
    pub fn HttpFlushResponseCache(
        ReqQueueHandle: HANDLE, pUrlPrefix: PCWSTR, Flags: ULONG, pOverlapped: LPOVERLAPPED,
    ) -> ULONG;
    // pub fn HttpGetCounters();
    pub fn HttpInitialize(Version: HTTPAPI_VERSION, Flags: ULONG, pReserved: PVOID) -> ULONG;
    pub fn HttpPrepareUrl(
        Reserved: PVOID, Flags: ULONG, Url: PCWSTR, PreparedUrl: *mut PWSTR,
    ) -> ULONG;
    pub fn HttpQueryRequestQueueProperty(
        Handle: HANDLE, Property: HTTP_SERVER_PROPERTY, pPropertyInformation: PVOID,
        PropertyInformationLength: ULONG, Reserved: ULONG, pReturnLength: PULONG, pReserved: PVOID,
    ) -> ULONG;
    pub fn HttpQueryServerSessionProperty(
        ServerSessionId: HTTP_SERVER_SESSION_ID, Property: HTTP_SERVER_PROPERTY,
        pPropertyInformation: PVOID, PropertyInformationLength: ULONG, pReturnLength: PULONG,
    ) -> ULONG;
    pub fn HttpQueryServiceConfiguration(
        ServiceHandle: HANDLE, ConfigId: HTTP_SERVICE_CONFIG_ID, pInput: PVOID, InputLength: ULONG,
        pOutput: PVOID, OutputLength: ULONG, pReturnLength: PULONG, pOverlapped: LPOVERLAPPED,
    ) -> ULONG;
    pub fn HttpQueryUrlGroupProperty(
        UrlGroupId: HTTP_URL_GROUP_ID, Property: HTTP_SERVER_PROPERTY, pPropertyInformation: PVOID,
        PropertyInformationLength: ULONG, pReturnLength: PULONG,
    ) -> ULONG;
    pub fn HttpReadFragmentFromCache(
        ReqQueueHandle: HANDLE, pUrlPrefix: PCWSTR, pByteRange: PHTTP_BYTE_RANGE, pBuffer: PVOID,
        BufferLength: ULONG, pBytesRead: PULONG, pOverlapped: LPOVERLAPPED,
    ) -> ULONG;
    pub fn HttpReceiveClientCertificate(
        ReqQueueHandle: HANDLE, ConnectionId: HTTP_CONNECTION_ID, Flags: ULONG,
        pSslClientCertInfo: PHTTP_SSL_CLIENT_CERT_INFO, SslClientCertInfoSize: ULONG,
        pBytesReceived: PULONG, pOverlapped: LPOVERLAPPED,
    ) -> ULONG;
    pub fn HttpReceiveHttpRequest(
        ReqQueueHandle: HANDLE, RequestId: HTTP_REQUEST_ID, Flags: ULONG,
        pRequestBuffer: PHTTP_REQUEST, RequestBufferLength: ULONG, pBytesReturned: PULONG,
        pOverlapped: LPOVERLAPPED,
    ) -> ULONG;
    pub fn HttpReceiveRequestEntityBody(
        ReqQueueHandle: HANDLE, RequestId: HTTP_REQUEST_ID, Flags: ULONG, pBuffer: PVOID,
        EntityBufferLength: ULONG, pBytesReturned: PULONG, pOverlapped: LPOVERLAPPED,
    ) -> ULONG;
    pub fn HttpRemoveUrl(ReqQueueHandle: HANDLE, pFullyQualifiedUrl: PCWSTR) -> ULONG;
    pub fn HttpRemoveUrlFromUrlGroup(
        UrlGroupId: HTTP_URL_GROUP_ID, pFullyQualifiedUrl: PCWSTR, Flags: ULONG,
    ) -> ULONG;
    pub fn HttpSendHttpResponse(
        ReqQueueHandle: HANDLE, RequestId: HTTP_REQUEST_ID, Flags: ULONG,
        pHttpResponse: PHTTP_RESPONSE, pCachePolicy: PHTTP_CACHE_POLICY, pBytesSent: PULONG,
        pReserved1: PVOID, Reserved2: ULONG, pOverlapped: LPOVERLAPPED, pLogData: PHTTP_LOG_DATA,
    ) -> ULONG;
    pub fn HttpSendResponseEntityBody(
        ReqQueueHandle: HANDLE, RequestId: HTTP_REQUEST_ID, Flags: ULONG, EntityChunkCount: USHORT,
        pEntityChunks: PHTTP_DATA_CHUNK, pBytesSent: PULONG, pReserved1: PVOID, Reserved2: ULONG,
        pOverlapped: LPOVERLAPPED, pLogData: PHTTP_LOG_DATA,
    ) -> ULONG;
    pub fn HttpSetRequestQueueProperty(
        Handle: HANDLE, Property: HTTP_SERVER_PROPERTY, pPropertyInformation: PVOID,
        PropertyInformationLength: ULONG, Reserved: ULONG, pReserved: PVOID,
    ) -> ULONG;
    pub fn HttpSetServerSessionProperty(
        ServerSessionId: HTTP_SERVER_SESSION_ID, Property: HTTP_SERVER_PROPERTY,
        pPropertyInformation: PVOID, PropertyInformationLength: ULONG,
    ) -> ULONG;
    pub fn HttpSetServiceConfiguration(
        ServiceHandle: HANDLE, ConfigId: HTTP_SERVICE_CONFIG_ID, pConfigInformation: PVOID,
        ConfigInformationLength: ULONG, pOverlapped: LPOVERLAPPED,
    ) -> ULONG;
    pub fn HttpSetUrlGroupProperty(
        UrlGroupId: HTTP_URL_GROUP_ID, Property: HTTP_SERVER_PROPERTY, pPropertyInformation: PVOID,
        PropertyInformationLength: ULONG,
    ) -> ULONG;
    pub fn HttpShutdownRequestQueue(ReqQueueHandle: HANDLE) -> ULONG;
    pub fn HttpTerminate(Flags: ULONG, pReserved: PVOID) -> ULONG;
    pub fn HttpWaitForDemandStart(ReqQueueHandle: HANDLE, pOverlapped: LPOVERLAPPED) -> ULONG;
    pub fn HttpWaitForDisconnect(
        ReqQueueHandle: HANDLE, ConnectionId: HTTP_CONNECTION_ID, pOverlapped: LPOVERLAPPED,
    ) -> ULONG;
    pub fn HttpWaitForDisconnectEx(
        ReqQueueHandle: HANDLE, ConnectionId: HTTP_CONNECTION_ID, Reserved: ULONG,
        pOverlapped: LPOVERLAPPED,
    ) -> ULONG;
}
