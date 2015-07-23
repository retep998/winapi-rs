// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
#![cfg(windows)]
extern crate httpapi;
use httpapi::*;
#[inline(never)] fn bb<T>(_: T) {}
#[test]
fn functions() {
    bb(HttpAddFragmentToCache);
    bb(HttpAddUrl);
    bb(HttpAddUrlToUrlGroup);
    bb(HttpCancelHttpRequest);
    bb(HttpCloseRequestQueue);
    bb(HttpCloseServerSession);
    bb(HttpCloseUrlGroup);
    bb(HttpCreateHttpHandle);
    bb(HttpCreateRequestQueue);
    bb(HttpCreateServerSession);
    bb(HttpCreateUrlGroup);
    bb(HttpDeleteServiceConfiguration);
    bb(HttpFlushResponseCache);
    bb(HttpInitialize);
    // bb(HttpPrepareUrl); ------> Windows 8 API
    bb(HttpQueryRequestQueueProperty);
    bb(HttpQueryServerSessionProperty);
    bb(HttpQueryServiceConfiguration);
    bb(HttpQueryUrlGroupProperty);
    bb(HttpReadFragmentFromCache);
    bb(HttpReceiveClientCertificate);
    bb(HttpReceiveHttpRequest);
    bb(HttpReceiveRequestEntityBody);
    bb(HttpRemoveUrl);
    bb(HttpRemoveUrlFromUrlGroup);
    bb(HttpSendHttpResponse);
    bb(HttpSendResponseEntityBody);
    bb(HttpSetRequestQueueProperty);
    bb(HttpSetServerSessionProperty);
    bb(HttpSetServiceConfiguration);
    bb(HttpSetUrlGroupProperty);
    bb(HttpShutdownRequestQueue);
    bb(HttpTerminate);
    bb(HttpWaitForDemandStart);
    bb(HttpWaitForDisconnect);
    bb(HttpWaitForDisconnectEx);
}
