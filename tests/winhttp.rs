// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
#![cfg(windows)]
extern crate winhttp;
use winhttp::*;
#[inline(never)] fn bb<T>(_: T) {}
#[test]
fn functions() {
    bb(WinHttpAddRequestHeaders);
    bb(WinHttpCheckPlatform);
    bb(WinHttpCloseHandle);
    bb(WinHttpConnect);
    bb(WinHttpCrackUrl);
    bb(WinHttpCreateUrl);
    bb(WinHttpDetectAutoProxyConfigUrl);
    bb(WinHttpGetDefaultProxyConfiguration);
    bb(WinHttpGetIEProxyConfigForCurrentUser);
    bb(WinHttpGetProxyForUrl);
    bb(WinHttpOpen);
    bb(WinHttpOpenRequest);
    bb(WinHttpQueryAuthSchemes);
    bb(WinHttpQueryDataAvailable);
    bb(WinHttpQueryHeaders);
    bb(WinHttpQueryOption);
    bb(WinHttpReadData);
    bb(WinHttpReceiveResponse);
    bb(WinHttpSendRequest);
    bb(WinHttpSetCredentials);
    bb(WinHttpSetDefaultProxyConfiguration);
    bb(WinHttpSetOption);
    bb(WinHttpSetStatusCallback);
    bb(WinHttpSetTimeouts);
    bb(WinHttpTimeFromSystemTime);
    bb(WinHttpTimeToSystemTime);
    bb(WinHttpWriteData);
}
#[cfg(target_env = "msvc")]
#[test]
fn functions_msvc() {
    bb(WinHttpCreateProxyResolver);
    bb(WinHttpFreeProxyResult);
    bb(WinHttpGetProxyForUrlEx);
    bb(WinHttpGetProxyResult);
    bb(WinHttpResetAutoProxy);
    bb(WinHttpWebSocketClose);
    bb(WinHttpWebSocketCompleteUpgrade);
    bb(WinHttpWebSocketQueryCloseStatus);
    bb(WinHttpWebSocketReceive);
    bb(WinHttpWebSocketSend);
    bb(WinHttpWebSocketShutdown);
}
