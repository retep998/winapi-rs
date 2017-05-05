// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to secur32.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    // pub fn GetComputerObjectNameA();
    // pub fn GetComputerObjectNameW();
    // pub fn GetSecurityUserInfo();
    // pub fn GetUserNameExA();
    // pub fn GetUserNameExW();
    // pub fn LsaCallAuthenticationPackage();
    // pub fn LsaConnectUntrusted();
    // pub fn LsaDeregisterLogonProcess();
    // pub fn LsaEnumerateLogonSessions();
    // pub fn LsaFreeReturnBuffer();
    // pub fn LsaGetLogonSessionData();
    // pub fn LsaLogonUser();
    // pub fn LsaLookupAuthenticationPackage();
    // pub fn LsaRegisterLogonProcess();
    // pub fn LsaRegisterPolicyChangeNotification();
    // pub fn LsaUnregisterPolicyChangeNotification();
    // pub fn SealMessage();
    // pub fn SeciAllocateAndSetCallFlags();
    // pub fn SeciAllocateAndSetIPAddress();
    // pub fn SeciFreeCallContext();
    // pub fn TranslateNameA();
    // pub fn TranslateNameW();
    // pub fn UnsealMessage();
}
