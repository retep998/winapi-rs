// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to winhttp.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    // pub fn WinHttpAddRequestHeaders();
    // pub fn WinHttpAutoProxySvcMain();
    // pub fn WinHttpCheckPlatform();
    // pub fn WinHttpCloseHandle();
    // pub fn WinHttpConnect();
    // pub fn WinHttpCrackUrl();
    // pub fn WinHttpCreateProxyResolver();
    // pub fn WinHttpCreateUrl();
    // pub fn WinHttpDetectAutoProxyConfigUrl();
    // pub fn WinHttpFreeProxyResult();
    // pub fn WinHttpGetDefaultProxyConfiguration();
    // pub fn WinHttpGetIEProxyConfigForCurrentUser();
    // pub fn WinHttpGetProxyForUrl();
    // pub fn WinHttpGetProxyForUrlEx();
    // pub fn WinHttpGetProxyResult();
    // pub fn WinHttpOpen();
    // pub fn WinHttpOpenRequest();
    // pub fn WinHttpProbeConnectivity();
    // pub fn WinHttpQueryAuthSchemes();
    // pub fn WinHttpQueryDataAvailable();
    // pub fn WinHttpQueryHeaders();
    // pub fn WinHttpQueryOption();
    // pub fn WinHttpReadData();
    // pub fn WinHttpReceiveResponse();
    // pub fn WinHttpResetAutoProxy();
    // pub fn WinHttpSaveProxyCredentials();
    // pub fn WinHttpSendRequest();
    // pub fn WinHttpSetCredentials();
    // pub fn WinHttpSetDefaultProxyConfiguration();
    // pub fn WinHttpSetOption();
    // pub fn WinHttpSetStatusCallback();
    // pub fn WinHttpSetTimeouts();
    // pub fn WinHttpTimeFromSystemTime();
    // pub fn WinHttpTimeToSystemTime();
    // pub fn WinHttpWebSocketClose();
    // pub fn WinHttpWebSocketCompleteUpgrade();
    // pub fn WinHttpWebSocketQueryCloseStatus();
    // pub fn WinHttpWebSocketReceive();
    // pub fn WinHttpWebSocketSend();
    // pub fn WinHttpWebSocketShutdown();
    // pub fn WinHttpWriteData();
}
