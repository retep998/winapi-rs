// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
#![cfg(windows)]
extern crate credui;
use credui::*;
#[inline(never)] fn bb<T>(_: T) {}
#[test]
fn functions() {
    bb(CredPackAuthenticationBufferA);
    bb(CredPackAuthenticationBufferW);
    bb(CredUICmdLinePromptForCredentialsA);
    bb(CredUICmdLinePromptForCredentialsW);
    bb(CredUIConfirmCredentialsA);
    bb(CredUIConfirmCredentialsW);
    bb(CredUIParseUserNameA);
    bb(CredUIParseUserNameW);
    bb(CredUIPromptForCredentialsA);
    bb(CredUIPromptForCredentialsW);
    bb(CredUIPromptForWindowsCredentialsA);
    bb(CredUIPromptForWindowsCredentialsW);
    bb(CredUIReadSSOCredW);
    bb(CredUIStoreSSOCredW);
    bb(CredUnPackAuthenticationBufferA);
    bb(CredUnPackAuthenticationBufferW);
    // bb(SspiGetCredUIContext);
    // bb(SspiIsPromptingNeeded);
    // bb(SspiUnmarshalCredUIContext);
    // bb(SspiUpdateCredentials);
}
