// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
#![feature(test)]
#![cfg(windows)]
extern crate mpr;
extern crate test;
use mpr::*;
use test::black_box as bb;
#[test]
fn functions() {
    bb(MultinetGetConnectionPerformanceA);
    bb(MultinetGetConnectionPerformanceW);
    bb(WNetAddConnection2A);
    bb(WNetAddConnection2W);
    bb(WNetAddConnection3A);
    bb(WNetAddConnection3W);
    bb(WNetCancelConnectionA);
    bb(WNetCancelConnectionW);
    bb(WNetCancelConnection2A);
    bb(WNetCancelConnection2W);
    bb(WNetCloseEnum);
    bb(WNetConnectionDialog);
    bb(WNetConnectionDialog1A);
    bb(WNetConnectionDialog1W);
    bb(WNetDisconnectDialog);
    bb(WNetDisconnectDialog1A);
    bb(WNetDisconnectDialog1W);
    bb(WNetEnumResourceA);
    bb(WNetEnumResourceW);
    bb(WNetGetConnectionA);
    bb(WNetGetConnectionW);
    bb(WNetGetLastErrorA);
    bb(WNetGetLastErrorW);
    bb(WNetGetNetworkInformationA);
    bb(WNetGetNetworkInformationW);
    bb(WNetGetProviderNameA);
    bb(WNetGetProviderNameW);
    bb(WNetGetResourceInformationA);
    bb(WNetGetResourceInformationW);
    bb(WNetGetResourceParentA);
    bb(WNetGetResourceParentW);
    bb(WNetGetUniversalNameA);
    bb(WNetGetUniversalNameW);
    bb(WNetGetUserA);
    bb(WNetGetUserW);
    bb(WNetOpenEnumA);
    bb(WNetOpenEnumW);
    bb(WNetUseConnectionA);
    bb(WNetUseConnectionW);
}
